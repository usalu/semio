# W4-G report — the `APP_ID = "surface"` ownership-check bug

Lane 4-G. Lease: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` (the `🏗️builder` and the main
`🦀️component.rs`) — confirmed the `MUTATION-OUTCOMES-…` ticket that held these is `"status": "closed"`
with no `📌️important.md` before touching anything.

## 1. Reproduction

Lane 4-F's diagnosis (`📓️w4-f-report.md`): `EditorApp<E>`/`ViewerApp<E>`'s `ArtifactApp::APP_ID` is a
hardcoded `"surface"` placeholder, and `VcsArtifactApp::handle_action_invocation`'s ownership check
(`if address.app_id != A::APP_ID`) still compared against that literal, rejecting every real click —
`ShellHost` addresses every action with the surface's real canonical id
(`s.space.home@1/*#editor`-shaped), which can never equal `"surface"`.

Verified independently before writing any fix. New unit test
`handle_action_invocation_accepts_the_real_canonical_surface_app_id`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `SurfaceTestkit` region, next to
`editor_fixture_still_mutates_normally`) constructs an `EditorApp<SurfaceEditorFixture>`, computes the
real canonical id via `surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), AppRole::Editor)`, and dispatches
an `ActionInvocation` addressed with that real id. Run against the pre-fix code:

```
thread '...handle_action_invocation_accepts_the_real_canonical_surface_app_id' panicked:
the real canonical app id must satisfy the ownership check, got:
action app owner testkit.surface@1/*#editor does not match surface
```

Exact match to lane 4-F's live repro (`action app owner s.space.home@1/*#editor does not match
surface`). Confirms the defect and its mechanism precisely before any code changed.

## 2. The fix

`ArtifactApp::APP_ID` must stay a `&'static str` **const** — the trait requires it with no `&self`
receiver, so nothing about its type can change. But the real canonical id (`surface_app_id(dialect,
role)`) is only computable at runtime (it `format!()`s `E::DIALECT`'s fields, not a `const fn`), so
`EditorApp<E>`/`ViewerApp<V>` structurally cannot make `APP_ID` equal the real id. The fix adds a
second, receiver-taking seam instead of trying to force the const to be correct:

- **`ArtifactApp::instance_id(&self) -> &str`** (new trait method, line 9138 area, next to `APP_ID`'s
  own declaration) — defaults to `Self::APP_ID`. For every hand-written direct `ArtifactApp` impl
  (where the const genuinely already IS the real id — `document_app`, the testkit fixtures, etc.) this
  default is exactly correct and those apps needed zero changes.
- **`EditorApp<E>`/`ViewerApp<V>` override `instance_id`** to return `self.surface_id()` — the real
  derived id already computed once in `Default::default()` and stored in the `id: String` field (this
  machinery already existed from the `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket; lane 4-G only
  added the trait seam that lets the runtime actually read it for ownership).
- **Three ownership-relevant read sites switched from `A::APP_ID` to the instance value**:
  1. `impl PluginApp for VcsArtifactApp<A>::app_id(&self)` — was `A::APP_ID`, now
     `self.app.instance_id()`. This is the manifest/runtime-facing "what app am I" identity.
  2. `handle_action_invocation`'s `address.app_id != owner_app_id` check (line ~11774-11778) — **this is
     the exact line lane 4-F pinpointed**, the standard `onAction` path every button click goes through.
  3. `dispatch_command`'s two ownership checks (App-owned and Mode-owned typed-command addressing, line
     ~11592-11607) — same class of bug (`app_id != A::APP_ID`), same root cause, fixed for the same
     reason: `ShellHost` addresses typed commands with the real canonical id too, so this check would
     have failed identically the moment anything exercised the typed-command channel through a real
     surface. Not separately reported by lane 4-F (the collab scenario never got that far), but the code
     shape is identical and left unfixed it would have been the next wall.

**Why leaving `const APP_ID: &'static str = "surface"` in place is correct, not sloppy**: nothing reads
it for an ownership comparison anymore (every site that mattered was moved to `instance_id()`). The
const still has to exist to satisfy the trait's structural requirement, and there is no way to make it
equal the real id without either (a) making every `E: ArtifactEditor` author hand-copy their own already
-correct `surface_app_id` string as a redundant literal — reintroducing exactly the copy/paste drift this
surfaces migration was designed to eliminate — or (b) unsafe/leaky const-eval machinery in a generic
trait. `instance_id(&self)` is the minimal, receiver-shaped seam that actually can be correct, and it is
now the ONLY thing any runtime ownership check reads. I audited every remaining `A::APP_ID` occurrence in
the file: `DocumentCodecSpec::of`/`PluginBuilder::document_app_ids` (compile-time-only assembly
bookkeeping — a plugin's own declared-app-ids preflight, self-consistent since both sides read the same
const, never compared against a client-supplied address) and `VcsArtifactApp::with_registry`'s internal
document/config/draft/interaction envelope-id construction (cosmetic internal store ids, not part of any
ownership check). Both are already flagged by `EditorApp`'s own pre-existing doc comment as a distinct,
deferred future cleanup — out of scope for this defect and untouched here.

## 3. Regression evidence (no regressions found)

| Check | Baseline (📓️w4-d-report.md / 📓️w4-f-report.md) | After this fix |
|---|---|---|
| `cargo test -p semio-framework-plugin --lib` | 217 passed / 5 failed | **218 passed / 5 failed** — same 5 pre-existing failures (`identities_and_locales_are_explicit_and_conflicts_do_not_overwrite`, `plural_definition_carries_every_artifact_capability_without_a_dispatch_edit`, `registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically`, `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`, `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads`, all attributed to other tickets per `📓️w4-d-report.md`), +1 for the new reproduction test. Log: `🧪️4-g-framework-plugin-lib-test.txt`. |
| `cargo test -p semio-s-plugin-space --lib` | 210 passed / 0 failed | **210 passed / 0 failed** — exact match. Log: `🧪️4-g-space-lib-test.txt`. |
| `cargo check -p semio-s-plugin-dag` | compiles | **clean** (`🧪️4-g-check-dag.txt`) |
| `cargo check -p semio-s-plugin-norm` | compiles | **clean** (`🧪️4-g-check-norm.txt`) |
| `cargo check -p semio-s-plugin-writer` | compiles | **clean** (`🧪️4-g-check-writer.txt`) |
| `cargo check -p semio-s-plugin-puzzle` | compiles | **clean** (`🧪️4-g-check-puzzle.txt`) |

## 4. `bun ./📜️script.ts verify collab` — before/after per-step

The fix touches `semio-framework-plugin`, which nearly every plugin depends on, so this run rebuilt all
59 plugin crates from scratch (`[DEBUG] program build scope: all (59 plugin crates)`); the run completed
in one blocking call. Full log: `🧪️4-g-collab-e2e-run1.txt` (25,716 lines, almost entirely build output).
Screenshots from this run: `🧪️3-c-step1-user1.png` / `🧪️3-c-step1-user2.png` (the harness's
`collabScreenshot` helper writes to a fixed `3-c`-prefixed name regardless of caller lane — both
timestamped this run, Aug 17 06:28).

| Step | Baseline (0/8 — lane 3-C `🧪️3-c-collab-e2e.txt`, reconfirmed by lane 4-F `🧪️4-f-collab-e2e-run2.txt`) | After this fix (`🧪️4-g-collab-e2e-run1.txt`) |
|---|---|---|
| 1 create space | FAIL — click blocked/dispatch rejected: `action app owner s.space.home@1/*#editor does not match surface` (the `APP_ID` bug) | **FAIL, but past the bug**: `timeout waiting for a new space: row`. Dialog opens, `#name`/`kind`/`visibility` fill and submit all succeed with no error (screenshots show a clean table, dialog closed, no error banner) — the ownership check no longer rejects the dispatch. The created space's event never reaches either browser's read model within the 30s budget. See analysis below. |
| 2 share + open | FAIL — skipped (no space id) | FAIL — skipped (no space id from STEP 1) |
| 3 create artifact | FAIL — skipped/palette had no item | FAIL — skipped (no space id from STEP 1) |
| 4 co-edit | FAIL — skipped | FAIL — skipped (no artifact id from STEP 3) |
| 5 presence | FAIL — 0/2 peers (no document ever opens) | FAIL — same, 0/2 peers — downstream of STEP 1, not independently new |
| 6 check-in | FAIL — skipped | FAIL — skipped |
| 7 admin connections | FAIL — `/admin/api/connections: []` | FAIL — same, `[]` — downstream of STEP 1 (no real sync session ever opens), not independently new |
| 8 hub restart | FAIL — skipped | FAIL — skipped |

**True count: 0/8, unchanged from baseline** — but STEP 1 now fails at a materially later, different
point in the pipeline. This is real forward progress from the fix (proven by the qualitatively different
error text and by the screenshots showing a fully-clean UI with no stuck dialog/error, where the pre-fix
screenshots showed the click either not landing or the dialog never opening at all), not yet enough to
flip any step to PASS.

## 5. STEP 1's new failure — diagnosis and attribution

Traced the full intended path for `createSpace` after my fix removes the ownership blocker:

1. `create_space::handle` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/…/🎮️commands/🌱create-space/🦀️component.rs`)
   emits `HostEffect::ReplayShellCommand { action_id: "os.directory.create-space", args }`.
2. `ShellHost`'s `applyHostEffects` (`🟦️component.tsx` ~line 2703-2722) recognizes
   `actionId.startsWith("os.directory.")`, maps it via `directoryCommandFromAction`, and posts
   `{ kind: "directory-command", requestId, command }` (wire-encoded) to the backbone worker.
3. `🟦️backbone-worker.ts`'s `submitDirectoryCommand` calls `DirectoryClient.command()` →
   `POST /directory/commands` on the hub.
4. Hub (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs::post_directory_commands` →
   `🌎️hub/📇️directory/🦀️component.rs::DirectoryService::execute`) persists the event and — I confirmed
   by reading the code — **does** broadcast it: `for event in &persisted { let _ =
   self.tx.send(DirectoryStreamMessage::Event { event: event.clone() }); }`, the same channel
   `/directory/ws` subscribers read from.
5. The worker's live stream callback (`openDirectory`'s `client.stream(...)`) should post
   `{ kind: "directory-message", message }` back to `ShellHost`.
6. `ShellHost`'s `onmessage` (line ~1145-1148) calls `dispatchDirectoryEventsRef.current([event])` →
   `dispatchDirectoryEventBatch` → `onActionRef.current({ action: "foldDirectoryEvents", args:
   { events } })` — a plain `onAction` dispatch, which (now that my fix lands) should reach
   `HomeCommand::FoldDirectoryEvents` and fold the new space into `SHomeSnapshot`, causing the row.

Every individual link in this chain reads as correctly wired on inspection (hub does broadcast; Home's
`foldDirectoryEvents` command exists and is registered — confirmed via grep,
`✏️s/🔌️plugins/🪐️space/…/home/…/✏️editor/🦀️component.rs:53,151,259`). Yet end-to-end it did not
produce a row for either browser within 30s. Two concrete, code-read findings that narrow — but do not
close — the remaining gap:

- **`ShellHost`'s `directory-command-result` handler discards its own success payload.**
  `🟦️component.tsx` ~line 1153-1156:
  ```ts
  if (message.kind === "directory-command-result") {
    if (!message.ok) console.error("[os-shell] directory command failed", message.requestId, message.error);
    return;
  }
  ```
  The worker's `submitDirectoryCommand` attaches `events: result.events` to this exact message on
  success, and `ShellHost` never folds them — the read model relies *entirely* on the live
  `directory-message` broadcast round-tripping back to the SAME client that issued the command. Per
  lane 2-C's own report (`📓️w2-c-report.md`, task 3: "zero optimistic local mutation — the read model
  only ever updates from `directory-message`/`foldDirectoryEvents`"), this was a **deliberate** design
  choice, not an oversight — but it makes the read model fragile to any timing/reconnect gap in the
  actor's own live subscription. Lane 2-C's report also states this path was "not observed working
  end-to-end this lane" because the `s.space` crate didn't compile yet at the time — meaning this may be
  the first time this exact path has ever been exercised for real, by this run.
- **The collab-e2e harness cannot see the answer.** It only captures `page.on("pageerror", …)`
  (uncaught exceptions), never `console.log`/`warn`/`error` — confirmed by reading
  `🧰️…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`'s three `page.on(...)` call sites (lines 2074,
  2761-2762, 3069). So the two most likely silent-failure branches in `ShellHost` —
  `console.warn("[os-shell] replayShellCommand: directory command dropped, no signed-in identity", …)`
  and `console.error("[os-shell] directory command failed", …)` — are both invisible in
  `🧪️4-g-collab-e2e-run1.txt` even if either fired. I cannot distinguish "identity wasn't ready yet",
  "the hub rejected the command", "the WS subscription wasn't open yet", or "the broadcast fired but
  something downstream of `dispatchDirectoryEventBatch` dropped it" without that capture.

**Attribution**: not the bug I was assigned, and not introduced by my fix — this is new territory the
fix unblocked, not a regression. `git log --date=iso` gives no useful attribution: the entire
`📇️directory` hub module and `ShellHost`'s directory lane are uncommitted, in-flight additions from this
same ticket (`git status --porcelain` shows them as `A`/`AM`, no prior commits to `--follow`), spread
across multiple concurrent lanes with the auto-commit system's messages carrying no per-lane
information (confirmed unreliable per this session's own memory notes). This is squarely hub +
`ShellHost` directory-lane territory — outside `🔌️plugin/**`/`🏗️builder`, and touching the hub crate is
outside this lease entirely.

**What would fix it** (for whoever picks this up next, not attempted here — out of lease and out of
this lane's scope):
1. Add `console.log`/`console.warn`/`console.error` capture to the collab-e2e harness alongside the
   existing `pageerror` listener (`🧰️…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`) so the next
   investigator can see which of the four candidate causes above actually fired.
2. Regardless of root cause, as defense-in-depth: fold `directory-command-result`'s own `events` field
   directly into the read model on the issuing client (mirroring the `directory-message` path) instead
   of relying solely on the live broadcast finding its way back to the same socket that issued the
   command — the current design is correct only if the WS subscription is guaranteed already-open and
   healthy at command-issue time, which a fresh page load racing identity bootstrap does not guarantee.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`:
  - `ArtifactApp` trait: added `fn instance_id(&self) -> &str { Self::APP_ID }` (default).
  - `EditorApp<E>`/`ViewerApp<V>`'s `impl ArtifactApp`: each gained `fn instance_id(&self) -> &str {
    self.surface_id() }`, overriding the default.
  - `impl PluginApp for VcsArtifactApp<A>::app_id`: `A::APP_ID` → `self.app.instance_id()`.
  - `handle_action_invocation`: ownership check now compares `address.app_id` against
    `self.app.instance_id()` instead of `A::APP_ID`.
  - `dispatch_command`: both the App-owned and Mode-owned ownership checks now compare against
    `self.app.instance_id()` instead of `A::APP_ID`.
  - New test `handle_action_invocation_accepts_the_real_canonical_surface_app_id` in the
    `SurfaceTestkit` → `surface_testkit_tests` region.

## What is NOT done

- **STEP 1 still fails**, now on a different, later gap in the directory-broadcast → read-model fold
  pipeline (§5 above) — hub + `ShellHost` directory-lane territory, out of this lane's lease. Not
  attempted.
- **STEPS 2-4, 6, 8** remain untested past their own logic (downstream of STEP 1).
- **STEP 5/7** remain consequences of STEP 1, not independently verified as broken or fixed.
- No `[DEBUG] ` temporary logging was left in any changed file.

Ticket not closed (coordinator owns that).

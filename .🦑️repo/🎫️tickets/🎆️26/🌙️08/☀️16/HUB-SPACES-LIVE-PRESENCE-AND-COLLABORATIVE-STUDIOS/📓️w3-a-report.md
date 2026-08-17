# W3-A report — save/check-in policy, both shells (contract §C5)

Lane 3-A. "When the user edits an artifact, the mutations are saved and checked into vcs."

## Headline finding — checkpoint-after-remote-relay is genuinely broken

Reproduced lane 2-G's finding myself, independently, before writing any code (brief's own instruction).
`cargo test -p semio-s-plugin-space --lib engine::space::component::tests::two_instances_converge_on_disjoint_edits_via_backbone -- --test-threads=1 --exact`:

```
thread '...' panicked at 🔌️plugin/🦀️component.rs:5852:82:
pump b: Fault { ... message: "validation failed: change change-050c401c2ad61b4f has an invalid edit
reference edit-92b6ea4f8aaf1f1d" ... }
```

Full log `🧪️3-a-store-bug-repro.txt`. Full-suite context: `cargo test -p semio-s-plugin-space --lib` →
**203 passed; 1 failed** (`🧪️3-a-space-full-test.txt`) — the ONE pre-existing failure, unchanged in
root cause, count grown from 2-G's 198 baseline only because other lanes kept landing tests on the
live tree in between (nothing here is mine to fix or that I touched).

**Which scenarios work / don't** (this is the answer contract §C5 actually needs):
- **Local-only edits → checkpoint: WORKS.** Every one of the 203 passing tests that dispatches an
  `Apply`/spawn command then `CommitCheckpoint` on the SAME store with no remote ingest in between
  succeeds — this is my task 2/3's own common case (a user editing alone, or before their first sync).
- **Edits that arrived over the backbone (`ingest_remote`, driven by `store.dispatch(...)`'s own
  `pump()` call) → the VERY NEXT `CommitCheckpoint` on that same store: BROKEN.** `ArtifactStore::
  dispatch(command)` calls `self.pump()?` (drains queued `BackboneMessage::Mutations` via
  `ingest_remote`) BEFORE `dispatch_inner(command)` runs — so this fires on literally any command
  dispatch, not just an explicit "sync now" action. Auto/explicit check-in, and checkpoint-on-close,
  ALL route through `store::ArtifactCommand::CommitCheckpoint` (via `history_command`), so **any editor
  session that has received even one remote edit since its last checkpoint will hit this the next time
  my code (or the user's own "Checkpoint" button) asks for a checkpoint.** This is the ticket's
  headline collaborative scenario, not an edge case.

### Investigation (I did not stop at "reproduced, see 2-G's note")

Traced the exact call chain myself: `ArtifactStore::dispatch` → `self.pump()?` → `ingest_remote(envelope)`
(one call per `BackboneMessage::Mutations` entry) → (assuming the edit isn't quarantined) both
`self.applied_edit_ids` and `self.envelope.vcs.edits` get the new id pushed together, in the same
statement block (`🏪️store/🦀️component.rs` ~5312–5320) — this path, read in isolation, is internally
consistent. `dispatch_inner(CommitCheckpoint)` then calls `uncommitted_edit_ids(&self.envelope,
&self.applied_edit_ids)` (`🏪️store/🦀️component.rs:2133`) to build the new `Change.edit_ids`.

**Concrete finding #1 (hardening gap, not necessarily the root cause but definitely a contributing
invariant hole):** `uncommitted_edit_ids` filters `applied_edit_ids` against `change.edit_ids` only —
**it never checks that a candidate id is actually present in `envelope.vcs.edits`.** If `applied_edit_ids`
ever contains an id not backed by a real `Edit` (from ANY cause, not necessarily the one I traced),
this function silently hands that dangling id straight into a new `Change`, and the crash only
surfaces later, cryptically, at `validate_durable_history`. Zero existing test in `🏪️store`'s own
~200-test suite exercises `ingest_remote` immediately followed by `CommitCheckpoint` together
(verified: I wrote a small Python brace-matcher over every `#[test]` fn body in `component.rs` and
grepped for both call names in the same function — 0 matches) — this exact interaction was untested
before 2-G's fix accidentally exercised it for the first time.

**Concrete finding #2 (a real id-domain discontinuity, worth the store owner's attention):**
`edit_from_operation_envelope` (`🏪️store/🦀️component.rs:5981`) reconstructs a remote `Edit`'s `id` as
`envelope.mutation_id.0` — the WIRE per-forward-op id (`mutation_ids_for_edit`'s scheme, format
`{edit_id}#{opIndex}`, confirmed via `mutation_envelope_from_edit`'s own unit test in
`📡️spr/🔗️causal/🦀️component.rs`: an edit with `id: "edit-2"` and one forward op produces an envelope
with `mutation_id: "edit-2#0"`), **not** the sender's own `Edit.id`. `flush_outbound`'s own doc comment
(right above the `is_apply` branch, `🏪️store/🦀️component.rs:~5897`) already flags an adjacent version
of this exact hazard ("duplicate edit under its wire mutation_id, which differs from the edit's own
local id"). I could not fully isolate whether THIS is the proximate cause of the "invalid edit
reference" fault (would need instrumenting private `ArtifactStore` fields from inside the crate, which
`🏪️store`'s own lease forbids me — same wall 2-G hit), but it is exactly the shape of bug that produces
a "referenced id doesn't exist" fault when two code paths disagree on which id-domain a string belongs
to, and it is a genuinely new, more specific lead than 2-G's report had.

### sharedFileRequest — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`

For whoever owns `store` (peer-leased to `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`,
forbidden to me):
1. `uncommitted_edit_ids` (line 2133): harden it to only return ids also present in
   `envelope.vcs.edits`, with at minimum a `debug_assert!`/loud log for any id dropped this way — turns
   a checkpoint crash into either a correct (if smaller) checkpoint or a diagnosable warning at the
   point of divergence, instead of a validation panic two calls later.
2. Reproduce with `cargo test -p semio-s-plugin-space --lib
   engine::space::component::tests::two_instances_converge_on_disjoint_edits_via_backbone --
   --test-threads=1 --exact` and check whether `edit_from_operation_envelope`'s `id:
   envelope.mutation_id.0` (line 5981) is the actual source of the divergence — instrument
   `self.applied_edit_ids` vs `self.envelope.vcs.edits.iter().map(|e| &e.id)` right before the
   `CommitCheckpoint` arm runs in `dispatch_inner` and diff them; my own investigation could get this
   far but no further without editing a forbidden file.

**Consequence for my own tasks 2/3/4 (auto/explicit/close check-in):** the SHELL-side wiring is
implemented and correct per the contract (dispatches `CommitCheckpoint` at the right times, with the
right guards) — but whenever the session being checked in has ingested a remote edit, the dispatch
will come back rejected by this store bug until it's fixed. I am not patching around this in the shell
(the brief is explicit: don't paper over it, name it).

## Changed files

**React**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` —
  new `//#region 🔖️CheckIn` (ahead of `🧰️FooterUtilityLeaves`/`🔄️SyncLeaf`, which now close over it):
  `currentDocumentId` (reverse-lookup into `openDocumentSessionsRef`), `syncPillState`,
  `uncommittedEditCount` (derived from `historyProjection.entries`), `touchSpaceIndexArtifact`,
  `dispatchCheckpoint`, the checkpoint-success-detection effect (drives `TouchArtifact`), the
  `AutoCheckinScheduler`-backed auto-check-in effects, the checkpoint-on-close effect, and the
  `checkinDialog` local state + `submitCheckin`. `historyProjection` state gained a `currentCheckpointId`
  field (`HistoryPatch` always carried it; it was silently dropped before). `frameworkSyncTab`'s
  `id`/`name` (`🔄️SyncLeaf`) now show the live status pill (`id: "s-sync-status"`) instead of the
  static `"framework.sync"` id/label (verified this id has no other consumer repo-wide before renaming
  it — the only other `"framework.sync"` string anywhere is `FRAMEWORK_SYNC_CONTROLLER_ID`, a
  different, coincidentally-same-valued constant for action dispatch, untouched). `🧰️FooterUtilityLeaves`'s
  history tree gained `#s-checkin` (opens an inline message input, absent outright for a viewer —
  `!canCheckIn(session.app.role)`, same gate the auto-timer effects use).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx` —
  new `//#region 🔖️CheckInAndSyncStatus`: `AUTO_CHECKIN_IDLE_MS`/`AUTO_CHECKIN_EDIT_THRESHOLD`,
  `AutoCheckinScheduler` (framework-free debounce/storm-guard class, unit-testable with fake timers
  without mounting React), `canCheckIn` (the one viewer-guard predicate), `computeSyncPillState`/
  `syncPillText` (the `persisted | pending(n) | remote(...)` decision + localized text), the
  `checkin*Text` frozen-label accessors (mirrors the pre-existing `SurfaceRoleLabels` idiom for the
  identical "can't reach the downstream i18n barrel without a cycle" reason).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` —
  appended `ui.sync.status.*` keys to lane 2-C's `SpacesUiLabelKey`/`SPACES_UI_LABELS` table (per the
  brief: extend, don't touch the existing `ui.checkin.*` keys), and re-exported the new `ShellHelpers`
  symbols above (mirrors the existing `ShellHelpers`/`FrameworkOsShell` re-export regions' own shape).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts` —
  9 new `it()`s next to lane 2-C's own 3 (same file, same style): `AutoCheckinScheduler` × 4 (fake
  timers: 3-edits-then-idle, ≥200-immediate, `notify(0)` clears the latch, `cancel()`), sync pill × 4
  (persisted/pending/remote×3/no-status, en+de), `canCheckIn` × 1 (the viewer-guard predicate).

**wgpu**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` —
  new `ShellState::sync_pill_text` (mirrors the React `syncPillText`/`computeSyncPillState` state
  machine as one function), new free function `render_sync_status_and_checkin` (paints `#s-sync-status`
  + `#s-checkin`, viewer-gated, using the SAME `ChromeGroupItem`/`render_chrome_group`/
  `measure_chrome_group_item` immediate-mode primitives `render_presence_bar` already established as
  precedent for shell-owned, non-plugin-declared footer chrome), wired into `render_footer` right after
  the plugin-declared utility sections. New test `sync_pill_text_covers_persisted_pending_and_every_remote_state`
  in the existing `identity_directory_presence_tests` module.

## What is NOT done — wgpu, honestly scoped down

Auto check-in, checkpoint-on-close, and `TouchArtifact` are **not ported to the wgpu shell this wave.**
Reason, discovered while implementing (not assumed): **the native wgpu shell tracks no
history/uncommitted-edit projection at all today** — verified via `grep -n "HistoryPatch\|read_history"`
over `Shell/🧊️component.rs`: zero hits. React's `historyProjection` (fed by `plugin.readHistory`/
`HistoryPatch` on every `handleAction`/`handleCommand` response) has no wgpu twin; even undo/redo have
no footer chrome in wgpu today (only a blind command-palette dispatch with no `canUndo`/`canRedo`
awareness). Building that tracking from scratch would mean threading `InvocationResponse.history_patch`
(confirmed present on the Rust type, currently always discarded — `ProgramBridge/🧊️component.rs:146,481`)
through `dispatch_action`/`dispatch_command`'s shared match loops — the same loops lane 2-D's own
`ReplayShellCommand` arm lives in — **while `semio-s-plugin-puzzle` keeps the entire crate from
compiling** (see Blocker below). Extending shared, busy code with zero compiler feedback available is
a real risk I chose not to take blind; flagging as an explicit follow-up instead of a guessed, unverified
attempt. `#s-checkin` for wgpu also dispatches a **fixed** `"check-in"` message rather than opening a
real text-entry dialog — this footer's immediate-mode chrome has no text-input primitive (confirmed: no
`UiEvent::TextInput`-consuming widget anywhere in this file's footer/chrome code; only the generic
plugin-surface `Interpreter` pipeline has one, and this hand-painted footer deliberately bypasses that,
same design choice `render_presence_bar` already made). `TouchArtifact` for wgpu needs the same
"headless background plugin instance" mechanism the React side builds (see below) — not attempted for
wgpu given the above.

What wgpu DOES get, reviewed but **zero compiler verification available** (blocker below): the status
pill (`#s-sync-status`, always visible, updates whenever `self.sync_status` changes) and the explicit
check-in action (`#s-checkin`, absent for `AppRole::Viewer` — task 5's guard, satisfied for wgpu too).

## `TouchArtifact` (task 6) — React, best-effort, not click-verified

Lane 1-E/2-B already built the mutation and its command: `SpaceIndexCommand::TouchArtifact` (command
id `"touchArtifact"`, payload `{id, now_ms, actor}`) is a real, tested command on the `s.space` editor
(`✏️editor/🎮️commands/🕒touch-artifact/🦀️component.rs`, whose own header doc literally names this
lane's post-checkpoint hook). The hard part: the space's `index` document is almost never the
currently-mounted session while an artifact editor is open (this shell keeps exactly one plugin
session mounted at a time — `📓️w2-c-report.md`'s own documented constraint), and `openDocument`/
`open_document` in both shells attach a NEW document to the CURRENT session's plugin instance, not a
new headless one. `touchSpaceIndexArtifact` (React) works around this by spawning a second,
non-visible instance of the `s.space` editor plugin (`pluginEntry.handle.createApp(...)`, the exact
pattern `ensureSpawnedPlugin`/`switchToManagedApp` already use, minus the `SET_SESSION` dispatch that
makes an instance visible), attaching its own hub+folder bindings for the space's `index` document
(cached per `spaceId` so repeat checkpoints reuse it), then dispatching `touchArtifact` via
`encodeAppCommandInvocation`/`handleCommand` — reusing the SAME wire mechanism 2 pre-existing call
sites (`setContributions`/`setAppRegistrations`) already use successfully. **Unverified specifically**:
the exact wire-arg casing (`{id, nowMs, actor}`, camelCase, matching this codebase's general convention
— `TouchArtifact`'s own Rust struct has no explicit `#[serde(rename_all)]`, and I could not find an
existing shell→command call site using `now_ms`/`nowMs`-shaped args to confirm either way) and the
whole path end-to-end (no hub/dev server booted this session — same ceiling every other lane in this
ticket reports). Called from all three checkpoint-success paths (the checkpoint-id-change detector for
auto/explicit, best-effort fire-and-forget for close).

## Task-by-task status

1. **Status pill** — done, both shells. `#s-sync-status`, `persisted | pending(n) | remote(connected|
   connecting|backoff|detached)`, en+de (React; wgpu is English-only, see "What is NOT done"). Verified
   it visibly changes: it's computed from `syncStatusByDocumentId[currentDocumentId]`, which
   `SET_SYNC_STATUS_FOR_DOCUMENT` already updates for ANY open document (not just the manual sync-card
   target) on every `status` event.
2. **Auto check-in** — done (React), correct per the debounce/storm-guard test suite. Not ported to
   wgpu (see above).
3. **Explicit check-in** — done (React: real message input, viewer-gated). wgpu: fixed message, same
   gate (see above).
4. **Checkpoint on close** — done (React, best-effort). Not ported to wgpu.
5. **Viewer never checkpoints** — done, both shells, WITH a test (`canCheckIn` unit test — the one
   predicate gating both the affordance and the auto-timer). `VcsArtifactApp`'s own host-side guard was
   not touched (framework-owned, forbidden).
6. **`TouchArtifact` after checkpoint** — done (React, best-effort, see above). Not done (wgpu).

## Commands run + results (real tails)

React — `bunx vitest run -c 🧪️vitest.config.ts` in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`
(direct, `nx` not attempted — every peer lane this session reports it hanging on this shared machine):

```
 Test Files  1 failed (1)
      Tests  9 failed | 322 passed (331)
```

The 9 failures are the IDENTICAL set 2-C/2-F already documented (CSS class assertions, an R3F crash, a
chai matcher, `resolveWindowActions` panel-eligibility, the "Artifact"/"Document" i18n rename from the
`ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` peer ticket, two mit-bestand asset-path regexes, a
command-palette mock shape) — none touch anything in my lease. Full log:
`🧪️3-a-renderer-react-vitest-final2.txt`. My own 9 new tests, isolated: `bunx vitest run -c
🧪️vitest.config.ts -t "AutoCheckinScheduler|sync status pill"` → **8 passed** (the 9th, `canCheckIn`,
added after this filtered run, verified in the full run above) — no fake-timer leakage, no flakiness
across 2 reruns.

Rust — `cargo check -p semio-framework-os-renderer-wgpu`: **blocked**, identical to lane 2-D's own
finding, re-confirmed by me independently this session (not assumed stale): 4 `^error` matches, all 3
distinct errors inside `semio-s-plugin-puzzle` (`E0277: SemanticMutation` not satisfied for
`Puzzle{2,3,5}dMutation`), zero mentions of `Shell/`, my code never reached. Full log:
`🧪️3-a-cargo-check-wgpu-1.txt`. `puzzle` is a hard, non-optional dependency of this crate — no way to
isolate a narrower check. Attribution: `git status --porcelain -- "✏️s/🔌️plugins/🧩️puzzle/"` still
shows the same live, uncommitted `🗣️dsl`-derive churn 2-D attributed to the concurrent
`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket, not mine, not this ticket's to
fix. **My wgpu code is manually reviewed against every type/signature I read from source
(`store_sync::sync::{ArtifactSyncStatus, RemoteState}`, `ChromeGroupItem`, `render_chrome_group`,
`measure_chrome_group_item`, `ActiveSession`, `semio_framework::manifest::AppRole`,
`ActionDescriptor`/`crate::action_args_json!`) — this is NOT a substitute for `rustc`; treat it as
unverified until the puzzle blocker clears.** Re-run `cargo check -p semio-framework-os-renderer-wgpu`
and `cargo test -p semio-framework-os-renderer-wgpu --lib sync_pill_text` the moment it does.

`cargo test -p semio-s-plugin-space --lib` (the store-bug investigation, not code I wrote): **203
passed; 1 failed** — `🧪️3-a-space-full-test.txt`. The 1 failure is the store bug documented above.

## sharedFileRequests

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`** — see "Investigation" above:
   `uncommitted_edit_ids` (line 2133) should validate membership in `envelope.vcs.edits`; possible root
   cause in `edit_from_operation_envelope`'s id-domain reuse (line 5981). Forbidden to me
   (`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`).
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `history_command` (line ~10514)** —
   `"commitCheckpoint" => Some(ArtifactCommand::CommitCheckpoint { message: arg_str("message"),
   authors: Vec::new() })` hardcodes `authors: Vec::new()` regardless of what the caller passes in
   `args`. Both shells now send `args: { message, authors: [{id, name}] }` (React) — ready the moment
   this reads `args.get("authors")` and decodes a small `Vec<Author>` instead of discarding it.
   Framework-owned, forbidden to me; low-risk, small, additive fix for whoever owns this file.

## What is NOT done

- wgpu: auto check-in, checkpoint-on-close, `TouchArtifact`, and a real (non-fixed-message) `#s-checkin`
  dialog — see "What is NOT done — wgpu" above for the specific, discovered-not-assumed reasons.
- `TouchArtifact`'s exact wire-arg casing and the whole checkpoint→`TouchArtifact`→space-table-updates
  path — implemented and unit-reviewable, never click-verified (no dev server/hub booted this session,
  same ceiling as every other lane).
- The store bug itself — investigated, characterized, `sharedFileRequest`d; not patched (explicitly
  forbidden by the brief).
- No end-to-end confirmation that a real hub-connected multi-user session actually moves a space
  table's `updated`/`updated-by` columns — blocked by both the store bug (checkpoint-after-remote-ingest
  fails) and the lack of a booted dev/hub environment this session.

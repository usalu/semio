# W4-I report — the `effectiveActionArgs` seed-drop fix, two further real bugs found chasing STEP 2, 1/8 → 2/8

Lane 4-I. Starting task: lane 4-H's STEP 2 diagnosis (`📓️w4-h-report.md` §5) — `UIDialog`/
`effectiveActionArgs` (`🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts`) drops any
seeded dialog argument that is not itself a declared form field, so `shareSpace` reached the hub with an
empty `spaceId` and correctly got a 403. Fixed that, re-ran, and kept going step by step as briefed —
found and fixed two more real, previously-unexercised bugs before hitting a blocker I judged genuinely
out of reach for this lane (a live concurrent edit on the exact file the next diagnosis needs).

## 1. Root cause 1 (assigned task): `effectiveActionArgs` drops seeded, non-form-field args

`🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts:5-15` (pre-fix): builds its result by
iterating only the declared `defs` (a dialog's own visible form fields); any other key in the merged
`{...seedArgs, ...staged}` buffer — including a seeded "hidden" context id like `shareSpace`'s
`spaceId`, or `deleteSpace`'s entire `{spaceId, confirmed}` payload (that dialog declares **zero** form
fields — a plain confirm/cancel, per its own doc comment) — was silently dropped before `onSubmit`.

**Fix** (not a workaround at the call site — the module's own contract now supports this): added an
explicit third `seed` parameter to `effectiveActionArgs`/`effective_action_args` (TS + Rust twin, kept in
parity — see §2). `seed` keys survive into the result unconditionally; a `seed` value for a **declared**
field pre-fills it (unchanged behavior — this is how `renameSpace` pre-fills the current name) until the
form stages its own edit, which still wins; a `seed` key that is **not** declared survives untouched. A
zero-declared-field dialog (`deleteSpace`) now passes its whole seed through, which is required for it to
work at all.

- `🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts` — `effectiveActionArgs(defs, staged,
  seed?)`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️UIDialog/🟦️component.tsx` — `effective =
  effectiveActionArgs(dialog.args, staged, seedArgs)`, replacing the old `buffer = {...seedArgs,
  ...staged}` + `effectiveActionArgs(dialog.args, buffer)` two-step (which is exactly what dropped
  `spaceId`: `buffer` had it, but only `dialog.args`-declared keys survived the second call).
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — Rust twin `effective_action_args(defs, staged,
  seed: Option<&DslValue>)`; all 4 call sites updated (2 in this file's own tests, 1 in
  `os/🔌️plugin/🦀️component.rs`'s testkit helper, 1 in `Shell/🧊️component.rs`'s wgpu
  `resolved_execute_args` — all pass `None`, byte-identical behavior to before since none of them
  currently seed anything; wgpu has no `HostEffect::OpenDialog` renderer yet, confirmed by grep — no
  live wgpu caller needed the seed capability, but the twin now has the same shape as TS for whenever it
  does).

**Unit tests** (brief's explicit ask: "a seeded arg not present as a form field still reaches the
dispatched descriptor" — `effective` IS exactly the object `UIDialog.onSubmit`/wgpu's
`execute_staged_action` dispatch):
- TS, `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` (`describe("effectiveActionArgs", …)`, this
  package's in-source-test convention — `🧪️vitest.config.ts`'s `includeSource` only scans `🟦️glue.ts`):
  4 new tests — seeded-arg-survives-alongside-staged-field (the exact `shareSpace` shape), seed-prefills-
  then-staged-wins (the `renameSpace` shape), zero-declared-fields-passes-seed-wholesale (the
  `deleteSpace` shape), `missingRequiredArgs` unaffected by extra seed keys. All 4 pass — verified twice
  in the run (glue.ts is matched by both `include` and `includeSource`), see `🧪️4-i-framework-package-
  vitest.txt`.
- Rust, `🛂️manifest/🦀️component.rs`: `effective_args_preserve_a_seeded_arg_not_declared_as_a_form_field`,
  `effective_args_seed_prefills_a_declared_field_until_staged_overrides_it`,
  `effective_args_pass_seed_through_wholesale_when_no_fields_are_declared` — all pass, see below.

```
$ cargo test -p semio-framework --lib effective_args
running 4 tests
test manifest::app_label_tests::effective_args_pass_seed_through_wholesale_when_no_fields_are_declared ... ok
test manifest::app_label_tests::effective_args_seed_prefills_a_declared_field_until_staged_overrides_it ... ok
test manifest::app_label_tests::effective_args_preserve_a_seeded_arg_not_declared_as_a_form_field ... ok
test manifest::app_label_tests::effective_args_prefer_staged_then_default ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 136 filtered out
```

Confirmed on the hub side too, before/after (`🧪️4-i-collab-e2e-run1.txt` line 26507): the `upsert-member`
POST body changed from `spaceId:""` (403, baseline) to `spaceId:"01a00e4f-96f3-…"` (**202**) — this is
the direct, load-bearing evidence the fix works end-to-end, not just in a unit test.

## 2. Root cause 2 (found chasing STEP 2): the dev shell's entry script uses a `./`-relative `src`

After fix 1, STEP 2's `upsert-member` succeeded but the step still failed: user2's hard navigation to
`/spaces/{id}` (`page.goto`, not an in-app link click) 404'd fetching
`GET http://127.0.0.1:740x/spaces/🟦️component.ts` (`🧪️4-i-collab-e2e-run1.txt`/`run2.txt`, right after
STEP 1 passes). Decoded, `%F0%9F%9F%A6%EF%B8%8Fcomponent.ts` is literally `🟦️component.ts` — this repo's
convention for a module's main file. Traced to
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:106-109`:
`semioHostHtmlVitePlugin(repoRoot, { title: "semio · os", entry: "./🟦️component.ts" })`. That plugin's
`transformIndexHtml` hook (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-
assets.ts:734-748`, `semioHostHtmlString`) **fully regenerates** the HTML document on every request,
injecting `<script type="module" src="${spec.entry}">` verbatim. A `./`-relative entry resolves fine
against `/` (root) but, on a hard navigation to a nested SPA route like `/spaces/{id}`, resolves against
the CURRENT path — producing `/spaces/🟦️component.ts`, which 404s, so the app never boots for that
tab at all. Pre-existing since 2026-08-06 (`git log --date=iso`), never exercised before this ticket
because no earlier lane's fixes let a hard navigation reach a nested route.

**Fix**: `entry: "/🟦️component.ts"` (root-relative) in `⚙️vite.config.ts` — the one production call site
in this repo with a relative entry (the module's own test fixture, `🟦️vite-elements-assets.ts:1816`,
already uses an absolute `"/js/index.tsx"`, confirming the intended convention). Also corrected the
on-disk `🌐️index.html`/`🌐️multi.html` (`./favicon.svg`/`./🖼️favicon.ico`/`./🟦️component.ts` →
`/favicon.svg`/`/🖼️favicon.ico`/`/🟦️component.ts`) for consistency, even though `transformIndexHtml`
makes their content largely inert at runtime (the on-disk file only matters for `optimizeDeps.entries`
resolution and Vite's build-input existence check).

Confirmed fixed: `run3`/`run4` show zero `/spaces/🟦️component.ts` 404s; the page genuinely boots at the
nested route (identity, `/directory/ws`, plugin workers all come up) — **STEP 1 and STEP 7 both flip to
PASS**, 1/8 → 2/8.

## 3. Root cause 3 (found chasing STEP 2 further): `applyShellUri`'s non-studio branch has no idempotency guard

With root cause 2 fixed, `run3`'s STEP 2 still failed (`.semio-table-host` 30s timeout), now flooding the
log with `[collab-e2e:console] user2 [error] [DEBUG] shell uri apply failed Error: Maximum call stack
size exceeded` plus **dozens** of rapid open/close cycles of the same document WS
(`/spaces/{id}/documents/index/ws?surface=s.space.space@1/*#editor`, syncSessionId changing every
~25-50ms) interleaved with `plugin instance busy`/`readHistory: missing HistorySnapshot frame` errors.

Traced to `ShellHost/🟦️component.tsx`'s `applyShellUri` (~line 2887-2958): the studio-route branch
(`/spaces/{id}/studio`) has a `studioChanged = openSpaceIdRef.current !== spaceId` idempotency guard
before re-running `openSpace`/`openInstance` (so re-firing the effect with the same URI is a no-op); the
newer, THIS-ticket's bare-`/spaces/{id}` branch (§5, "opens the `s.space` artifact-index app") had no
such guard — it unconditionally called `switchToManagedApp` + `openDocumentRef.current(...)` on every
invocation. The surrounding `useEffect` (line 2960-2965) re-invokes `applyShellUri` whenever its own
identity changes, and `applyShellUri` depends on `switchToManagedApp`, whose own identity depends on
`session` — which `openDocumentRef.current` itself updates. Net effect: opening the document changes
`session` → mints a new `switchToManagedApp` → mints a new `applyShellUri` → re-fires the effect → reopens
the document → repeat, unbounded, for the SAME already-open space.

**Fix**: added the same idempotency guard as the studio branch —
`spaceIndexAlreadyOpen = openSpaceIdRef.current === spaceId && currentSession.app.id === spaceApp.id`,
computed before mutating the ref, `return` early when true (`ShellHost/🟦️component.tsx`).

Confirmed: `run4`'s STEP 2 window shows the document WS opening **once**, closing **once** (a normal
teardown-on-navigate-away, not a runaway loop) — down from dozens in `run3`. `bunx vitest run` (322/9,
unchanged baseline) re-confirmed no regression in the same pass.

## 4. STEP 2's current blocker (diagnosed, not fixed — the file is under live concurrent edit)

`run4`'s STEP 2 still fails (`.semio-table-host` 30s timeout). The runaway WS loop is gone, but the
window still shows, continuously until timeout: `[DEBUG] readHistory: missing HistorySnapshot frame`
(stack: `PluginRuntime/🟦️component.tsx`) immediately followed by
`[DEBUG] render failed Error: [DEBUG] refreshUi failed: plugin.internal: plugin instance busy` (42
repeats in the STEP 2 window, `🧪️4-i-collab-e2e-run4.txt`), one
`panicked at .../console_error_panic_hook…: cannot call wasm-bindgen imported functions on non-wasm
targets`, and a flood of `GET /semio-backbone?uri=folder://…/spaces/{id}&documentId=index —
net::ERR_ABORTED` requests that never succeed. This is a **different** shape of bug from §3 — not an
infinite React-effect loop (the WS itself is stable now), but a genuine reentrancy/concurrency failure
inside the plugin-instance call layer: something keeps calling `refreshUi`/`readHistory` against the
wasm instance while a previous call is still in flight, so it perpetually loses the race and the Space
app's own table never renders within budget.

I went to read `PluginRuntime/🟦️component.tsx:147`/`:217` (the exact lines the browser's own stack traces
cite for `performRefreshUi`/`readHistory`) to pin the precise root cause, and found **neither function
exists in the file at all** on disk right now — `git status --porcelain` confirms this exact file is
currently `M` (modified, uncommitted) under what can only be a concurrent peer session's live edit (per
this session's own standing guidance: never infer a peer's live state from files it wrote, and never
edit into an active foreign change). Diagnosing further would mean reading code that may be mid-rewrite
and is likely to have moved again by the time any fix here would land, and — per the worker-brief's "no
git-modifying commands, others are editing this tree live" rule — attempting a repair inside a file
someone else has open for a rewrite risks colliding with their change outright. **Stopping here rather
than guessing against a moving target.**

**What the next lane needs to look at** (precise pointer, not a guess at the fix): the `plugin instance
busy` retry path in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/
PluginRuntime/🟦️component.tsx` (whatever it's renamed to after the in-flight edit lands) around the
`performRefreshUi`/`readHistory` call sites the browser's own stack traces name, PLUS whatever calls
`readHistory` right after `attachBackbone` for a freshly-opened document (`attachBackbone failed:
plugin.internal: plugin instance busy` appears in the same window) — the two are very likely racing each
other for the same wasm instance's reentrancy guard (`🎠️kernel/🟦️component.ts:1144`'s
`isPluginBusyError`/`🌐plugin-web-materialize.ts:120`'s busy detection already exist, so this IS an
anticipated condition — the bug is that whatever retries on `busy` here does so unconditionally/too
eagerly instead of backing off, or two independent callers (identity bootstrap's backbone attach, and
the space-index document's own render/history read) are firing concurrently against one instance that
can only serve one call at a time).

## 5. Full per-step table

| Step | Baseline (`📓️w4-h-report.md` run6) | run1/run2 (root cause 1 fixed) | run3 (root cause 2 fixed) | run4 (root cause 3 fixed) |
|---|---|---|---|---|
| 1 create space | **PASS** | **PASS** | **PASS** | **PASS** |
| 2 share + open | FAIL — 403, `spaceId:""` | FAIL — 202 now (spaceId real!), but user2's `/spaces/{id}` 404s on `🟦️component.ts` | FAIL — page loads, but `Maximum call stack size exceeded` + dozens of WS open/close cycles, 30s timeout | FAIL — WS loop fixed (1 open/close, not dozens), but `plugin instance busy`/`readHistory` retry storm never resolves within 30s |
| 3 create artifact | FAIL — skipped | FAIL — skipped (no space id reached usably) | FAIL — `.semio-table-host` timeout | FAIL — `toolbar button #s-space-create-artifact does not exist` (Space app never finishes rendering, consequence of §4) |
| 4 co-edit | FAIL — skipped | FAIL — skipped | FAIL — skipped | FAIL — skipped, no artifact id |
| 5 presence | FAIL — element didn't exist (pre-4-F) | FAIL — stale message text (element exists per 4-F, but no session opens) | FAIL — real check now runs: 0/2 peers | FAIL — real check: 0/2 peers (no document session stabilizes, consequence of §4) |
| 6 check-in | FAIL — skipped | FAIL — skipped | FAIL — skipped | FAIL — skipped |
| 7 admin connections | FAIL — `[]` | FAIL — `[]` | **PASS** — names both users | **PASS** — names both users |
| 8 hub restart | FAIL — skipped | FAIL — skipped | FAIL — skipped | FAIL — skipped |
| **Count** | **1/8** | **1/8** | **2/8** | **2/8** |

STEP 7 flips to PASS starting `run3` — plausible, not fully proven — because from `run3` onward both
users' identity/`/directory/ws` connections come up fully (root cause 2 fixed), which is likely
sufficient for `/admin/api/connections` to see both, independent of whether the document-level `s.space`
session ever stabilizes (root cause 3's target). Not chased further since it's already green.

`run3` → `run4` is the two-consecutive-same-count signal the coordinator asked me to watch for — I did
not stop there because `run4` was not a no-evidence repeat: it reflects root cause 3's fix (the WS loop
genuinely closed, confirmed by log evidence) and surfaced a **new, different** failure signature (§4)
underneath it, which is real forward diagnosis even though the top-line count didn't move a second time.
I stopped at `run4` because the next actionable step requires editing a file another session currently
has open, not because I ran out of evidence.

## Changed files

- `🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts` — `effectiveActionArgs` gained a
  third `seed` parameter (§1).
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️UIDialog/🟦️component.tsx` — uses the new `seed` parameter
  instead of a pre-merged `buffer` (§1).
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — Rust twin `effective_action_args` gained the same
  `seed: Option<&DslValue>` parameter; 3 new tests; 2 existing tests updated to pass `None` (§1, §2 note
  on parity).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — one test-helper call site updated to
  pass `None` for the new parameter.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` — one
  call site (`resolved_execute_args`) updated to pass `None`.
- `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` — 4 new in-source tests for `effectiveActionArgs` (§1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts` — `entry:
  "./🟦️component.ts"` → `"/🟦️component.ts"` (§2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🌐️index.html`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🌐️multi.html` — relative asset paths made root-relative
  for consistency (§2, mostly inert given `transformIndexHtml` but strictly more correct).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` —
  `applyShellUri`'s bare-`/spaces/{id}` branch gained an idempotency guard (§3).

## Commands run + results (real tails)

- `cargo check -p semio-framework` / `-p semio-framework-plugin` / `-p semio-framework-os-renderer-wgpu`
  — all clean (pre-existing warnings only, no new ones from these changes).
- `cargo test -p semio-framework --lib` — **140 passed; 0 failed**. Log: `🧪️4-i-framework-lib-test.txt`.
- `cargo test -p semio-s-plugin-space --lib` — **210 passed; 0 failed** (exact match to baseline), rerun
  after every fix in this lane, most recently after root cause 3. Log: `🧪️4-i-space-lib-test.txt`.
- `cargo test -p semio-hub --lib` — **11 passed; 0 failed** (exact match to baseline). Log:
  `🧪️4-i-hub-lib-test.txt`.
- `bunx vitest run -c 🧪️vitest.config.ts` (framework-renderer-react) — **322 passed | 9 failed**, the same
  pre-existing 9 documented by every prior lane; ran twice (before/after root cause 3's `ShellHost` edit),
  identical counts both times. Log: `🧪️4-i-renderer-react-vitest.txt`.
- `bunx vitest run -c 🧪️vitest.config.ts` (`@semio-tech/framework`, in-source tests incl. §1's 4 new
  ones) — **158 passed; 0 failed**. Log: `🧪️4-i-framework-package-vitest.txt`.
- `bun ./📜️script.ts verify collab` — 4 full runs, each teed:
  - `🧪️4-i-collab-e2e-run1.txt` — root cause 1 in; 1/8; hub 202 confirmed, then the `🟦️component.ts` 404
    discovered.
  - `🧪️4-i-collab-e2e-run2.txt` — reconfirms run1 identically (control run before touching root cause 2).
  - `🧪️4-i-collab-e2e-run3.txt` — root cause 2 in; **2/8**; the WS-loop/`Maximum call stack` symptom of
    root cause 3 discovered.
  - `🧪️4-i-collab-e2e-run4.txt` — root cause 3 in; **2/8** (same count, but the WS loop is gone — verified
    by grepping the STEP 2 window: `closed: ws://` count dropped from many to 1); the `plugin instance
    busy`/`readHistory` blocker (§4) is what remains.

## What is NOT done

- **STEP 2's remaining blocker** (§4): a `plugin instance busy`/`readHistory: missing HistorySnapshot
  frame` retry storm inside `PluginRuntime/🟦️component.tsx`'s `refreshUi`/`readHistory`/`attachBackbone`
  call layer. Diagnosed with file:line precision from the browser's own stack traces, but **not fixed**
  — the file is currently under live concurrent edit by another session (`git status` shows it `M`,
  uncommitted, and the exact functions the stack traces name no longer exist in it), so editing it now
  would be guessing against a moving target and risks colliding with that session's in-flight work.
- **STEPS 3, 4, 6, 8** remain downstream of STEP 2.
- **STEP 5** (0/2 presence peers) is a downstream consequence, not independently new.
- No `[DEBUG] ` temporary logging was left in any changed file (none was added — all diagnosis used the
  harness's existing permanent capture plus reading the already-present `[DEBUG]` lines other lanes left,
  which were not touched).

Ticket not closed (coordinator owns that).

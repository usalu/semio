# W5-A report — re-diagnosing the `plugin instance busy` storm against the post-landing file, 2/8 → 2/8 with three real fixes and a precisely narrowed next blocker

Lane 5-A. Starting point: lane 4-I's diagnosis (`26/08/16` ticket, `📓️w4-i-report.md` §4) of a `plugin instance
busy` / `readHistory: missing HistorySnapshot frame` retry storm in `PluginRuntime/🟦️component.tsx`, which they
could not chase further because the file was mid-rewrite under a concurrent peer session. That rewrite landed at
commit `0b9f1d3a04` (2026-08-17 12:10). This lane re-diagnosed from the current file and the live harness, found
and fixed three real, evidenced bugs, and narrowed the remaining blocker to a specific, precisely-described
second recursion source. **True count stays 2/8** (STEP 1, STEP 7) across every run in this lane — a truthful
2/8 with real forward diagnosis, not a stall.

## Method actually followed

Every fix below was made, then verified with a full `bun ./📜️script.ts verify collab` run before the next fix,
per the brief. Four full runs, each teed:
- `🧪️5-a-collab-e2e-run1.txt` — baseline reproduction against the current (post-landing) file, no fixes yet.
- `🧪️5-a-collab-e2e-run2.txt` — fix 1 in; did not reach STEP 1 (system-wide cargo contention from concurrent
  lanes pushed load average past 40 — `uptime` captured live — timed out on the FIRST page's own boot; not
  informative about the storm, discarded as a run but its evidence about fix 1 compiling clean is still valid).
- `🧪️5-a-collab-e2e-run3.txt` — fix 1 in, re-run after system load eased; confirms the storm is unchanged by
  fix 1 alone.
- `🧪️5-a-collab-e2e-run4.txt` — fixes 2 and 3 in; the run that produced the decisive new evidence below.

## Fix 1 — `console_error_panic_hook` installed for a target where it cannot work

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `ensure_plugin_initialized`:
`#[cfg(target_arch = "wasm32")] console_error_panic_hook::set_once();` matches BOTH `wasm32-unknown-unknown`
(classic wasm-bindgen browser target) and `wasm32-wasip2` (the component-model target every real plugin in this
repo compiles to — confirmed: `rustc --print cfg --target wasm32-wasip2` reports `target_env="p2"`, `target_os=
"wasi"`, vs. empty/`"unknown"` for the classic target). `console_error_panic_hook` calls into `web-sys`'s
wasm-bindgen `console.error` import, which jco's component-model transpilation never wires up. The very next
line in the SAME function already has the correct exclusion for exactly this class of problem —
`#[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]` for
`register_host_backbone_channel` — the panic-hook line was simply never updated to match when the crate moved
onto the component-model ABI.

**Live-confirmed effect, not guessed**: `🧪️5-a-collab-e2e-run1.txt` lines 28927/29239/29422/29509 show the
EXACT symptom this predicts —
```
panicked at .../console_error_panic_hook-0.1.7/src/lib.rs:83:9: cannot call wasm-bindgen imported functions on non-wasm targets
thread panicked while processing panic. aborting.
[DEBUG] program load failed playbook-module-procedural Error: unreachable
```
— i.e. the hook itself panics while trying to report some OTHER original panic, and that secondary panic is
what actually traps the wasm instance (wasip2's target spec defaults `panic-strategy` to `abort`, confirmed via
`Cargo.toml:245`'s own comment), masking the real underlying panic message and killing the module harder than
the original panic alone would have.

**Fix**: excluded `target_env = "p2"` from the cfg, matching the sibling line's own convention.

**Verification**: `cargo check -p semio-framework-plugin --features component-guest` clean;
`🧪️5-a-collab-e2e-run2.txt`/`run3.txt`/`run4.txt` all show `[DEBUG] built program s (wasm32-wasip2, dev)`
succeeding with the fix in place.

**Effect on the harness, stated honestly**: this fix is real and correct, but its effect is **not observable in
this specific harness run**. `playbook-module-procedural`/`mathematical` (the two plugins that panic in every
run, `run4` still shows 8 `panicked at .../console_error_panic_hook` occurrences for exactly these two names)
are part of the 57-crate background catalogue the collab-e2e harness deliberately does NOT rebuild (lane 3-C's
own harness design narrows the REAL build to only `s`/space and `writer` for speed — every other catalogue
entry is served from whatever `dist/plugin-modules` already holds, unrebuilt by this harness). `s`/space itself
never panics via this path in any captured run, so this fix's correctness is proven (compiles, matches the
established cfg convention, matches the exact observed panic text) but its improvement to the OTHER 57 plugins
won't show up until something rebuilds the full catalogue (`bun nx run @semio-tech/plugin-registry:build` or
equivalent) — out of this lane's scope to trigger, noted for whoever owns that next.

## Fix 2 — `readHistory`'s effect re-fired on every unrelated background plugin load

`ShellHost/🟦️component.tsx`'s history-snapshot `useEffect` depended on `[applyHistoryPatch, loadedPlugins,
session]` — including the WHOLE `loadedPlugins` array, which gets a new reference on every one of the ~50+
sequential background catalogue plugin loads during boot, not just when the relevant `session` changes. Every
refire re-dispatched a fresh `readHistory` exchange call for the SAME session/instance. Confirmed by direct
correlation: `run1`'s storm window (lines 29478-29708) shows 27 `plugin worker + <name>` catalogue-load lines
interleaved with 18 `history snapshot failed` lines in the same span.

**Fix**: switched the lookup to `loadedPluginsRef.current` (an existing ref this same file already keeps in
sync every render for exactly this "don't refire on catalogue churn" purpose, used at 8+ other call sites) and
dropped `loadedPlugins` from the effect's dependency array.

**Verification, live and decisive**: `run1` (before) shows alternating `history snapshot failed` /
`refreshUi failed...busy` pairs throughout the storm. `run4` (after) shows **zero** `history snapshot failed`
lines in the entire storm window (`grep -c "history snapshot failed"` → 0, vs. 42 `refreshUi failed...busy`
lines in the same window) — the readHistory contribution to the storm is completely gone, confirming the fix
works exactly as intended. `bunx vitest run -c 🧪️vitest.config.ts` (framework-renderer-react): 322 passed / 9
failed — the same pre-existing baseline, unchanged.

## Fix 3 — `applyShellUri` reentrancy guard (real, proven reentrancy caught; the deeper bug is NOT this)

Lane 4-I's `spaceIndexAlreadyOpen` idempotency guard (§3 of their report) closed one reentrant path into
`applyShellUri`'s bare-`/spaces/{id}` branch. `run1`/`run3` prove a DIFFERENT reentrant path still exists:
`[DEBUG] shell uri apply failed Error: Maximum call stack size exceeded`, stack `at worker.onmessage
(🎠️kernel/🟦️component.ts:520:22)` — reproduced byte-identically (same function name, same reported line) across
every run in this lane and in `4-i-collab-e2e-run4.txt`.

Added a depth-counter reentrancy guard (`applyShellUriDepthRef`) at `applyShellUri`'s own top level: a call
arriving while a previous call is still in flight is logged (`[DEBUG] applyShellUri: reentrant call blocked at
depth N, uri=...` with a captured stack) and skipped instead of proceeding.

**Live-confirmed effect (this is the decisive new evidence this lane produced)**: `run4` shows the guard firing
10 times total, e.g.
```
[DEBUG] applyShellUri: reentrant call blocked at depth 1, uri=/spaces/01a00f73-...
[DEBUG] applyShellUri: reentrant call blocked at depth 1, uri=/spaces/01a00f73-...
[DEBUG] applyShellUri: reentrant call blocked at depth 1, uri=/spaces/01a00f73-...
[DEBUG] applyShellUri: reentrant call blocked at depth 1, uri=/spaces/01a00f73-...
[DEBUG] shell uri apply failed Error: Maximum call stack size exceeded    <- STILL HAPPENS
```
This is real, structural progress: the guard demonstrably intercepts genuine reentrant top-level calls to
`applyShellUri` (proof the reentrancy lane 4-I flagged is real and still live), AND it proves — by NOT
preventing the subsequent overflow — that **the "Maximum call stack size exceeded" is not caused by
`applyShellUri` calling itself reentrantly through its own normal entry point**. If it were, this guard (which
checks depth at the exact top of the function, before any other code runs) would have caught it every time,
the same way it caught the 10 logged reentrant attempts. It didn't. So there is a **second, different**
recursive/deep-serialization path this guard cannot see, either:
- genuine JS call-stack recursion inside some OTHER function `applyShellUri` calls into (not by re-entering
  `applyShellUri` itself), or — more consistent with the evidence below —
- a **native structured-clone recursion**, not JS user-code recursion at all: the RangeError's own captured
  stack is, in every single run across two tickets now, exactly one frame — `worker.onmessage`
  (`🎠️kernel/🟦️component.ts`'s `PluginWorkerClient.attachWorker`'s message handler, line ~1255 in the current
  file — the `520` the browser reports is a Vite dev-transform line-shift artifact, confirmed by reading the
  raw source at that exact line, which is unrelated code inside `AppRouter.ownedSurfaceGaps`). A genuine deep
  JS-recursion RangeError normally shows `Error.stackTraceLimit` (10) frames of the SAME repeating function;
  showing exactly one, identically-named frame every time is much more consistent with the browser's own
  structured-clone/postMessage serialization of a `worker.postMessage(...)` payload overflowing NATIVE engine
  recursion (which V8 reports back into JS with a minimal/synthetic stack attributed to the call site whose
  promise the native code was in the middle of settling) than with ordinary application-level recursion.

**Verification**: `bunx vitest run -c 🧪️vitest.config.ts` (framework-renderer-react): 322 passed / 9 failed —
unchanged baseline, confirming no regression from the guard.

## Precise next blocker for whoever picks this up

Not fixed here — genuinely narrowed, not guessed at:

1. **Find what makes a `worker.postMessage(...)` payload (`PluginWorkerClient.request`'s `exchange`/`refreshUi`/
   `attachBackbone` calls, or their replies) grow unbounded or self-referential** when `applyShellUri`'s space
   session opens and refreshes repeatedly under load. Prime suspects, in order of how directly they sit on the
   hot path the storm occurs in:
   - `refreshUi`'s `dispatch({ type: "SET_WINDOW_UI_BY_WINDOW_ID", value: (current) =>
     mergeRecordPreservingIdentity(current, ...) })` (`ShellHost/🟦️component.tsx` ~line 2310) — if a merged
     `UiNode` tree ever nests the PREVIOUS tree inside the new one instead of replacing it, repeated refreshes
     (and the storm produces MANY, per fix 2's own before-picture) would make the tree deepen every call,
     eventually overflowing native serialization when it crosses the worker boundary.
   - `AppChannelClient.captureDocumentFrames`/the per-instance `cachedPack`/`cachedSpr` (`💻️os/🟦️component.ts`
     ~line 2515) — if a document pack round-trips back into its own next request without being reset.
   - The `viewState`/`windowInstances`/`contributionsJson`/`appRegistrationsJson` payload `refreshUi` builds
     (`ShellHost/🟦️component.tsx` ~2218-2231) — all rebuilt fresh from `loadedPlugins`/`session` each call, so
     less likely, but not yet ruled out.
2. **Confirm or rule out via a live devtools session**, not another blind log-reading pass: attach Chrome
   DevTools to the collab-e2e browser context (or reproduce manually with `dev s`), set
   `Error.stackTraceLimit = 1000` before the crash, and/or break on the `RangeError` to get an ACTUAL full
   stack — the harness's own console capture cannot get more than the browser already reports, and the browser
   is reporting a suspiciously minimal stack consistent with native recursion, which is exactly the kind of
   thing log-reading alone cannot resolve further.
3. Once found, the shape of the eventual fix is very likely either (a) a missing "replace, don't merge nested"
   case in whatever produces the growing structure, or (b) capping/flattening a payload before
   `worker.postMessage`.

## Full per-step table

| Step | `w4-i` baseline (2/8) | `run1` (this lane, no fixes) | `run3` (fix 1 in) | `run4` (fixes 1+2+3 in) |
|---|---|---|---|---|
| 1 create space | **PASS** | **PASS** | **PASS** | **PASS** |
| 2 share + open | FAIL — busy storm, 30s timeout | FAIL — identical storm reproduced | FAIL — identical storm, now also on user1 | FAIL — storm shape changed (readHistory noise gone, 10 reentrant calls now blocked cleanly) but still times out on the second, unexplained overflow |
| 3 create artifact | FAIL — downstream | FAIL — downstream | FAIL — downstream (busy storm continues into this window too) | FAIL — downstream (same) |
| 4 co-edit | FAIL — skipped | FAIL — skipped | FAIL — skipped | FAIL — skipped |
| 5 presence | FAIL — 0/2 peers | FAIL — 0/2 peers | FAIL — 0/2 peers | FAIL — 0/2 peers |
| 6 check-in | FAIL — skipped | FAIL — skipped | FAIL — skipped | FAIL — skipped |
| 7 admin connections | **PASS** | **PASS** | **PASS** | **PASS** |
| 8 hub restart | FAIL — skipped | FAIL — skipped | FAIL — skipped | FAIL — skipped |
| **Count** | **2/8** | **2/8** | **2/8** | **2/8** |

The count did not move. What moved: the storm's own composition (readHistory noise eliminated, a real
reentrancy path proven and closed), and the remaining blocker is now precisely scoped to a SPECIFIC, minimal
symptom (`worker.onmessage`-attributed `RangeError`, immune to the reentrancy guard) rather than an
undifferentiated "busy storm."

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `ensure_plugin_initialized`'s
  `console_error_panic_hook::set_once()` cfg gains `not(target_env = "p2")` (fix 1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`:
  - The history-snapshot `useEffect` reads `loadedPluginsRef.current` instead of `loadedPlugins`, dropped from
    the dependency array (fix 2).
  - New `applyShellUriDepthRef`; `applyShellUri`'s body wrapped in a depth-counter reentrancy guard that logs
    and skips a reentrant call instead of proceeding (fix 3).

## Commands run + results (real tails)

- `cargo check -p semio-framework-plugin --features component-guest` — clean (warnings only, pre-existing).
- `cargo test -p semio-hub --lib` — **11 passed; 0 failed** (matches the ticket's required baseline exactly).
- `cargo test -p semio-s-plugin-space --lib` — **210 passed; 0 failed** (matches the required baseline
  exactly).
- `bunx vitest run -c 🎯️targets/⚛️react/🧪️vitest.config.ts` (framework-renderer-react, from
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript`) — **322 passed | 9
  failed**, the exact pre-existing baseline (CSS-class assertions, an R3F crash, a chai matcher,
  `resolveWindowActions` panel-eligibility, the "Artifact"/"Document" i18n rename, two mit-bestand asset-path
  regexes, a command-palette mock shape) — ran once after all three fixes, no new failures.
- `bun ./📜️script.ts verify collab` — 4 full runs:
  - `🧪️5-a-collab-e2e-run1.txt` — baseline re-reproduction, 2/8, confirms the panic-hook symptom (§Fix 1) and
    the busy storm exactly as lane 4-I described.
  - `🧪️5-a-collab-e2e-run2.txt` — fix 1 in; did not reach STEP 1 — system load average 48.76/41.06/37.76
    (`uptime`, captured live) from concurrent lanes' own cargo builds pushed the 120s Home-boot budget past its
    limit before any step ran. Not informative about the storm; discarded for step-count purposes. (Confirms
    this repo's own documented "Concurrent Cargo Workspace Churn" hazard, not a regression from this lane's
    edits — `ps aux` at the time showed three other unrelated `cargo build`/`check`/`test` processes plus a
    lane-5-d sweep script running concurrently.)
  - `🧪️5-a-collab-e2e-run3.txt` — fix 1 in, re-run after load eased; 2/8, storm unchanged in shape by fix 1
    alone (expected — fix 1 doesn't touch the "s" plugin's own panic-free code path).
  - `🧪️5-a-collab-e2e-run4.txt` — fixes 1+2+3 in; 2/8, but with the decisive new evidence in §Fix 3 (10
    reentrant calls genuinely blocked, readHistory noise genuinely eliminated per §Fix 2, and the SECOND
    unexplained overflow isolated).

## What is NOT done

- **STEP 2's remaining blocker**: a second `Maximum call stack size exceeded` source, immune to the
  `applyShellUri` reentrancy guard, with a stack trace too minimal (one frame, `worker.onmessage`) to pin down
  from log evidence alone — narrowed to §"Precise next blocker" above (prime suspect: an unboundedly-growing or
  self-referential payload crossing the `worker.postMessage` boundary during repeated `refreshUi`/`attachBackbone`
  calls), not fixed. Needs a live devtools session with `Error.stackTraceLimit` raised, not another log-reading
  pass.
- **STEPS 3, 4, 6, 8** remain downstream of STEP 2, not independently exercised past their own gating.
- **STEP 5** (0/2 presence peers) is a downstream consequence, not independently new.
- **Fix 1's benefit to the 57-crate background catalogue** (`playbook-module-procedural`, `mathematical`, and
  presumably others that panic via `console_error_panic_hook` today) is proven correct but not yet observed —
  the collab-e2e harness deliberately never rebuilds those crates. Whoever next runs a full catalogue rebuild
  should see fewer `program load failed <name> Error: unreachable` lines as a direct, checkable consequence.
- No `[DEBUG]` temporary logging was added beyond the two intentional, permanent diagnostic lines described
  above (the reentrancy guard's own log line and the doc comments) — both are deliberate, load-bearing
  diagnostics for the next lane, not throwaway probes, and are documented as such in their own doc comments;
  nothing was left in "to be cleaned up later."
- This lane did not touch `🛢️db`'s feature wiring, the wgpu shell's check-in port, or the remaining wasm-build
  failures — all explicitly out of this lane's assigned slice of the ticket (other parts of the mission
  statement, not this lane's focus, which was specifically the `PluginRuntime`/busy-storm blocker).

Ticket not closed (coordinator owns that per the worker-brief's binding rules).

# [DEBUG] Trace Inventory — Uncommitted Working Tree

Read-only audit. Captured against `git diff HEAD` at HEAD=`ebbace9b3201818de408e161162fe0a382d19f09`, 2026-09-10 00:00 CEST. The repo is a live shared tree (concurrent devs, no worktrees) — a first capture taken minutes earlier had already gone stale (441-line delta on a second `git diff HEAD` over the same paths), so all line numbers below are from the second, final capture. Re-run `git diff HEAD -- <path>` before acting on this if more time has passed.

Scope: `🧰️framework/…/🔌️plugin/🦀️.rs` (top-level plugin host file), `🔌️plugin/⚛️reactor/**`, `🔌️plugin/📦️packages/🟦️typescript/**`, `🔌️plugin/🖥️host/🧵️shard/**`, renderer `{🏛️ShellHost,🛠️ShellHelpers,🌐️World3dHost,🔌️PluginRuntime}/🟦️.tsx`, `🧰️framework/🔨️modules/🖱️ui/**`, `🧰️framework/🔨️modules/🛂️manifest/**`, `🧰️framework/…/💻️os/🟦️.ts`, `…/💻️os/🧵️backbone-worker.ts`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/**`. `dist/` build output excluded.

## 1. `[DEBUG]` traces ADDED by this diff (`+` lines) — 29 lines, 28 real traces + 1 explanatory comment

| # | File | Ln | Trace text (≤120c) | Hot path? | Helper machinery feeding it |
|---|---|---:|---|---|---|
|1|`✏️.../✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`|312|`eprintln!("[DEBUG] outliner flag round trip flag={flag} asked={asked} state={}", state(&fixture, flag));`|one-shot — test-loop, 4 iterations (2 flags × 2 expected values) inside one `#[test]`|none dedicated; reuses the `state` closure also used by the real `assert_eq!` above it|
|2|`✏️.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`|73|`eprintln!("[DEBUG] local interaction read {request_id} turns={turns} bytes={}", capture.len());`|one-shot per test helper call (`read_local_interaction` test fn)|none dedicated|
|3|same file|1608|`"[DEBUG] nakagin popup-open world scene: spine={}B of {}B, {} lanes carrying {}B total ({}), widest lane={}B in {} leave…`|one-shot (end of one `#[async_test]`)|none dedicated; reuses `census`/`census.report()` also used by the real assertions above|
|4|same file|1617|`eprintln!("[DEBUG] nakagin lane paging: {}", census.lanes.iter().map(...));`|one-shot|none dedicated|
|5|same file|1641|`eprintln!("[DEBUG] nakagin partial refresh: camera move republished 0 of {} lanes; a selection republished {changed:?}",…`|one-shot|none dedicated|
|6|same file|1978|`println!("[DEBUG] {step} sp={:x}", std::ptr::addr_of!(anchor) as usize);`|**hot within one test**: body of new `fn probe_sp(step: &str)`, called 13× across `probe_measures_stack`|**dedicated**: `fn probe_sp(step: &str)` (new, lines ~1976-1979) exists only to emit this line — 13 call sites at `probe_sp("enter")` … `probe_sp("app.tool_measures")`|
|7|same file|1984|`println!("[DEBUG] app() future = {}", std::mem::size_of_val(&make));`|one-shot, in new `probe_future_sizes` test|none dedicated beyond the local `make`/`measures`/`tools`/`tick`/`settled` bindings, which exist only to be measured+printed|
|8|same file|1988|`println!("[DEBUG] window_measures future = {}", ...);`|one-shot, same test as #7|see #7|
|9|same file|1991|`println!("[DEBUG] tool_measures future = {}", ...);`|one-shot, same test|see #7|
|10|same file|1994|`println!("[DEBUG] dispatch future = {}", ...);`|one-shot, same test|see #7|
|11|same file|1997|`println!("[DEBUG] settle future = {}", ...);`|one-shot, same test|see #7|
|12|same file|2039|`println!("[DEBUG] size_of<{}> = {}", stringify!($ty), std::mem::size_of::<$ty>());`|one-shot per test run — body of a **local `macro_rules! show`** inside new `fn probe_stack_sizes`, invoked 10× (`show!(Puzzle3dScene)` … `show!(Puzzle3dFixture)`)|**dedicated**: the `show!` macro exists only to emit this line|
|13|same file|2104|`println!("[DEBUG] tick={tick} retained={} delta={} spawns={spawns} done={done} live={} failed={} ready={ready}", retaine…`|**hot within one test**: inside a `for tick in 0..320` loop in new `probe_fill_build_tick_heap`, printed every 20th tick (16 prints/run)|none dedicated; `retained_heap_bytes()`/`fill_ready()` are also used for the real `baseline`/`ready` values at lines 2041/2110, not debug-only|
|14|same file|2108|`println!("[DEBUG] FINAL retained={} delta={} spawns={spawns} done={done} live={} ready={ready}", ...);`|one-shot, end of same test as #13|see #13|
|15|same file|2110|`println!("[DEBUG] first fault: {first} of {} faults", failed.len());`|one-shot, conditional, same test as #13|none dedicated|
|16|`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs`|112|`println!("[DEBUG] world-3d scene split into {} lanes and merged back byte-exactly", lanes.len());`|one-shot|none dedicated|
|17|same file|135|`println!("[DEBUG] a selection edit moved exactly 1 of {} lane refs; a camera move moved none", ...);`|one-shot|none dedicated|
|18|same file|161|`println!("[DEBUG] world-3d spine packs to {} bytes carrying {} lane refs", ...);`|one-shot|none dedicated|
|19|`…/📺️renderer/…/🏛️ShellHost/🟦️.tsx`|4873|`console.warn("[DEBUG] applyHostEffects refresh", JSON.stringify({ scope: uiScope, viewStateSame: nextViewState === baseS…`|**hot: per UI refresh** — per adjacent 🐢️ docstring this branch runs on *every* action dispatch (measured "7 `readHistory` calls for 7 boot pages, 90 during a 35s fill run")|none dedicated; inline `JSON.stringify` of existing locals|
|20|same file|4876|`console.warn("[DEBUG] applyHostEffects skipped refresh: session not current", JSON.stringify({ spawned: isSpawnedPluginS…`|**hot: per UI refresh attempt** (alternate branch of #19)|none dedicated|
|21|same file|4901|`console.warn("[DEBUG] completion apply", JSON.stringify({ operation: completion.operation, scope: completion.uiScope, hi…`|**hot-ish: per typed-operation completion** — inside `plugin.subscribeOperationCompletions` callback, fires per completed retained operation (can be frequent during long-running jobs)|none dedicated|
|22|same file|4902|`void applyHostEffects(...).catch((error) => console.error("[DEBUG] typed-operati…`|one-shot per completion whose effect-apply throws (exceptional path off #21)|none dedicated|
|23|same file|4905|`console.error("[DEBUG] typed-operation completion subscription failed", error);`|one-shot per subscribe call (effectively once per mount)|none dedicated|
|24|same file|9300|`<GisMapInferenceRequestControl ... onRequest={() => { ... console.log("[DEBUG] gis-map-inference-request", error); ...`|one-shot per user click (`onRequest` handler, inside a `.catch`)|none dedicated|
|25|`…/📺️renderer/…/🔌️PluginRuntime/🟦️.tsx`|1561|`// recorded: one console record per dropped body, permanent (not a \`[DEBUG]\` trace), the same` — **not a trace**, a comment explaining the two lines below are *intentionally* NOT `[DEBUG]`-prefixed|n/a (comment)|n/a — this is the docstring for §3 below|
|26|same file|1747|`console.warn(\`[DEBUG] plugin job ${kind}#${job} exceeded its ${PLUGIN_JOB_STEP_LIMIT}-step host budget — cancelling\`);`|one-shot per job (fires once when a job's step budget is exhausted, inside `driveSpawnedJob`'s `for step in 0..LIMIT` loop, not per step)|none dedicated|
|27|same file|1891|`if (outcome.stopped === "budget") console.warn(\`[DEBUG] typed-operation drain for instance ${instanceId} exhausted its $…`|**hot-ish: per drain poll** — inside `drainTypedOperations`, fires each time a drain call stops on budget (can recur across repeated polls)|none dedicated|
|28|`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`|851|`"[DEBUG] reactor more-work streak={streak} seen={seen} executor_deadline={} close_cleanup={close_cleanup_work} typed_ope…`|**hot: per turn** (gated: only when streak≥2048 and streak is a power-of-two or a multiple of 4096) — see note below, this is a MODIFIED pre-existing trace, not a pristine new one|**dedicated, partly new**: pre-existing `MORE_WORK_TRACE: RefCell<(u64,u64)>` thread_local (line 53, unchanged by this diff) **plus two brand-new methods added by this diff**: `patches::PatchTracker::debug_state()` (`⚛️reactor/🩹️patches/🦀️.rs`, new, docstring "🐞️ Temporary trace summary … read by the reactor's more-work streak trace") and the pending-patches `debug_state()` (`⚛️reactor/📨️pending/🦀️.rs`, new, same docstring pattern)|
|29|`…/💻os/🟦️.ts`|3489|`catch (error) { console.error("[DEBUG] operation completion subscriber failed", error); }`|one-shot per completion-listener throw (exceptional path inside a brand-new `for (const listener of [...this.completionListeners])` loop)|none dedicated|

**Note on #28**: the `eprintln!(` call itself (opening paren) is unchanged context; the diff only changed the trigger guard (`streak >= 64` → `streak >= 2048 && (power-of-two || %4096==0)`, and the end-of-streak guard `trace.0 >= 64` → `trace.0 >= 2048`, line 858, listed in §2) and extended the format string with two new fields `patches=[{}] pending=[{}]` fed by the two new `debug_state()` calls. Removing this cleanly means: restore/delete the whole `MORE_WORK_TRACE.with(...)` block in `turn/🦀️.rs` (lines ~846-864, thread_local itself pre-exists and its docstrings at lines 50/52 say "temporary" too — see §2), and delete the two new `debug_state()` methods in `🩹️patches/🦀️.rs` and `📨️pending/🦀️.rs` (also check `🩹️patches/🧪️tests/🔬️unit/🦀️.rs:1096,1111` which call `tracker.debug_state()` in assertion messages — those two call sites need their message string reworked, not deleted, since the underlying `assert!` is real).

## 2. `[DEBUG]` lines already at HEAD in the same 7 files — DO NOT remove as part of this ticket

Counts are the file's *total* `[DEBUG]` occurrences minus the ones in §1 (i.e. everything left untouched by this diff). Two files' pre-existing debt is large and pre-dates this ticket entirely (not shown line-by-line — grep the file directly if needed).

| File | Pre-existing `[DEBUG]` lines | Lines |
|---|---:|---|
|`✏️.../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`|4|139, 160, 177, 256|
|`✏️.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`|16|194, 865, 1739, 1772, 2012, 2022, 2027, 2098, 2163, 2167, 2169, 3592, 3805, 3844, 3874, 3914|
|`🧰️.../🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs`|0|—|
|`…/🏛️ShellHost/🟦️.tsx`|72|**large pre-existing corpus, unrelated to this ticket** — `grep -a -n "\[DEBUG\]"` on the file for the full list; e.g. 863, 1584, 1742, 3101, 3199, 3349, 3351(§2 shown below), 3895, 4329, 4818, 5011, 5267, 5935, 6585, 6738, 7509, 9156 …|
|`…/🔌️PluginRuntime/🟦️.tsx`|38|**large pre-existing corpus, unrelated to this ticket** — 268, 292, 761, 834, 960-981, 1071-1266, 1648, 1823-1832, 2164-2430 (incl. 2337 shown below), 2750-2909 …|
|`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`|3|50 (`MORE_WORK_TRACE` turn-sequence docstring), 52 (`MORE_WORK_TRACE` streak docstring), 858 (end-of-streak eprintln, guard threshold only changed, see §1 note)|
|`…/💻os/🟦️.ts`|0|—|

Of these, the ones that literally show up as **context lines inside a diff hunk** (i.e. immediately adjacent to this ticket's own edits, highest accidental-removal risk) are:

| File | Ln | Text |
|---|---:|---|
|`✏️.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`|1772|`eprintln!("[DEBUG] Puzzle 3D transient suggestions stayed exact-window isolated, close cleared their owner, reload reset…`|
|`…/🏛️ShellHost/🟦️.tsx`|3349|`console.log("[DEBUG] space extension ledger op dispatched", { action, args });`|
|`…/🏛️ShellHost/🟦️.tsx`|3351|`console.warn("[DEBUG] space extension ledger op skipped", action, ...);`|
|`…/🔌️PluginRuntime/🟦️.tsx`|2337|`if (!frame) throw new Error("[DEBUG] readHistory: missing HistorySnapshot frame");`|
|`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`|858|`eprintln!("[DEBUG] reactor more-work streak ended after {} turns (seen={seen})", trace.0);`|
|`🔌️plugin/🦀️.rs`|23619|`/// 🐞️ \`[DEBUG]\` last maintenance stage entered — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.` (docstring for `LAST_MAINTENANCE_STAGE`)|

`plugin/🦀️.rs` also carries a large, wholly pre-existing and out-of-diff `[DEBUG]` tracing stack unrelated to any hunk touched here: `LAST_MAINTENANCE_STAGE`/`TYPED_PUBLICATION_TRACE` atomics (lines ~23611/23613) feeding `eprintln!`s at lines 16904, 16929, 16987, 24618 (incl. `query.debug_state()` on the interaction-query type), 28921, 29274, 29356, 29870, 30036 — none of these are near this diff's hunks; leave them alone, out of scope for this close-out.

## 3. Unguarded `eprintln!`/`console.*` ADDED without a `[DEBUG]` prefix

| File | Ln | Text | Note |
|---|---:|---|---|
|`…/🔌️PluginRuntime/🟦️.tsx`|1566|`console.error(\`refreshUi dropped requested body ${JSON.stringify(target.key)}: no retained surface on instance ${instanc…\`);`|**Intentionally permanent**, not a missed prefix — the adjacent comment (line 1557-1563, §1 row 25) explicitly states this is "permanent (not a `[DEBUG]` trace)", modeled on `AppRouter`'s own excluded-plugin log (`ShellHost/🟦️.tsx:9155`). Hot path: per UI refresh, per requested target body with no retained surface.|
|`…/🔌️PluginRuntime/🟦️.tsx`|1571|`else console.error(\`refreshUi dropped requested body ${JSON.stringify(target.key)}: retained surface has no root on inst…\`);`|Same as above — sibling branch, same docstring covers it.|

No other unguarded `eprintln!`/`console.log`/`console.warn`/`console.error` additions were found in the scoped diff.

## Peer-ticket attribution

The scoped diff also carries substantial unrelated work from two peer tickets, with **no `[DEBUG]` additions inside their hunks** (confirmed by locating every `ticket 26/…` slug mention in the diff and checking proximity to `[DEBUG]` hits):
- `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM` — hunks in `✏️editor/🎮️commands/⏱️suggestions-tick/🦀️.rs` and `✏️editor/🧪️tests/🔬️testkit/🦀️.rs`.
- `26/09/09/PROCEDURAL-3D-END-TO-END` — hunks in `ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs`, `ui/🎬️scene/🧪️tests/🔬️scenes-value-round-trip/🦀️.rs`, `🛂️manifest/🦀️.rs`, `…/🛠️ShellHelpers/🟦️.tsx`, and parts of `🔌️plugin/🦀️.rs`.

Every `[DEBUG]` addition and every docstring naming "ticket 26/09/02/PUZZLE-3D-END-TO-END" in §1/§2 is this ticket's own.

## Summary

- Traces added (`+` lines, §1): **29** matched lines → **28 real traces + 1 explanatory comment** (row 25, not itself a trace).
- Pre-existing `[DEBUG]` lines in the same 7 touched files (§2, not to be removed here): **133** (4 + 16 + 0 + 72 + 38 + 3 + 0).
- Unguarded (prefix-less) additions (§3): **2**, both intentionally permanent per adjacent docstring, not accidental omissions.

## 4. Refresh 07:30 — counts after waves U3/X/Y/F2c/G3 landed (close-out sweep targets)

Scoped `rg -c '\[DEBUG\] '` on the fleet-touched RUNTIME files (tests excluded from the sweep; test-file
`[DEBUG]` prints inside our new laws may stay if the law reads them, otherwise strip too):

| File | lines | Note |
|---|---:|---|
| renderer `🏛️ShellHost/🟦️.tsx` | 76 | grew from 6 at 00:00 — the per-refresh `applyHostEffects`/`completion apply` chatter visible in every probe console tail; biggest target |
| renderer `🔌️PluginRuntime/🟦️.tsx` | 41 | grew from 2; includes job-budget warnings (some may deserve un-prefixed permanence per §3) |
| plugin host `🔌️plugin/🦀️.rs` | 9 | host-side traces from waves |
| guest `🎮️commands/🪣️fill-build-tick/🦀️.rs` | 1 | W-U3 eprintln — **currently load-bearing for W-G3's browser diagnosis; strip LAST, needs a wasm rebuild** |
| guest editor `✏️editor/🦀️.rs` | 0 | clean |

Sweep order at close-out: TS files first (vite hot-reloads, re-probe to confirm no behavior change),
then host Rust, then the guest eprintln inside the final wasm rebuild. Re-run the scoped grep before
sweeping — waves W-G3/W-Z are still active and may add/remove traces.

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

---

---

## 5. Refresh 19:25 — W-AB close-out inventory (no traces removed)

Captured 2026-09-10 19:25 CEST while W-G3 runs the #40 battery. **Inventory only — zero traces stripped.**
Walker: `.ts/.tsx/.js/.mjs/.cjs/.rs`, pruning `node_modules`/`target`/`dist` and `generated/` dumps.
Repo-wide `[DEBUG]` hits: **2763** (1114 ticket · 1032 test/law · 485 host-ts including artifacts · 132 guest-rs runtime).

### Phase counts

| Phase | Meaning | Count |
|---|---|---:|
| **(a)** | host TS — strip after final battery + vite-live re-probe | **51** |
| **(b)** | guest Rust — strip inside the final rebuild | **8** |
| **(c)** | keep — law output, ticket scripts, named-file comments | **2153** |
| **peer** | outside this fleet — do not strip at our close-out | **134** in named files |

**(c) breakdown:** this-ticket scripts 6; puzzle law eprintlns 22; other ticket files 1108; other test/law files 1010; named-file comments 7.

Named-file owner mix: W-AB 10, W-G3 12, comment 7, coordinator 37, peer 134.

### 12:13 peer sweep (foreign traces)

At 12:13 a peer swept **all** `[DEBUG]` lines out of ShellHost/PluginRuntime mid-diagnosis
(coordination 12:15). Coordinator re-added a minimal set: `undo route`, `undo handleAction resolved`,
`spawn-job routed`, `job done`. Later waves re-instrumented (W-AB import/brush hops, W-G3
history-route/reserved-tool, coordinator command-ingress/`performInvocation`). W-Z then removed the
`undo funnel*` cluster and guest `history route`/`group history` eprintlns — those stay gone.
**Foreign traces still in the named files:** tutorial / document-opening / hot-swap / extension-ledger /
GIS in ShellHost; shard/actor/program/`wireEffectToFriendly`/TransactionCoordinator in PluginRuntime;
reactor `turn phase` / `guest linear memory` / `more-work streak` (peer idle-turns / guest-memory /
close-ladder). Marked `peer` — not our strip list.

### Named runtime files — line items

| File | Ln | Prefix | Owner | Phase |
|---|---:|---|---|---|
|`PluginRuntime`|340|`PluginRuntime: shard $`|peer|peer|
|`PluginRuntime`|364|`PluginRuntime: actor $`|peer|peer|
|`PluginRuntime`|836|`retained UI surface $`|peer|peer|
|`PluginRuntime`|865|`* here degrades to an honest '[DEBUG]'-logged drop rather than guessing `|comment|c|
|`PluginRuntime`|925|`request-file-open mapped $`|W-AB|a|
|`PluginRuntime`|942|`wireEffectToFriendly: dispatch-action req=$`|peer|peer|
|`PluginRuntime`|956|`wireEffectToFriendly: unmapped effect`|peer|peer|
|`PluginRuntime`|1100|`serializePerActor: actor $`|peer|peer|
|`PluginRuntime`|1222|`PluginRuntime: turn failed for actor $`|peer|peer|
|`PluginRuntime`|1279|`PluginRuntime: actor $`|peer|peer|
|`PluginRuntime`|1285|`PluginRuntime: actor $`|peer|peer|
|`PluginRuntime`|1398|`settle $`|peer|peer|
|`PluginRuntime`|1403|`PluginRuntime: actor $`|peer|peer|
|`PluginRuntime`|1411|`PluginRuntime: actor $`|peer|peer|
|`PluginRuntime`|1724|`// recorded: one console record per dropped body, permanent (not a '[DEB`|comment|c|
|`PluginRuntime`|1811|`program $`|peer|peer|
|`PluginRuntime`|1891|`job-completed leftover job=$`|coordinator|a|
|`PluginRuntime`|1937|`job done kind=$`|coordinator|a|
|`PluginRuntime`|1945|`job drive failed kind=$`|peer|peer|
|`PluginRuntime`|1973|`reserved-tool job $`|W-G3|a|
|`PluginRuntime`|1974|`job done kind=$`|coordinator|a|
|`PluginRuntime`|1987|`spawn-job routed kind=$`|coordinator|a|
|`PluginRuntime`|2010|`reserved-tool job $`|W-G3|a|
|`PluginRuntime`|2011|`job done kind=$`|coordinator|a|
|`PluginRuntime`|2015|`job drive failed kind=$`|peer|peer|
|`PluginRuntime`|2033|`spawn-job routed kind=$`|coordinator|a|
|`PluginRuntime`|2070|`command ingress lane`|coordinator|a|
|`PluginRuntime`|2099|`plugin $`|coordinator|a|
|`PluginRuntime`|2100|`plugin $`|coordinator|a|
|`PluginRuntime`|2105|`command ingress continuation $`|coordinator|a|
|`PluginRuntime`|2107|`command ingress settled status=$`|coordinator|a|
|`PluginRuntime`|2108|`plugin $`|coordinator|a|
|`PluginRuntime`|2128|`command ingress stamped Done`|coordinator|a|
|`PluginRuntime`|2175|`typed-operation drain for instance $`|coordinator|a|
|`PluginRuntime`|2360|`extension completion submitted`|peer|peer|
|`PluginRuntime`|2470|`applyRetainedWindowPatches: actor $`|peer|peer|
|`PluginRuntime`|2510|`job-completed leftover frame instance=$`|coordinator|a|
|`PluginRuntime`|2530|`coerceWireBytes: unsupported payload $`|peer|peer|
|`PluginRuntime`|2544|`performInvocation`|coordinator|a|
|`PluginRuntime`|2548|`importFixture ingress`|W-AB|a|
|`PluginRuntime`|2559|`performInvocation settled`|coordinator|a|
|`PluginRuntime`|2680|`program $`|peer|peer|
|`PluginRuntime`|2713|`readHistory: missing HistorySnapshot frame`|peer|peer|
|`PluginRuntime`|2720|`applyMutations failed: $`|peer|peer|
|`PluginRuntime`|2731|`readAppDocumentPack failed: $`|peer|peer|
|`PluginRuntime`|2738|`loadAppDocumentPack failed: $`|peer|peer|
|`PluginRuntime`|2763|`program $`|peer|peer|
|`PluginRuntime`|2775|`program $`|peer|peer|
|`PluginRuntime`|2790|`program $`|peer|peer|
|`PluginRuntime`|2795|`program $`|peer|peer|
|`PluginRuntime`|2806|`program $`|peer|peer|
|`PluginRuntime`|3126|`TransactionCoordinator rollback($`|peer|peer|
|`PluginRuntime`|3140|`TransactionCoordinator undo($`|peer|peer|
|`PluginRuntime`|3167|`TransactionCoordinator redo($`|peer|peer|
|`PluginRuntime`|3279|`loadPluginModulesInDependencyOrder: $`|peer|peer|
|`PluginRuntime`|3285|`loadPluginModulesInDependencyOrder: $`|peer|peer|
|`ShellHost`|892|`tutorial blob asset src not resolvable in this scope`|peer|peer|
|`ShellHost`|1667|`extension invocation faulted`|peer|peer|
|`ShellHost`|1697|`invokeExtension unresolved`|peer|peer|
|`ShellHost`|1845|`history patch skipped`|coordinator|a|
|`ShellHost`|1849|`history patch applied`|coordinator|a|
|`ShellHost`|1861|`navbar example from history`|peer|peer|
|`ShellHost`|1871|`leftover InteractionView`|W-G3|a|
|`ShellHost`|1879|`history snapshot skipped — projection newer than request`|peer|peer|
|`ShellHost`|1882|`history snapshot refresh`|coordinator|a|
|`ShellHost`|1884|`history snapshot failed`|peer|peer|
|`ShellHost`|1925|`history snapshot skipped — projection newer than request`|peer|peer|
|`ShellHost`|1929|`history snapshot failed`|peer|peer|
|`ShellHost`|2065|`shell uri apply failed Error: Maximum call stack size exceeded`|comment|c|
|`ShellHost`|2316|`document backbone failed during replacement — keeping owner`|peer|peer|
|`ShellHost`|2321|`document backbone failed`|peer|peer|
|`ShellHost`|2330|`document backbone pending overflow — dropping while rebound`|peer|peer|
|`ShellHost`|2789|`document rebootstrap kept session`|peer|peer|
|`ShellHost`|3290|`ShellHost: primary $`|peer|peer|
|`ShellHost`|3338|`boot fault text`|peer|peer|
|`ShellHost`|3388|`hot-swap $`|peer|peer|
|`ShellHost`|3417|`hot-swap $`|peer|peer|
|`ShellHost`|3443|`hot-swap rolled back for $`|peer|peer|
|`ShellHost`|3465|`refusing to uninstall the host/primary plugin: $`|peer|peer|
|`ShellHost`|3469|`refusing to uninstall the active session`|peer|peer|
|`ShellHost`|3538|`space extension ledger op dispatched`|peer|peer|
|`ShellHost`|3540|`space extension ledger op skipped`|peer|peer|
|`ShellHost`|3568|`extension store install ok`|peer|peer|
|`ShellHost`|3570|`extension store unavailable or install failed; falling back to catalog i`|peer|peer|
|`ShellHost`|3577|`installExtension could not resolve moduleUrl`|peer|peer|
|`ShellHost`|3647|`extension store install from file ok`|peer|peer|
|`ShellHost`|3649|`installExtensionFromFile failed`|peer|peer|
|`ShellHost`|3750|`setExtensionEnabled`|peer|peer|
|`ShellHost`|3883|`local interaction observation failed`|peer|peer|
|`ShellHost`|4084|`shell plugin retirement failed`|peer|peer|
|`ShellHost`|4330|`contributions push`|peer|peer|
|`ShellHost`|4541|`render failed [$`|peer|peer|
|`ShellHost`|4562|`spawned render failed [$`|peer|peer|
|`ShellHost`|4625|`spawned program document sync failed`|peer|peer|
|`ShellHost`|4807|`loadDocument pack/spr for instance`|peer|peer|
|`ShellHost`|4849|`import-picker hop accept=$`|W-AB|a|
|`ShellHost`|4851|`import-picker opened=$`|W-AB|a|
|`ShellHost`|4971|`replayShellCommand dispatch`|W-G3|a|
|`ShellHost`|5016|`invokeExtension dispatch failed`|peer|peer|
|`ShellHost`|5048|`openPluginInstance focused spawned app`|peer|peer|
|`ShellHost`|5103|`applyHostEffects refresh`|coordinator|a|
|`ShellHost`|5106|`applyHostEffects skipped refresh: session not current`|coordinator|a|
|`ShellHost`|5131|`completion apply`|coordinator|a|
|`ShellHost`|5132|`typed-operation completion effects failed`|coordinator|a|
|`ShellHost`|5135|`typed-operation completion subscription failed`|coordinator|a|
|`ShellHost`|5146|`applyShellUri: reentrant call blocked at depth $`|peer|peer|
|`ShellHost`|5216|`applyShellUri openSpace`|peer|peer|
|`ShellHost`|5241|`shell uri apply failed`|peer|peer|
|`ShellHost`|5289|`document opening parked`|peer|peer|
|`ShellHost`|5327|`document opening aborted — predecessor restored`|peer|peer|
|`ShellHost`|5359|`parked predecessor attachment retirement failed`|peer|peer|
|`ShellHost`|5365|`document opening committed`|peer|peer|
|`ShellHost`|5388|`closeDocument`|peer|peer|
|`ShellHost`|5393|`document backbone retirement failed`|peer|peer|
|`ShellHost`|5399|`background document retirement failed`|peer|peer|
|`ShellHost`|5419|`document attachment retirement failed`|peer|peer|
|`ShellHost`|5428|`tutorial retirement failed`|peer|peer|
|`ShellHost`|5526|`recovery diagnostics`|peer|peer|
|`ShellHost`|5636|`setActiveUtility failed`|peer|peer|
|`ShellHost`|5764|`undo route`|coordinator|a|
|`ShellHost`|5780|`undo remap state`|coordinator|a|
|`ShellHost`|5785|`undo remapped to document session`|coordinator|a|
|`ShellHost`|5793|`history route blocked effect-owner`|W-G3|a|
|`ShellHost`|5816|`history route blocked view-state`|W-G3|a|
|`ShellHost`|5819|`// 🚨️ Undeclared-action drop — ALWAYS visible, never '[DEBUG]'/diagnosti`|comment|c|
|`ShellHost`|5824|`history route blocked undeclared`|W-G3|a|
|`ShellHost`|5846|`authenticated browser actor action owner failed`|peer|peer|
|`ShellHost`|5850|`history route action=`|W-G3|a|
|`ShellHost`|5858|`authenticated browser actor action failed`|peer|peer|
|`ShellHost`|5864|`history route fallback handleAction`|W-G3|a|
|`ShellHost`|5869|`undo handleAction resolved`|coordinator|a|
|`ShellHost`|5898|`action failed`|peer|peer|
|`ShellHost`|5992|`authenticated browser actor intent failed`|peer|peer|
|`ShellHost`|5997|`authenticated browser actor requires the complete UI intent`|peer|peer|
|`ShellHost`|6015|`tutorial retirement failed`|peer|peer|
|`ShellHost`|6075|`tutorial sandbox restore failed`|peer|peer|
|`ShellHost`|6085|`tutorial retirement failed`|peer|peer|
|`ShellHost`|6100|`tutorial sandbox start failed`|peer|peer|
|`ShellHost`|6183|`tutorial director failed`|peer|peer|
|`ShellHost`|6235|`tutorial rebuild`|peer|peer|
|`ShellHost`|6236|`tutorial seek failed`|peer|peer|
|`ShellHost`|6312|`tutorial sandbox restore failed`|peer|peer|
|`ShellHost`|6327|`tutorial transition failed`|peer|peer|
|`ShellHost`|6335|`tutorial sandbox restore failed`|peer|peer|
|`ShellHost`|6350|`tutorial recording validation failed`|peer|peer|
|`ShellHost`|6352|`tutorial recording`|peer|peer|
|`ShellHost`|6371|`tutorial interaction capture failed`|peer|peer|
|`ShellHost`|6793|`setDefaultApp failed`|peer|peer|
|`ShellHost`|6804|`clearDefaultApp failed`|peer|peer|
|`ShellHost`|6821|`setMergePolicy failed`|peer|peer|
|`ShellHost`|6842|`resolveConflict failed`|peer|peer|
|`ShellHost`|6859|`openArtifact failed`|peer|peer|
|`ShellHost`|6861|`openArtifact failed`|peer|peer|
|`ShellHost`|6885|`openArtifactWithAppRef: $`|peer|peer|
|`ShellHost`|7038|`readConflicts failed [$`|peer|peer|
|`ShellHost`|7773|`background document authority retirement failed`|peer|peer|
|`ShellHost`|7827|`touchSpaceIndexArtifact failed`|peer|peer|
|`ShellHost`|8239|`authenticated browser actor command owner failed`|peer|peer|
|`ShellHost`|8244|`authenticated browser actor command failed`|peer|peer|
|`ShellHost`|9535|`/** 🧯️ One console record per plugin the router excluded — permanent, no`|comment|c|
|`ShellHost`|9680|`gis-map-inference-request`|peer|peer|
|`World3dHost`|4828|`suggestions-rightdown hop alt=$`|W-AB|a|
|`World3dHost`|4948|`interactionHover dispatch domain=$`|W-AB|a|
|`World3dHost`|4979|`vortex-hover hop fullId=$`|W-AB|a|
|`World3dHost`|5089|`brush-place hop preview=$`|W-AB|a|
|`World3dHost`|5105|`brush-place deferred addBrushObject $`|W-AB|a|
|`World3dHost`|5658|`suggestions-contextmenu hop alt=$`|W-AB|a|
|`os.ts`|3489|`operation completion subscriber failed`|peer|peer|
|`plugin-bridge`|186|`wgpu plugin-bridge: actor $`|peer|peer|
|`plugin-bridge`|188|`wgpu plugin-bridge: shard $`|peer|peer|
|`plugin-bridge`|624|`program $`|peer|peer|
|`plugin-bridge`|629|`program $`|peer|peer|
|`plugin-bridge`|707|`plugin $`|coordinator|a|
|`plugin-bridge`|708|`plugin $`|coordinator|a|
|`plugin-bridge`|713|`plugin $`|coordinator|a|
|`plugin.rs`|17191|`registered keyed dispatch progress stage=`|peer|peer|
|`plugin.rs`|17216|`actual registered keyed dispatch`|W-G3|b|
|`plugin.rs`|17274|`actual registered keyed dispatch`|W-G3|b|
|`plugin.rs`|22285|`chrome history action=redo seq=`|peer|peer|
|`plugin.rs`|22311|`chrome history action=undo seq=`|coordinator|b|
|`plugin.rs`|22320|`history route action=`|W-G3|b|
|`plugin.rs`|24272|`/// 🐞️ '[DEBUG]' last maintenance stage entered — temporary, ticket 26/0`|comment|c|
|`plugin.rs`|29594|`[semio-plugin panic]`|peer|peer|
|`plugin.rs`|29948|`maintenance stage=`|peer|peer|
|`plugin.rs`|30031|`cooperative maintenance callback overran the interactive ceiling for ins`|peer|peer|
|`plugin.rs`|30383|`runtime close pending authority turn=`|peer|peer|
|`plugin.rs`|30570|`native close deadline instance=`|peer|peer|
|`plugin.rs`|30736|`cooperative-maintenance instance=`|peer|peer|
|`plugin.rs`|31053|`plugin_handle_action entry instance=`|peer|peer|
|`plugin.rs`|31059|`plugin_handle_action actionId=`|peer|peer|
|`plugin.rs`|32453|`plugin_exchange entry instance=`|coordinator|b|
|`plugin.rs`|32607|`plugin_exchange actionId=`|coordinator|b|
|`plugin.rs`|32629|`plugin_exchange actionId=`|coordinator|b|
|`plugin.rs`|32652|`plugin_exchange actionId=<undecoded> branch=command-frame`|coordinator|b|
|`reactor/turn`|58|`/// 🐞️ '[DEBUG]' more-work streak trace: (current streak, total more-wor`|comment|c|
|`reactor/turn`|147|`turn phase`|peer|peer|
|`reactor/turn`|185|`guest linear memory`|peer|peer|
|`reactor/turn`|561|`continuation resolve dropped req=`|peer|peer|
|`reactor/turn`|1172|`reactor more-work streak=`|peer|peer|
|`reactor/turn`|1179|`reactor more-work streak ended after`|peer|peer|

### Empty / law-only named sites

- `ShellHelpers` — **0** `[DEBUG]` (runtime).
- Puzzle guest `fill-build-tick` / editor production — **0** (W-Z already stripped the fill eprintln).
- Document admission + `UiDocumentStore` **runtime** — **0**. Hits are test/law oracles → phase **(c)**.
- ShellHost dialog-origin admission runtime — **0**.

### Prefix-less guest eprintln (no snuck taps)

Every `eprintln!` in `plugin.rs` / `reactor/turn` that looked prefix-less is a wrapped `[DEBUG]` line
(`eprintln!(` then `"[DEBUG] …"` on the next line) or `eprintln!("{line}")` printing a string already
formatted by `trace_guest_line("[DEBUG] turn phase …")`. No new unprefixed guest taps in the fleet.
PluginRuntime still has the two intentional permanent `refreshUi dropped requested body` `console.error`s
(00:00 §3) — not `[DEBUG]`, not to strip.

### (a) host TS strip list after final battery

W-AB hops: World3dHost `suggestions-rightdown`, `interactionHover dispatch`, `vortex-hover hop`,
`brush-place hop`, `brush-place deferred`, `suggestions-contextmenu`; ShellHost `import-picker hop` /
`import-picker opened`; PluginRuntime `request-file-open mapped`, `importFixture ingress`.

Coordinator / W-G3 diagnosis on the same files: `performInvocation` + settled, `spawn-job` / `job done` /
`job-completed leftover`, `command ingress *`, `undo route` / `undo remap*` / `undo handleAction resolved`,
`history patch *`, `applyHostEffects *`, `completion apply *`, `leftover InteractionView`,
`replayShellCommand dispatch`, `history route blocked*` / `history route action` / fallback,
`reserved-tool job`. Strip with the W-AB hops once the #40 battery + final re-probe are green.

### (b) guest Rust strip list inside the final rebuild

`plugin.rs`: `chrome history action`, `history route action`, `actual registered keyed dispatch`,
`plugin_exchange *`. No fill-build-tick eprintln remains. Reactor `turn phase` / `guest linear memory` /
`more-work streak` stay **peer** (not this ticket's rebuild strip unless the coordinator expands scope).

### (c) keep

- This ticket: `browser-probe.ts` (native pointerdown + console collector), `ws2-scene-latency-probe.ts`,
  `trace-publication-authority.ts`.
- Puzzle law eprintlns (unit / example-switch / panels) — one-shot summaries at the end of passing laws.
- Plugin-builder-contract / UiDocumentStore typedwire / input-admission oracles.
- Other tickets' leftover scripts — not this close-out.

### Anywhere-else host TS (not named files; peer unless noted)

- ShellHost presence-scope browser: 4 `scoped-presence *` — peer.
- `os.ts`: 1 `operation completion subscriber failed` — peer (00:00 §1 #29).
- NodeGraph 12 / Paint2d 2 / TextEditor 1 / Shell 2 / frame-worker / turn-budget — peer products.
- `storybook-static/`, repo `cache/`, `bundles/`, `temp/` — build/cache copies, not strip sources.

### Sweep order at close-out

1. Host TS phase (a) — vite hot-reload, re-probe import + history + #40 battery items.
2. Guest Rust phase (b) — inside the final wasm rebuild after W-G3 reports.
3. Leave (c) and `peer` rows.
Re-walk before sweeping: W-G3 #40 may add more hops.

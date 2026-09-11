# Wave B4 — attributing and cutting the main-thread cost of a Nakagin refresh

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, 2026-09-11. Targets ranked ceiling #2 / "Wave 2" of
`📓️2026-09-11-audit-A3-perf-ceilings.md`: *"every Nakagin example switch / full refresh pays ~5.4 s of
main-thread long tasks per window"* (W-S2 §2.3: `gap=24.3s longtaskCpu=5.4s tasks=21 workerPosts=8799`,
attributed only at the round-trip level).

Predecessors read first: `📓️2026-09-10-wave-S2-scene-latency.md` (§2.3, §7),
`📓️2026-09-10-wave-P5-lanes-render.md`, `🔍️ws2-scene-latency-probe.ts`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD `46c3cb9de0`. No `git commit`/`stash`/`checkout`,
  no worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`. The ticket was not opened/closed by this wave.
- The repo MCP server did not connect (`repo (-32602): invalid initialize params`), so the ticket
  folder is managed on disk.
- Live target: the already-running `:6013` dev serve, release wasm #43. No server was started or
  stopped; every probe run waited until `pgrep -f browser-probe` was clear (one run, `b4-after-1.txt`,
  overlapped a peer probe and is kept only as a duplicate — the clean pair is `b4-profile.txt` /
  `b4-after-2.txt`).
- Peers edited the same files throughout: wave B2 landed
  `PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT`/`pluginTurnStalledError` into `settlePluginTurn`, wave
  B1 landed `world3dCameraDomJson`/`data-camera-json` into `World3dHost`, wave B6 added
  `[DEBUG] b6 refresh panels` to `ShellHost`. None of their hunks was touched or reverted.
- Probe: `🔍️b4-main-thread-probe.ts` (new, kept). Outputs under `🗑️generated/b4-*.txt` (+ `b4-nakagin.png`).

## 1 Attribution — the three named candidates are all small

Temporary `[DEBUG]` spans (a `globalThis.__b4` table of `{ms, n}` per phase, one `performance.now()`
pair per synchronous primitive, so the numbers are CPU, not wall-clock-with-awaits) were added around:

| span | wrapped call |
| --- | --- |
| `intake.advance` / `intake.publish` / `intake.close` | `OwnedUiPatchIntake.advance`/`closeStep` inside `acceptUiPatches`/`closeIntake` |
| `maintenance.read` / `maintenance.read-retirement` | `owner.advanceMaintenance` inside `advanceUiMaintenance` |
| `project.subscribe` / `project.acknowledgeRead` / `project.unsubscribe` / `project.node` / `project.wall` / `project.nodes` | every step of `projectOwnedUiSurface`'s `build` walk, plus its wall time and node count |
| `interp.assemble` / `interp.pagedSurfaceRender` | `PagedSurfaceView`'s lane reassembly and its render |
| `world3d.renderToCommit` | `World3dHost` render → `useLayoutEffect` (render + commit of the 180-instance subtree) |

plus three page-level instruments in the probe itself that need no source edit (`Worker.postMessage`,
`Worker.onmessage`, `console.*`, and a V8 sampling profile).

**Before, one Nakagin switch on the live target** (`🗑️generated/b4-profile.txt`, 2026-09-11):

```
gap=86.3s longtaskCpu=14.2s tasks=55 workerPosts=4970          (the switch DID NOT land — see §4)
  project.nodes                   1275 nodes  ×51 projections
  project.wall                     132.4ms  ×51
  intake.advance                   123.6ms  ×294186
  page.console.warn                108.1ms  ×2505
  page.workerMessage                72.5ms  ×10006
  page.workerPost                   45.2ms  ×5002
  maintenance.read                  16.7ms  ×81600
  world3d.renderToCommit            13.5ms  ×3
  maintenance.read-retirement        8.7ms  ×45900
  interp.pagedSurfaceRender          0.6ms  ×2
  interp.assemble                    0.1ms  ×2
  → instrumented total 527ms against 14 200ms of long-task CPU (3.7 %)
```

and the same instruments over the BOOT publication (the one publication that does complete today):

```
intake.advance:334ms/844214   world3d.renderToCommit:156ms/10   project.wall:54ms/12   interp.assemble:0ms/2
```

Three conclusions, and they decide the wave:

1. **(a) first-publication intake is NOT the ceiling.** The full Nakagin intake runs — 294 186
   `advance` steps across the switch, i.e. both windows' first publications — and costs **123.6 ms**,
   not the ≈0.7 s W-S2 extrapolated (its 3.4 µs/step figure is 8.5× the 0.42 µs/step measured here).
   Paging the guest's `world_instances_geometry_json` first publication would buy ≈0.1 s and would add
   a multi-turn publication protocol to the one path that is already cheap. **Not done.**
2. **(b) the React commit of 180 instances is NOT the ceiling.** `World3dHost` renders **3 times**
   across a whole switch, **13.5 ms** total. A lane-hash `React.memo` would save single-digit
   milliseconds. **Not done.**
3. **(c) projection IS a real, and entirely avoidable, cost** — and the larger half of it is not the
   132 ms. See §2.

Everything else (≈96 % of the long-task CPU) is per-round-trip churn — 10 006 worker messages, and a
`console.warn` storm from `settlePluginTurn` — neither of which is this wave's region. See §5.

## 2 The fix: a refresh must not re-project a body the host already holds

`buildUiRefreshRequest` (`🛠️ShellHelpers/🟦️.tsx:4318-4337`) has always stamped every requested window,
panel and section with the host's own cached hash, and says so in its docstring: *"Every requested entry
carries the host's cached hash so the plugin can omit payloads for sections that didn't change."*
`PluginUiRefreshSectionResponse.value` is optional (`🛂️manifest/🟦️.ts:1232`) precisely to carry that
omission.

**No projector ever read it.** Both the owned projector (`ownedUiRefreshResponse`) and the
language-neutral oracle (`retainedUiRefreshResponse`) rebuilt every requested tree on every refresh.
That cost twice:

1. the retained walk itself — measured above at 51 projections / 1 275 node reads / 132 ms per switch,
   with 81 600 `advanceMaintenance` drains riding along; and
2. — the larger term — a **fresh `BuiltNode` object for an unchanged body**, which defeats
   `applyUiRefreshResponseToCache` → `mergeRecordPreservingIdentity` → `InterpretedUiNode`'s
   `React.memo`. `ShellHost`'s own comment (`🏛️ShellHost/🟦️.tsx:4344-4346`) states that identity
   preservation *"is what lets `InterpretedUiNode`'s `React.memo` (and `modeWindows`'s `useMemo`) skip
   reconciling the whole shell on every interaction"* — and it never happened, because the value was
   always a new object.

So the brief's option (c) and option (b) are the same defect, and the wire already carries everything
needed to close it. No touched-validation program, no memo keys, no new wire field.

| file:line | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1528-1551` | new `uiRefreshSectionUnchanged(cached, view)` — exact equality of the request's cached hash against the surface's own current view hash, on a rooted surface only. Never heuristic: both sides are the same `view.hash`. |
| … `🔌️PluginRuntime/🟦️.tsx:1640-1662` (`retainedUiRefreshResponse`) | the oracle hashes first and answers `{ key, hash }` with no `value` when the host's hash still matches; only a changed body is walked. |
| … `🔌️PluginRuntime/🟦️.tsx:1812-1855` (`ownedUiRefreshResponse`, `project` + `projectSections`) | production path: same skip, and `projectOwnedUiSurface` is not called at all for an unchanged body. Response entries are typed `PluginUiRefreshSectionResponse` instead of a local `value`-required shape. |

Both implementations now agree, which is what makes the oracle a usable law subject.

## 3 After

`🗑️generated/b4-after-2.txt` (clean run, no concurrent probe, same instrumentation, same target):

| span | before (`b4-profile.txt`) | after (`b4-after-2.txt`) |
| --- | --- | --- |
| `project.wall` | **132.4 ms × 51 projections** | **4.1 ms × 3 projections** |
| `project.nodes` (retained nodes walked) | **1 275** | **75** |
| `refresh.skipped` / `refresh.sectionSkipped` | — (the skip did not exist) | **56 / 4** |
| `maintenance.read` | 81 600 drains | **4 800** |
| `maintenance.read-retirement` | 45 900 drains | **2 700** |
| `intake.advance` | 123.6 ms × 294 186 | 133.5 ms × 314 057 (unchanged, as predicted) |
| `world3d.renderToCommit` | 13.5 ms × 3 | 14.2 ms × 3 (unchanged, as predicted) |
| long-task CPU over the window | 14.2 s | 11.1 s |

**60 of 63 requested bodies per switch are now answered without a projection (95 %)**; retained-node
walks drop 94 %; the projection's own main-thread cost drops 97 % (132.4 ms → 4.1 ms). The
identity-preservation effect (no fresh `BuiltNode` for an unchanged window ⇒ `React.memo` actually
bites) is not separately quantified here because the switch does not complete on this target (§4).

The long-task total (14.2 s → 11.1 s) is NOT claimed as this wave's result: the two runs differ in
machine load and in peers' in-flight edits, and 96 % of that CPU was never in the instrumented phases.
The load-bearing numbers are the span counts, which are per-page and load-independent.

## 4 What could NOT be measured, and why

**The end-to-end `gap` has no after-number, because the Nakagin switch does not complete on this target
today.** Every one of the four probe runs this wave reports `DID NOT land` with both viewports still at
`{"bytes":266,"instances":1}` after 84–143 s, and the console is dominated by

```
[DEBUG] settle puzzle#1 empty-required stop continuation=…   ×2 505 … ×6 624 per run
```

i.e. `settlePluginTurn`'s `requiredEmpty && acknowledgements.length === 0` break
(`🔌️PluginRuntime/🟦️.tsx`, committed at HEAD `46c3cb9de0`, and still present after wave B2 added its
zero-progress fast-fail beside it). That is wave B2's region and was deliberately not touched. Until it
lands, W-S2's `gap=24.3s` baseline has no comparable successor measurement, and ceiling #2's headline
number ("5.4 s of long tasks") cannot be re-measured on a healthy switch.

What CAN be stated without it: on the path the app actually executes today, of the three candidates the
audit named, two are ≤ 14 ms and the third is now 4 ms.

## 5 Residual — where the main-thread CPU actually is

Measured, not inferred, from the same runs:

1. **Per-round-trip cost.** 10 006–26 404 worker messages per switch window; `page.workerMessage` +
   `page.workerPost` charge 118–240 ms at the handler boundary alone, and the promise continuations
   they resolve (the settle ladder, `submitPluginTurn` decode, `hasRequiredUiPatches` over a `results`
   array that grows with every continuation — an O(n²) scan) run inside the same long task but outside
   any wrapped frame. This is ceiling #1 / W-S2's `UI_TURN_PATCHES_MAXIMUM = 1` residual, owned by
   waves 1/B2.
2. **An ungated `[DEBUG] ` console storm.** `settlePluginTurn`'s `empty-required stop` line is an
   unconditional `console.warn` — not behind `runtimeDiagnosticsEnabled()` like `ShellHost`'s traces —
   and fired 2 505–6 624 times per switch, charging **108–244 ms** inside `console.warn` alone. Gating
   it (and the `% 512` continuation line) behind the shell's existing diagnostics predicate is a
   one-line main-thread win, but the line belongs to wave B2's live edit region and was left alone.
3. **Dev-build React.** The V8 profile's top JS frames are `exports.jsxDEV` (455 ms),
   `getBoundingClientRect` (480 ms), `logComponentRender`/`addObjectDiffToProperties` — dev-only React
   instrumentation. Any main-thread figure taken on `:6013` is inflated by it; a production-build
   measurement would be a different, and fairer, baseline. Worth noting before anyone re-targets 5.4 s.
4. **Not attacked, by the numbers:** paged first publication of the instances lane (§1.1) and per-lane
   `React.memo` in `World3dHost` (§1.2). Both remain available if a healthy switch ever shows them.

## 6 Laws

Both in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`,
driven through the language-neutral oracle `retainedUiRefreshResponse` (the same predicate production
uses).

1. **`omits the payload of a requested body whose hash the host already holds and re-projects only the
   one that changed`** — publishes two retained window surfaces, refreshes once with no hashes (both
   carry `value`), refreshes again echoing the returned hashes (both answer `{key, hash}` with
   `value === undefined`), then re-publishes ONE of them and refreshes with the stale hashes (the
   untouched one still answers without a payload; the changed one answers with a NEW hash and a
   payload carrying the new text).
2. **`holds the skip predicate to exact hash equality on a rooted surface`** — `uiRefreshSectionUnchanged`
   is true only for a non-empty, exactly equal hash on a surface with a root; a differing hash, an
   absent hash, an empty hash and a root-less surface are all false.

Fails before / passes after, proven by neutralizing the predicate (`return false && …`) and re-running:

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts -t "hash the host already holds"
⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯
AssertionError: expected [ { key: 'steady', …(2) }, …(1) ] to deeply equal [ { key: 'steady', …(1) }, …(1) ]
      Tests  1 failed | 866 skipped (867)
```

and with the predicate restored:

```
 Test Files  1 passed | 22 skipped (23)
      Tests  1 passed | 866 skipped (867)          ← "hash the host already holds"
 Test Files  1 passed | 22 skipped (23)
      Tests  1 passed | 866 skipped (867)          ← "skip predicate to exact hash equality"
```

No cargo law: no guest Rust was touched (the attribution ruled out the paged-first-publication fix).

## 7 Verification

The command in `📓️2026-09-10-wave-Z-debug-sweep.md`
(`bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'`) **no longer exists**: the
root test router was refactored by a peer this session and now rejects `--run`
(`Unknown workspace test selection: --run 🔌️PluginRuntime 🔬️engine-contract`), while its replacement
`bun ./📜️script.ts test long run …` runs a repo-wide `testing/layout` contract phase that fails on ~40
pre-existing breaches in unrelated files before any vitest starts. The lane was therefore run through
its own config directly — the same suites, the same `SEMIO_TEST_LEVEL=long` selection the router would
have passed:

| command | result |
| --- | --- |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/🎯️targets/⚛️react/vitest.config.ts` | `Test Files  3 failed \| 20 passed (23)` / `Tests  9 failed \| 858 passed (867)` — **the identical 9 failures before and after this wave's edits**, none in the refresh path: 6 in `🧩️package-integration` (generated-worker byte/provisioning laws), 1 in `🔬️engine-contract` (`buildNoteShellCommandAction`), and 2 in `🔌️PluginRuntime` — `uiRefreshSurfaceEvents` ("binds two instances of one body…") and `readAppDocumentPack()` (`expected … to deeply equal …` over an extra `"ops": ""` field). Neither symbol appears in this wave's diff (`git diff HEAD` on both the source and the test file contains only the refresh-skip hunks and the two new laws), so all 9 are pre-existing/peer-owned. |
| the two new laws, individually | `Tests  1 passed \| 866 skipped` each (quoted in §6) |
| `bun x tsc --noEmit -p tsconfig.json`, filtered to touched files | **zero** errors in `🔌️PluginRuntime/`, `🗣️Interpreter/`, `🌐️World3dHost/` or `🧪️tests/🔌️plugin-runtime/`. The repo-wide count (3 556) is a peer baseline dominated by generated `.d.ts` files and was not moved by this wave. |
| cargo | **not run — no guest Rust touched.** The wasm component build was not run, per brief. |

## 8 Instrumentation removed

Every `[DEBUG]` span added for §1 was removed; `grep -c __b4` is 0 in `🔌️PluginRuntime/🟦️.tsx`,
`🗣️Interpreter/🟦️.tsx` and `🌐️World3dHost/🟦️.tsx`, and `git diff HEAD` on the latter two now contains
only peers' hunks. No permanent `performance.mark` surface was kept: the behaviour the spans measured
is pinned by the two laws instead, and the probe re-creates the spans on demand (its docstring says how).

## 9 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` —
  `uiRefreshSectionUnchanged` + the skip in `retainedUiRefreshResponse` and `ownedUiRefreshResponse`;
  the predicate added to the module's test-surface export.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` —
  the two laws of §6, and the matching destructure entry.
- `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️b4-main-thread-probe.ts` — the attribution probe (kept).
- `.🧬semio/…/PUZZLE-3D-END-TO-END/🗑️generated/b4-profile.txt`, `b4-before.txt`, `b4-before-2.txt`,
  `b4-after-1.txt`, `b4-after-2.txt`, `b4-after-3.txt`, `b4-nakagin.png` — the runs quoted above.
- this report.

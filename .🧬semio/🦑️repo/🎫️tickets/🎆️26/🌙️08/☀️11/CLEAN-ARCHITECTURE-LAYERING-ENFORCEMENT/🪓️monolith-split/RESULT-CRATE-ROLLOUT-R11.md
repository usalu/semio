# Result: CRATE-ROLLOUT-R11 (prefix `r11`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r11-tools/` (`step.sh <tag> <cmd...>`, `r11-census.ts`, `oracle.sh`, `cmp.py`, `plan-dump.ts`, `plan-kept.ts`, `slice-trace.ts`, `extract-base.py`, `repoint.py`, `apply-root.py`, `window-ids.py`, `split-bridge.py`, `fix2.py`, `orig/`). Logs: `F/🗑️generated/r11-*`. Sandboxes: `/Users/ueli/.cache/r11/`. Started 2026-10-17 14:30.

## 0. Verdict (kept current)

| Package | Verdict | Notes |
| --- | --- | --- |
| flow flow (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow`) | DONE (static) | 11 producer crates plus facade, all inside the gate (lib and test); `--check-applied --base=dir:` exit 0, 0 / 0 extras; cfg(test) scan 0 hits; no real compile (compiler ingress) |

## 1. Found state

`RESULT-W4-FLOW-FLOW.md` is about a different crate (`semio-framework-artifact-flow-flow`, the framework host snapshot crate, already split into model/document/copy by W4). The plugin artifact `semio-s-artifact-flow-flow` was never touched by it; there was nothing half applied. The artifact has the fleet-baseline staged changes of earlier slices (23 files, file splits and TS type fixes by peers); no unrelated edits were reverted.

## 2. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r11-census-base0.tsv`)

| crate | lib files/bytes/lines | test files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-flow-flow | 131 / 530,825 / 8,480 OVER(files) | 216 / 910,549 / 12,900 OVER(files) |

Already inside the gate and untouched: the other flow plugin crates (bim, list, dictionary, text, primitive, draw, logic, math, brep + brep-mesh + brep-tests, all with their `-component` crates) and `semio-flow-mutation-bridge`.

## 3. Cause found (planner, `F3_SLICES=1 ... --explain`, `slice-trace.ts`)

1. The editor root file (`✏️editor/🦀️.rs`, 2,958 lines, 200 KB) was a hub: ids, action-value helpers, the "graph" interaction vocabulary, `FlowHost` bridging and the store capacities lived next to the retained job routes, `FlowCommand`, `FlowPlayApp` and the manifest, and every command, window and panel reached them as `crate::editor::flow::X`. One 53-file, 328 KB, 5,237-line cycle group.
2. The default test aggregation (`aggregate`) puts one test-only kid into the inline `commands` wrapper, which unions all 32 command slices into one (`buildSlices`: any `testOnly` kid unions all kids). Use `--no-test-aggregate` for this artifact.
3. Snapshot cycle: `FlowSnapshot::to_host_snapshot` called `flow_working_scene(&FlowSnapshot)` (a root function), and the root `ContentBridge`/`WorkingScene` regions form one hosted unit that also holds `flow_composed_snapshot(&FlowSnapshot)`, so snapshot and its io codecs formed a cycle with the schema crate (34 files over the limit).
4. The root file module `retirement` (`store_owners`, `impl RetireOwned for FlowMutation`) was named as `crate::retirement::...` by the editor and the viewer: a kept root file module cannot be reached from a lower crate (oracle blind spot, caught by the layout differential).

## 4. Source changes (no compatibility layers; every caller re-pointed in the same change)

1. New editor layers mounted in the artifact nest (`editor::flow::base`, `editor::flow::host`):
   * `✏️editor/🧱️base/🦀️.rs`: `FLOW_PLAY_APP_ID`, the four window ids `FLOW_PLAY_WINDOW_{MAIN,GENERATIONS,GENERATE_FORM,GENERATE_PREVIEW}` (they left their windows, which removed the `main::transient` <-> generate windows and `config` -> main window cycles), `flow_action`, `ui_value_*`, `ui_node_list`, `pick_item`, the interaction ids and target helpers, the store capacities and `FLOW_GRAPH_OPERATION_RAW_BYTES`.
   * `✏️editor/🖥️host/🦀️.rs`: `seed_host_catalogue`, `apply_canvas_options`, `cold_grant`, `host_with_session`, `host_from_snapshot`, `with_host_from_snapshot`, `with_live_host_snapshot`, `flow_composed_content`, `flow_content_leaves_emit`, `flow_removal_leaves`, `sync_host_selection*`, `focus_selection_camera`, `value_fault` (the two `pub(crate)` items are `pub`).
   * The editor root keeps `FlowCommand`, the context menu, the five retained routes, `FlowPlayApp`, the manifest and its tests, and imports the moved names (`extract-base.py`, `apply-root.py`); 25 callers were re-pointed by `repoint.py`, the window ids by `window-ids.py`.
2. Artifact root (`🦀️.rs`): `flow_working_scene`, `flow_genesis_content_pack` and `flow_composed_snapshot` moved into a new `🔖️SnapshotBridge` region (`split-bridge.py`); `FlowSnapshot::to_host_snapshot` reads `flow_working_scene_for_handle(&self.content)`. `widget_from_node` and `flow_content_child_from_digest` are `pub`.
3. `♻️retirement` is mounted inside `standards::v1::subsets::any::schema` (`schema::retirement`), the editor root and the viewer name `crate::schema::retirement::store_owners*`.
4. `flow_eval_tick::tick_result` is `pub`; `FlowDirectStoreWork` (struct, `new`, `preview_off`) is `pub` for its retirement tests; the sqlite snapshot test imports `crate::FlowSnapshot`.

## 5. Gates so far

* Planner with `--no-test-aggregate`: exit 0, no problem (`r11-slices3.log`), 11 producer crates plus the facade.
* `new crates <pkg> --verify --oracle --no-test-aggregate --sandbox=/Users/ueli/.cache/r11/verify2` (`r11-verify2.log`): exit 0; trees equal, visibility 0, orphan 0, glue 0, unresolved crates 0; oracle native 1533/1533 and `--test` 2629/2681 with `onlyCandidate` 0; layout native 1533/1534 and `--test` 2629/2683 with `onlyCombined` 0. The first run (`r11-verify1.log`) found the three defects fixed by section 4 items 3 and 4.

## 6. Applied state (final)

Sequence: first apply (`r11-apply1`, default target) passed `--check-applied`, then reverted with `r11-tools/revert-apply.py` from `F/🗑️generated/r11-backup-flow` (tree byte-identical to the pre-carve snapshot `/Users/ueli/.cache/r11/flow-pre`, checked with `diff -rq`) so the plan could use a lower packing target. Final: `--verify --oracle --no-test-aggregate --target=28/200000/3000` (`r11-verify3.log`, exit 0, oracle and layout 0 extras), apply `r11-apply2.log` (133 files, 11 crates, owner rows, backup `r11-backup-flow2`), then the two crate names derived from their first slice were renamed repo-wide by `r11-tools/rename.py` (copy of the bd9 rename tool that also reads untracked files): `-flow` -> `-editor`, `-connect-media-ports` -> `-editor-leaves` (directories keep their place).

* `--check-applied --base=dir:/Users/ueli/.cache/r11/flow-pre` (`r11-check3.log`, after the rename): exit 0; native 1533 base / 1534 combined, `--test` 2629 / 2683, `onlyCombined` 0 / 0; `admission`, `shared`, `sharedForeign` empty. The base must be `dir:` because git HEAD lacks the layering edits; no git command modified anything.
* Census (`r11-census-applied2.out`, cgc7 reader, host cfg; lib / test files/bytes/lines), before: one crate 131/530,825/8,480 and 216/910,549/12,900. After: schema 28/67,812/810 / 29/69,065/832; diff 5/5,686/128 / 6/8,615/182; inferences (holds `base`) 26/52,166/825 / 28/53,276/852; tests 13/28,433/579 / 19/41,824/832; config 7/11,009/239 / same; mutations (holds `host`, simple commands) 27/65,150/1,249 / 28/65,988/1,266; editor-leaves 25/84,947/1,526 / 29/95,341/1,704; viewer 10/41,811/765 / 16/57,826/1,105; editor (top) 4/201,206/2,973 / 5/205,352/3,047; original-command-retirement-tests 2/9,090/141 / 9/94,723/1,383; unit-tests 2/14,311/216 / 27/249,445/3,890; facade 1/17,974/309 / 4/48,595/605. `crates 12 over-or-error 0`.
* Headroom: `unit-tests` is a test-only crate at 3,890 of 4,000 test lines (0.97) and 249,445 of 262,144 bytes (0.95). It cannot be split by the planner: every hoisted test names the `cfg(test)` harness `unit_tests::context` (and a cross-crate `cfg(test)` module is forbidden), so they form one component whatever the packing target is (`--target` with 3,000 lines changed nothing there). A split needs the harness behind the `testkit` idiom in its own crate; left for CRATE-HEADROOM.
* Repo checks (`r11-checks2.log`): both launch JSON files parse (no new command, no launch route needed); owners scan: no flow row missing; taxonomy `flow`: only the existing plugin-root manifest finding; `lib-wiring-budget`: no flow failure after I lowered the ratchet pin of the flow facade root from 783 to its exact 315 lines (`🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`); the remaining failures (sequence, writer, bim, pdf, raster, jack content, generation2d) are foreign.
* cfg(test) cross-crate scan (`ctc-tools/ctc-scan.ts --only` the 12 crates, `r11-ctc2`): 0 hits over 67 gated pub items.
* Oracle blind spot (kept root file modules): the facade keeps only `art_flow_demo_tests`, `tests` and `hoisted_snapshot_0` (all `cfg(test)`); `git grep` finds no `crate::<kept-root-mod>::` path; `retirement` was the real case and moved (section 4 item 3).

## 7. Generator changes (with test)

`new crates --target=<files>/<bytes>/<lines>` (`parseCrateBudget` in `🗺️plan`, exported by the crate-tree index, passed as `PlanOptions.target`); test `crate budget flag` in `L/🧪️tests/📦️crate-tree/🟦️.ts` passes. `--no-test-aggregate` is required for this artifact (section 3 item 2).

## 8. Open items and risks

* Not compiled (compiler ingress): privacy and type-check beyond the rustc resolution oracle are unverified. Compile queue: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-flow-flow{,-schema,-diff,-inferences,-tests,-config,-mutations,-editor-leaves,-viewer,-editor,-unit-tests,-original-command-retirement-tests} --tests --features testkit --message-format=short`.
* `Cargo.lock` of the hub (`🌎️hub/Cargo.lock`), `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.lock` and the other workspace locks that list the flow facade were not refreshed (same as the other carves); only the artifact lock was.
* Crate names stay descriptive of packing, not layers: `-inferences` holds `base`, `-mutations` holds `host` and the simple commands, `-viewer` holds editor windows and four commands, `-editor-leaves` holds the remaining commands, windows and panels.
* `FlowDirectStoreWork` is a public struct now (its retirement tests are hoisted out of the top crate).
* Debug logs: none added; the scratch sandboxes under `/Users/ueli/.cache/r11/` hold only copies.

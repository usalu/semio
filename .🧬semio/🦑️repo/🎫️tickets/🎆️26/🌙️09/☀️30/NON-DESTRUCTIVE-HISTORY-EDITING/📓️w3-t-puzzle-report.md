# W3-T-PUZZLE — Puzzle 3d / 5d Gestures as Tool Machines

Executor W3-T-PUZZLE, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Scope: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d` and `🖐️5d`.

Paths below are relative to the subset roots:

- `P3` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `P5` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any`

## 0. Status

**The source conversion is complete for both artifacts. Compile and test verification is PENDING (§7).** The puzzle crates cannot be compiled right now because of a non-fleet peer's `ArtifactSqliteSnapshot` codec rollout (§5).

- **Puzzle 3d: complete in source.**
  - The gumball verbs `translateSelection`, `rotateSelection` and `scaleSelection`, the Relocate-utility drop `worldRelocate`, the target-volume gumball `relocateTargetVolume` and the inspector origin nudge `patchInspector` all commit through `transform_tool` as ONE `ToolTransaction` of relative leaves.
  - The HostOnly `transformBegin`/`transformEnd` handlers are **deleted**, together with their command variants, retained ids, extents, publication contracts, tool list, manifest actions and the `🔏️publication-authority` fixture entry.
  - No Rust or TS source in `✏️s/🔌️plugins/🧩️puzzle` names those verbs any more. The one exception is the negative law `gumball_gestures_declare_no_bracket_verbs`. Only the generated descriptors and stale wasm still mention them (§6).
  - W3-T-SPATIAL (`aac13b60d073dd9a1`) was told it can drop the bracket half of `GUMBALL_GESTURE_BRACKET_ACTION_IDS`.
- **Puzzle 5d: complete in source.** The same verbs, plus `patchPart` `x`/`y`/`origin.*` nudges, the engagement `move`/`rotate`/`scale` submits and `applyBoardEvents` board drags, all commit through 5d's `transform_tool`.

## 1. Census — every gesture that wrote absolute poses

| Artifact | Gesture (verb / event) | Before | After |
|---|---|---|---|
| 3d | gumball `translateSelection` | `Puzzle3dTranslateWork`-style absolute `move-object`/`move-target-volume` + coalesce key `gumball-translate` | `drag-selection` leaf, one `ToolTransaction` |
| 3d | gumball `rotateSelection` | absolute `rotate-object` (+ `gumball-rotate` coalesce) | `rotate-selection` leaf |
| 3d | gumball `scaleSelection` | `Puzzle3dScaleWork` absolute `scale-object`/`scale-target-volume` | `scale-selection` leaf |
| 3d | `transformBegin` / `transformEnd` bracket | empty HostOnly handlers + manifest/publication entries | deleted (host sends one release delta) |
| 3d | Relocate utility drop `worldRelocate` | `Puzzle3dWorldRelocateWork` absolute origin + re-derived attractions | `drag-selection` + `connect-vortices` (deterministic ids) |
| 3d | target-volume gumball `relocateTargetVolume` | `Puzzle3dRelocateVolumeWork` absolute after-pose | relative motion `before → after` as one selection leaf |
| 3d | inspector `patchInspector{origin.x/y/z, delta}` | absolute origin write | `drag-selection` |
| 5d | gumball `translateSelection` (also the typed `move dx dy [dz]` engagement submit) | `Puzzle5dTransformWork` absolute `move-part3d` + `move-part2d` (coalesce key) | `drag-selection3d` leaf (carries the board pin) |
| 5d | gumball `rotateSelection` (also `rotate deg`) | absolute `rotate-part3d` | `rotate-selection3d` |
| 5d | gumball `scaleSelection` (also `scale f`) | absolute `scale-part3d` / `scale-target-volume` | `scale-selection3d` |
| 5d | world drop `worldRelocate` | `Puzzle5dWorldRelocateWork` absolute `move-part3d` + `connect-grips` with `DefaultHasher` ids | `drag-selection3d` + `connect-grips` (deterministic ids) |
| 5d | target-volume gumball `relocateTargetVolume` | `Puzzle5dRelocateVolumeWork` absolute after-pose | relative `drag-`/`rotate-`/`scale-selection3d` |
| 5d | board drag (`applyBoardEvents` `gesture{kind: drag}`) | `DragMove`/`FindMovePart` stages pushing absolute `move-part2d` per target | `drag-selection2d` leaves, ONE transaction for the batch |
| 5d | inspector `patchPart{x/y/origin.*, delta}` | absolute `move-part2d`/`move-part3d` | `drag-selection2d` (board) / `drag-selection3d` (world) |

Neither artifact binds keyboard nudges; the 3d and 5d grep for `nudge`/`Arrow*` finds only inspector steppers. Neither sets `gumballLiveDispatch`, so both follow protocol (1) from W3-T-SPATIAL: the local preview stays unchanged, and one pose delta without `phase` is sent on release. Puzzle 2d's select tool belongs to W2-D and was not touched.

## 2. Design

- **Tool shape.** Each artifact has its own `transform_tool` statechart. It lives in its Transform utility module: `P3/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️.rs` and `P5/…/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs`.
  - The chart has a single `idle` state with `on Records if records_apply => idle do yield_records`. A host dispatches one delta on release, so every event is a one-shot `ToolTransaction`: it yields upserts, then `ToolYield::Commit`.
  - The chart keeps no tool state between events, and it needs no window-transient persistence.
  - The tool id is `<appId>#<verb>`, e.g. `s.puzzle.puzzle5d@1/*#editor#translateSelection`.
  - The ref is minted by `ToolMachineRunner` from the admission's `authoring_seed` and the host clock. A transaction is only stamped when a seed exists. The retained works and the `handle` path get it from `request.authoring_seed` / `doc.operation_optional()`.
- **Records.**
  - `Puzzle3dSelectionRecord{targets, motion, attractions}` uses `Puzzle3dSelectionMotion::{Drag, Rotate, Scale}`. Its parsers are `from_gumball` (`dx…`, `ax…angle`, `sx…`) and `from_pose_delta` (offset, `after·before⁻¹` turn, per-axis ratios).
  - 5d reuses that motion vocabulary as `Puzzle5dSelectionMotion::{Board{dx,dy}, World(Puzzle3dSelectionMotion)}` with `Puzzle5dSelectionRecord{targets, motion, fastenings}`. `admissible`/`moves` now live on `Puzzle3dSelectionMotion`, so both artifacts share one definition.
- **Relocate.** A world drop is a drag: the offset is the position minus the base origin.
  - 3d adds `(attracting, attracted)` vortex pairs within the window's proximity radius. Their six parameters are derived from the moved poses.
  - 5d adds `(moved grip, peer grip)` fastenings within the session `proximity_radius`. Zero offsets are kept, as the old drop wrote them.
  - Ids are pure functions of the pair and the document, never process counters: `attraction-<a>-<b>[-n]` and `fastener-<source>-<target>[-n]`.
- **Leaves.**
  - 3d: `drag-selection`, `rotate-selection`, `scale-selection`.
  - 5d: `drag-selection2d`, `drag-selection3d`, `rotate-selection3d`, `scale-selection3d`.
  - All are relative and parametric. Their diff reads the base values, and each target is transformed in place about its own origin.
  - Missing or locked targets give `mutation.partial`. No survivors give `mutation.target-missing` (Error). An identity transform gives `mutation.no-op`. Empty or repeated ids, non-finite values or factors ≤ 0 give a Fatal `mutation.invariant`.
  - The inverse is exact absolute setters, never a negated delta.
  - 5d `drag-selection3d` moves the board pin by `dx/F, −dy/F` with `F = PUZZLE5D_FLAT_TO_WORLD = 1/48`, as the live code did. `drag-selection2d` moves only the flat pose.
- **Refusals.** If nothing named exists, the result is the `nothing_selected` notice. If every addressed target is locked, it is the `selection_locked` notice. A motionless gesture leaves zero trace. When some targets are locked, the movable rest is yielded and the leaf reports the skipped ones as `mutation.partial`.
- **Retained works.**
  - 3d: `Puzzle3dTransformWork`. 5d: `Puzzle5dTransformWork`. Each runs `Read`, then `Commit`, then `Complete`, with extent `Some(2)`. They replace six bespoke works in total.
  - The 5d board-events work collects every drag record of a batch. At `Complete` it runs them as ONE transaction and splices the leaves where the first drag stood. The transaction stamps the whole edit, as puzzle 2d does.
  - The mounted `handle_action_impl` paths no longer coalesce. Arms push the tool's mutations, and the epilogue publishes them ahead of the scene delta, stamped with the transaction.

## 3. Changes

### Schema (both artifacts)

- New leaf directories, each with `🦀️.rs`, `🔺️diff`, `↩️inverse`, descriptor `🔣️.json`, `🧬️schema/🔣️.json` (full `x-semio-ui`), `🦠️mutation/🟦️.ts` and per-case tests:
  - `P3/🧬️schema/🧬️mutations/{✋️drag-selection,🔄️rotate-selection,🔍️scale-selection}`
  - `P5/🧬️schema/🧬️mutations/{✋️drag-selection2d,🚚️drag-selection3d,🔄️rotate-selection3d,🔍️scale-selection3d}`
- Aggregates were updated: enum variants, KINDS (3d: 38, 5d: 39), re-exports, a `🔖️SelectionTransform` region with shared diff/inverse helpers, the aggregate `🔣️.json` oneOf, the `🟦️.ts` union, the three grammars, and binary tags (3d 35–37, 5d 35–38).
- `PUZZLE5D_FLAT_TO_WORLD` moved into the 5d schema, and the editor re-exports it. `typed_arc()` was added on both play snapshots.
- Fixture quintets plus invariant fixtures live under `P3|P5/🧫️fixtures/🧬️mutations/<leaf>/`. The oracle catalogs are `P3|P5/🔮️oracles/🔣️.json`.
- The mutate cases `🧊️mutate-puzzle-3d-1` and `🖐️mutate-puzzle-5d-1` have new `🥒️.feature` rows, updated KINDS, and an independent Python implementation (`🐍️.py`).
- The crate mount trees `🧊️3d/🦀️.rs` and `🖐️5d/🦀️.rs` were updated.
- `Cargo.toml` for both artifacts: `component-app-assembly` now pulls in `machine` (= `semio-framework-machine`, macros) and `semio-framework-tool-machine`.
- Store laws `a_drag_edited_in_history_replays_its_downstream` (3d) and `a_board_drag_edited_in_history_replays_its_downstream` (5d) edit a drag's payload in history and replay the downstream.

### Editor — puzzle 3d

- `P3/✏️editor/🦀️.rs`:
  - `Puzzle3dActionCtx` gains `base`, `authoring_seed`, `artifact_mutations`, `transaction`, `commit_gumball` and `commit_selection`.
  - `dispatch_step` stamps the transaction, and the coalesce key is gone.
  - Added `puzzle3d_inspector_origin_nudge`.
  - `Puzzle3dTransformWork` replaces `Puzzle3dScaleWork`, `Puzzle3dWorldRelocateWork` and `Puzzle3dRelocateVolumeWork`.
  - The `transformBegin`/`transformEnd` command variants, retained ids, extents, publication contracts, tool list, manifest actions and audience entries are deleted.
  - `puzzle3d_apply_translate/rotate/scale`, `scale_value_mul` and `puzzle3d_rederive_moved_attractions` are deleted.
- Command arms now commit through the tool: `🚀️translate-selection`, `🔄️rotate-selection`, `📏️scale-selection`, `🌍️world-relocate` (refuses locked or hidden objects), `🚚️relocate-target-volume` and `🩹️patch-inspector` (origin nudge).
- `P3/✏️editor/🧪️tests/🧪️transform-tool/🦀️.rs` is new: pure tool laws.
- `P3/✏️editor/🧪️tests/🔬️unit/🦀️.rs`:
  - The static law for transform routing was rewritten. The no-bracket law is `gumball_gestures_declare_no_bracket_verbs`.
  - The coalesce tests were replaced by one-edit/one-row/one-transaction laws, the two-gesture law, the rotate/scale law and the zero-trace law.
  - The Nakagin relocate tests now expect `drag-selection` + `connect-vortices` under `#worldRelocate`.
- Fixtures:
  - `P3/🧫️fixtures/🗄️retained-jobs/🔣️.json`: tool cursors `gestureRead → toolCommit → closeOwner`, brackets removed.
  - `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json`: the 3d brackets are removed.

### Editor — puzzle 5d

- `P5/✏️editor/🦀️.rs`:
  - `Puzzle5dActionCtx` gains `authoring_seed`, `artifact_mutations`, `transaction`, `commit_gumball` and `commit_selection`.
  - `handle_action_impl(authoring_seed, …)` publishes the tool's mutations ahead of the scene delta and stamps the transaction. The `gumball-*` coalesce key is deleted.
  - The callers pass the seed: `handle` uses `doc.operation_optional()`, `Puzzle5dWindowCommandWork` carries `request.authoring_seed`, and the bounded reducer passes `""`.
  - `puzzle5d_inspector_nudge` is new.
  - `Puzzle5dTransformWork` (tool-driven) replaces the old `Puzzle5dTransformWork`, `Puzzle5dWorldRelocateWork`, `Puzzle5dRelocateVolumeWork` and `PUZZLE5D_RELOCATE_GRIPS_PER_PART`.
  - Routing: `translate/rotate/scaleSelection | worldRelocate | relocateTargetVolume` and `patchPart` nudges go to that work.
  - `Puzzle5dBoardEventsWork::new(seed)`: the `DragMove`/`FindMovePart` stages are gone, the drag records are committed once at `Complete`, and the locked notice is kept.
  - The editor's duplicate `quat_mul`/`quat_from_axis_angle` now `pub use` the schema helpers.
- Utility and tool module `P5/…/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs` contains:
  - `Puzzle5dSelectionMotion` and `Puzzle5dSelectionRecord`
  - `puzzle5d_relocate_record`
  - the `transform_tool` chart and host
  - `puzzle5d_transform_tool_commit` and `puzzle5d_selection_yields`
  - `puzzle5d_minted_fastener_id`
- Command arms commit through the tool: `🚀️translate-selection`, `🔄️rotate-selection`, `📏️scale-selection` (the engagement `move`/`rotate`/`scale` submits go through these), `🌍️world-relocate`, `🚚️relocate-target-volume`, `🩹️patch-part` (nudges) and `🎲️apply-board-events` (drags).
- `P5/✏️editor/🧪️tests/🧪️transform-tool/🦀️.rs` is new. It covers the leaf transaction, the board drag, zero trace, the drop with its fastening and minted id, and the one-state chart.
- `P5/✏️editor/🧪️tests/🔬️unit/🦀️.rs`:
  - Static laws updated: board route with tool commit, window route with seed, and `selection_transform_hostile_static_law_rejects_one_grant_reducers_and_bypassed_tools`, which replaces the world-relocate cursor law.
  - The coalesce test was replaced by these laws: `one_gumball_translate_is_one_edit_one_row_and_one_transaction`, `two_gumball_gestures_are_two_transactions`, `a_board_drag_is_one_board_leaf_in_one_transaction`, `an_inspector_nudge_is_one_relative_transaction` and `a_motionless_gumball_release_leaves_zero_trace`.
- `P5/🧫️fixtures/🗄️retained-jobs/🔣️.json`:
  - The transform cursors are now `gestureRead/toolCommit/closeOwner`. The board cursors replace `dragMove`/`partScan` with `toolCommit`.
  - Hostile source mutations were re-anchored. Five transform mutations were added, and the world-relocate grip-cap mutations and vectors were removed.
  - The vectors now name the relative leaves and the transaction tools.

Code removed from the editors was saved for reference in `🗑️generated/w3-t-puzzle/removed-3d-works.rs` and `removed-5d-works.rs`.

## 4. Verification (commands run, results)

All cargo commands ran gated (`rustc < 8`) with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-w3t-puzzle`.

| Command | Result |
|---|---|
| `cargo test -p semio-s-artifact-puzzle-3d --lib -- <selection + mutations filter>` (2026-09-30 21:57) | **377 passed**, 0 failed |
| `cargo test -p semio-s-artifact-puzzle-5d --lib -- selection mutations::` (after the board-pin change) | **463 passed**, 0 failed |
| Self-check of the independent Python implementations `P3/🧪️tests/🧊️mutate-puzzle-3d-1/🐍️.py` and `P5/🧪️tests/🖐️mutate-puzzle-5d-1/🐍️.py` against every committed selection vector (run 2026-09-30 after the last vector regeneration) | 12/12 non-rejected vectors each: after and inverse match |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d` | 92/92 inputs of 38 leaves carry a UI descriptor, **0 findings** |
| `… schema mutation-payloads --under …/🧊️3d` | 54/54 payloads meet their leaf schema, 4 negative witnesses rejected, 38/38 leaves witnessed, **0 findings** |
| `… schema mutation-inputs --under …/🖐️5d` | 99/99 inputs of 39 leaves, **0 findings** |
| `… schema mutation-payloads --under …/🖐️5d` | 69/69 payloads, 4 negative witnesses, 39/39 leaves witnessed, **0 findings** |
| `tsc --noEmit --strict` over the 9 new or changed mutation `🟦️.ts` files (3d and 5d aggregates + leaves) | exit 0 |
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib --keep-going` (latest run 2026-10-01) | **blocked** (§5). The latest run reached `semio-s-artifact-puzzle-3d`, and type checking listed only peer errors: `Edit.verb` (since landed) and `Puzzle3dSnapshot: ArtifactSqliteSnapshot`. None was in my code, but borrow check never ran and 5d was never reached, so this is NOT proof that it compiles. |
| `bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d` (and `🖐️5d`) | **blocked**: the tool throws `Nested Cargo catalog digest drift` (`📚️library/🔍️discovery/🟦️.ts:5492`), a repo-wide catalog digest unrelated to puzzle |

Run the lints from cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` with a **repo-relative** `--under`. An absolute path silently discovers 0 leaves and reports 0 findings.

## 5. Open items

- **Compile gate pending (peer breakage).** The wasm32-wasip2 check of `semio-s-artifact-puzzle-5d --features component-app-assembly` compiles the 3d and 5d editors. Three runs never reached the puzzle crates, each blocked by a different peer:
  1. `semio-s-plugin-stdio` was missing from `workspace.dependencies` while a new hub composition crate referenced it.
  2. `semio-s-artifact-stdio-{las,dwg,gltf,deflate}` failed with `X: ArtifactSqliteSnapshot` not satisfied, and stdio-pdf with an unresolved `content` module. Puzzle 3d depends on these crates directly.
  3. `semio-framework-plugin`'s `🛠️tool-machine/🦀️.rs:326` failed with `typing_fold` not found on `A`.
  4. The latest run, 2026-10-01, reached puzzle 3d. It stopped there because the codec rollout now requires `Puzzle3dSnapshot: ArtifactSqliteSnapshot` for `store::ArtifactCodec::of` at `P3/🦀️.rs:56`; puzzle 2d and 5d have the same call.

  Per the coordinator, the rollout owner decides between a sqlite snapshot and `ArtifactCodec::bare`, so I did not touch it. The last full 3d editor check that got past its dependencies ran on 2026-09-30 22:13; its only errors were in my schema test file, which I then fixed. The puzzle 3d/5d editor tests have not run since the editor changes. Everything still to verify is listed in §7.
- **Rotate pivot.** The wgpu preview rotates the selection about the gumball pivot. The committed leaf rotates each target about its own origin, as the old committed path and the React preview did. A rotation of several targets therefore previews differently from what lands in the wgpu host.
- **No attraction re-solve.** Selection leaves move the targets only, matching the live retained path. Attractions and fasteners attached to moved items are not re-derived.
- **Board pin side effects.** `drag-selection3d` keeps the board pin in sync. As a result, a 5d world drop (`worldRelocate`) and an inspector `origin.x/y` nudge now also move the flat pose; before, they moved only the 3d origin. A locked part (`2d.locked`) now refuses a world drop; before, it moved.
- **5d relocate radius.** The drop now uses the session's `proximity_radius` setting (default `PUZZLE5D_PROXIMITY_RADIUS`), which the setting's own doc promises. The old code used the constant.
- **One-step scan.** The transform work reads and commits in two steps. The relocate proximity scan is therefore O(parts × grips) inside a single step; the old works paged it per part or grip with a 64-grip cap. 3d made the same trade earlier in this wave.
- **Lowpoly and other plugins** still have their own gumball brackets. They belong to W3-T-SPATIAL, who was told that puzzle 3d's handlers are gone.
- Pre-existing stale hostile anchors remain in both retained fixtures and are not in my scope. 3d: `Puzzle3dScalarConfigWork`, `Puzzle3dEngagement*Work`, among others. 5d: `apply_world3d_sun_action`, `SetBrushCandidateIndex`, `SetEngagementInput`.

- **Macro warning.** `machine::statechart!` emits `#[cfg(feature = "serde")]` into the user crate, so `semio-s-artifact-puzzle-3d` warns `unexpected cfg condition value: serde` at the chart. 5d will warn the same way. Other plugin tool charts (cad, flow, shooting) carry the same warning; the fix belongs in the macro.

## 6. Regeneration list (for the coordinator)

- Puzzle plugin descriptors `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` and `🛂️.descriptor.semio`. They still list `transformBegin`/`transformEnd`, and they lack the seven new leaf kinds.
- The schema catalog, which needs the new 3d/5d mutation leaves and their `x-semio-ui` inputs.
- Built wasm under `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/{dist,pkg}`, which is stale.
- `launch.json`: no new commands.

## 7. Verification pending (run once the puzzle crates compile again)

All commands run from `/Users/ueli/Documents/semio`, gated with `until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 8 ]; do sleep 20; done`, and prefixed with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-w3t-puzzle`.

1. **Wasm compile gate.** This covers both editors, because 5d's feature enables 3d's:
   `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib --message-format=short`
2. **Puzzle 3d crate tests.** This runs the editor laws, the tool laws, the leaf/store laws and the retained-fixture oracle:
   `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib --message-format=short`
   For a focused re-run of what this wave touched, append:
   `-- transform_tool selection gumball relocate bracket nakagin motionless drag_edited`
3. **Puzzle 5d crate tests:**
   `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib --message-format=short`
   For a focused re-run, append:
   `-- transform_tool selection gumball board_drag inspector_nudge motionless translate_selection_moves_both_poses board_node_delete retained hostile_static_law board_drag_edited`
4. **Whole puzzle plugin, as puzzle 2d activation builds it:**
   `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2 --message-format=short`
5. **Leaf tests without the feature** (already green before the editor work; re-run once):
   - `cargo test -p semio-s-artifact-puzzle-3d --lib --message-format=short -- selection mutations::`
   - `cargo test -p semio-s-artifact-puzzle-5d --lib --message-format=short -- selection mutations::`
6. **TS.** I touched no host TS file: World3dHost and Canvas2dGumballOverlay belong to W3-T-SPATIAL. The only TS I changed are the mutation interface files, which pass strict `tsc` (§4). Run the puzzle package's TS suite once with `bun nx run @semio-tech/puzzle-js:test`.
7. **Taxonomy,** once the repo-wide `Nested Cargo catalog digest drift` is resolved:
   `bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d`, then the same command with `--scope ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d`.

Expected failure modes to check first: borrow-check errors in the new 5d code (`Puzzle5dBoardEventsWork` `Complete` stage, `Puzzle5dActionCtx::commit_selection`), and source-scan laws whose literals were rewritten (§3).

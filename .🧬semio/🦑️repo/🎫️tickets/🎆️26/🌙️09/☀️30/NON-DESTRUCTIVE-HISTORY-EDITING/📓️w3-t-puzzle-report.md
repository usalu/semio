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

## Session 2 — 2026-10-01

Successor of W3-T-PUZZLE (S2-PUZZLE). This section is updated at each milestone. Paths are relative to `P3`/`P5` as above unless written in full.

### S2.0 Status (10-02 02:50, CARGO HOLD rule 26)

- **Source: done** for every resume item below. **Compile/test: WRITTEN BUT UNVERIFIED (peer breakage).** Every `cargo` run since 12:00 died in peer crates before reaching puzzle: first `semio-framework-os-kernel` (59 errors, `HistoryPageStack` mid-edit of the PAGED-ARTIFACT-HISTORY-LEDGER session), then `semio-s-artifact-stdio-gltf` (210–498 errors, `DslValue::Bytes` / `Mutation` trait rollout). Puzzle 3d depends on stdio-gltf directly. Last attempt 13:57, still blocked.
- **Verified without cargo:** both schema lints (0 findings, both artifacts), the independent Python oracles (every selection vector), a numpy third-party placement oracle, and `jsonschema` validation of the new time-travel corpus (§S2.4).

### S2.1 Repair (fleet rule 21)

- The predecessor's last edits were all in the 11:16 auto-commit:
  - the machine-derive codec opt-in (`event <Name>: serde`; tests 11/11 at 07:54);
  - the 5d editor at 07:40, which makes `Puzzle5dTransformWork::read` take the typed `&Puzzle5dSnapshot`, so the `crate::Puzzle5dSnapshot` import the 07:35 wasm check flagged as unused is now used.
- I read the code: neither edit is half-finished. Compile proof is pending.
- The predecessor's 07:35 `cargo check … puzzle-5d --features component-app-assembly --target wasm32-wasip2` had **Finished**, 0 errors, 5 warnings (`🗑️generated/w3-t-puzzle/check-wasm-5d.txt`). That covered the 3d and 5d editors before this session's edits.
- **Peer collision, resolved.** The REPO-PATH-BUDGET rename (`26/10/01/REPO-PATH-BUDGET`) renamed long fixture case dirs while I regenerated vectors:
  - 3d: `🎯️drags-object-and-volume`→`🎯️drags`, `🎯️turns-…`→`🎯️turns`, `🎯️scales-…`→`🎯️scales`;
  - 5d: `⚠️skips`, `🔄️turns`, `🔍️scales`, `🚚️drags`.
  - I adopted the new names in the author script.
  - I deleted only the stale copies of the old-named fixture dirs that my first regeneration had re-created.
  - I moved my three 3d selection schema-test twins to the new names, which the crate mount already pointed at. The peer later finished every other twin, and 0 mount paths are missing in 2d/3d/5d.
  - New cases use names of 12 bytes or less, following the peer's rule.

### S2.2 Changes

1. **Coordinator: set-active-example coalesce key removed.**
   - `PUZZLE3D_SET_ACTIVE_EXAMPLE_COALESCE_KEY` and its `coalesce_key` are deleted from `P3/✏️editor/🦀️.rs`. An example load is ONE document-replacement edit.
   - `✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` now asserts `coalesce_key == None`; the test is renamed `…_emits_one_uncoalesced_edit`.
2. **Rotate pivot parity (wgpu preview = commit).**
   - Every 3D gumball leaf turns and scales each target about its OWN origin: puzzle `rotate-selection`/`rotate-selection3d`, shooting `rotate-assets`, and the predecessor's leaves. React's instance preview does the same.
   - The wgpu world engine was the outlier. It orbited instance translations about the selection centroid without turning them, so a single object showed no rotation at all.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`: the new `gumball_preview_in_place` serves both `retained_gumball_preview_model` and the test-only `apply_gumball_preview`. A turn rotates the instance basis about the handle axis in place; a scale stretches the instance's own local axis.
   - New law `world_gumball_turn_and_scale_preview_each_instance_in_place` (`♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`).
   - No leaf change and no React change. Open: React's vortex-marker preview still rotates markers about the gumball anchor (S2-SPATIAL, `World3dHost` `worldVorticesWithGumballPreview`).
3. **Attraction re-solve on move (3d).**
   - The leaves (not the tool) now keep the document resolved, so editing a gesture in time travel re-solves on whatever base it replays.
   - `P3/🧬️schema/🧬️mutations/🦀️.rs`, new region `🔖️AttractionPose`. The placement kernel moved from the editor into the schema: `puzzle3d_attraction_child_pose`, `derive_attraction_params`, the vec/quaternion helpers and `puzzle3d_vortex_full_id`. The editor `pub use`s them, so there is no duplicate.
   - New `puzzle3d_selection_follow`. A drag or turn re-places every UNLOCKED object an attraction hangs off a moved object, breadth-first, using the kernel and the unchanged parameters, so subtrees follow exactly as `resolve_puzzle3d_attractions` would.
   - Every other attraction touching a moved object is re-derived from the moved poses. A scaling moves no pose and re-solves nothing.
   - The diff carries the object and attraction patches plus one Info `mutation.cascade` naming the followers and the re-derived attractions. The inverse adds exact `replace-attraction-geometry` restores.
   - Leaf diffs: `✋️drag-selection`/`🔄️rotate-selection` pass `follow = true`, `🔍️scale-selection` passes `false`. The schema descriptions say so.
   - Why the kernel and not a rigid carry: the numpy oracle showed the document kernel is not rigid-equivariant. Resolving after a rigid carry would snap followers, so followers are placed by the kernel.
   - **5d: nothing to re-solve.** No 5d code places parts from fastener parameters. World fastener lines are drawn grip to grip from part poses, and drops write zero parameters, as before.
4. **Store footprints declared per row.**
   - Puzzle 3d and 5d declared `work_items: 2` for every mutation. A selection leaf over N targets, or a removal with cascades, then failed the bounded publication with `batched item candidate failed its exact fixed fold contract`.
   - 3d: drag/rotate use `for_one_item(PUZZLE3D_SELECTION_INVERSE_ROWS)` (the one-item byte budget ÷ `size_of::<Puzzle3dMutation>()`). Scale uses `targets.len()`. `delete-object`/`remove-object-vortex` use `1 + 64`.
   - 5d: `drag-selection2d` N, `drag-selection3d` 2N, `rotate-`/`scale-selection3d` N, `delete-part`/`remove-part-grip` `1 + 64`.
5. **Paged relocate scan with progress and cancellation.**
   - `Puzzle3dRelocateScan` (3d `…/🧊️main/🪛️utilities/🔄️transform/🦀️.rs`) and `Puzzle5dRelocateScan` (5d `…/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs`) work as begin → step(page) → progress → finish. The connected or fastened set is indexed once.
   - `puzzle3d_relocate_record`/`puzzle5d_relocate_record` are now the one-call drive of the same scan.
   - Both transform works go `Read → Scan (one Progress per 16 objects/parts, en/de) → Commit`. `extent = 2 + pages`, and close releases the scan.
   - Laws:
     - `a_paged_relocate_scan_finds_exactly_what_the_one_call_scan_finds` (3d) and `a_paged_world_drop_scan_finds_exactly_what_the_one_call_scan_finds` (5d), over every page size;
     - `world_relocate_scan_pages_progress_and_cancels_with_zero_trace` (3d, Nakagin);
     - `world_relocate_extent_fits_within_cap_for_nakagin`, updated;
     - both hostile static laws now also require `…Stage::Scan` and `…RelocateScan::begin(document`.
6. **Machine-derive warning:** the predecessor's opt-in is in place, and both transform charts declare plain `event Event {…}`. Proof that the warning is gone needs the next wasm check (pending).
7. **Time-travel laws for every relative leaf (language-agnostic).**
   - Corpus: `P3|P5/🧫️fixtures/🧫️selection-time-travel/🔣️.json` (8 cases each), schema `P3|P5/🧬️schema/🔣️selection-time-travel/🔣️.json`.
   - Each case edits ONE recorded gesture's offset, angle, factors or targets, including a no-op edit, a partial (locked) edit and a blocking target-missing edit. It states the preview (state before + draft, nothing downstream), the fresh-fold replay, the per-mutation outcomes and the finalize verdict.
   - Rust laws `every_corpus_edit_previews_replays_and_overwrites_like_the_fresh_fold` (`P3|P5/🧬️schema/🧬️mutations/🧪️tests/🧪️selection-time-travel/🦀️.rs`) check, per case:
     - one gesture = one edit carrying its `TransactionRef`;
     - the `state_before` preview;
     - a cancelled replay leaves zero trace;
     - the Report-replay outcomes and `blocks_finalize`;
     - the replay equals the fresh fold;
     - the overwrite folds to it, supersedes the edited op and keeps every `TransactionRef`.
   - Editor law `a_gumball_drag_carries_its_attracted_objects_in_the_same_transaction` (3d, Nakagin): one row, one `drag-selection`, the follower moves, one undo restores both.
   - **React vs wgpu:** both hosts dispatch the same release verb with identical motion keys (`dx/dy/dz`, `ax/ay/az/angle`, `sx/sy/sz`; wgpu adds `surfaceId`/`windowId`, which the guest ignores). With the in-place wgpu preview, both hosts now preview what they commit, except followers: neither host previews attracted objects, and they land on commit in both.
8. **Vectors and oracles.**
   - 3d selection vectors were regenerated with the re-solve. 6 existing vectors changed, as intended: re-derived attractions and followers.
   - New: `✋️drag-selection/⛓️follow`, `🪢️cross`, `🔄️rotate-selection/⛓️orbits`, `🔍️scale-selection/⛓️stays` (on a resolved chain scene). They are mounted in `🧊️3d/🦀️.rs` and registered in `🔮️oracles/🔣️.json`.
   - The independent oracle `🧪️tests/🧊️mutate-puzzle-3d-1/🐍️.py` gained the re-solve; its docstring states the numeric port.
   - 5d vectors regenerate byte for byte; only the assertion-message template changed.
   - Ticket scripts: `🧪️w3-t-puzzle-author-vectors.py` (extended), `🧪️w3-t-puzzle-author-time-travel.py` (new), `🧪️w3-t-puzzle-oracle-selfcheck.py` (new).
   - `P3|P5/🧫️fixtures/🗄️retained-jobs/🔣️.json`: the `worldRelocate` cursor and boundary lists gain `proximityScan`.

### S2.3 Verification so far

| Command (cwd `/Users/ueli/Documents/semio` unless noted) | Result |
|---|---|
| `python3 T/🧪️w3-t-puzzle-oracle-selfcheck.py 3d` | 16/16 vectors: oracle after + inverse exact; 15 attractions re-placed by numpy within 1e-6; 8/8 time-travel cases schema-valid (`jsonschema` + `referencing`) and folded exactly by the oracle; **0 failures** |
| `python3 T/🧪️w3-t-puzzle-oracle-selfcheck.py 5d` | 12/12 vectors, 8/8 time-travel cases, **0 failures** |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | 92/92 inputs of 38 leaves, **0 findings** |
| `… schema mutation-payloads --under …/🧊️3d` | 58/58 payloads, 4 negative witnesses, 38/38 leaves witnessed, **0 findings** |
| `… schema mutation-inputs --under …/🖐️5d` | 99/99 inputs of 39 leaves, **0 findings** |
| `… schema mutation-payloads --under …/🖐️5d` | 69/69 payloads, 39/39 witnessed, **0 findings** |
| `bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d` (16:46) | exit 0; 17 findings (15 `directory-kind-unresolved`, 1 `path-too-long` in `🖌️brush/🧪️tests/🔬️unit`, 1 `projection-member-unresolved` `💾️binary`), all pre-existing; **none in a directory this WP added** (`🧫️selection-time-travel`, `🔣️selection-time-travel`, `🧪️selection-time-travel`, `⛓️follow`, `🪢️cross`, `⛓️orbits`, `⛓️stays`) |
| `… verify taxonomy report --scope …/🖐️5d` (16:52) | exit 0; 26 pre-existing findings (25 `directory-kind-unresolved`, 1 `projection-member-unresolved`); none in this WP's new directories |
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib` (12:00) | blocked: os-kernel, 59 peer errors |
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d --lib` (12:51, 13:06, 13:24, 13:57, 16:35) | blocked: stdio-gltf, 498 → 210 → 16 peer errors (`Option::to_value` in `📸️snapshot/📦️pack`) |
| same, `-j 4` (17:00 SIGKILL under swap; 17:28 `semio-framework-ui` `wgpu::layout`/`stepper` peer mid-edit; 17:41 cut by the usage limit; 21:37 `semio-framework-plugin` `HistoryPatch.remote_replay` missing, peer mid-edit) | blocked by peers; no error in a puzzle file was ever reported |
| `cargo check -j 2 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d --lib --message-format=short` (23:14 → 23:24) | **Finished**, 0 errors. The new schema code (re-solve, pose kernel, footprints) type-checks. 1 warning: the schema root's test-only duplicate `puzzle3d_vortex_full_id` (`🧬️schema/🦀️.rs:927`), now deleted; `🧬️schema/🧪️tests/🔬️precompute-model` uses the mutations one. Editor/tests not covered (no feature, no test target) |

Gimbal tolerance: at a tilt of ±90° the kernel's `asin` resolves the angle only to about 1e-7, so the numpy placement check uses 1e-6 (documented in the script).

### S2.4 Open / coordinator actions

- Re-describe the puzzle plugin after this lands. The descriptor still lists `transformBegin`/`transformEnd` and lacks the 7 selection leaf kinds. Central `schema generate` is needed for the new `🔣️selection-time-travel` schemas and the changed 3d leaf descriptions. Re-activate puzzle (stale wasm).
- The obsolete `#![allow(unexpected_cfgs)]` workarounds for the old macro behaviour can go once a wasm check shows the warning gone: shooting `…/🎮️commands/🧭️gumball/🦀️.rs:10` (S2-SPATIAL) and `🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/🦀️.rs:13` (S2-DRAW / hub owner).
- S2-SPATIAL: React vortex/attraction marker preview of a multi-object turn orbits the gumball anchor (`worldVorticesWithGumballPreview`). It should turn each marker about its owning instance's origin.

### S2.5 Verification still to run (after rule 26 CARGO HOLD lifts)

One at a time, each gated (`until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 8 ]; do sleep 20; done`), from `/Users/ueli/Documents/semio`. Tests use `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s2-puzzle`.

1. `cargo check -j 2 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib --message-format=short` — both editors, and proof the `unexpected cfg serde` warning is gone.
2. `cargo test -j 2 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d --lib --message-format=short -- selection mutations:: selection_time_travel` — leaf vectors, the re-solve vectors and the time-travel corpus law.
3. The same for `semio-s-artifact-puzzle-5d`.
4. `cargo test -j 2 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib --message-format=short -- transform_tool selection gumball relocate bracket nakagin motionless drag_edited example coalesce scan` — editor and tool laws.
5. The 5d twin: `-- transform_tool selection gumball board_drag inspector_nudge motionless retained hostile_static_law board_drag_edited scan`.
6. `cargo test -j 2 -p semio-framework-os-infinite --lib --message-format=short -- world_gumball` (root workspace) — the in-place preview law.

## Session 3 — 2026-10-02

Successor S3-PUZZLE. Paths are relative to `P3`/`P5` as above unless written in full; `P2` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

### S3.0 Status (updated at every milestone)

- **Activation blocker (b): FIXED (data) and proven for 3d** (law 4/4 green, 11:53); 2d + 5d law runs pending. `main` notified 11:55.

### S3.1 Activation blocker (b) — descriptor probe panic `concrete-forest example dsl parses`

- **Symptom** (`🗑️generated/e2e/activate-retry-17.log`): `materialize-dev` → descriptor probe → `P3/📚️examples/🌲️concrete-forest/🦀️.rs:31` panics `expected LBrace, found Ident 'id' at 5:140` → wasm `unreachable`.
- **Root cause: a peer DSL grammar change, not puzzle code.** Since 2026-10-01 ~20:08 (uncommitted peer work in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs`, `parse_shape`/`print_shape` `Shape::List` arms; law `🗣️dsl/🧬️schema/🧪️tests/🧾️record-list` "canonical braces preserve ordered optional and empty record-list items") every element of a `Shape::List(Shape::Record)` is read and printed **braced** (`[ { k=v } { k=v } ]`). The old printer wrote them bare (`[ k=v k=v ]`). All 7 puzzle example DSL fixtures (2d ×2, 3d ×2, 5d ×3) were authored in the bare form (last touched 09-02…09-17) and nobody migrated them. 2d (`handles`), 3d (`representations`, `vortices`, …) and 5d (`grips`) all broke; 3d's concrete-forest is simply the first the probe builds. The example data was never changed; the editor/leaf/schema code is not involved.
- **Fix (data, all at once):** ticket script `🧪️s3-puzzle-brace-record-lists.py` rewrites every bare record list into the braced canonical form with the OLD parser's exact boundary rule (`parse_record_fields`: a record ends at the first key it already consumed). Bare nested record fields inside list elements (5d `grip-2d=angle=…`, `grip-3d=position=…`) stay bare — the grammar still prints `Shape::Record` fields bare — and their extent follows the same rule with the key sets of `Puzzle5dGrip2d`/`Puzzle5dGrip3d`. Self-check built in: deleting the inserted braces gives back the source byte for byte.
  - 2d concrete-forest 1 list / 11 records; 2d nakagin 180 / 358; 3d concrete-forest 5 / 35; 3d nakagin 204 / 410; 5d concrete-forest 1 / 11 (+22 nested); 5d nakagin 180 / 358 (+716 nested); 5d capsule-dream 2880 / 5824 (+11 648 nested).
- **New law (native, every registered example, 2d + 3d + 5d):** `every_registered_example_builds_its_document` in `P2|P3|P5/🧪️tests/🧪️every-example/🦀️.rs`, mounted from each subset root `P2|P3|P5/🦀️.rs` (`#[cfg(test)] mod every_example_tests`). It enumerates the subset's own `examples()` registry (the list `SubsetDeclaration.examples` hands the descriptor), so a new example cannot escape. Per example: the authored deferred DSL parses, `ExampleSource::document_json()` (the producer the probe runs) decodes to the same snapshot, and the snapshot round-trips through DSL and pack. Needs `--features component-app-assembly` (the subset root and `examples` are gated by it).

### S3.2 Verification (cwd `/Users/ueli/Documents/semio`; cargo gated `rustc < 14`, `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s3-puzzle` for tests, `--manifest-path ✏️s/Cargo.toml`)

| Command | Result |
|---|---|
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- every_registered_example dsl_asset_parses puzzle3d_example_fixtures` (11:19 → 11:53, 33 min cold) | **4 passed, 0 failed** (`every_registered_example_builds_its_document`, both `dsl_asset_parses_and_round_trips`, `puzzle3d_example_fixtures_parse_and_round_trip_as_dsl`); no `unexpected cfg` warning in the native test build |
| `python3 T/🧪️w3-t-puzzle-oracle-selfcheck.py 3d` / `5d` | 3d: 16 vectors, 15 numpy placements, 8 time-travel cases, **0 failures**; 5d: 12 vectors, 8 cases, **0 failures** |
| `bun ./📜️script.ts schema mutation-inputs\|mutation-payloads --under ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d\|🖐️5d` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | 3d 92/92 inputs, 58/58 payloads, 38/38 witnessed; 5d 99/99, 69/69, 39/39; **0 findings** each |
| `bun ./📜️script.ts verify taxonomy report --scope <P2\|P3\|P5>/🧪️tests` | 3d, 5d clean; 2d 2 pre-existing `directory-kind-unresolved` (`🌐️third-party-puzzle-2d-1`, `🕸️third-party-puzzle-2d-1`), none for `🧪️every-example` |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib` (whole suite, 12:00 → 12:32, peer `semio_framework_schema` → `…_schema_registry` rename forced a rebuild) | **899 passed, 9 failed** — analysed in S3.3 |
| `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib` (12:38) | **blocked**: peer mid-edit, `semio-framework-schema-registry` 28 errors (`ArtifactSchemaRegistry` defined twice, unresolved `semio_framework_os_kernel`, …) |

### S3.3 Puzzle 3d suite — the 9 reds

| Test | Cause | Action |
|---|---|---|
| `selection_time_travel::every_corpus_edit_previews_replays_and_overwrites_like_the_fresh_fold` (3d) | **Harness, not leaves.** The law decoded the corpus through `serde_json::Value` (no `float_roundtrip`), which lands `1.9999999999999993` one ulp low (`…91`); the Rust leaves produce exactly the corpus value. | Both 3d and 5d corpus laws now parse with the framework JSON reader (`dsl::json::parse`, correctly rounded `str::parse::<f64>`). |
| `a_gumball_drag_carries_its_attracted_objects_in_the_same_transaction` | Test premise: the shipped 3d Nakagin has NO attraction (empty table); the predecessor's law was never run. | The law first attracts the compatible door pair `25b0dba0-…:link` → `5f0266bc-…:sl0_d0` (both unlocked), then drags. |
| `relocate_target_volume_undoes_and_redoes_as_one_mutation` | Test premise: `addTargetVolume` grid-snaps its origin, so the gumball's `before` (1,2,3) was not the volume's pose; the relative leaf correctly moved the real pose by (3,3,3). | `before` = the volume's real pose, `after` = before + (3,3,3). |
| `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` | Store units per translate: 22 (concrete forest, 1 object) vs **1590** (Nakagin, 180 objects); delete the same. The ladder is supposed to be document-independent. Nakagin could not load before this session's example fix, so this is the first run since the store changes of session 2. | **Open — owner guess S3-W1G** (store publication / prefix-snapshot ring per edit O(document)); routed to the coordinator. |
| `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes` | The second announcement of an already-requested mesh widens the scope again: `request_mesh_reupload` answered "new" twice, i.e. the standing request set did not survive the render between the two dispatches (precompute session swap). | **Open** (puzzle 3d brush/precompute session keying; not touched by this WP since session 1). |
| `every_maintenance_unit_stays_inside_the_interactive_step_budget`, `brush_suggestions_run_step_stays_below_the_interactive_ceiling_for_nakagin`, `penetration_of_flush_thousand_triangle_parts_stays_interactive`, `fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin` | Wall-clock laws (8 ms / 2 ms ceilings) on a debug build with 20–30 peer rustc running; the fill law's replay diverges once the recording run is clock-starved (93 vs 4249 turns). | Re-run in isolation when the load drops; not a code change. |

### S3.4 Coordinator follow-ups (12:25)

- **G7 `labelHandwritten`:** puzzle 3d `Puzzle3dSetActiveExampleWork` no longer sets `Emit.description` (`PUZZLE3D_SET_ACTIVE_EXAMPLE_DESCRIPTION` deleted); the row is labelled from its leaves. `🔬️example-switch` asserts `emit.description == None` (2 sites). 5d has no hand-written description.
- **Closure census §(b) S3-PUZZLE:** B none, V none. The M items (Edit literals `coalesce_key: None`, 5d retire `puzzle5d_retire_optional_string_step(&mut owner.coalesce_key…)`, test asserts) die with the API in S3-CLOSURE's mechanical sweep; the F items wait for CLOSURE's derived-footprint helper (none exists yet: `ArtifactStoreOneItemFootprint` has only `for_one_invertible_item`/`for_one_item`). Nothing behavioural is left for this WP.
- **N3 hook (chips + preview highlight), puzzle 3d and 5d — written, compile pending:**
  - 3d `puzzle3d_entity_label` (typed snapshot; object = outliner label → kind catalog label/name → kind id; vortex `<object> · <vortex label|kind>`; attraction `<object> → <object>`; target volumes/references keep the generic label) + `ArtifactEditor::entity_label`.
  - 3d highlight: `Puzzle3dInteractionSnapshot.referenced` (from `InteractionView::draft_references`), `Puzzle3dInstanceResidency::refresh(…, referenced)` stamps `"highlighted": true` on exactly the referenced instance records (fingerprinted, so open/close of a draft republishes only those records through the delta lane); both hosts already paint the `highlighted` instance row.
  - 5d `puzzle5d_entity_label` (part = volume label → flat text → kind; grip `<part> · <grip kind>`; fastener `<part> → <part>`) + `entity_label`; world instances carry `highlighted`, the board scene's `highlighted_ids_json` = `Puzzle5dInteractionSnapshot::referenced_json()` (was a constant `[]`).
  - Laws: 3d `history_edit_reference_chips_name_entities_as_the_outliner_does`, `a_history_edit_draft_highlights_exactly_the_objects_it_references` (app level: drag → `historyEditBegin` → world highlights the target → `historyEditExit` clears); 5d `history_edit_reference_chips_name_entities_as_the_outliner_does`, `world_instances_highlight_the_parts_a_history_draft_references`, `board_paints_the_live_selection_hover_and_history_draft_references`.

### S3.5 Resume after the usage cut (~13:05) and the machine reboot (~17:00) — 18:40

- Every S3 edit is on disk and was swept into the 17:04 auto-commit (`git status` clean for `✏️s/🔌️plugins/🧩️puzzle`): the 7 braced example DSLs, the three `🧪️every-example` laws and their mounts, the two time-travel corpus laws on `dsl::json::parse`, the two 3d test-premise fixes, the G7 description removal, and the complete N3 change set for 3d and 5d (including the 5d world + board highlight assertions that were the last edit before the cut). Nothing was half-written.
- `🛂️manifest/🦀️.rs` now names `semio_framework_schema_registry` (peer schema split, 14:21); cold rebuild of `semio-framework-plugin` started 18:41 to see whether the split is green.
- 19:17 the root-workspace kernel checked green once, but the `✏️s` workspace kernel is red (278 errors: `🚪️io` `dsl::Diagnostic`, `📡️spr/🧵️channel` `crate::Fault*`, store/vcs) — the Codex peer's `semio_framework_dsl` extraction; the coordinator owns the "TREE GREEN" signal, so no more polling from this WP.
- **N3 correction — React did not paint the guest highlight.** `World3dHost` overwrote the record's `highlighted` with its own catalog-kind hover (`{ ...instance, highlighted: chrome.highlighted }`), so a draft-referenced 3d object or 5d part would have highlighted on wgpu only. New pure `worldInstanceHighlighted(hostHighlighted, instance)` (`📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`) ORs the two and is used at the instance mesh; the `WorldInstanceRecord.highlighted` doc names both sources. wgpu already reads the record flag (`♾️infinite/🌍️world` `highlighted_instance_ids`). Law in `🧪️tests/🔬️engine-contract/🟦️.ts` ("paints the guest-stamped highlight …"), imported straight from the host module (the renderer barrels are untouched).
  - `cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run "--testNamePattern=guest-stamped highlight|resolves mesh style by priority"` → **2 passed**, 696 skipped (19:37).
- `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes` (S3.3): the puzzle 3d session registry is process-global and both `check_out` and `check_in` use `try_lock()` plus a process byte cap (`PUZZLE3D_SESSION_PROCESS_BYTES`); under the 900-test parallel run a contended or capped check-in drops the standing re-upload set, so the second announcement looks new. Expected to pass alone — to be re-run in isolation once the tree is green.
- **Resume item "multi-object rotate preview/commit pivot parity" — React marker half (routed to S2-SPATIAL in S2.4, never picked up).** `World3dHost` previewed vortex markers and attraction ends of a gumball turn by orbiting them about the gumball pivot, while the instances (and the committed `rotate-selection`/`scale-selection`, and the wgpu preview since S2.2) turn each object about its OWN origin; a scale did not move markers at all. Now:
  - `worldGumballOwnerPoses(instances)` (drag-start origin + orientation per instance) and `gumballPreviewOwnedWorldPoint/Direction(owner, …)`: translate adds the offset, a turn rotates about the owner's origin, a scale stretches along the owner's local axes (`R·diag(s)·R⁻¹` about the origin; directions renormalized) — the exact counterpart of `applyGumballLivePreviewDeltaToPose`.
  - `worldVorticesWithGumballPreview`/`worldAttractionsWithGumballPreview` take the owner map (the component memoizes it from `instances`); the pivot-orbit helpers `gumballPreviewWorldPoint`/`gumballPreviewWorldDirection` are deleted together with their re-exports in `🎯️targets/⚛️react/🟦️.tsx` (no other user).
  - Law (`🔬️engine-contract`): "previews a multi-object turn and scale of markers about each owner's origin, where the selection leaves land them" (two owners, one turned 90°; a marker of a third, unselected object stays the same object; attraction ends follow; local-axis scale of a point and a direction). The existing translate assertion moved to the owned helper.
  - **Run blocked (19:41):** a peer's 19:38 edit of `🧬️schema/🧫️fixtures/🧬️vendor-annotation-vocabulary/🔣️.json` makes every strict-Ajv suite fail at load (`strictRequired` on `#/not/anyOf/0` `bounded`); reported to the coordinator. The highlight law passed at 19:37, before that edit.
  - Re-run after the peer's 19:44 fixture fix: `bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run "--testNamePattern=guest-stamped highlight|resolves mesh style by priority|multi-object turn and scale of markers|gumball rotate handles commit rotateSelection|gumball transform preview"` → **5 passed**, 0 failed (19:45).
- Typecheck of the two TS files: `bunx tsc -p T/🧪️s3-puzzle-typecheck-world3d.tsconfig.json` (19:44; World3dHost + `🔬️engine-contract`, extends the React renderer tsconfig) → 14 errors, **none in `World3dHost` or in any line this WP touched**; all pre-existing peer ones: `🏪️store/👷️worker` (3776, 3842) and `🏪️store/🔄️sync/…/🔬️backbone-parity` (77) `HistoryOpMeta.line` missing, `🧪️tests/🔌️plugin-runtime` 1883 + `🏛️ShellHost` 6835 `loadAppDocumentPack`, `🐚️Shell` 1112 `idleInstalledServiceStatusV1`, `🔬️engine-contract` 4788–4865 UI `Component` literals. The package target `bun ./📜️script.ts typecheck` (renderer react, which does not include the engine tests) reports only the first 6 of those.

### S3.6 Resume — TREE GREEN (10-03 05:50)

- The interrupted edit (owner-pose map typing in `worldGumballOwnerPoses`) was complete on disk (explicit `Map` loop) and its laws passed at 19:45. All puzzle-side S3 edits re-verified present.
- Overnight peer sweep: both selection time-travel laws now decode through `semio_framework_pack_json::parse(…, JsonMemberPolicy::Reject)` (the extracted framework JSON reader) — same correctly-rounded parse, kept.
- 05:48 a peer's half-written `🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/Cargo.toml` (only a `[dev-dependencies]` section) broke `✏️s` workspace loading for ~2 min; restored by its owner at 05:50.
- 05:50: full puzzle 3d featured suite started (cold).
- 05:50–06:38: the 3d run never got a rustc slot — it sat in cargo's `prebuild_lock_exclusive` (sampled) behind a fleet of ~12 peer cargos on the shared build-dir, was killed (137) in a peer's lock-cycle break at 06:38; relaunched 06:42 at low load and failed in 40 s: the kernel is red again (`🚪️io/🦀️.rs:2406ff` `IoError` has no field `message` / no `From<String>`, `🏪️store/…/🪶️sqlite/🦀️.rs:64`, `🏪️store/🦀️.rs:10917` — a peer's `IoError` reshaping in progress). Waiting for the tree; no polling beyond one cheap check per ~20 min.
- 07:04 the `✏️s` kernel checked green once; the session was cut by the usage limit ~07:15 before the 3d suite produced a result.
- 10:45 resume: the owner-pose typing edit is complete (`worldGumballOwnerPoses` explicit `Map` loop, `World3dHost/🟦️.tsx:3080`); no edit is half-written. `python3 T/🧪️s3-puzzle-brace-record-lists.py --check <7 example DSLs>` → **0 bare record lists in all 7** (the migrated data is intact). Waiting for the coordinator's TREE GREEN (stdio ply/dxf/pdf sqlite migration in flight).

### S3.7 Open items and coordinator actions (current)

**Owed runs (blocked only by the tree; run in this order, one gated cargo at a time, `--manifest-path ✏️s/Cargo.toml`, tests with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s3-puzzle`):**
1. `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib` — every-example law (3d already 4/4 on 10-02), N3 3d laws, both corpus laws, the two re-premised tests, G7 example-switch asserts.
2. `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib` — **never run** (audit Z2): every-example law over 3 examples incl. capsule-dream's 11 648 nested grips, N3 5d laws, S2.5 items 3 + 5.
3. `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- every_registered_example dsl_asset_parses` — **never run** (Z2).
4. `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib` (S2.5 item 1, both editors in wasm; the native 3d test build already shows no `unexpected cfg serde`).
5. `cargo test -p semio-framework-os-infinite --lib -- world_gumball` (root workspace, S2.5 item 6).
6. Isolated re-runs of the 3d wall-clock laws and `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`.

**Coordinator actions:** re-describe + re-activate puzzle (examples now load; descriptor still lists the deleted `transformBegin`/`transformEnd` and lacks the 7 selection leaf kinds); central `schema generate` (`🔣️selection-time-travel` schemas, changed 3d leaf descriptions); GATES may drop the obsolete `#![allow(unexpected_cfgs)]` in shooting `🎮️commands/🧭️gumball/🦀️.rs:10` and `🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/🦀️.rs:13` once the wasm check is clean.

**Blockers / owners:** `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` (store units 22 → 1590 with document size) — S3-W1G. Every other S3.3 red is fixed in source or classified (load / test isolation).

## Session 4 — 2026-10-04

Successor S4-PUZZLE (Opus executor, coordinator `⚪487b04ad…`), inherits S3-PUZZLE (this report) and S3-W2D (`📓️w2-d-report.md`, pointer there).
Scratch: `🗑️generated/s4-puzzle/`. Paths: `P2`/`P3`/`P5` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`ED` = `P2/✏️editor`, `BOARD` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board`.

### S4.0 Status (updated at every milestone)

- 02:10 started. Rule 34: `git diff HEAD --stat -- ✏️s/🔌️plugins/🧩️puzzle` = 847 files (mostly peer waves: value/DSL extraction, `warn`→`warning`,
  ownership compute package). S4-INFRA owns the three sqlite owners (`P2|P3|P5/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`, ValueError conversion scripts in
  `🗑️generated/s4-infra/`) and the generated 3d example; this WP does not touch them. INFRA's own featured check of 2d+3d+5d was running at 02:08.
- 02:20–02:33 non-cargo owed runs, all green (table S4.2). `x-semio-ui` completion for puzzle 2d edge geometry (S4.3).
- 02:40 fixed `for_leaf` E0283 in all three editors (`ArtifactStoreOneItemFootprint::for_leaf::<Puzzle{2,3,5}dPlaySnapshot, _>`; every
  puzzle aggregate implements both `Mutation<Value>` and `Mutation<…PlaySnapshot>`). INFRA's 02:25–02:28 editor edits (`json` imports,
  `close_step` → `ValueError`, geometry test `DslValue`) verified present, no duplicates.
- 02:55 **puzzle 3d featured lib compiles natively** (0 errors, 99 warnings). 2d + 5d blocked by peer `semio-s-artifact-stdio-semio` (25 errors,
  graph `♻️restore-node`/`🔁restore-edge` leaves half-added). 03:01/03:08 3d test build blocked by peer edits in `semio-framework-plugin`
  (`⏪️time-travel/🦀️.rs:3870/3901` `Label: From<&String>`, `🦀️.rs:32203` `for_gesture`, `🦀️.rs:24470` `VcsError::TooLarge`).

### S4.1 D7 — the publication census was process-wide (02:50)

`one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` compares `TypedOperationUnitCensus` deltas taken around one
dispatch. The census was seven process-wide `AtomicU64`s (`🔌️plugin/🦀️.rs` region `📊️PublicationUnitCensus`; its only readers are the puzzle
3d/5d test harnesses). In a ~900-test parallel run every concurrently driven fixture app adds its units to the delta, so S3's
"22 vs 1590 store units" is not attributable to the measured dispatch. Fix (region-scoped, compile-atomic): the counters are a
`thread_local!` `[Cell<u64>; 7]`; every `record_typed_operation_unit` call sits in `advance_typed_operation_publication_unit`, i.e. on the
host-turn thread that the reading belongs to. Re-measure owed (law alone + in-suite) once the plugin crate compiles again.

### S4.2 Non-cargo owed runs (02:20–02:33)

| Command (cwd repo root unless noted) | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; repo-relative path — an absolute path silently matches 0 leaves) | 2d 103/103 inputs of 36 leaves, 3d 92/92 of 38, 5d 99/99 of 39; **0 findings** each (2d re-run after S4.3: same) |
| `… schema mutation-payloads --under …` | 2d 115/115 payloads (10 negative), 36/36 witnessed; 3d 58/58 (4), 38/38; 5d 69/69 (4), 39/39; **0 findings** |
| `python3 ED/🧪️tests/🧪️select-tool-history/🐍️.py` | **PASS** 4 scenarios, 12 head nodes agree |
| `python3 ED/🧪️tests/🧪️history-edit-runtime/🐍️.py` | **PASS** 5 scenarios, 20 head nodes agree |
| React (`⚛️react/📦️packages/🟦️typescript`) `bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run "--testNamePattern=guest-stamped highlight\|resolves mesh style by priority\|multi-object turn and scale of markers\|gumball rotate handles commit rotateSelection\|gumball transform preview"` | **5 passed**, 695 skipped |
| React `bun ./📜️script.ts test long engine-contract -t "puzzle 2d\|board 2d\|live mirror\|hover out of the board"` | **41 passed**, 659 skipped |
| React `bun ./📜️script.ts test long board-event-coalescing float32-decimal` | **29 passed** (2 files) |

### S4.3 Puzzle 2d leaf inputs — complete `x-semio-ui`

Audit of every number input of the 36 puzzle 2d leaves (widget, en/de label, unit, step/precision, soft/hard bounds, snaps/snapSource): the
selection leaves were already complete (drag `dx`/`dy` steppers snapping to `gridFactor`; rotate pivot steppers + `angle` dial rad→deg with
quarter-turn detents; scale pivot steppers + `factor` log slider 0.1–10, detents 0.25/0.5/1/2/4, hard `exclusiveMinimum: 0`). Gaps fixed:
- `connect-handles` and `replace-edge-geometry` carried the semio connection geometry as unit-less 0.1 steppers. They are the same
  `gap/shift/rise` (metres) and `rotation/turn/tilt` (degrees — 5d `📐️geometry` applies `to_radians()`; puzzle 3d `connect-vortices` already
  declares them so) that the 2d example loader passes straight into the 3d attraction. Now: lengths `unit: m`, step 0.01, precision 3;
  angles `widget: dial`, `unit: deg`, step 1, precision 1, soft ±180, detents −180/−90/0/90/180. Labels unchanged (2d's `Wendung` avoids two
  `Drehung`s). Script: `🧪️s4-puzzle-edge-geometry-ui.py` (idempotent, `--check` → 0 pending).
- `scale-selection.factor` German description said "vom Drehpunkt" while the pivot is labelled "Bezugspunkt" → "vom Bezugspunkt aus".
- Deliberately unchanged: region/node sizes keep no hard minimum because the leaves only refuse non-finite values (a schema stricter
  than the fold would reject witnessed payloads).

### S4.4 Editor peer fallout (03:10–03:26) and the native milestone

INFRA's wasip2 closure check listed the remaining 2d/5d reds, all in `✏️editor/**` (this WP):
- `dsl::json` / `dsl::os_pack::json` are gone from the kernel (`dsl` = `extern crate semio_framework_os_kernel as dsl`). Script
  `🧪️s4-puzzle-pack-json-sweep.py` moved 119 files of puzzle 2d + 5d (editor, viewer-free lib code AND every leaf/editor test) onto
  `semio_framework_pack_json`, mirroring the peer conversion of puzzle 3d: `from_json_str(x)` → `from_json_str(x, JsonMemberPolicy::Reject)`
  (inserted before the call's matching parenthesis, turbofish kept), other functions renamed, doc references renamed; sqlite owners
  (INFRA) skipped; idempotent (`--check` → 0 pending).
- `MediaPayload::Intrinsic { schema, value }` (new variant): the 2d and 5d `kit:in` import jobs now take it explicitly — the typed
  value is encoded to the JSON text their cursorized decoder already reads (`semio_framework_pack_json::to_json_string(&value)`); no wildcard.
- 5d completion-rejection retirement stepped over `Emit.description`, which §20.6 deleted: the step is removed.
- 5d transform re-exported `puzzle3d_transform_tool_clock`, which the CLOSURE-5 sweep replaced in 3d by
  `semio_framework_tool_machine::authoring_clock(0)`: the re-export is deleted and the three 5d commit sites call `authoring_clock(0)`.
- 5d precompute `ToolRunJobRequest { children, member_ops }` (S4-WIRES-MATH's child-target change) was fixed by its owner before my check.

**03:26 milestone:** `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-5d --lib
--features …3d/component-app-assembly,…2d/component-app-assembly,…5d/component-app-assembly --keep-going` → **exit 0, 0 errors** (warnings 3d 99, 2d 22,
5d 17). `main` notified. (`semio-s-plugin-puzzle` no longer exists: the component crate is `semio-hub-puzzle` in `🌎️hub/Cargo.toml`.)
- **03:46 wasm32-wasip2 green:** `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-puzzle` → exit 0 (3d/2d/5d libs, 14m41s).
  `main` notified (activation input). 03:52 coordinator ACTIVATION FREEZE (rule 40) on puzzle/framework/stdio/hub edits.

### S4.5 `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes` — test isolation (source, 02:55)

Root cause: every 3d fixture app bound instance id `1` (`app()` → `bind_instance_id(1)`), and the puzzle 3d session registry is a
process-global slot row keyed by instance id (`✏️editor/🦀️.rs` region `🎟️SessionRegistry`). Under the ~900-test parallel run concurrent laws
adopted, evicted and re-keyed (different parent document ids → `retire`) each other's slot between this law's two dispatches; the standing
mesh re-upload set lives in that slot's collision session, so the second announcement looked new. In addition `check_out`/`check_in` used
`try_lock()`, so a contended registry silently dropped the state — a production behaviour issue, not only a test one: the session carries
behaviour (the B32 storm guard), not just speed.
- Production: `puzzle3d_session_check_out`/`_check_in` WAIT for the registry lock (poison-tolerant); the critical section is a slot move.
  `retire` = new `vacate` (empty slot, bump generation) + the page-run upload sweep.
- Tests: `next_fixture_instance_id()` (process-unique `AtomicU32`) replaces `FIXTURE_INSTANCE_ID`; `Puzzle3dApp.instance_id` is the bound id
  and the result-page receiver; `Drop for Puzzle3dApp` calls the new `#[cfg(test)] puzzle3d_session_release(id)` (vacates its slots without
  the process-wide upload sweep, so one law's drop never sweeps a concurrent law's open upload).
- Follow-up (production, needs the framework): an instance close does not release its slot (the owner built by
  `build_instance_operation_owner` does not know its instance id); bounded by 64 slots + LRU + 96 MiB, but a closed document's caches linger.

### S4.6 Wall-clock laws → deterministic work-meter laws (source, 03:40–03:52)

All six 3d laws that asserted `Instant` elapsed times (the 4 named in S3.3 plus `penetration_steps_stay_within_interaction_watchdog`,
`empty_fill_transition_stays_below_watchdog_ceiling`, `adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms`) now assert counts:
- New `#[cfg(test)]` thread-local **precompute work meter** (`⏳️precompute/📐️geometry/🦀️.rs` region `🧮️WorkMeter`: `precompute_work(units)` — a
  no-op outside test builds —, `precompute_work_done()`, `precompute_work_clock()`). Charged at the primitive costs: each surface-distance and
  each containment query (`probe`), every near face clipped (`step_face`), every spatial-index lookup including the `nth(member_cursor)` walk
  (`step_query`, which therefore exposes the cursor's O(cursor) iteration honestly), index mutation steps, every mesh vertex + triangle ingested
  (`collision_body_from_buffers`), every brush loop unit / placed-object scan / placed-entry AABB test / candidate listed, every fill
  builder transition.
- Job laws (brush suggestions, fill run) drive the REAL jobs with `precompute_work_clock` as the job clock and a `budgetWork`-unit slice, so the
  jobs' own deadline checks slice deterministically, and assert the worst step ≤ `stepWorkCeiling` (fixture keys; the old `budgetUs`/`runs`/
  `coldRuns` and the replay clocks are deleted). Fill (b) asserts ops per appending tick ≤ `appendOpsCeiling`, each folding onto the overlay.
- Penetration and fill-builder laws bound per-step and total work by named constants.
- Maintenance law: a unit is bounded by its grant, not the clock — `MaintenanceStageGrants` (replaces the wall-time `MaintenanceStageBudget`)
  audits every `maintenance_step` `Pending { released_items, released_bytes }` against `(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP)` per stage over
  both flagship documents; one round (deterministic).
- Root gate `📜️script.ts` `interactivityPuzzleFillRunJobFailures` now pins `budgetWork === 1000`, `0 < stepWorkCeiling ≤ 100 000`,
  `appendOpsCeiling > 0`, `turns ≥ 1` instead of `budgetUs === 2000`/`turns ≥ 771`; its self-test fixture
  (`🧪️tests/🔬️interactivity-puzzle-fill-run-job/🟦️.ts`) follows, plus a new `loosened-step-ceiling` case:
  `bun ./📜️script.ts verify puzzle-fill-policy-self-tests` (cwd `✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript`) → **checks=73**, pass.
- Ceilings are provisional (100 000 per step); the first run prints `[DEBUG]` counts, then the fixtures/constants are pinned to the measured
  maxima with headroom and the `[DEBUG]` lines removed (after the freeze).
- Remaining wall-clock asserts in the 3d unit file (`PUZZLE3D_INTERACTIVE_STEP_CEILING` at the openVortexSuggestions / acceptSuggestion /
  setActiveExample worst-turn laws) are next.
- Noted, not mine: the real fill fixture's `delivery.candidates` is 2500 while the root gate demands ≥ 5000 (pre-existing).

### S4.7 Resume after the usage cut (~04:15 → 06:45)

- Rule 34 re-check: every S4 edit is on disk (thread-local census region, session `vacate`/`puzzle3d_session_release`, unique fixture instance
  ids, `MaintenanceStageGrants`, work meter + the converted brush/fill/penetration laws and fixtures, root gate + self-test). Nothing
  half-written: the maintenance conversion had completed before the cut.
- 04:04 the first 3d TEST build after the meter edits showed 38 errors, all peer-sweep leftovers in three 3d TEST files the lib sweep had not
  reached (bare `json::` of the deleted `use dsl::json`, one-argument `parse`/`from_json_str`, `protocol::{Terminology, Locale}` no longer
  re-exported). Script `🧪️s4-puzzle-3d-test-pack-json.py` (idempotent, `--check` → 0) converted them: `semio_framework_pack_json::`,
  `JsonMemberPolicy::Reject`, `semio_framework_ui_locale::{Terminology, Locale}`.
- `check-3` (04:14 → 04:27, libs, featured, 2d+3d+5d) → exit 0, 0 errors: the meter/session/census edits keep the native closure green.

### S4.8 Queued items after the freeze (06:50–07:05, source)

- **Z4 (audit-s3-tools, routed via S4-TOOLS-A):** new 3d law `the_world3d_local_gumball_cases_land_as_their_guest_edits` (`✏️editor/🧪️tests/🔬️unit`,
  region `🔖️Gumball`) replays the four `live: false` cases of `🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json` against the real app: each
  host dispatch with its exact wire args (`ids` swapped for the fixture object) must publish exactly the case's `guest.edits` history rows
  and move the object by `guest.offset` (absent `guest` = 0 edits, no motion; the no-target case dispatches nothing). Asserts 4 cases.
- **Fault notices (S4-GATES scoped mode):** the 3d fill tool's anonymous faults are named refusals — `puzzle3d.fill.scene-unavailable`
  (document builds no engine scene) and `puzzle3d.fill.provisional-unreadable` (finalize revalidation cannot decode the provisional ops);
  every puzzle editor now implements `ArtifactApp::fault_notices()` with a literal `(code, LocalizedLabel::native(en, de))` table
  (3d: those two + `puzzle3d.action.flag-value-required`; 2d/5d: their `…action.flag-value-required`).
  `bun ./📜️script.ts schema fault-notices --census` → puzzle row **codes 5, labelled 5, declared 5**, missing/unresolved 0 (was 3/0/0 with 3
  missing); the remaining puzzle findings are 311 repo-wide anonymous faults (out of the scoped mode) + 1 `faultNoticeDescriptor` (the
  committed descriptor predates the notices → describe, coordinator).

### S4.9 Closure rule `footprint-default` (coordinator routing from S4-GATES, 07:20) and runs

- The hand `impl Mutation<Value>` / `impl Mutation<…PlaySnapshot>` bridges of all three aggregates (`🧬️schema/🧬️mutations/🦀️.rs`) forwarded
  descriptors and input schemas but not `inverse_rows`, so they fell back to the trait default 1 (rotate-selection declares 65 per target) — an
  under-declared fold footprint for `ArtifactStoreOneItemFootprint::for_leaf`. All 6 bridges now forward `Mutation::<…Snapshot>::inverse_rows`.
  `bun ./📜️script.ts verify history-closure --json` (root) → census `footprint-default 0`, `footprint-hand 0`, `coalesce-key 0`; `bracket-verb 11`
  remain, the 2 puzzle ones being the stale committed descriptor `🌎️hub/🧩️compositions/🧩️puzzle/🔣️.json` (`transformBegin`/`transformEnd`) → describe.
- `cargo test -p semio-framework-os-infinite --lib -- world_gumball directed_normal` (root, private target) → **62 passed, 0 failed** (07:59).
- 3d test build: 07:05 and 06:45 runs SIGKILLed by the coordinator deadlock breaker (`coord/deadlock-breaker.txt` 07:30:58 "killed 27653 age 1538s
  cpu 1.12s" — my cargo sat behind peer locks); 07:31 retry hit a pruned shared-build-dir fingerprint; 07:42 retry stopped on PEER reds in two
  regular dependencies of puzzle 3d: stdio-stl `📸️snapshot/📦️pack/🦀️.rs:30,35` (`ValueError` not imported; edited 07:16) and stdio-ply
  `📸️snapshot/🪶️sqlite/🦀️.rs:111,112` (`Result<_, ValueError>` expected) — reported to `main`.

### S4.10 OWED (rule 43: no puzzle `cargo test` until "TESTS RESUMED"; run after the describe wave + activation)

Every command from `/Users/ueli/Documents/semio`, gated by rule 42 (`until [ "$(pgrep -x cargo | wc -l | tr -d ' ')" -lt 8 ] && [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 14 ]; do sleep 30; done`),
prefix `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s4-puzzle`, output to `T/🗑️generated/s4-puzzle/<name>.txt`:
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib --message-format=short` — full 3d suite incl.
   every-example, N3 laws, the session-isolation fix (`an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes` in-suite), the work-meter
   laws (read their `[DEBUG]` counts, pin `stepWorkCeiling`/`*_WORK_CEILING` to measured maxima + headroom, delete the `[DEBUG]` lines), Z4
   `the_world3d_local_gumball_cases_land_as_their_guest_edits`, `every_maintenance_unit_stays_inside_the_interactive_step_budget`.
2. **D7 re-measure** (thread-local census): `… -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib --message-format=short -- one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns b54_measures_the_turns_and_units_one_mutation_costs_per_document_size --nocapture --test-threads=1`
   then the same filter inside the full parallel run (1.) — compare `translate_census.store` small vs Nakagin; numbers to `main` for S4-STORE/S4-RUNTIME.
3. `… -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib --message-format=short` (never run; audit Z2), then the W2D filter
   `-- a_board_gesture_drag board_node_delete apply_board_events language_neutral_fixtures`.
4. `RUST_MIN_STACK=67108864 … -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib --message-format=short` (full), then filters
   `-- every_registered_example dsl_asset_parses`, `-- select_tool` (machine + transactions + corpus incl. `select_tool_history` and the Use-selection
   click), `-- history_edit_runtime_tests`, and the two fill laws alone: `-- board_fill_job_large_host_has_no_step_at_or_above_eight_ms fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin`.
5. `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d` (root; W2D's wgpu board laws; RUST_MIN_STACK=67108864).

### S4.11 Resume 11:35 (cut ~08:55)

- 08:37–08:49 featured native check (`check-4`) after the 07:2x edits: 3d + 5d green (warnings 102/18), **2d red only in its sqlite owner**
  (`🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:171,177–179`, E0282/E0283: the four `native.scoped_stage(|native|{…; Ok(())})` closures in `native_rows` left
  the closure's error type to inference once `?` converts). INFRA's 08:56 census showed the same 4 errors (no puzzle `close_step` error remained).
  The coordinator handed the fix to this WP: each closure now states `->Result<(),semio_framework_value::ValueError>` (the 5d owner's form).
- Inherited from the 07:2x waves and still part of this check: the `inverse_rows` bridges (S4.9) and the fault-notice tables (S4.8).
- 11:40 first re-check SIGKILLed by the deadlock breaker (12:06:17, my cargo sat 25 min in a lock cycle); re-run once:
  **`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-3d -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-5d --lib --features …3d/component-app-assembly,…2d/component-app-assembly,…5d/component-app-assembly --keep-going`
  → exit 0, 0 errors** (12:16 → 12:36; warnings 3d 102, 2d 34, 5d 18).
- **`cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-puzzle` → exit 0, 0 errors** (12:43 → 12:49; the 12:36 try failed
  writing a fingerprint the disk guard had pruned, re-run green).
- 12:49 `main` notified **"COMPOSITION GREEN puzzle"** (rule 44: my last cargo until the coordinator's describe + activation).

### S4.12 Coordinator actions (current)

1. `describe` + activation of puzzle (design §21.4, coordinator-owned): picks up the fault-notice tables (gate `faultNoticeDescriptor` 1 →
   0), drops the stale `transformBegin`/`transformEnd` bracket verbs from `🌎️hub/🧩️compositions/🧩️puzzle/🔣️.json` (`verify history-closure`
   bracket-verb −2), the 7 selection leaf kinds, the `BoardSession.setHighlightedIdsJson` binding and the Nakagin 2d manifest registration
   (W2D S3.5). No channel bump from this WP.
2. Central `schema generate` (§3.1): 2d edge geometry `x-semio-ui` (connect-handles, replace-edge-geometry), scale-selection de description,
   3d `🔣️selection-time-travel`, 2d `editor/select-tool-history`.
3. After "TESTS RESUMED": the OWED list S4.10 (3d/5d/2d suites, D7 re-measure with numbers to S4-STORE/S4-RUNTIME, wgpu board2d).
4. Production follow-up (framework, not this WP): an artifact-instance close should release its puzzle 3d session slot — the instance
   operation owner does not know its instance id today (S4.5).

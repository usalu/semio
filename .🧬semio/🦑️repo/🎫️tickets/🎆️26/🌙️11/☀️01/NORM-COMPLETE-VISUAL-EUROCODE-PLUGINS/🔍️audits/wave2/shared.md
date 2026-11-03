# Shared agent, wave 2 (compliance, structural-scene, registry/app-surface, mechanics)

Nothing here is compiled by me; all files parse (`rustc -Zparse-crate-root-only` over all 193 shared files), mounts/orphans/Cargo deps checked by script, source-boundary scan shows no shared crate over budget. Runner logs contained no rustc errors (only infra failures).

## Done
- Facade: `render_diagrams`, `DiagramsWindowSpec` re-exported at the app-surface root (coordinator request).
- Live: `NormFamily::live_responsive` (default sync), `NormHost::live_within`, `LiveView::{within, pending, checks_view, cancel}`, `settled`, `PAINT_BUDGET`/`DRAG_BUDGET`; `with_lanes` is budgeted (drag report only cached when settled). Test `pending_progress...`, compliance test `a_responsive_paint_returns_within_its_budget...`.
- Scene: `projection/live-scene` with `evaluation_of`, `LiveScene` (scene/preview with delta) and macro `live_scene!`; structural-scene root now `pub extern crate ... as norm`. 5 unit tests + feature.
- Properties: inspector built from `FieldSpec`/`UnitSystem` (kinds, angle/percent/ratio, range, soft range slider, step, precision, help, clause row, NDP symbol + reset, `visible_if`, `tier`); duplicate unit table removed; `parse_quantity` accepts any unit of the family; stale test fixed (`NormFieldMeta`); 5 new tests, feature scenarios.
- Outliner: `OutlinerState.interaction_domain`, rows are scoped pick targets (`collection/id`, granularity entity), `NormSelection::outliner_state_in`; test + feature.
- Requests: build-blocker items verified fixed and marked done; new answers for artifact agents appended to requests.md.

## Wave 2b
- Newer runs: none include my edits yet (latest integrate/mechanics logs 14:25/14:40, green, predate them).
- Rolled out in all ten editors: `live_responsive`, properties `selection.inspector_options()`, outliner `outliner_state_in(INTERACTION_DOMAIN)`. `live_scene!` not rolled out: no artifact implements `StructuralProjection` (logged).
- Unit picker: `ViewLane.units` (ephemeral local lane; twins ts/graphql/proto edited by hand), `LaneEvent::ViewUnits`, `viewportSetUnits` via `viewportAct`, picker group at the top of the inspector (live editors only), `LiveView::with_lanes` sets `selection.units`. NOT persisted.
- Viewport window-config persistence NOT done: needs a new schema-first window-config crate (schema, diff, mutations, 4 io files, twins, goldens, workspace/index registration, owner macro) that cannot be verified blind; ViewLane keeps colour field/track/units ephemerally. Design: copy `🪟️results/🎚️config` as `🪟️viewport/🎚️config` with camera preset, layers, ramp, section; owner macro like `norm_results_window_config_owner!`.
- Latency tests: checks commit p95 < 400 ms (existing) plus settled-paint p95, and drag preview p95 < 12 ms on a 1000-member scene (release; debug budgets 20x).
- Mechanics: build green per coordinator; files parse; no deeper read done.

## Open / not done
- No unit picker control (typed units are parsed, preset chooses display). Viewport window config persistence not added. Artifacts must add `live_responsive`, `live_scene!`, `outliner_state_in`, `openClause`/`setChecksView`/`cancelEvaluation` commands.
- Mechanics crate: build reported green by coordinator; not re-read.
- Rebuild requested in integrate/latest.md.

## Wave 2c
- (A) Investigation: the scenes are reached only through the free function `project(&SceneInput)` (monolithic, private `kit::Out`), called from `scene_of` in each editor/viewer viewport window; there is no trait and no per-entity fragmenting, so `StructuralProjection` impls would each require splitting every entity kind out of the pipeline, not a delegating shim. Delivered instead `SceneMemo` + `live_whole_scene!` (structural-scene `projection/scene-memo`): one projection per revision across windows plus the minimal `SceneDelta`; 3 tests. Adoption in `scene_of` handed to the SCENE agent (requests.md) to avoid clashing with its placement edits. `live_scene!`/`LiveScene` stay ready for the day entities project individually.
- (B) Viewport window-config crate NOT created. Reason: the pattern (results-config) is ~30 files per config: schema + diff + one mutation leaf per field (each with inverse/diff/apply tests), text+binary codecs, ts/proto/graphql/json twins, wire-witness fixtures whose bytes/hashes I cannot generate without running the framework, plus workspace/Cargo.lock/index registration. A wrong crate would break all ten artifact builds because they depend on the results-config crate. Recommended path: run the repo's mutation scaffolding / witness generator (`SEMIO_UPDATE_GOLDEN=1`) on a copy of `results/config` renamed `viewport/config` with fields `camera_preset, layers, color_map, units` (persisted local-only; presence/camera ghost stays ephemeral shared). The unit picker stays on the ephemeral view lane until then.
- (C) Twins: `wp02-gen-twins.py` regenerates the lanes twins; `--check` passes (exit 0) with `ViewLane.units`.

## Wave 2d
- (B) Created `🪟️viewport/🎚️config` (crate `semio-s-plugin-norm-viewport-config`): schema `NormViewportWindowConfig {camera_preset, hidden_layers, colour_map, unit_system}`, diff, single mutation leaf `change-viewport-view` (+diff, inverse, inverse-sum-law test), text/binary codecs, rs/ts/proto/graphql/json twins, owner macro `norm_viewport_window_config_owner!`, protocol-law and DSL tests, feature. Isolated: not in the workspace, not depended on. Missing until the runner generates them: wire-witness fixture, window-ownership TS test, hex goldens (requests.md has the steps). Unit picker stays on the ephemeral lane until then.
- (A) Per-entity projection not implemented; reasoning and the staged plan are in requests.md. `SceneMemo`/`live_whole_scene!` remain the interim.
- Logs: newest runner logs (15:33-15:51) have no rustc errors in shared crates (en1991 source-boundary error is in an artifact mutation crate; en1992 "Admitted Cargo artifact plan Source changed" is infra/concurrent edits).

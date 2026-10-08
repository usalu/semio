# r7 API: the model graph and `ModelInferenceSession` (label `z-graph`, for `z-consumers`)

`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `I` = `S/🧬️schema/💡️inferences`. Design and rationale: `r7-design-model-graph.md`. Status: this API is **frozen**; names below do not change, only bodies fill in. Module path: `semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::model_graph` (source `I/🕸️model-graph`).

## What exists

One `InferredField` over the whole model. Every derived value is a node of one DAG:
`Storey -> WallLayout (+ the Bands of the walls it touches) -> Host -> OpeningFrame -> Solid / Quantity -> Totals`, `Room(storey)`, `Plan(storey)`, `Diagnostics(scope)`. Each node is computed once from its parents' values; no consumer recomputes a layout, host, top, validity, run or room. `ModelInference` keeps its ten `#[derived]` fields; they are projections of the graph.

## Session (the one entry point for editor, viewer and export)

```rust
use ...::inferences::model_graph::{ModelInferenceSession, UpdateReport};

pub struct ModelInferenceSession { .. }                       // Default + new()
impl ModelInferenceSession {
    pub fn new() -> Self;                                       // cache enabled, 256 MiB budget
    pub fn with_cache_budget(bytes: usize) -> Self;
    pub fn update(&mut self, snapshot: &ModelSnapshot, diff: &ModelDiff) -> &ModelInference;
    pub fn refresh(&mut self, snapshot: &ModelSnapshot) -> &ModelInference;
    pub fn inference(&self) -> &ModelInference;                // what the last update/refresh produced (default before the first)
    pub fn report(&self) -> &UpdateReport;                     // what the last update/refresh did
}
pub struct UpdateReport {
    pub gated: bool,                                           // the diff touched nothing the graph reads: stored result served, plan not walked
    pub nodes: usize,                                          // nodes of the graph after the call
    pub computed: usize,                                       // nodes whose compute ran (engine call counter)
    pub reused: usize,                                         // nodes served by the cache
    pub computed_by_kind: BTreeMap<&'static str, usize>,       // "wall-layout", "opening-frame", "solid", "plan", ... -> compute calls
}
```

- `update(snapshot, diff)`: `diff` is the `ModelDiff` that turned the snapshot of the previous call into `snapshot` (the one `protocol::apply_diff` was given). It runs `protocol::infer_field_after_diff` with the session's enabled `InferenceCache`: a diff that touches none of the graph's `reads` returns the stored result; otherwise the plan is walked, every node whose dependency chain is unchanged is a cache hit and only the changed nodes and their descendants compute. The returned `ModelInference` is updated entry by entry (only changed nodes are copied).
- `refresh(snapshot)`: the same without a diff (always walks the plan, so it is always correct; cost = plan + one dependency hash per node + the changed nodes). Use it when the diff is not at hand (viewer, export, first mount). Never pass a diff that does not describe the change: a wrong diff can serve a stale result.
- Cache transparency: `update`/`refresh` results equal `ModelInference::infer(snapshot)` (tested for every projection, warm = cold = uncached).
- One session per open document is enough; the viewer and the export read the same session (`inference()` is a plain `&ModelInference`).
- Cancellation/progress: the stepped engine driver (`protocol::infer_field_step`, `InferenceCursor`) works on the graph unchanged (`ModelGraph<ALL>` is a normal `InferredField`), see `r7-design-model-graph.md` section 9.

## Whole-model entry points (unchanged signatures)

- `ModelInference::infer(&ModelSnapshot) -> Result<ModelInference, ValueError>` (no cache, whole graph).
- `ModelInference::fields()` / `InferenceSpec` rows are unchanged (the editor session still reads them until it moves to the session above).

## Thin projections that stay (signatures unchanged, one graph run restricted to the ancestors of what they return)

`compute_storey_levels`, `compute_wall_layout`, `compute_curtain_layout`, `compute_stair_runs`, `compute_spaces`, `compute_opening_frames`, `compute_element_solids`, `compute_plan_linework`, `compute_diagnostics`, `compute_quantities(snapshot, &ModelInference)` (the second argument is now ignored, see below), `rooms_of(snapshot, storey, level)` (editor area gesture; `level` is ignored, the graph derives it).

**Delete these when the consumer moved to the session** (they exist only so editor/viewer/IO/examples compile now): every `compute_*` above, `rooms_of`, `render::{solids, plans}` memo callers, `IFC export`'s five private runs.

## Removed or changed (consumers outside `I` that used them: none besides the ones listed)

- Removed: the per-field `InferredField` types (`StoreyLevelsField`, `WallLayoutField`, `CurtainLayoutField`, `OpeningFramesField`, `ElementSolidsField`, `StairRunsField`, `SpacesField`, `PlanLineworkField`, `DiagnosticsField`), their key/node enums (`LayoutKey`, `FrameKey`, `SolidKey`, `SolidSource`, `SolidNode`, `Rooted`, `DiagnosticKey`, ...), `host_extent`, `layout_of` (now takes the touching bands), `frame_of` (now takes the siblings), `storey_bodies`/`plan_of` inputs.
- Renamed: `wall_layout::top_of` -> `storey_levels::top_of` (IFC `🧭️frames` import updated).
- `OpeningIssue::OutsideTrimmedExtent` (new) and `DiagnosticCode::{OpeningOutsideTrimmed, RoofFallbackCurved, RoofFallbackNonConvex, RoofFallbackDegenerate, RoofFallbackPitch}` (new; en + de rows).
- A valid opening is now exactly an opening the wall solid cuts: flush with a wall end, a joined corner or the top is `OutsideTrimmedExtent` (it used to be `valid` and silently produce no hole). `ElementQuantity.opening_area` is the sum of the valid cut rectangles (no more subtraction of holes the solid never cut).
- `ElementSolid.family` of door/window placeholders is now the true family; `SolidSource` is gone (one `SolidFamily`).

## Facts consumers can rely on

- Keys are typed (`ModelNode`), ordered, stable ids; values are `Arc` payloads inside `ModelValue`, cloning is cheap.
- `UpdateReport.computed == 0 && !gated` is the proof that an edit changed nothing the graph derives; moving one wall in a 500-wall model computes ~dozens of nodes (see the test `moving_one_wall_recomputes_only_its_neighbourhood`).

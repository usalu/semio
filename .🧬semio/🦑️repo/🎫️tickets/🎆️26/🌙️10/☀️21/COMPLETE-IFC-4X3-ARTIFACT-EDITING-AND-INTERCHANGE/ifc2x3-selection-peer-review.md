# IFC2x3 Selected Inspector Peer Review

Read-only native source/UI seam review plus actual TS test execution. No production edits/native build/UI execution.

## Actual Source Test Evidence

Executed runIfc2DetailsChecks against fresh own2source:9groups passed,0TypeScript diagnostics. This includes actual ifc2x3SelectionPath exactu64MAX path translation before/after row reorder, invalid identity rejection, missing ID refusal and original callback cancellation. Tests live own2Details suite, importing inspector TS helper; inspector test directory itself contains neutral JSON/native tests. Removed only own generated ifc2-details-check output.

Native translation resolves stable /ifc-ID prefix against initiating own snapshot with per-instance originalcallback, then passes translated path into own Details planner. Provider maps selected stable root to current row only during revision-pinned rendering, preserving instance identity across reorder. Exact numeric IDs are strings in UI and u64/bigint in graph. Deleted/missing selected identity produces localized message; topology contains actual current instance IDs for framework synchronization. No graph conversion appears.

## Concrete Native Compile Errors Found

Fresh standard2/io/geometry source changed render_scene to require selection argument, but render_with called render_scene(scene,selection) with no selection variable, and render_context called render_scene(scene) missing argument. Both reported to High; intended calls are None for render_with and actual selection for render_context. Syntax-only parsing cannot detect either unresolvedname/arity error. Verify correction in fresh source before native build claim.

## Coverage Still Expanding

At exact inventory, only base2editor/viewer roots wired inspector window/domain/topology and render_selected_in_context. CV2.0,SAV,COBie six remaining roots still used previous unselected flow. High confirmed expansion is pending under current parent request, so do not claim all8selected roots complete yet. Their geometry projection itself already uses original context; selected/domain forwarding is the additional change.

## UI and Authority Seams

Viewer Provider(editable=false) disables edit/remove/reorder/rename, insertion/templates/variants and has_source remains defaultfalse. Viewer window actions are cleared; ephemeral reference navigation selection remains appropriate. Editor uses existing retained snapshot actions and own mutation authority. render_snapshot_details_provider_revisioned receives original render-pinned revision and TreeWindows for inspector BODY_KEY, preserving framework draft/action target routing. Locale is explicit EN/DE for inspector title, prompts, cancellation/limit/missing states and reference navigation. Navigation uses real INTERACTION_SELECT domain/targets/granularity/method pick through original controller and declared interaction topology.

Topology domain/method registration signatures match existing4source patterns; base root trait uses ArtifactView+ConfigView signature. Framework stale-ID pruning depends on mounted topology synchronization; native authored tests inspect topology presence, not an executed host stale-selection lifecycle. Do not claim deletedselection host behavior was exercised. Large selected-instance Details provider rendering is still synchronous generic UI work; navigation scan controls do not make full provider rendering resumable. Existing one-shot instance edit materialization/control gates remain explicit.

No other concrete native borrow/API mismatch identified in inspected selectedprovider mapping. Broader typed structured admission plan remains ifc-structured-edit-admission-plan.md; selected path translation must preserve that future predicate rather than bypass it.


## Fresh render call-site resolution

Fresh read confirms standard2 geometry render_with passes None and render_context passes selection to render_scene (lines7,45). Prior two native call-site defects are resolved in source; native compilation remains unexecuted.

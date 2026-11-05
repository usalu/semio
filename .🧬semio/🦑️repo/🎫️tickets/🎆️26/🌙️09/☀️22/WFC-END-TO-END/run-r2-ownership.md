# WFC Instance Ownership Review

Read-only audit2026-11-04; only this report is edited. Exact source references below are relative to artifact root ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts.

## Grid3d Global State

🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:
- RESIDENCY99 is process-global Mutex<BTreeMap<String,Grid3dPreviewResidency>>. publish152-160 keys it solely by window_id. Entry previous/current/revision70-75 is retained across callers and has no app, document or source revision identity.
- LAST_FILL101 is process-global Option<Vec<Assignment>>; commit_fill_result104-109 overwrites it for any caller. last_fill_commit113-117 reads it without instance identity. paint_assignments135-146 does not consume this as its finished result.
- No removal/instance-close hook occurs in this module. The bounded-by-live-window claim is unsupported by the actual map lifetime. The render solve path updates entry.current before projection, so direct display of a foreign old full assignment was not established; foreign delta base/revision history and retained memory are nevertheless concrete ownership defects. LAST_FILL can return another app result to its observer.

A clean repair removes both globals, including test-only observer APIs that production tests currently rely on. Keep no compatibility shim.

## Existing App Transient Integration Reference

Use the graph3d pattern under 🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor:
- 🔗️graph-view-and-editor/🦀️.rs61-62 associates app Transient/TransientMutation.
- Lines79-80 install bounded_transient_root_retirement_factory.
- Lines108-109 install bounded_transient_store_disposer.
- Lines112-113 install bounded_transient_preparation_factory.
- 🧵️retained-commands/🦀️.rs145-156 decodes CommitFill and emits CompleteWithEphemeral with EphemeralEmit.transient.
- 🎭️modes/✏️edit/🛠️tools/📑️fill/🦀️.rs550 accepts ToolRunJobPort,649-653 dispatches completion through the port,958 obtains request.port from the mounted tool admission.

For grid3d define app-local Grid3dTransient with assignment rows, satisfiable/contradiction and source_revision. Add its schema-first SetSolve mutation and retained stores/retirement/projection just as graph owns them. Add an internal CommitFill command and explicit Transient publication contract; the job port dispatches its completed payload. Bind the payload to original ToolRunIdentity.base_revision, not current revision at callback delivery. Render with the actual app transient and doc.render_operation_ref canonical revision. Live payloads are fenced by ToolRunView.identity.base_revision. Absent/stale cache paints unsolved. No cold solve in render and no process-global cache.

Full scene JSON is a clean initial representation; no persistent delta cache is required. If scene deltas are retained, their previous/current/revision must be owned by the addressed instance/window lane and retired with that owner.

## Other Five-Artifact Globals

Scanned production Rust/TypeScript/TSX in WFC plugin and hub composition, excluding tests/oracle paths, for static mutable state, thread_local, globalThis and Map construction. Other production statics found are immutable LazyLock notice labels and OnceLock language/IO/inference/example catalogs. These contain schema/registered source metadata, not mutable authored documents or per-user solve assignments. SQLite Maps are local reconstruction variables.

No additional shared mutable user-result global was established outside grid3d preview. This is a scoped source search, not a whole-framework isolation certification.

## Required Ownership Laws

Create two app instances with identical window IDs but different documents/seeds. Complete/dismiss A and require B unsolved; complete B and require A assignment unchanged. Change A canonical revision and require its old cache invisible while B remains valid. Closing A must retire its result backing without touching B. Abort a new A run must preserve its previous valid cache if the canonical source stayed unchanged. These tests should inspect actual app transient and mounted scene, not LAST_FILL.

## Exact Live Fence Test API And Completion Timing

Bitmap output test reference:
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🧪️tests/🔬️unit/🦀️.rs154-155.

Imports semio_framework_plugin::ToolRunView and semio_framework_tool_run::{ToolRunId,ToolRunIdentity,ToolRunState}. Constructor:
ToolRunView::new(fill_tool::TOOL_ID, ToolRunIdentity::new(ToolRunId { app_instance_id:1, run:9 }, [0;32]), ToolRunState::Running).
Then set run.payload=Some(payload.encode().into()) for Grid2dFillPayload; bitmap uses encode_json().into_bytes().into() instead.

Grid2d fill module60-66 defines Grid2dFillPayload { assignments:Vec<Grid2dFillCell>, contradiction:bool, done:bool, trace:Vec<Grid2dFillTraceEvent> }. Cell48-53 has x:u32,y:u32,tile_id:Option<String>. encode84-85 returns Vec<u8>. Construct one valid existing tile assignment at0,0 and compare matching [0;32], mismatched [1;32] and None current render revision. All three must be tested through actual render-bound adapters as well as pure projection helpers.

Grid2d finish_success623-638 dispatches commit_effect then publishes Complete. It caches BEFORE Finalize. commit_effect148-154 queues DispatchAction with solveJson. Interactive advance checks cancellation514-515, and outer step checks cancellation798-800 before continuing work. This prevents ordinary cancelled work from deliberately computing/dispatching a new completion; it does not retract an effect already queued at completion.

Framework declaration/⏯️tool-run/🦀️.rs ToolRunJobPort::dispatch120-124 pushes into effects queue without lifecycle gating. Framework ⏯️tool-run/🧵️instance/🚀️action-dispatch-and-start/🦀️.rs drain_tool_run_port201-210 drains every entry queue to the effect outbox without checking run state. Thus Abort-after-Complete can coexist with already queued/delivered ephemeral cache publication. Do not claim that no cache change occurs after Abort solely because no document edit occurs. Revision fencing remains necessary when asynchronous completion reaches a changed document.

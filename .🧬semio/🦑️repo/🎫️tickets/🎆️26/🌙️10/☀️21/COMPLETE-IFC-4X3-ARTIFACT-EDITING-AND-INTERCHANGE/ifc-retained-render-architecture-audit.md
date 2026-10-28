# IFC Retained Render Architecture Audit

Read-only source audit, 2026-10-21. No production changes or native execution.

## Finding

A retained owner/cache is available, but it cannot resume the current synchronous projector or BSP stack. Yielding its tick callback unwinds that stack. Increasing the next turn allowance and replaying from the beginning is not continuation. The engine needs explicit retained frames for each expensive inner loop (including Boolean validation/refinement), plus a retained payload cursor.

## Actual mounted call path

Framework plugin root: C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin

- 📜️declaration/🪟️surface/{✏️artifact-editor-trait,👁️artifact-viewer-trait}/🦀️.rs expose synchronous render_with_request_context(owner, body_key, doc, cfg, view, transient, interaction, cx) -> OriginalRenderResult<ComponentTree>.
- 📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs exposes the async app wrapper; editor-app/viewer-app adapters forward the same original cx.
- 🧵️instance/🔌️plugin-app-impl/🖼️render-surface/🦀️.rs:65,94 awaits the app hook. Ok(None) retains no replacement tree. Existing completed tree is retained by dirty-poll rather than replacing it with partial geometry. Pending render tree has OriginalRenderTreeIdentity, controlled resource close, and original failure custody in 🧵️instance/👷️jobs/📥️staging/🖼️render/🦀️.rs.

Important capture gap: normal render constructs ArtifactView::with_render_context(..., None) around lines78-94. Thus ArtifactView::take_snapshot_read() cannot currently succeed there. The similarly named API is not proof that a render owns a transferable source. pending_effects separately prepares store.snapshot_read() when A::mounted_job_prepare_snapshot_read authorizes it, and gives that lease to ArtifactPendingView. Choose an explicit framework render-source preparation seam or a properly admitted pending-job route.

## Authority and lifecycle

📜️declaration/👁️view/🪪️operation-contexts/🦀️.rs AppRenderOperationContext contains app_instance_id, base_revision, generation, canonical_base_revision, mounted_policy. matches_revision compares all document authority. ArtifactView also exposes provisional_generation and parent_document_id. Cache identity must include the exact effective snapshot/overlay authority, geometry options and algorithm version; selection/locale should decorate completed geometry independently where possible.

📜️declaration/👁️view/🛬️lifted/👷️job/🦀️.rs:

- ArtifactInstanceOperationOwner: as_any_mut, retirement_demands(body), maintenance_step(cx, loan), close_step(cx, loan), terminal_is_empty.
- Handle::with_mut accesses a concrete mounted owner without a new job context.
- Handle::with_inference_session<T: RetireOwned>(name, &mut ArtifactInferenceSessionAccess<T>, cx, init, apply) provides controlled registry/frame admission, original op/generation identity checking, retained-grant child loan and typed controlled retirement. The access object itself must survive turns. inference_admission_demands quotes next actual operation; close_inference_step handles owner shutdown.
- with_inference_session_cold is explicitly cold and is inappropriate for interactive expensive initialization.
- Admission and progression must use cx.consume_fuel and cx.consume_retained; original_source_parent/source loans remain authoritative. No fresh unlimited StepContext is needed or justified.

## Real geometry precedent

C:/git/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧿️commands-and-instance-operation-owner/🦀️.rs owns GeometryHost plus FlowEvalSession. Its pending_effects and build_tool_run_job wire previewEval through an existing ToolRunJobPort, identity and progress publication. The render hook at ✏️editor/🪟️artifact-editor-impl/🦀️.rs:661 reads the prepared session; it ignores cx and is not a general resumption mechanism.

C:/git/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service/🦀️.rs owns GeometryEngine and an EngineAdmission donor. GeometryEngine at C:/git/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚂️engine/🦀️.rs:175 calls infer_field_step with retained InferenceCursor and InferenceValues under the original cx. Its close_original_step cancels and closes plan, values, base snapshot, cancellation ID and original source loan in order. Concrete compute jobs use step_on_original and explicit read/field cursors. Reuse the lifecycle pattern, not domain-specific procedural snapshots or services.

CAD and PDF hooks inspected are synchronous render calls; no prepared-scene continuation precedent was established there.

## Futures do not solve synchronous loops

⚛️reactor/🔄️turn/⏱️turn-execution-and-poll-entry/🦀️.rs with_turn_execution polls a pinned future and meters executing microseconds, excluding Pending gaps. This is a poll wrapper, not stackful suspension of a synchronous Rust closure. A future may retain algorithm state only if the algorithm itself awaits at safe points and does not retain a borrowed caller StepContext across distinct caller turns. No generic stackful geometry coroutine was found in inspected framework sources.

## Smallest sound integration

1. Capture an admitted immutable own-version snapshot/read once, with render revision and original source loan; never reconstruct it on every short turn.
2. Retain a geometry cursor in the mounted instance owner: admission, product/representation traversal, primitive tessellation, ring triangulation, Boolean operand validation/BSP/refinement, normals/bounds, payload measurement/emission, ready, closing. Each phase stores exact next indices/frames and bounded owned buffers; no replay of completed work.
3. Give each step only the original caller fuel and retained loan. Quote allocation/copy/release demands before actions and retain donor/partial outputs on refusal. Cancellation and stale revision enter controlled close, not a bare drop.
4. Expose GeometryScene through framework JsonWriteSource; use semio-framework-pack-json JsonBorrowedWriteCursor for native measurement/allocation/emission rather than a second JSON writer. Complete mesh+instance+status bytes atomically under one cap. TS needs a corresponding first-party cursor contract; no existing exported production TS controlled writer was established.
5. Publish/cache only a completed validated scene/payload. Subsequent interaction renders reuse it; changing effective source invalidates the entry and retires prior owned buffers through the same owner. Pending short turns must eventually complete the official bath/slab, without higher per-turn fuel or replay.

## Required proof

Use one original caller fixture across many tiny turns; compare bytes/mesh with one-shot reference; assert monotonic progress and no repeated expensive-stage counter; cancel in every phase; stale revision during projection and payload; exact byte cap and zero-fuel boundaries; controlled close releases source and buffers. Native syntax checks alone do not prove host scheduling or lifecycle execution.

## IFC2x3 selection correction

Fresh standard2 🚪️io/📐️geometry/🦀️.rs now calls render_scene(scene,None) at line7 and render_scene(scene,selection) at line45. The two previously reported native call-site defects are resolved in source. High reports expansion of all eight roots; this report does not claim native compilation or GUI execution.

## Budget Evidence Boundary

The inspected render hook forwards the original StepContext from mounted render and reactor poll_kernel; no independent render-specific numeric fuel limit was established in this audit. A command tool contract value (including 7500) is not evidence of render fuel. Therefore failure to finish bath/slab in an ordinary native UI turn remains conditional on that caller budget; no normal-host failure or native GUI execution is claimed. The proven issue is lack of resumption when any actual caller grants less than the complete synchronous workload. The reactor with_turn_execution wrapper meters poll execution against lifecycle timing, and poll_kernel accepts an already supplied cx; upstream platform caller/default grants require separate tracing.

# Shared Mounted Geometry for Picking

## Verified Boundary and Remaining Gap

The editor canvas now projects the completed vector plan. The authored pointer cursor still walks source paths and independently calculates shape/path hit tests. Its Boolean and trace fallback bounds are a fixed rectangle; actual resolved vector geometry is absent from that cursor. Selection presentation now uses completed geometry and the shared authored lock/group relation for handle bounds, while transform handle admission still uses that authored traversal. These are concrete source findings in `canvas-pointer-down/🦀️.rs`, particularly `TracePointerWork::Visit`, `trace_layer_bounds_with_matrix` and `consider_trace_candidate`. This report proposes the next implementation, not completed picking acceptance.

## Ownership and Identity

A gesture may borrow completed geometry only when the mounted instance, real source generation and canonical revision match its admitted operation. It must not use an older displayed picture for a newer command. Each query step borrows the matching plan; it stores scalar indices and bounded kernel state rather than cloning path/paint buffers. A source change invalidates the gesture through the existing operation authority. The old scene may retire only after no such borrowed step is checked out; no reference survives a callback.

If the exact plan is not ready, the query yields and retains its command owner while the real background producer runs. This must have an explicit host scheduling/progress law so it cannot freeze the UI or spin indefinitely inside one dispatch. Missing or failed preparation yields a named failure; it must not select approximate placeholder rectangles.

## Implemented Ancestry and Next Policy

Prepared nodes now carry the bounded authored `sourcePath` and unsigned `lockedAncestors` contract in both implementations. Ordinary group lineage and resolved trace/Boolean ancestry have neutral laws and an independent Three hierarchy oracle. See [ancestry evidence](🧭️prepared-scene-ancestry.md) for current TypeScript/native verification. The pointer is still not connected to that data. See [prepared selection relations](🎯️prepared-selection-relations.md) for the next schema-first lock/group policy and the existing stroke outline reuse boundary.

## Schema Work

Prepared leaf records need authored source lineage for selection and locking. Existing `groups` contains compositing scopes only and omits ordinary non-isolated groups; it cannot supply complete interaction ancestry. Introduce an explicit bounded authored path/lineage contract and inherited selection lock semantics in the preparation plan, with Rust and TypeScript twins. Update all constructors, copier/retirement boundaries and neutral fixtures together. Treat the new scalar/fixed-width fields as bounded storage, with an explicit byte census before any allocator admission claim.

Cached world bounds belong to scheduled preparation, not repeated render callbacks. Maintain separate geometry/control and painted bounds if handle geometry excludes stroke. Painted bounds must address cap/join behavior explicitly; the current affine circular envelope fixtures prove that envelope only, not all miter/square extents or shaped text bounds.

## Query Behavior

A grant-driven cursor traverses the completed paint order in reverse, skips hidden/transparent or locked targets according to authored lineage, and runs the actual first-party winding/proximity kernel over borrowed resolved paths. Image extent hit testing uses the prepared dimensions and transform. Semantic text continues through an explicit measurement port; fallback extents must not be represented as glyph outlines.

Keep selection policy separate from geometry: replace/add/toggle, selected ancestor preservation, locked descendant handling, direct-node controls and selected group transformations are authored interaction laws. Point edit ids remain bound to authored path geometry and are not minted for calculated Boolean/trace outlines. Marquee and lasso use the same scheduled world geometry and documented containment/crossing policy.

## Acceptance

- Shared neutral cases for resolved Boolean holes/differences, traced silhouettes, affine and reflected nesting, invisible/zero-opacity ancestors, lock ancestry, paint order and group selection.
- Independent SVG/pixel and Three oracles for geometric results, with actual native/runtime diagnostics.
- Every positive grant produces the same completed query; cancellation and stale source publication retain no partial selection.
- Actual registered editor tests connect displayed paths, transform handles and pointer admission, preserving one history edit on release and cancellation with zero edits.
- Runtime source edits while queries wait, old scene retirement during replacement, exact read handback and slot reuse.
- Browser pointer, keyboard and localized progress/cancellation checks on the current staged component.

The original editing goal remains active. This next boundary must be implemented and verified before claiming consistent completed-scene selection.

## Current Command Scheduling Findings

The actual gesture reducer in `✏️editor/🦀️.rs` advances `query.cursor.advance(snapshot)` before grab preparation and selection publication, returning `None` while work remains. Its retained owner carries the admitted AppOperationContext separately from the renderer authority; AppOperationContext does not carry the render base counter. Exact scene access therefore needs a checked bridge to the genuine renderer-owned source identity, not an invented base copied from a command id. The renderer already suppresses handles and previews when `with_visual` reports an older source, but a current pointer command still independently traverses the authored snapshot. Trace-pointer selection also retains a separate cursor and dispatch continuation, so migrating only generic point queries would leave the trace tool inconsistent.

The host law for waiting on a not-yet-ready complete scene must be established before replacing that authored cursor. A retained query returning `None` while waiting must prove host fairness, genuine producer progress and cancellation. The actual `StepOutcome::Yield` protocol is distinct from the BoundedJob reactor's stall guard. Eager producer execution inside a query would violate cancellation and responsiveness. A genuine job/operation wait relationship is required alongside actual stale-source refusal and completed-scene borrowing.

## Verified Source Admission Bridge

Current host source in `capture_typed_command_roots` and `live_render_operation` derives the same genuine base lane from the first eight bytes of the content hash. The actual gesture factory now captures `ArtifactOwnedToolJobRequest.operation.base_revision` alongside instance/generation/hash, and both preview entry points check the full captured source identity. The mounted registry has a matched completed-plan callback and persistent current failure state. See [exact full-width identity](🪪️source-authority.md) and [query source admission](🎯️query-source-admission.md) for contracts and current verification. Pointer cursor migration and actual waiting/painted-query runtime acceptance remain required.

## Current Host Revalidation — 2026-10-04

The genuine DrawingGestureOperationJob implements framework InteractiveJob. Its current step first checks cancellation, deadline and fuel, performs one raw-wire/reducer action, and returns StepOutcome::Yield after dispatch returns None, consuming one fuel unit. The framework StepOutcome definition explicitly treats Yield as nonterminal and resumable. No InteractiveJobYield type exists in current source; the earlier prose label is corrected above.

The mounted scene producer is a separate BoundedJob retained by the real reactor; GeometryJob advances DocumentVectorJob under the admitted fuel and an eight-millisecond deadline, returns a changed work checkpoint when it makes progress and publishes only matched source output. A query must never drain this producer itself. Current plugin host source resumes a checked-out nonterminal worker outcome after owned payload retirement, rather than treating Yield as an immediate terminal failure. The cold-relay pump performs one caller opportunity and returns Poll::Pending, registering/waking the retained owner as appropriate. These are source findings in Drawing editor step, framework job StepOutcome/drive_worker_job_authority, mounted geometry GeometryJob and plugin host GuestRelayMountedRegistry::finish_outcome/pump.

Those individual source paths establish resumability, not the missing Draw integration fairness law. The actual mounted query and scene producer must still run together under the editor host, with a user event admitted between waits, genuine producer work advancing, source replacement refused, cancellation reaching empty retirement and one publication only after Ready. Generic and trace point cursors, marquee/lasso and transform handle admission must move together onto the exact completed-scene relation. No current waiting query/browser acceptance is claimed from this read-only investigation.

## Painted Query Implementation — 2026-10-05

Current source revalidation confirmed that PathHitCursor treats every stroke as a scalar world-space radius. It does not account for butt/square caps, miter/bevel joins, dash gaps or anisotropic stroke thickness. The immediate schema-first change is a borrowed-source painted query cursor using the existing first-party flatten and stroke-outline kernels. Each grant admits one source segment or advances one real child/edge/retirement unit. Fill winding and stroke polygon unions share the renderer geometry algorithms; output is withheld until actual private children and buffers retire. Tests precede implementation and compare neutral results to an independent SVG renderer.

This stage establishes the reusable painted query, not completed-scene host migration. Generic and trace pointers, region selection and handle admission still require the matched complete-plan integration and host waiting/cancellation law described above. The original editing goal stays active.

## Painted Regions and Mounted Source Runtime — 2026-10-05

The new Rust/TypeScript painted query reuses actual first-party flatten/stroke geometry, handles caps/joins/dashes/nonuniform transforms, and consumes genuine child retirement. A resolved-leaf constructor and neutral Boolean/PNG trace samples connect it to actual completed geometry. A registered native mounted source/query test exercises Pending, actual separate producer advancement, Ready segment-pointer borrowing, real deletion/replacement Stale refusal, interrupted actual child cleanup and complete source return. See [painted query implementation and evidence](🎨️painted-query.md). This does not yet replace gesture pointer traversal or establish editor InteractiveJob fairness/browser acceptance.

Preparation should cache painted contours and world bounds once per source revision before migrating hover/point/area/handle consumers. Per-hover stroke reconstruction would conflict with maximum performance and the existing fixed gesture byte declaration. Those cached owners need their own genuine scene-plan retirement, source count/byte admission and read handback; no per-buffer allocator timing is inferred from structural cleanup laws.

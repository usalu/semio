# Drawing Text Current Font Contract and Repair Plan

No Source edits or semantic runs have been made for this plan. Native11 is the actual compiler RED: outer45307 EXIT1/Cargo101 before tests, five Drawing scene text/font API errors. Its finalized receipt and log remain under generated/layout-native-11.

## Current defining contracts

Drawing scene text Source is schema/scene/text/🦀️.rs, current SHA239d6c456edadd67ad7f25430f65e4d7eda3c75ef5b7ed76a80d50dd026b0f88. It owns Unicode admission, catalog jobs, admitted font vectors, the active/completed runs, actual glyph output and converted geometry until paid retirement. Its current item-only advance cannot grant byte/capacity/release/depth authority.

Current first-party FontWorkLedger is font/work/🦀️.rs. It accepts an actual RetainedCloneGrant, consumes item/copy/capacity/release/depth through physical reservations and retained writes, and exposes receipt(), remaining(), blocked() and next_demand(). One ledger must span a parent turn and all Unicode/catalog/run descendants; creating fresh per-child ledgers would replenish authority. Slot reservation and source/child pointer checks must remain.

FontRunStyle now explicitly distinguishes metric-only versus actual outline production. Drawing must set outlines:true because its result is real glyph path geometry. Current uniform Drawing text has one size/family and no style ranges or forced direction, so a stable empty FontRunSpan slice and UnicodeBidiDirection::Auto are the faithful semantic inputs. They preserve current Unicode normalization, bidi/mirroring, shaping, fallback and source identity. No outlines:false shortcut or fabricated span is proposed.

UnicodeDataAdmissionJob.advance, FontCatalogAdmissionJob.advance and FontOutlineRunCursor.advance now receive the same mutable FontWorkLedger. Run birth additionally captures the stable spans pointer/count and direction. The current shipped constructors are allocation-free initial cursors; actual retained page/vector births happen under the ledger.

## Proposed first-party interfaces

Preserve the item quantum as semantic scheduling, add the actual parent ledger explicitly:

- DocumentTextJob.advance(items:usize, work:&mut FontWorkLedger) -> Result<DocumentTextProgress,DocumentSceneError>.
- DocumentTextJob.next_work_demand() -> RetirementDemand, holding the actual pending demand from the previous attempted turn; no authority is returned.
- DocumentVectorJob.advance_scene_source(source, items, work), advance(items,work), and DocumentVectorPreparationJob.advance(source,items,work) forward the SAME ledger through the text phase. Their next_work_demand() delegates to the current text owner when present.
- DocumentRasterJob.advance(items,work) forwards it through vector preparation. Its next_work_demand() delegates while the vector is resident.

Replace the old signatures and all real callers; do not keep a permissive numeric-only compatibility path. The caller owns the ephemeral ledger and always reads/consumes its actual receipt, including on error. No new runtime dependency or public foreign type is introduced.

Text-owned completed-run/output PagedList reservations/pushes will use ledger.slot/push. Geometry contiguous capacity and per-segment writes will use ledger.contiguous/vec_slot/vec_push rather than uncharged try_reserve_exact/push. Fixed owner transfers/phase publication must be admitted before mutation. Denial with no receipt must not burn semantic work or advance custody. Preparatory allocation with actual receipt remains observed progress. Existing max_work ceiling stays unchanged; no operation-count-to-byte conversion or unlimited grant is allowed. Cancellation still refuses further semantic publication while all actual owners drain in the existing controlled retirement path.

## Actual caller map

Drawing owned export has genuine mutable StepContext at io/export/owned/🦀️.rs:32. Vector preparation and raster branches currently call advance(1) and consume fuel. At these exact boundaries construct one ledger from cx.retained_grant(), call the new producer interface, consume the actual ledger receipt BEFORE propagating success/error, and retain the producer on pending/refusal. Reconcile scheduling fuel with the existing StepContext receipt semantics so physical work is not double-counted.

Drawing editor geometry has the original mutable StepContext at its BoundedJob.step. Its internal advance currently borrows that context immutably; change that owned local seam to mutable so the same actual grant/receipt can be passed and accounted. No synthetic context or registry replacement is needed.

DocumentVectorJob/PreparationJob/RasterJob internal forwarding and current test callers must migrate coherently. Layout DrawingCompositionState does NOT use these classes: it already uses DrawingPlanCursor and LayoutParagraphState with a real five-axis grant and folded receipts. Do not change that working funded Layout paragraph path.

## Schema-first regression and existing independent parity

Preserve the fourteen existing neutral glyph cases and all three existing Drawing text laws, including rustybuzz shaping, ttf-parser outlines and tiny-skia geometry comparison behind their existing test-only interfaces. The genuine owner is semio-s-artifact-draw-drawing-scene-tests in the Drawing nested Cargo workspace; Layout full19 only compiles this dependency and does not execute its cfg(test) companion.

Before production integration, add a small neutral work contract/cases defining denied axes (items/copy/capacity/depth), finite paid turns, semantic work exhaustion and cancellation at Unicode/catalog/run/geometry/publication boundaries. Actual tests will verify zero physical heap/progress/publication on denied turns where demanded work cannot proceed; paid receipts fit original five axes and actual heap; nonempty glyph paths match existing independent geometry, and cancellation/exhaustion retain output privately until controlled close. No fixture-only pass substitutes for actual owning Native execution.

Dep's companion readiness repair will put the inner doc at the start and replace inaccessible Owners.plan inspection using public progress.nodes at single-item boundaries, exact aggregate refusal/no completion/no output and controlled retirement. No private API export is needed.

Proposed verification after Source review: full Drawing scene-tests companion --lib with actual original funded caller/decoder/heap harness for the new law, plus unchanged full Layout nineteen-owner acceptance. Root must release exact normal canonical commands; no runtime is authorized by this plan. Source budgets/policies remain unchanged, and actual canonical admission remains the fit proof.

## Reviewed clarification: CPU, demand and caller composition

Exact agreed text methods are advance(items:usize, work:&mut FontWorkLedger), next_work_demand()->RetirementDemand (direct), and last_turn_work()->u64. TS names are advance(items,work), nextWorkDemand() and lastTurnWork(). Vector/Preparation/Raster owners expose the same pending-demand/last-turn-work observations and pass the same mutable ledger. last_turn_work resets per call and records actual outer semantic/preparatory turns, including work performed before a child error; a purely denied no-effect turn records zero. No item-only compatibility overload remains.

The actual StepContext defining Source at framework/modules/job/🦀️.rs:248–251 confirms consume_retained only posts retained progress; it does not spend CPU fuel. consume_fuel:363–364 is a separate wallet. Native boundaries must therefore consume the one actual child receipt exactly once, then consume last_turn_work exactly once. Child copied_items is not added a second time to the parent's CPU count. A parent turn delegates to one child quantum; successful structural handoffs and actual preparatory allocations count as actual work. Pure denied entry does not. Cumulative ledger receipts must not be reposted repeatedly: each actual native turn creates one ledger from that caller's remaining grant, all children share it, and only the final receipt for that turn is posted.

max_work remains the original finite ceiling and is checked before another semantic turn. No fresh ledger resets the owner-wide counter. Child/source faults retain all admitted owners and publish no result; their observed retained effects and actual CPU are charged before returning the primary error. Entry/exhaustion refusal without doing semantic work does not invent a work charge. The job caches the actual pending next demand after blocked or error turns; preparation and raster delegates preserve it while that child remains resident. Structural non-font handoff/header and text geometry turns need their own quoted copy/capacity/depth demand rather than only replaying a stale font demand. Closed/failed state is not a renewed work grant.

Export's existing pending branch uses continue inside a fuel/deadline loop. A blocked zero-effect text/vector/raster turn must instead return pending to its real caller; it cannot spin while no fuel was consumed. Nonzero preparatory effects are charged and retained before yielding. Geometry's real mutable BoundedJob.step context is available: its local immutable advance seam must become mutable and charge actual turn work, including errors, without a replacement context or clock/deadline.

Every directly identified production migration is within Drawing: scene/text; scene/preparation VectorJob, PreparationJob and RasterJob; io/export/owned vector and raster phases; editor/geometry producer phase. The sibling Layout composition is independent and already funded, and must remain untouched. All direct public Rust/TS consumers and serializer/scene/editor tests of the removed numeric-only signatures must be mapped and migrated coherently before SourceReady; compilation of a dependency is not execution of those test laws. Dep owns exact TS mapping and test-first contract; no test grant will be manufactured from item count or a permissive shared helper.

Dep's neutral contract includes meaningful positive-axis advance denials and all five actual controlled-close denial axes, including genuine positive release demand. Existing actual close methods remain authoritative; no zero-release fake case will be labeled release-denial proof. I own genuine StepContext composition regressions at Drawing export/geometry seams, not flat mock ports: one actual granted receipt is visible on child error, semantic fuel is spent once, and blocked work returns without spinning. These complement, rather than substitute for, Dep's pure text/geometry parity laws.

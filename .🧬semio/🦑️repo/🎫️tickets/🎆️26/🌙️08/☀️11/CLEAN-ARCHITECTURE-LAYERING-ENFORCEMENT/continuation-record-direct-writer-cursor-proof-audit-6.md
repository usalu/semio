# Direct Record Writer Cursor Proof Matrix

Read-only review of the parent's proposed performance/custody change. Current inspected production still uses an eager twelve-deferred sequence at encoding282; no implemented or passing claim is made here.

The direct cursor is a sound canonical direction: a fresh owned writer has exactly one Box<Self> constructor birth; the already boxed child writer has constructor birth0 and must transfer the same allocation by unsizing, not dereference/move/rebox. Terminal physical release is sizeof(Self) for either origin, but only after all owned fields are empty and all transferred children have closed. No payload buffer is cloned.

## Required field transitions

Preserve original sequence order: source, spec, order, text, intrinsic_frames, intrinsic_path, emitter.output, compound, wire, nested, child_spec, retiring. An inline retirement position distinct from encoding phase/index is necessary; cancellation may start at any encoding phase. RetireOwned may initialize that position before Box allocation without allocating other scaffolds.

| Field | Required next authority | Transfer/empty behavior |
| --- | --- | --- |
| source Option<RecordValue> | exact selected RecordValue constructor | Take only after admitting birth; empty None advances0 |
| spec Option<RecordSpec> | exact RecordSpec constructor | Same; preserve whole fields Vec backing |
| order Vec<usize> | exact Collection constructor for nonzero capacity | Reserved-empty Vec is not terminal; empty zero-capacity advances0 |
| text Option<RecordTextStep> | selected leaf constructor if present | None advances0; keep payload work under3 |
| intrinsic_frames Vec<RecordIntrinsicFrame> | original Collection constructor | Preserve reserved-empty backing |
| intrinsic_path Vec<usize> | original Collection constructor | Preserve original backing and scalar-copy3 fallback |
| emitter.output Option<String> | exact String cursor constructor if physical capacity exists | None/Some zero-capacity can be cleared with Advanced0; never silently free reserved capacity |
| compound Option<RecordCompoundStep> | selected leaf constructor if present | Must preserve copy3 leaf semantics |
| wire Option<RecordWireStep> | selected leaf constructor if present | Same |
| nested Option<Box<Writer>> | zero constructor birth | Return identical owned Box as cursor Child; do not invoke generic Box RetireOwned wrapper |
| child_spec Option<RecordSpec> | selected RecordSpec constructor | Do not drop when bypassing empty options |
| retiring Option<ControlledRetirement<RecordRetirement>> | exact Controlled<Self> Box constructor | Preserve original inline value/frontier; nested cursor forwards independent grant/depth |

## Denied-grant and accounting obligations

For each present field, next_birth_bytes must match the actual constructor used by close_step. Do not take/replace/increment before checking the admitted capacity and depth. A missing Option can advance with no birth/work/release. A zero-capacity Vec can clear with no allocation; nonzero-capacity reserved-empty cannot. Zero item grants leave every field/position unchanged. Generic constructor cannot synthesize a hidden Option/Deferred/BoxedOwner allocation.

RecordRetirement::Writer birth must change to0 and retirement return `value` coerced to Box<dyn RetirementCursor>. Calling `value.retirement()` retains the generic Box<T> constructor and defeats the saving. RecordRetirement::Spec still delegates its exact constructor. Writer terminal release must be its original sizeof(Self), and terminal witness must include emitter output, all twelve fields, nested ownership, and inner controlled frontier. Scalar encoding metadata can remain inline because dropping it frees no backing.

Existing controlled parent admits frontier pages independently, then requires the queried child birth before transfer and measures terminal cursor release. Dynamic depth guards must remain prior to page reservation. If the writer itself drives an existing controlled retirement rather than returning it as Child, it must forward the full remaining grant, categories, and actual next-depth demand; never reconstruct byte-only authority.

The saving is repeated scaffolding, not a permission to stop charging actual fresh birth. Original table64 1MB and copy3 budgets should remain fixed. Verify denied0/one-below/exact heap at initial writer root, original nested Box transfer, each twelve field positions, reserved-empty buffers, and cancellation with partially retired inner owner. Keep the original semantic text/Serde fixtures and original turn caps. Separate heap receipts must prove no hidden scaffold allocation, exact Box terminal release, and unchanged final payload.

A release observer proves allocator events but not source order by itself: retain per-field position/preimage and output/semantic assertions. A cancelled direct cursor must not run an eager recursive Drop. No whole-package acceptance follows until the original owning target closes on exact selected sources.

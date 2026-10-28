# Native Contextual Enumeration Attachment Independent Audit

Read-only source checkpoint, 2026-10-21. No new confirmed P1/P2 defect in the reviewed attachment/index paths. This is limited source review, not native compile/runtime approval. Authentic compiler ingress remains refused before rustc.

Reviewed shared IFC details `contextual.rs`, `contextual_index.rs`, `contextual_render.rs`, `contextual_edit.rs`, their index/render test sources, plus work.rs routing/close and value_edit.rs native supply. Paths are rooted at `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details`.

## Source observations

`contextual_index.rs:45–141` quotes metadata, growth allocation, old capacity release and row relocation before state changes. It binds snapshot pointer/revision, checks cancellation and grants first, builds one ID/ordinal row per turn, and performs at most one heapsort swap per turn. Completed lookup is binary search plus immediate duplicate neighbors. Exact u64 IDs are preserved. No source-value clone or per-property full entity scan occurs in the render's indexed query route. Index identity includes its stable rows pointer and length and is checked by contextual.rs:311 before query continuation.

`contextual_render.rs:103–200` captures snapshot pointer/revision, completes one shared index, then scans entity/root and complex constituent addresses incrementally. Query results become small scalar entries in source traversal order, matching the later binary-search ordering by entity/part. Only complete work is exposed through validate/at. Source pointer/revision mismatch refuses; cancellation does not publish partial metadata. Entry vector growth and query close are explicit quoted phases. Actual original Arc/full canonical revision retention is owned by the generic host/editor wrapper and remains a separate lifecycle dependency, not supplied by these pointer checks alone.

`contextual.rs:132–245` resolves actual EnumerationReference, rejects duplicate owner/target identities, checks root and actual complex constituents, and refuses ambiguous enumeration constituents. It selects the actual schema EnumerationValues slot and borrows complete allowed values; no static PSD choice substitution. Unavailable/unset/empty/wrong-kind are explicit metadata outcomes.

`contextual_edit.rs:30–55` admits only closed small selection requests, exact positive definition IDs, canonical decimal ordinals and explicit list mode. `advance:105–192` pins original event/snapshot, requires whole attribute replacement or complete aggregate-item address, re-queries the actual addressed property's enumeration, checks original definition/choice availability, and uses the captured SnapshotSource projection with the retained clone cursor. Whole-attribute replacement wraps the complete actual value in one paid singleton aggregate; item insertion/replacement transfers the complete native owner. No UI label or request payload substitutes for the native value.

`value_edit.rs:133 supply_native` transfers only into an unstarted empty receiver and leaves the supplied owner intact on refusal. `work.rs:468–473` runs this receiver before reference/value staging and preserves the complex constituent route for actual source choices. Central mutation/inverse emission is not replaced by the choice query. Work close includes enumeration before source owner release; contextual_edit close drains clone cursor, received value retirement and query before clearing binding.

## Verification boundary and remaining coverage

Index/render authored tests deny items/copy/capacity/release/depth/fuel before each accepted continuation; render tests enumerate cancellation prefixes and controlled close. These test sources support the intended laws but were not compiled or executed natively. Allocation capacity equality, actual grant receipt acceptance, central complex mutation/inverse behavior and host retry lifetime therefore remain genuine native execution gates.

The edit receiver uses a separate progressively scanned query rather than the per-render index. This is bounded per continuation, not an eager cold scan; its total O(entity count) cost for a single submitted edit should be described separately from the render index's shared O(N log N) preparation and logarithmic per-property lookup. No blanket constant-time lookup claim applies to edits.

This review does not certify the entire existing details tree renderer, ViewContext decoder/clone ingress, generic host retirement, or the still-developing guided action form. The earlier TS signed-zero membership and complex-target fixes are recorded in details-enumeration-independent-audit.md and are not new findings here.

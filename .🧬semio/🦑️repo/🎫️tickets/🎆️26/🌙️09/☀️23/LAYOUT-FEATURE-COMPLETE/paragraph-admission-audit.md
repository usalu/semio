# Original Paragraph Admission Authority

Follow-up: the direct authority and lazy source-count admission repair is recorded in [font-authority.md](font-authority.md). The findings below describe the earlier audit state; see that report for current verification and remaining boundaries.

## Focused Result

NumericIndex structural reservation is now funded through its original PagedList owner, and Layout MetadataIndex, FramePlanState and FlowProjection use that contract. The portable laws and original-context seams are recorded in [frame-preparation.md](frame-preparation.md).

The shared paragraph pipeline still needs its own original-grant contract. Adding an outer demand alone would not fund the following child operations; this report does not present them as controlled allocations.

## Source Findings

1. Shared scene/paragraph begin_story_box allocates spans, colors, italics and style metadata with Vec::with_capacity(style_runs.len()+1) before a StepContext is available. begin_label similarly allocates four one-element Vec buffers. FontParagraphCursor captures borrowed contiguous spans, so replacing these with a paged owner also requires a real borrowed span-source interface or an admitted contiguous materialization phase.
2. Shared ready_one advances line-break and Unicode admission 64 work units, then FontFaceAdmissionJob 64 units. Success in those three admission jobs retains only fixed scalar metadata and original static byte borrows; their current successful phases do not allocate a source-sized buffer. Their failure strings and work-limit paths still allocate owned error text.
3. Shared ready_one reserves fonts Vec capacity and archives the admission job using font::reserve_slot. The latter reads next_allocation_bytes and directly calls reserve_one using that demand as its capacity argument. It has no incoming retained grant or returned physical receipt.
4. Shared style lookup is one style record per turn but compares complete original identifier strings in that turn. A long identifier comparison therefore remains unbounded by source-scalar work. The shared MetadataIndex is now available for a source-qualified incremental style-ID read model.
5. Framework font/run/paragraph phase 0 admits a whole spans Vec; phase 8 admits local_spans/local_ids for every line. Measurement, order, sorting, lines, placements, shaped runs, archived spans/IDs, glyphs and segments use the same reserve_slot helper. Several phases can call it twice. The wrapper advance(64) can therefore retain several original allocations in one externally visible Layout turn.
6. Paragraph measurement delegates FontOutlineRunCursor, which delegates Unicode normalization, clustering, bidi, OpenType substitutions/positioning and outline cursors. Those owners have bounded semantic cursors and controlled retirement, but their construction/advance APIs do not carry an incoming retained grant or return a retained receipt. Rewriting only the Layout wrapper cannot prove original physical authority for the nested operations.
7. Interactive LayoutRenderWork currently calls shared text.advance_one from its phase-4 cold step. ExportTextState likewise calls shared advance_one while pushing to a Vec<PdfItem>. Their original StepContext seams already fund flow projection and output publication, but they do not reach these active paragraph child operations.

## Required Direct Contract

A proper next change should extend the first-party font work contract with original RetainedCloneGrant and a scalar work allowance, an exact next-demand query for the actual pending child phase, and an actual retained receipt available even on allocation failure. Each parent must forward the original remaining grant and debit the child receipt before any further work. Demands remain quotes, never authority. Layout begin methods should only retain scalar original addresses; style buffers should be admitted later through that same owner before capture by FontParagraphCursor.

The permanent neutral suite should deny source work, copy, capacity and depth independently at each allocating phase, verify no retained progress on refusal, then resume with the exact quote and compare glyph IDs, source ranges, line placements and pixels against the existing OpenType/Sharp oracle. Native heap observations belong in the existing framework trace test interface; native execution remains unavailable through the current repo frontend.

## Verification Limit

This is a source audit, not an implementation or a native pass. It deliberately avoids introducing an unused outer grant wrapper or assuming nested allocations are authorized by a larger parent demand. No build infrastructure or unrelated framework API was changed for this audit.

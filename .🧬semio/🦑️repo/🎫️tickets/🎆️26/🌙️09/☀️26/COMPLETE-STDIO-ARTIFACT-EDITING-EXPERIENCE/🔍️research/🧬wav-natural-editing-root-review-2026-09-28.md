# WAV Natural Editing Root Review

Root inspected the new WAV-local natural edit command and main table source. Native behavior remains unverified. Findings assigned to the media execution worker:

- Channel rewrite rounds `frames_per_chunk` up to one even when one frame is larger than `PATCH_PAYLOAD_BYTES`. A wide frame therefore defeats the declared payload bound. Add explicit admission or transform at sub-frame granularity.
- `updated_fmt` clones `fmt.ext` during extent/planning; cancellation `close_step` ignores its byte grant and drops the plan and format extension at once. Use bounded retained copy and retirement, including zero-byte and tiny-byte close laws.
- Main builds all channel labels and all cells for each visible frame before the fixed UI list rejects overflow. Row windowing does not bound column width. The natural surface needs a window or selected-channel slice.
- Shared `render_structural_table` uses literal Add row/Add column/Headers labels. Changing action declaration labels does not change these rendered WAV buttons. Expose domain-neutral labels or render a WAV-owned frame/channel toolbar.
- Sample-rate and insert-channel actions require a raw document revision in palette arguments and lack visible controls carrying that revision. Add direct revision-bound controls. Verify exact keyboard dispatch for actions requiring frame/channel indices, not just key strings in declarations.

The worker owns repairs and evidence in the main WAV natural editing report. This is a source audit, not an executed runtime result.

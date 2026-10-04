# PNG Controlled Work Audit — October 4, 2026

Read-only review of the newly mounted controlled pipeline found three remaining boundaries. These are source findings, not runtime measurements. The Office execution lane owns repair and its first native gate is queued.

1. `PaintNativeRegionWork::step` calls the complete decode in `begin_native_paint` and complete filtering, deflate encoding, and output assembly in `finish`. Their callbacks consume fuel but do not yield on its exhaustion. Cancellation callbacks improve native interruption, but do not establish bounded interactive work or allow a blocked browser worker to receive cancellation. These phases need retained resumable primitives and tiny-grant progression laws.
2. `PaintNativeSamplesMutation::diff` calls `validate_completed_native_paint`, which decodes the base again, repaints all rows, and decodes the completed result with unconditional callbacks. Thus publication/replay still recomputes expensive codec work despite the completed result in the event. Preserve semantic and forged-result admission, but perform expensive validation in the retained pre-publication stage and keep replay bounded over its admitted result or patch.
3. `PaintNativeRegionWork::close_step` ignores its byte grant and sets the whole retained operation to `None`. That drops all row/sample buffers at once and reports completion. Retirement must consume explicit row/sample grants and report progress until its owned state is empty.

The existing controlled decode/encode cancellation and exact IDAT preservation tests remain useful. They must be complemented by actual retained-work and replay/retirement witnesses before this pipeline is accepted as complete.

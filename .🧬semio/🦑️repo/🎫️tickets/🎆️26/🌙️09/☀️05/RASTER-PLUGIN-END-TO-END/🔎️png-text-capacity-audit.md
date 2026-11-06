# PNG Text Capacity Audit

The text cursor first scans the CRC-validated immutable source to count tEXt/zTXt/iTXt chunks, then reserves count times the row tuple size once into an initially empty vector. At most that many rows are emitted, so ordinary row pushes do not grow or reallocate the vector. Key and value backing are measured and reserved separately before copying. Allocation receipts capture actual reserved capacity, including a partial inflater-history allocation on refusal. Metadata source remains in the decoder; a malformed compressed stream retains its inflater inside the text state for controlled retirement.

A compressed-text bomb can exceed the configured text ownership extent in computation: during the initial decompression/measurement pass text_byte increments value_length per inflated byte but checks the maximum only at the later value-allocation phase. This remains cancellable and bounded per turn, but may burn much more work than required before inevitable refusal. Check the incremental normalized length against remaining text ownership during measurement and retain the inflater for funded close on failure.

The renderer PngTexture output witness uses pixel pointer and length. That recognizes a different live allocation but cannot prove the content or image headers stayed unchanged: in-place pixel/header mutation preserves both, and allocator address reuse permits ABA. Do not treat that witness as a sealed immutable owner identity if callers can mutate the recipient after receipt failure.

Source review only; no native tests were run.

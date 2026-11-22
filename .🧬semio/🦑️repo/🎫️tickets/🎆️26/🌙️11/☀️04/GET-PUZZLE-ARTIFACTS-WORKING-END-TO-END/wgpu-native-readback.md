# Native WGPU Readback

The existing production readback module and its original tests linked against the warm native renderer dependency artifacts with the ticket's `verify-wgpu-readback-warm.py` probe. The SwiftShader Vulkan ICD ran the test with both desktop display variables unset and no production device-policy change.

- `prepared_readback_admits_dimensions_before_allocating` passed the nine dimension and row-alignment cases.
- `prepared_readback_normalizes_bounded_rows_without_padding_or_channel_loss` passed both RGBA and BGRA fixtures with misaligned row widths and mixed alpha.
- `prepared_readback_reads_the_production_target_and_never_publishes_cancelled_bytes` created a production headless device and the production composite target for each format, submitted WGPU commands for its six oracle colors, asynchronously read each result, and round-tripped the pixels through PNG. Both cancellation checks withheld bytes and the WGPU validation scope was clean.

The suite reported three passed, zero failed, and zero ignored. The harness uses the current `PreparedGpuReadback` source with warm device, composite, and codec libraries. It exercises the offscreen output path without a Puzzle UI scene or browser input.

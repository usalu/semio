# Raster PNG decode turns

The native browser accepted a 687-byte compressed PNG containing 256 × 256 pixels. The Mounted Raster import used one retained decoder transition per worker turn. PNG reconstruction emits up to one filtered scanline byte or one RGBA pixel per transition. That image requires at least 262,400 scanline transitions and 65,536 pixel transitions, before headers, allocation, and DEFLATE bookkeeping.

Raster now batches checksum bytes, inflate transitions and pixel samples under the original source effect. Each transition checks the cancellation witness and deadline, remaining authenticated work, and the aggregate retention grant. A checksum batch stops at the last byte in the active chunk, before CRC verification or the IDAT range reservation. The batch carries zero capacity and inflate history reservations retain their separate admission.

The native typed operation lends the original caller's remaining credits through Raster's mounted preparation policy, then applies the import tool's 4,096 work units and 7,500 microseconds per callback. The runner checks the host turn's 4,000-microsecond elapsed limit between callbacks. Under the former 32,768-byte preparation quota, the two inline Huffman tables alone account for at least 1,218 copied bytes per inflate transition. A turn could fund at most 26 filtered bytes. The 256 × 256 image would need at least 10,093 host turns for the filtered bytes alone, beyond one standing drain of 4,096 polls.

Raster preparation now lends up to 1,024 items and 1 MiB copy per turn under the existing item, copy, work and deadline authority. The WGPU drain uses MessageChannel task boundaries independent of display cadence. The earlier one-transition worker needed at least 327,936 callbacks. That would require an average under 92 microseconds per callback to meet a 30-second observation window before any setup, publication or rendering.

The neutral RGBA Paeth fixture checks exact checksum progress through IHDR and IDAT and that zero-capacity credit defers the separately funded IDAT range. It checks that one original turn performs and reports three inflate transitions before the continuation predicate yields, and that a resumed turn with a two-unit work limit reports exactly two further transitions. The completed image is checked against the independent Rust PNG decoder and the fixture. Python zlib independently verified the fixture CRCs, inflated its IDAT stream, and reconstructed the expected 3 × 2 Paeth pixels. The parent session owns the native component build and browser rerun.

Rustfmt parsed all four touched Rust sources. The scoped whitespace check and the neutral preparation-policy arithmetic passed. The exact native companion command entered the repository source admission and queued for 9.7 GiB of compiler-image headroom. It was cancelled before compilation to release capacity for the browser component build. The native law has no execution result yet.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --locked --manifest-path '🧰️framework/🔨️modules/🔲️pixels/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml' png_decode_batches_funded_transitions_and_resumes_after_yield -- --nocapture
```

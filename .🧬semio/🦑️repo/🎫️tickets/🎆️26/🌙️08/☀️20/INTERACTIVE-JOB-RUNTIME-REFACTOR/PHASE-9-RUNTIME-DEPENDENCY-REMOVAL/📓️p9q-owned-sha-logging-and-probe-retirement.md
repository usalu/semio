# P9q Owned SHA-256, Hub Logging, and Probe Retirement

## Owned SHA-256

`semio-framework-hash` now provides a dependency-free incremental `Sha256` state, one-shot digest, lowercase hexadecimal encoding, and `sha256_hex`. The implementation is streaming: updates retain only one 64-byte block and preserve caller-independent state.

The package descriptor emitter and Layout package manifest now consume the owned implementation. Their direct `sha2` manifest edges and source imports are deleted. Descriptor field names and output remain literal SHA-256; no algorithm substitution occurred.

An initial focused test exposed a real segmented-update defect: a partial buffered block was reset when a subsequent update did not fill it. The implementation now returns with the retained partial block intact. The final suite covers the empty and `abc` NIST vectors, the multi-block NIST vector, and seven-byte segmented input across the block boundary.

## Hub Logging

Hub used `tracing` and `tracing-subscriber` only for three startup warnings/information messages and formatter initialization. Those calls now emit owned `[WARN]`/`[INFO]` process-boundary diagnostics through the standard error stream. Both direct dependencies are deleted without changing server control flow.

## Retired Research Fixtures

The complete `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe` source/manifest set was removed. It was a standalone concluded Wasmtime 47/Tokio experiment, had no repository consumer, contained a hard-coded path into an older ticket, and duplicated questions now answered by the owned worker/job/plugin runtime. The removal includes both host/guest pairs and the driver-send spike. Historical content remains recoverable from Git history.

## Verification

| Gate | Result |
| --- | --- |
| `cargo test -p semio-framework-hash` | 6 passed, 0 failed |
| SHA-256 NIST and segmented vectors | passed |
| `cargo metadata --no-deps --format-version 1` | passed; both new internal dependency paths resolve |
| focused `cargo fmt --check` | passed |
| non-Compose `sha2`, `tracing`, `tracing-subscriber` source/manifest census | zero |
| dependency audit | 222 current versus 238 baseline; no additions |

The descriptor-emitter and Layout product boundary checks remain part of the final Phase 9 product sweep because both still traverse larger Wasmtime/stdio dependency graphs. This report does not claim those product-wide gates yet.

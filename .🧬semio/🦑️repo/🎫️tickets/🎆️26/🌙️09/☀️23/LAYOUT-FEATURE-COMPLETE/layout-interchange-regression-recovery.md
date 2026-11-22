# Native Layout Interchange Regression Recovery

The actual macOS Layout I/O run finished all three test binaries. The library passed five tests, Drawing Birth passed two of three, and Interchange passed twenty of thirty-five. This report covers the separately owned printable image, PDF proxy, ZIP, JSON and STEP failures from that run.

| Failure | Applied repair |
| --- | --- |
| Printable vector projection rejects a standard image data URL | The production projection decodes its payload with the owned RFC 4648 codec. The shared fixture carries the full data URL and expected bytes, and the native law compares the complete projected image payload. |
| Native PDF accepts and emits a Pack-prefixed string inside a standard image data URL | Layout decodes standard base64 on PDF export and encodes standard base64 on import. The existing independent PDF reader still checks the exact RGB8 color and alpha streams. The native import test decodes and checks the returned PNG samples. |
| The IDML path guard test fails before reaching the importer | The test asserts that the owned ZIP writer rejects the authored member. The independent ZIP oracle creates the external archive in memory, and the Layout importer must still return the authored error. |
| JSON export round trip compares token spellings as if they were different values | The test normalizes exactly representable integer tokens across nested objects and arrays before comparing every authored field. Non-numeric values, structure and integers outside the exact range keep the original structural equality check. |
| STEP test assumes the first projected curve is a background rectangle | The test identifies actual STEP polyline entities, then requires the authored number of full matching closed sequences after converting every coordinate from points to millimetres. Existing text, image-reference and paint assertions remain. |

The independent WHATWG Data URL parser and the `data-urls` library returned the exact authored MIME type and bytes. The `base64-js` library reproduced the authored payload. Node and `jsonc-parser` compared the full neutral snapshot after replacing the two grid integer tokens with decimal tokens. Neither JSON parse reported errors. The Three.js geometry oracle passed ten authored cases with zero TypeScript diagnostics across 199 sources. The pinned nightly Rust parser accepted the two changed test sources, and scoped whitespace checks passed.

The image, PDF and IDML implementation changes, both Layout manifests and four Cargo locks are applied. The fresh guest build was started after those changes and the ArtifactChild and offload repairs. An actual locked Layout native rerun and fresh browser publication are still required.

The applied Layout I/O source closure passed using the repository macOS, Windows MSVC and Linux GNU configurations. All three measured 28 paths and 232,816 bytes for the production library, 32 paths and 239,490 bytes for the test library, two paths and 32,552 bytes for Drawing Birth, and six paths and 74,554 bytes for Interchange. These are source admission checks executed on macOS.

The first fresh browser build caught an incorrect relative codec path in each Layout manifest before component admission. Both paths now resolve to the existing framework Base64 crate. Canonical workspace synchronization restored the two Hub lock edges. The updated verifier confirms both manifest targets and one edge for each Layout package in each of the four workspace locks on all three source configurations.

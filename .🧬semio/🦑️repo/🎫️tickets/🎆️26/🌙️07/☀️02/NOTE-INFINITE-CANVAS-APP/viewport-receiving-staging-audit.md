# Viewport receiving staging audit

Attempt 19 (driver `w1iSrRr`) reported two E0119 conflicts for the `Viewport2d` borrowed and owned receiving traits and an E0599 for `Self::Cursor::new()` in the captured manual implementation.

The shared viewport source now mounts the canonical record binding and the receiving module once from `🪟️viewport/🦀️.rs`. The record binding retains the planar and orbit wire codecs and declares their borrowed shapes. It ends after the `DslField` implementations. The receiving module owns the sole trait implementations through `dsl_owned_scalar_receiving!(copy Viewport2d, planar)` and uses the same mechanism for the orbit pose. The macro creates the owned cursor through `Default::default()`. The captured manual constructor and both conflicting implementations are absent.

The receiver first bounds the planar record to three fields, then calls its existing wire converter. The converter admits only the existing x, y, and zoom fields and checks the pose's finite-coordinate and positive-zoom requirements. The copy macro meters borrowed receiving through the native output owner and delegates moved-in source handling to `ScalarReceiving` with the pose's fixed creation size.

The three existing viewport binding tests passed with thirteen assertions, including the independent JSON/SQLite field projection. The existing viewport ownership check passed against Ajv and the JSON Patch implementation. Fresh Rust compilation remains part of Note staging. This audit made no source changes and started no competing Cargo jobs.

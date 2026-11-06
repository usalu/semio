# Raster Import Work Budget

The actual import-media reserved registration in 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs:257 sets the resumable contract's max_work_units_per_step to 4,096. The mounted operation in plugin/🧵️retained-command/🔄️full-operation/🦀️.rs:279 configures BatchDriveConfig.fuel_per_step from that contract; lines 369–371 loan min(cx.fuel_remaining(), contract.max_work_units_per_step) with the current/contract deadline. Real inherited fuel can be lower than 4,096.

Base64 cursor 🧰️framework/🔨️modules/🚪️io/🔤️base64/🎮️cursor/🦀️.rs:90–95 quotes up to 8,192 work for a 4,096-byte nonfinal validation block because the bytes are visited twice. Its step at 148–155 returns Pending without mutation when the requested work is smaller. Thus a nonfinal full block cannot progress under the import contract; raising a cap or inventing work is not a remedy. A bounded adaptive cursor must size the next validation/decode chunk, preserving quartet alignment and final padding semantics, to actual current fuel and report its real reads, work and retained effects. Progress should be demonstrated using the registered import contract and original caller rather than a larger test-only grant.

Public Semio types live at semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioImageSnapshot, SemioImageFrame, SemioImageMetadataEntry, SemioColorspace, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA}. Snapshot has schema String, width and height u32, source colorspace, bit_depth u8, frames Vec<SemioImageFrame>, icc Option<Vec<u8>>, metadata Vec<SemioImageMetadataEntry>. Frame has delay_ms u32 and rgba8 Vec<u8>; metadata entry has key/value Strings. Raster model also reexports SemioImageSnapshot.

This is source tracing, not a native execution verdict.

# Semio Audio Custom Complete Cell Blueprint

Read-only actual provider/factory review. No new owning runtime qualification. [Existing independent Source extents](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/semio-audio-video-text-flow-original-independent-null-zero-extents.json) were measured from actual Source projection in Bun SQLite, NULL=0, integrity ok and no FK violations.

## Authored Four Tables

[Actual DDL](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql): canonical schema966, table count4, maximum columns5.

| Table | Width | Complete row bytes | Original full rows/bytes |
|---|---:|---|---:|
| semio_audio_document | 4 | identity8 + schema UTF8 + sample_rate8 + format UTF8 | 1/36 |
| semio_audio_channel | 3 | identity/document/ordinal=24 | 2/48 |
| semio_audio_sample | 5 | identity/channel/ordinal/bits=32 + query REAL8 except NaN NULL0 | 9/352 |
| semio_audio_tag | 5 | identity/document/ordinal=24 + key/value UTF8 | 2/78 |

Full14/514. Empty clears channels only, retaining both ordered duplicate-key tags, schema, sample_rate=u32::MAX and f64 format:3/114. The original Source and Native factory both use the same nine u32 bit words and two tag tuples from the adjacent JSON; Native explicitly selects Float64, matching fixture format=f64. Do not replace tag list with a map or erase NUL/non-ASCII text.

**No class column exists.** Sample finite, signed zero, subnormal and ±infinity cost40; NaN cost32. Retain raw32-bit payload in the integer cell, including signaling/negative NaNs. Do not use RowWriter.insert_float, which would append an invented class column. [Actual typed projection](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:50) already correctly uses plain insert with conditional Null/Real and separate bits. Share that same actual row body through RowWriter, then use it for native typed admission and SQL projection.

## Pretyped Borrowed Binary

[Current custom decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🛬️native/🦀️.rs:8) consumes version byte1; schema via native::bytes/borrow_text; sample_rate read_u32_le; format byte0..5; varint channel count; each channel varint sample count and read_u32_le words; then varint tag count and two text slices per tag; finally reader.remaining==0. A scoped borrowed census must replay these exact primitives before allocate_vec/String copies. Charge each channel and sample independently, precheck declared counts against cumulative remaining rows, check complete bytes after every row, and preserve all width/overflow/trailing refusal. No typed channels, sample buffers or SQL database needed.

Format ordinal0..5 maps pcm8/pcm16/pcm24/pcm32/f32/f64 with lengths4/5/5/5/3/3. These names describe metadata; even Float64 snapshots own f32 samples. No invented sample-width or rate-positive constraint is justified.

An additional independent actual Source-projection→BunSQLite readback preserves the original full carrier while changing only format, then clears channels while retaining the same two tags. [All twelve observed cases](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/semio-audio-six-format-independent-extents.json) passed integrity/FKs: pcm8 full14/515 empty3/115; pcm16/24/32 full14/516 empty3/116; f32/f64 full14/514 empty3/114. These are explicit matched-carrier extensions, not replacements for the original Native default-snapshot all-format law or Source's separate one-empty-channel/no-tags all-format law. No owning test was run.

## Pretyped Borrowed Document

Use original native::fields names schema/sampleRate/format/channels/tags; schema required hex UTF8, sampleRate required existing parse_u32; format omitted defaults pcm16. Channels and tags omitted default []. Channels are nested Items lists of lexical hex u32 words, parsed by the same u32::from_str_radix(value,16) as the binder, not an invented exactly-eight-character grammar. Tags are exact two-field records of hex UTF8. Preserve original depth64, duplicate/unknown fields, commas and bracket refusals. Reuse the bounded scalar UTF8 validation design from Text; do not allocate hex_text merely to measure. Parse sample words into stack u32/f32, classify only NaN for query NULL, and keep raw bits unchanged.

All census work must use the caller's original NativeDecodeControl with scoped_stage around the complete census and nested bounded text/list work. Existing original cancellation requires total1024/completed256 in sample traversal; retain that actual stage workload and refusal. The census can reach that refusal before constructing sample backing; no grant change is needed.

## Physical Versus Semantic Controls

Audio still uses shared base native decode remaining.min(max_value_bytes), Bound::new forecasts and shared Writer's physical max_value coupling. Apply an explicit owner-selected admitted policy only after the complete Audio semantic visitor/census exists, as Text's held design does. Preserve original1024/16/128/2048 file forecast, all max_file checks, cumulative remaining max_allocation, exact original Writer two-pass framing, bounded progress, original tiny32 refusal, large100000-byte tag output laws and exact ordinary binary/DSL equality. Reconstruction's BTreeMap/BTreeSet/Vec frontier remains a separate unqualified scope.

## Owning Routes and Original Authorities

[Original Native laws](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs) cover all six format tags, raw IEEE words, independent SQL sample edit, owner capability/dialect guards, controlled decode and output/cancellation. Registered package @semio-tech/stdio-semio-rs:test-snapshot-sqlite-source/native is the existing whole owner route, 120000ms original budget; do not invent a new individual Audio target or reinterpret grouped scope. Full/empty exact and all copied-limit one-short additions need independent SQLite constants above, both native forms and original word comparisons/reprojection, not f32 PartialEq on NaNs.

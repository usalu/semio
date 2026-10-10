# Binary Original Receiving Read-Only Audit

2026-10-10. Read-only source exploration; no builds, tests, source edits, Git changes or runtime qualification. This report is the only authored file. Ticket lifecycle belongs to the parent agent.

## Actual Domain Shape

Base `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any`.

`🧬️schema/📸️snapshot/🦀️.rs` declares `BinarySnapshot { schema: String, bytes: Vec<u8> }` with DslRecord derive and `#[dsl(base64)]` on bytes. This generated Record representation is not its native carrier. Actual `🚪️io/💾️binary/📸️snapshot/🦀️.rs::ArtifactPack` copies raw octets and normalizes schema to `STDIO_BINARY_DOCUMENT_SCHEMA`; actual `🚪️io/📝️text/📸️snapshot/🦀️.rs::ArtifactDsl` parses/prints the authored hexadecimal envelope. No explicit Binary native Record producer or typed receiving binder was found in this domain. A replacement must preserve these raw/hex laws rather than treating generated DslRecord base64 as native hexadecimal.

SQLite base `🚪️io/🪶️sqlite/📸️snapshot` already owns handcrafted `🗄️.sql`: `binary_document(id=1,schema TEXT)` plus `binary_byte(id,document_id,ordinal,value 0..255)`. Mounted `💰️backing/🦀️.rs` actually implements `authored_schema`, `write_rows`, `native_cells`, `semantic`, `project`, and `reconstruct`. It uses borrowed RowWriter semantic admission, paid projection, ordered row references and DecodedFieldOwner reconstruction. Adjacent TypeScript also maps the same two tables. `🚦️native/🪶️copy/🦀️.rs` is an unmounted duplicate, not the owning implementation; editing it alone would not affect the route.

## Smallest Genuine Unpaid Frontier

Mounted `snapshot/🦀️.rs:11` still implements native decode with bare `NativeDecodeControl`, while current store trait `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12115` requires `NativeSnapshotDecodeOwner`. The domain supplies no explicit `to_sqlite_database_receiving`, `from_sqlite_database_receiving`, or direction receiving validators. Store receiving defaults at 12105/12107 return UnsupportedOwner. Source signature mismatch is observable; no compiler outcome was run.

`🚦️native/🦀️.rs::decode` performs borrowed semantic rows before allocation, then creates temporary `Vec<u8>`, advances after each raw 256-byte chunk or hex character, and only later copies schema and constructs BinarySnapshot. Cancellation after copying therefore returns through owned locals without typed destination/installed recipient custody. `encode` immediately takes `native_owner.native()` and creates temporary raw vector or hexadecimal body before envelope wrapping; body wallet is never used. Native allocation accounting alone does not prove original body grants or retained prefix conservation.

The narrowest useful change is one original typed BinarySnapshot Option receiving binder over its two genuine fields, plus raw/hex native owner integration. The broader mounted receiving P/R and direction validators are necessary to qualify the installed SQLite hop, but semantic schema work is already present and must not be counted as missing.

## Concrete Test-First Neutral Law

Handcraft a fixture describing canonical raw octets `[0,255,1,128]`, whitespace/mixed-case hex equivalence, empty input, invalid digit and odd extent, canonical schema normalization, and long multi-byte authored SQLite schema. Distinguish raw/hex native normalization from SQLite literal-schema preservation.

Run actual borrowed native validation before typed birth. Set independent original body items/copy/capacity/depth authorities to zero or one-short; require refusal with absent Option and zero physical birth. Run exact-demand equality success on each independent axis, not only a generous grant. Malformed hex must refuse before output birth.

For native octet and schema copies, install the original allocation observer and retirement recipient, start a parent stage17, cancel at first/interior/final UTF8 or octet checkpoints, and inspect the same original Option/recipient and cumulative receipt before closing. Require an exact raw/decoded octet prefix, UTF8 boundary schema prefix, physical capacity/release equality and restored parent progress. A zero native ceiling and capacity one-short case should cover allocation-before-birth refusal. Resume through explicit caller rearming and genuine detach/bind; do not swap to a fresh observer to make cleanup pass.

Fund close turns separately; deny an all-zero close grant without custody loss, then require terminal empty and total physical birth equal release with zero terminal-drop release. Use Bun Buffer hex decode/encode and Bun SQLite edited-byte/Unicode-schema witness as independent oracles. Existing `🧫️fixtures/🛬️native-control/🔣️.json` and `🧪️tests/🦀️.rs` check SQL limits and callback cancellation but do not specify these original five-axis custody laws; several test calls still use former owner-less native signatures.

For installed qualification, exercise the actual declaration factory and registered codec with named input Option, original run control, body grant and recipient in both raw and text directions. Ordinary typed snapshot roundtrip is insufficient evidence.

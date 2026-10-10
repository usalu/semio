# CSV Original Typed Binding Production Seam

Read-only fresh audit, 2026-10-10. No edits/builds/tests. All artifact paths below start at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`.

## Actual CSV Ownership Model And Retirement

`📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:21` derives `semio_framework_value::RetireOwned` on CsvField (`value:String`, `quoted:bool`); line35 derives it on CsvRecord (`fields:Vec<CsvField>`); line47 derives it on CsvSnapshot (`schema:String`, `has_header:bool`, `records:Vec<CsvRecord>`). This is the exact production RetireOwned implementation source: proc macro derivation, not a separate handwritten CSV impl. The derive entry is `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:92`.

CSV RecordValue fields are root 0 schema Text, 1 has_header Bool, 2 records List; each records element is Record with field0 fields List; each field is Record with field0 value Text and field1 quoted Bool. Preserve root booleans and each field's quoted flag before its text copy, every empty record, every empty field, record/field ordering, literal UTF8 including NUL, and schema identity. SQL projection has separate document/record/field tables; natural CSV semantics must not erase authored quote flags.

CSV `🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs:15` exposes borrowed `admit(...)->Result<(),ValueError>` only. Production `🚦️native/🦀️.rs:16` currently ignores `_body`, runs this semantic gate then `CsvSnapshot::__dsl_from_record_controlled`, and assigns output after full construction. Native controlled allocation alone does not preserve the original five-axis body wallet or a visible partial destination.

## Minimal Genuine Receiving Binder

Add binding_demands with bounded original checkpoints and a binder alongside CSV admission. Validate every required root/record/field shape and boolean during borrowed quotation. Use checked arithmetic for exact declared destination frontier; quotation is descriptive demand, never authority. Admission compares that demand with the independently supplied caller body grant and original native allocation ceiling before output birth. On insufficient items/copy/capacity/depth or canceled quote, output remains None, native owned bytes/body receipt unchanged, and heap allocation/release zero.

After admission, install `Some(CsvSnapshot{schema:String::new(),has_header,records:Vec::new()})` before root child allocation. Copy schema into that destination and reserve records through original NativeDecodeControl. Before reserving a record's fields, push CsvRecord with empty fields into the original records vector. Before copying a field value, push CsvField with its quoted flag and empty String into its original fields vector. Then copy text into that String through original native control. Never build a temporary row/field tree and assign only after copying; refusal must leave the actual field prefix in the authenticated Option.

Exact unavoidable owned capacity terms: schema UTF8 bytes; `recordCount * size_of::<CsvRecord>()`; sum of `fieldCount(record) * size_of::<CsvField>()`; sum of every literal field UTF8 byte count. Record allocator's actual capacities replace quoted requested counts in accepted receipts. Copied-byte demands must include root CsvSnapshot header, owned vector header assignments, actual moved record/field headers, and copied text bytes; empty vectors and empty strings have zero backing but still real typed/header operations. Item demand must count those actual performed frontier operations consistently with their receipts. Do not blindly use TSV's header sizes or its `3+rows+cells` item formula: CSV has additional record/field struct wrappers. Determine binder depth from its actual owner topology (root, records vector, record, fields vector, field/text) and test that declared boundary; do not promote a guessed depth into caller authority.

Maintain progress outside the fallible copying closure. For each successful or partially allocated copy count its actual String length/capacity, and for each vector allocation actual capacity times correct element size. Settle progress once after both success and refusal, preserving retained_progress on the original error. Body released_bytes remains zero during retained partial construction; no implicit cold retirement occurs. Enclosing Store receive already owns the typed Option under original pending recipient, so its failure path can hold this exact tree. Successful output is later caller-owned; both success and refusal require controlled retirement coverage through derived RetireOwned. This is distinct from `retire_sqlite_snapshot` forwarding to cold FromValue retirement.

## Existing TSV Reference And Neutral Mount

TSV reference base is `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot`.

- `🛂️admission/🦀️.rs:11–18`: original checkpoint quotation, checked counts/headers/text bytes, independent frontier admission demand.
- `🛂️admission/🦀️.rs:20–35`: bind receives caller Option + original native + body wallet, populates root first, rows before children, strings before copying, settles actual progress on refused copy.
- `🧫️fixtures/🛬️native-control/🔣️.json`: typedDestination normalGrant items16/copy262144/capacity262144/release0/depth5; closeGrant items1/copy65536/capacity1048576/release1048576/depth64; maximumTurns100000, terminalDropBytes0, quoteCancelCheckpoint3; cancelAt 65534,131068,null. Source text unit `文🌠`, 20000 repeats, 140000 UTF8 bytes.
- `🧪️tests/🦀️.rs:9–25`: quoted short independent grants refuse before destination; native original parent stage17 survives; canceled field prefixes stay in original nested rows; actual heap receipt, zero premature release, Serde prefix oracle, bounded controlled terminal conservation.
- `🧪️tests/🟦️.ts:19–26`: neutral grant parser checks every required five-axis field, independent Bun SQLite TEXT BLOB length and JSON/UTF8 prefix oracle. Fixture imported at line119.
- Rust SQLite snapshot `🦀️.rs` mounts tests with `#[path="🧪️tests/🦀️.rs"]`; TSV Rust package project target `test-snapshot-sqlite` at project.json52, `test-snapshot-sqlite-native`69, `test-snapshot-sqlite-source`86 call existing package 📜️script.ts.

Author a corresponding CSV neutral law for its own nested record and field struct topology, including quoted=false and true, empty records/fields, and cancel during schema plus field UTF8 copying. Its body grant must be explicit and finite, separately independent of the close grant and native maximum. Do not copy TSV numeric authority assuming equivalent layout. Validate independent literal/quote prefix semantics and actual RetireOwned terminal conservation. No runtime coverage claim is made by this audit.

# Stdio Coordinate Difference and Independent SQLite Metadata Readback

Source-only audit of the current mounted files. No Native or Source tests executed here.

## Exact Coordinate Difference

The generated editor law file contains 88 editor types. Resolving each actual `ArtifactEditor::DIALECT` implementation to its authored constant yields 88 distinct coordinates; every coordinate belongs to the 89 literal `document_codec_bare` bindings retained in the prior roster. The difference is exactly `s.stdio.json / rfc8259 / geojson`, not a duplicate editor or arithmetic inference.

Its authority is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs:109`: `document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, ... subset: SubsetId("geojson"))`. The schema constant resolves to `stdio.json` in the same root. Root lines 119 and 336–345 additionally install the GeoJSON validator and expose its schema/io modules. The generated editor roster has JSON any and i-json but no GeoJSON editor. This is a declaration membership outside the editor roster, not evidence of missing SQLite capability or a missing editor requirement.

Machine evidence: `📥️inputs/current-stdio-declared-versus-editor-coordinate-difference.json` retains all 88 editor implementation paths, resolved constants and the exact additional authored binding. PDF coordinates resolve their nonliteral kind through the actual `PDF_ARTIFACT_SCHEMA_ID = "s.stdio.pdf"` at PDF root line 26. All editor rows resolve uniquely; no editor coordinate lies outside the declaration roster.

## Independent Physical SQLite Helper

Current Hub editor-catalog helper lines 181–198 reads the actual exported bytes using Bun SQLite, checks integrity and foreign keys, compares the complete selected metadata row and requires at least one domain table. Its closed corpus is `🧫️fixtures/✏️editor-catalog/🪶️sqlite/🔣️.json`: version 1, one minimum domain table, integrity `ok`, no foreign key failures, binary/text encodings.

These assumptions match the Core authority: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql` declares `id INTEGER PRIMARY KEY CHECK(id=1)`, kind/standard/subset text, integer schema version 1 and encoding binary/text. `attach_sqlite_snapshot_metadata` at IO root lines 2296–2335 rejects a domain table claiming reserved metadata, pushes exactly six cells `[1, kind, standard, subset, 1, encoding]`, and publishes exactly one row with rowid 1. Thus `ORDER BY id` is valid, numeric version 1 is correct, and excluding `semio_snapshot` plus `sqlite_sequence` from the table census is coherent.

The helper compares exact metadata row count and field values but does not independently compare domain columns or semantic rows. Domain-table count proves a physical domain table exists; the following typed decode/full snapshot oracle supplies separate semantic evidence. This is appropriately stronger than trusting only the native provider, but must not be reported as an independent complete domain-table oracle.

The helper emits `[DEBUG] Stdio catalog physical SQLite ...` after its assertions. Its source presence is not an execution receipt: a qualifying Native log must show selected laws and that DEBUG evidence, especially because this helper was mounted while compilation was active. No Source validation is inferred from a queued or rejected Nx mutation.

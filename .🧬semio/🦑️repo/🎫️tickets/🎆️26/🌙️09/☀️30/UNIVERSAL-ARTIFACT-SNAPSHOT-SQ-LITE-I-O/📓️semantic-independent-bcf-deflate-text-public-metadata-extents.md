# BCF Deflate Text Public Metadata Extents

Independent inline Bun SQLite execution used the actual [metadata DDL](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql) and literal row values. [Exact retained observations](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/bcf-deflate-text-public-metadata-independent-extents.json). No public owning gate was run.

Raw DDL281 bytes includes semicolon/newline. SQLite stored SQL279 excludes both; table name semio_snapshot14 gives canonical additive schema293. Actual ordered six fields: id,artifact_kind,standard,subset,schema_version,native_encoding. Both integer values are1 and cost8 each. Text values use actual UTF8 lengths. [attach_sqlite_snapshot_metadata](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🦀️.rs:2296) agrees: trim().trim_end_matches(';'), 16+coordinate+encoding lengths, plus each actual domain table.sql/name and cell. It adds one row/table and requires max_columns>=6, paid metadata backing and table replacement through original control. Import validates the same ordered values/schema.

| Actual coordinate | Metadata binary/text bytes | Full public rows/bytes binary/text | Metadata-retaining empty rows/bytes binary/text | Public canonical schema/tables/max columns |
|---|---|---|---|---|
| s.stdio.bcf / 2.1 / * | 37/35 | 30 / 1296,1294 | 4 / 147,145 | 4213 / 16 / 29 |
| s.stdio.deflate / rfc1950 / * | 45/43 | 1026 / 32858,32856 | 2 / 90,88 | 983 / 3 / 6 |
| s.stdio.semio / v1 / text | 41/39 | 9 / 298,296 | 2 / 67,65 | 871 / 4 / 6 |

BCF domain full29/1259 and topics-only-empty3/110 are unchanged. Deflate uses canonical Native full1025/32813 and payload-empty1/45, not distinct logical Source dictionary fixture. Text full8/257 and runs-empty1/26 are unchanged. These public limits are separately authored complete-file contracts, not raised domain grants. Each public max_* one-short law must select these complete-file floors while native/domain laws retain their original exact limits. Actual output file byte length and physical backing remain separately measured/enforced; the cell totals above do not forecast either.

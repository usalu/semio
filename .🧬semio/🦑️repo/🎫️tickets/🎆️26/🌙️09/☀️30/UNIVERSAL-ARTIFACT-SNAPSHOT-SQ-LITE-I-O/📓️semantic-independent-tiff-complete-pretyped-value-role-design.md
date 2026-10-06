# TIFF Complete Pretyped DslValue Role Design

Read-only blueprint, no provider changes or Native qualification. Root Record remains exactly1Text schema,2Value byteOrder,3Value ifds. The nested authority is FromValue, not DslRecord. [/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs:40](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs:40) binds these borrowed values only after strict root shape checking.

Actual nested representations are DslValue::String for all-unit camelCase enums; Array for Vec and exact-length2 rational tuples; Object string-key fields for structs and adjacent tagged TiffValues. Scalar integers are DslValue::Number(Number), with primitive controlled codec using as_u64/as_i64 and checked target conversion. Do not demand only Number::UInt if a nonnegative exact Int is accepted by as_u64. Actual generic codec authority: [/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:197](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:197), tuple1119, Vec510. Value derive authority: [/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs).

Objects: Ifd entries/storage; Tag required tag/values; Storage required kind/offsetsKind/byteCountsKind, chunks defaultempty. Ifd entries defaultempty and storage missing uses authentic default(kind none, offsets/counts long, emptychunks). These are actual FromValue defaults, unlike mandatory DslRecord fields. No deny_unknown_fields annotation exists on these declarations: do not newly reject unknown nested Object keys unless actual controlled binder does. Present wrong shapes still refuse; absent defaulted fields are not automatically equivalent to explicit Null.

Values Object required kind/value, kinds byte/ascii/short/long/rational/sByte/undefined/sShort/sLong/sRational/float/double. All values are Array. Unsigned families u8/u16/u32 as declared; signed i8/i16/i32. Rational Array length2 u32; sRational length2 i32. Float element Object bits checkedu32; Double element Object bits checkedu64, preserved as words, never numeric float conversion before raw-bit classification. Byte/ascii/undefined and chunks are ordinary Arrays of checkedu8, not Bytes64. ByteOrder strings littleEndian/bigEndian; Storage none/strips/tiles; FieldType strings map twelve variants to TIFF codes1..12.

## Exact SQL Census

| Row | Cost including identity | Width |
| --- | --- | ---: |
| Document | 8+schemaUTF8+orderUTF8 | 3 |
| IFD | 40+storageKindUTF8 | 6 |
| Tag | 40 | 5 |
| Chunk | 24+actualBlobLength | 4 |
| Integer/octet value | 32 | 4 |
| Rational value | 40 | 5 |
| Float32 or Float64 value | 24+bits8+classUTF8+query(8 or NaN NULL0) | 6 |

Both float widths store SQL bits INTEGER8, not physical f32 width4; both NaN rows cost35 and finite46. Infinity cost uses the actual shared class spelling length; derive that spelling from existing shared IEEE authority rather than inventing aliases. Every value occurrence and every chunk emits its own ordered identity/parent/ordinal row. Empty values keep their Tag row; no fabricated payload row. Full static schema3886/table16/maxwidth6 required before typed binding.

The current manual check_sqlite_semantic_payload constants are not this exact census: its Document16, IFD48, Chunk32, Tag48 and fixedFloat32/Double40 must not be reused as independent exact role authority. Share one authored typed visitor through RowWriter owned/borrowed; add a borrowed DslValue census under original NativeDecodeControl/copylimits before actual from_record_controlled. Bound traversal at actual array elements and borrowed text256, scoped workloads; no typed TiffIfd/Values/Snapshot/JSON mirror or private SQL ledger. Fixed-depth domain structure permits nested loops; no speculative large frontier is needed.

Retain original format u32 offsets/counts, storage SHORT/LONG restrictions and none-versus-chunk obligations, actual native file ceiling and controlled construction ledger. Semantic role census does not waive these. The three measured full/empty cases remain independent authority; no ordinary wire omission demand is inferred beyond actual Value default rules.

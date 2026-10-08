# Native Total Fragment Exact Census Proposal

Read-only edit proposal adjacent existing `📏️cells/🦀️.rs`; no Native tests or production changes. Preserve eleven edge inputs and strict Draft7 schema unchanged. Boundary now eight columns including nullable field_tag; byte row five columns including role. Still39 total tables: original35 plusfloat/integer/byte/text.

Borrowed actual DSL layout: source record field4 is durable Map(String,FieldValue); each artifact is Record with fields0kind Text,1mime Absent/Text,2width UInt,3height UInt,4chunks List(Bytes64(Vec<u8>)). `RemodelingDurableArtifact::native` already validates0..3 scalar fields and source shape; use borrowed `field(value,0)` as F::Text for kind and `items(field(value,4))`, match F::Bytes64 to `&[u8]`. Do not convert to native Snapshot, clone bytes or build resolved mesh for census. Domain storage permits arbitrary chunks; no tag-order/envelope refusal here.

Exact semantic byte formulas (INTEGER and REAL8; NULL0; TEXT UTF8 bytes; BLOB length):

|Row|Columns|Semantic bytes|
|---|---|---|
|raw boundary|8|32 +3(raw)+payload length|
|structured boundary|8|32 +field UTF8len+type UTF8len+(tag present?8:0)|
|finite durable float|6|40 +6(finite)=46|
|nonfinite durable float|6|32 +class UTF8len|
|durable integer|4|32|
|durable byte|5|32 +role UTF8len (payload7 =>39; trailing-word13 =>45)|
|durable text|3|16 +content UTF8len|

Four boundary INTEGERs are id/artifact/ordinal/element_count. Optional field_tag is fifth integer. Do not charge field_tag when NULL. Byte values cost INTEGER8 regardless of magnitude. Nonfinite durable REAL is NULL and contributes0; exact bits/class mandatory. Current inline float representation/cost remains unchanged. Raw payload BLOB only generic/image/unknownkind.

Concise borrowed census pseudocode:

```rust
for (key, source) in map(field(snapshot,4)?)? {
    let artifact=record(source,RemodelingDurableArtifact::FIELDS)?;
    census existing durable_artifact metadata with original native scalar costs;
    let F::Text(kind)=field(artifact,0)? else { return invalid(...) };
    for leaf in items(field(artifact,4)?)? {
        n.step()?;
        let F::Bytes64(bytes)=leaf else { return invalid(...) };
        let shape=borrowed_shape(kind,bytes,n)?;
        c.row("remodel_durable_chunk",checked_boundary_cost(shape),n)?;
        match shape.type {
            raw => {},
            text => c.row("remodel_durable_text",checked_add(16,payload.len()),n)?,
            f32 => {
                for word in payload.as_chunks::<4>().0 {
                    let bits=u32::from_le_bytes(*word);
                    let class=class_from_bits(bits);
                    c.row("remodel_durable_float",if finite {46} else {checked_add(32,class.len())},n)?;
                }
                for _ in tail {c.row("remodel_durable_byte",45,n)?;}
            },
            u32 => {
                for _ in complete_words {c.row("remodel_durable_integer",32,n)?;}
                for _ in tail {c.row("remodel_durable_byte",45,n)?;}
            },
            u8 | invalid_text => for _ in payload {c.row("remodel_durable_byte",39,n)?;},
        }
    }
}
```

borrowed_shape: unknownkind raw; sparse samples/f32/no tag; mesh empty absent/u8/no tag; meshtag>=12 unknown/u8/literal tag; recognized fixedfield/type/tag; numeric count floor(len/4),tail len%4; validUTF8tag11 text/countbyte length; invalidUTF8tag11 invalid-text/payload bytes. ValidUTF8 scan must checkpoint by monotone crossed threshold, not exactmodulus, and borrow no strings. Empty known tagged text has one text row contentempty. Emptyheader has only boundary. Count every byte row through c.row so work/cancellation and row/cell limits are real. Use semantic_add/checked multiplication throughout. Original c.row charges budgets only for semantic DB cells; decode owner/control charge is a separate authority, not combined or substituted.

WIDTHS8 replace durable_chunk4->8; add durable_float6/integer4/byte5/text3. Existing extent checks derive tables/columns from WIDTHS; do not change base13 fixed rows. Existing native census41 raw-only cost becomes this visitor, shared with source-neutral semantics but not copied expected output. Source preflight and project currently share visit_rows; native predecode is independent borrowed DSL census. Third-party ordinary typeof/length(CAST(text AS BLOB)) query remains independent expected exactbytes/rows; all source lengths and table widths fixtures need updated. Add knownempty/tail/nonfinite/invalidUTF8 fixture roles before requiring every table has rows; retain raw binary control.

Light independent Bun SQLite probe executed (in-memory one REAL row, no Native build): inserting -0 with bits2147483648 reads REAL0 with Object.is(value,-0)==false while bits remain2147483648. Therefore expected SQL sample is normalized numeric0; compare exact sign via sample_bits, and verify imported source bytes preserve negativezero. Do not assert Object.is(SQL REAL,-0) or require SQL sign bit. Dedicated durable classfinite accepts numeric sample0 with negativezero bits; re-import uses bit authority.

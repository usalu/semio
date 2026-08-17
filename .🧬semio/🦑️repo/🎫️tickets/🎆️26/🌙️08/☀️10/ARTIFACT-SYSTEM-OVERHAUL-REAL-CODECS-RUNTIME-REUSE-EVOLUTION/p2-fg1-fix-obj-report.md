# P2-FG1 Fixup — `obj` Real `DiffCodec` Binary Upgrade

## Scope

Per the wave brief: `obj`'s `OpBinary::encode_op`/`decode_op` (`🧬️mutations/🦀️component.rs`) was
**already real** (derives via `dsl::variants_binary::encode_op`/`decode_op`, confirmed by direct
reading — untouched this wave, no work needed). Only `DiffCodec::encode_diff`/`decode_diff`
(`🧬️schema/🔺️diff/🦀️component.rs`) was still on the F6-era `Ok(self.print_diff().into_bytes())`
text-as-binary shortcut. This ticket upgrades that one impl to a real binary frame, matching this
wave's own md/xml/dxf reference upgrades.

## What changed

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`

`ObjDiff`'s whole type tree is confirmed genuinely flat (plain structs / `Vec` / `Option<T>`, zero
self-recursion — module doc comment's own §3b analysis, reconfirmed while writing this) — so
**every value in the diff tree gets a full field-by-field binary frame**, not just a
header+opaque-tail. Added:

- `#region 🔖️BinaryPrimitives`: `write_f64_bin`/`read_f64_bin` (fixed 8-byte LE), `write_str_bin`/
  `read_str_bin` (varint length-prefixed UTF-8), `write_u32_bin`/`write_usize_bin` (varint),
  `write_option_bin`/`read_option_bin`, `write_tristate_bin`/`read_tristate_bin` (for the
  `Option<Option<T>>` fields the F6 recon's §3b blocker already documented —
  `ObjVertexDiff::w`/`ObjTexCoordDiff::w`), `write_vec_bin`/`read_vec_bin`.
- `#region 🔖️ValueBinaryCodecs`: real binary twins of every `#region 🔖️ValueCodecs` text
  function — `enc_vertex_bin`, `enc_texcoord_bin`, `enc_normal_bin`, `enc_face_vertex_bin`,
  `enc_face_bin`, `enc_group_bin`, `enc_object_bin`, `enc_usemtl_bin`, `enc_smoothing_bin`,
  `enc_unknown_bin` (+ `dec_*` counterparts).
- `#region 🔖️DiffValueBinaryCodecs`: real binary twins of `#region 🔖️DiffValueCodecs` —
  `enc_vertex_diff_bin`, `enc_texcoord_diff_bin`, `enc_normal_diff_bin`, `enc_face_diff_bin`,
  `enc_group_diff_bin` (+ `dec_*`), each field written in fixed declaration order via
  `write_option_bin`/`write_tristate_bin` (field order IS the schema, same convention md's
  `MdBlockDiff::Heading` arm uses).
- `#region 🔖️CollectionBinaryCodecs`: generic `enc_index_triple_bin`/`dec_index_triple_bin` and
  `enc_named_triple_bin`/`dec_named_triple_bin` (mirrors dxf's own generic collection-triple binary
  helpers), hand-instantiated for all six collections (`vertices`/`texcoords`/`normals`/`faces`
  index-keyed; `groups`/`objects` name-keyed).
- `impl protocol::DiffCodec for ObjDiff`'s `encode_diff`/`decode_diff`: real binary frame —
  `format u8 | flags_lo u8 | flags_hi u8 | per-present-field payload`. `ObjDiff` has TEN
  independently optional top-level fields (one more bit than a single `flags u8` byte can hold,
  unlike dxf's four-field `DxfDiff`), so two flags bytes carry the presence mask (`flags_lo`
  bits 0-7 = vertices..usemtl, `flags_hi` bits 0-1 = smoothing_groups/unknown_statements). Each
  present field's own real payload follows — collection triples via the new binary codecs,
  `mtllib` via `write_option_bin` over the remaining `Option<String>` layer (the field's OUTER
  `Option` is already carried by its presence flag bit, so only the inner tri-state layer needs
  encoding), `usemtl`/`smoothing_groups`/`unknown_statements` via `write_vec_bin`.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`

Rewrote from the old "binary = text bytes verbatim, no denser form" framing to the real frame:
`header fixed 3` (`field format u8`, `field flags_lo u8`, `field flags_hi u8`) + `chain payload
bytes`. The collection-triple payload stays one opaque trailing `bytes` chain — not because any
individual VALUE in this artifact is recursive (`obj`'s whole model is flat, unlike md/dxf's own
node trees), but because `Prim::Ref` cannot express a `Vec<Modified{index,diff}>` record-array in
this protocol dialect's grammar (`protocol-prim-ref-recursion`/`protocol-array-of-records`, the
recipe's own consolidated gap table) — the identical wall dxf's own `DxfDiff` protocol file
documents and works around the same way. `schema`/`start` header lines (`schema stdio.obj.diff`,
`start diff`) were left as this file's pre-existing, internally-consistent convention (matching
`ObjDiff`'s own `artifact_schema(id = "s.stdio.obj.diff")`) — not touched.

## Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::obj"` — see `tests_passed`/`tests_failed`
  in the structured result. Blocked partway through by a concurrent sibling session's in-progress
  compile error in `🏗️ifc/🏅️standards/🔖️4/.../🔺️diff/🦀️component.rs` (`store::pack_rt::
  write_varint_i64` not found) — another agent's own P2-FG1-fix slice on this same wave, outside
  this ticket's ownership boundary (obj only); not touched, polled until it cleared.
- `diff_grammar_conformance_law`/`ops_grammar_conformance_law`/`protocol_walk_law` (in
  `🏅️standards/🔖️3.0/⚙️engine/🦀️component.rs`) and the diff file's own
  `diff_codec_text_binary_roundtrip_law` all exercise the new `encode_diff`/`decode_diff` — no
  test files were added or restructured, only the existing suites re-run.

## Deviations

- None from the brief. `OpBinary` was confirmed already-real (no change needed, as the brief
  itself anticipated) and is unmodified.

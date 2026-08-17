# P2-FG1-Fix: ifc (standard 4 only) — real binary-frame upgrade for `OpBinary`/`DiffCodec`

## Scope
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/**` only. `🔖️2x3` was never touched
(confirmed: no diff, no writes, no reads outside `🔖️4`).

## What was wrong
`IfcMutation::OpBinary` and `IfcDiff::DiffCodec` were both still on F6's `print_op()/print_diff()
.into_bytes()` text-as-binary shortcut — no format/tag/flags header, no field-by-field structure —
despite `md`/`xml`/`dxf` doing the real upgrade this same wave for equally recursive node types
(`MdBlock`/`XmlNode`/DXF entities), proving the upgrade is achievable and not blocked by a genuine
mechanism gap for the *flat* parts of the shape.

## Reference implementation read
Read `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/`
`🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` in full before writing any code — copied
its exact shape: `format u8` (`store::pack_rt::OP_BINARY_FORMAT`) + a real second fixed byte
(`tag`/`flags`) + recursive field-by-field binary payload, with `pub(crate)` binary primitives
living in the `diff` module and reused (not duplicated) by the `mutations` sibling, exactly md's own
intra-artifact-reuse split.

## What changed

### `🔺️diff/🦀️component.rs`
- New `//#region 🔖️BinaryPrimitives`: `write_str_bin`/`read_str_bin`, `write_option_bin`/
  `read_option_bin`, and a local `write_varint_i64` (zigzag; `store::pack_rt` only ships the
  unsigned writer — mirrors `zip`'s own diff module's identical local copy; the read side's zigzag
  decode is already built into `store::ByteReader::read_varint_i64`).
- New `//#region 🔖️IfcValueBinaryCodecs`: `enc_ifc_value_bin`/`dec_ifc_value_bin` — real
  per-variant binary twin of `enc_ifc_value`/`dec_ifc_value` (tags `0`-`8`, same order as the text
  codec's `U`-`T` range). `Integer`→varint-i64, `Real`→8 raw LE bytes, `String`/`Enum`→length-
  prefixed UTF-8, `Reference`→varint-u64, `Aggregate`/`TypedValue` recurse into
  `enc_ifc_value_list_bin` — genuinely field-by-field all the way down, not an opaque tail:
  `IfcValue` itself is fully flat/spec-expressible per variant.
- New `//#region 🔖️EntityBinaryCodecs`: `enc_complex_type_bin`/`enc_entity_bin`/`enc_entity_list_bin`
  (+ decoders), `id | name | args | complex` field-by-field, `pub(crate)` for the mutations sibling.
- New `//#region 🔖️DiffValueBinaryCodecs`: `enc_args_diff_bin`/`enc_entity_diff_bin`/
  `enc_entities_diff_bin` (+ decoders) — each collection triple (`removed`/`modified`/`added`)
  becomes three varint-counted lists, real field-by-field for every key (`index`/`id`) and nested
  `Option` flag, bottoming out through the already-real `IfcValue`/`IfcEntity` binary shape.
- `impl protocol::DiffCodec for IfcDiff`: `encode_diff`/`decode_diff` rewritten from
  `print_diff().into_bytes()` to a real frame — `format u8 | flags u8` (bit0..3 =
  `file_description`/`file_name`/`file_schema`/`entities` presence) + each present field's real
  binary payload in order.

### `🧬️mutations/🦀️component.rs`
- Imports extended to pull in the new `pub(crate)` binary primitives from the `diff` sibling
  (`enc_ifc_value_bin`, `enc_entity_bin`/`enc_entity_list_bin`, `write_str_bin`/`read_str_bin`, …).
- New `//#region 🔖️OpBinaryCodec`: `enc_ifc_header_bin`/`enc_ifc_snapshot_bin` (+ decoders) — the
  only genuinely new shape here (`IfcHeader`/`IfcSnapshot`), reusing the diff sibling's recursive
  primitives for `IfcEntity`/`IfcValue`.
- `impl protocol::OpBinary for IfcMutation`: `encode_op`/`decode_op` rewritten from
  `print_op().into_bytes()` to a real frame — `format u8 | tag u8` (the 11-variant ordinal, `0`-
  `10`, same order `parse_ifc_mutation`'s keyword match uses) + the variant's own real
  id/index/name/entity/value payload.

### Protocol dialect files (normative description kept in sync with the Rust code)
- `🔺️diff/💾️binary/📡️component.protocol.semio`: `start body`/`chain payload utf8` → `start record`
  / `framing record` / `header fixed 2` (`field format u8`, `field flags u8`) / `chain payload
  bytes`, with a rewritten comment explaining exactly which parts are now real field-by-field binary
  (`file_description`/`file_name`/`file_schema`, and `entities`' own keys) vs. the one honest opaque
  tail (`IfcValue::Aggregate`/`TypedValue`'s self-recursion, blocked at the protocol-dialect layer
  by the same `Prim::Ref` recursion gap the whole FG-wave pilot ladder hit — `protocol-prim-ref-
  recursion`, unchanged this wave, filed as `mechanism_gaps`).
- `🧬️mutations/💾️binary/📡️component.protocol.semio`: same shape change, `field tag u8` instead of
  `flags`, same honest-boundary comment adapted to `IfcMutation`'s variant payloads.

## Deviations from the brief
None. obj/stl/step were explicitly out of scope for this agent (ifc-only ticket); step's own
mutations/diff are the other agent's responsibility despite sharing IFC4's Part-21 value grammar
shape — no code was shared or touched there.

## Mechanism gap (unchanged, filed not fixed)
`Prim::Ref` still cannot describe self-recursion in a `.protocol.semio` dialect file
(`protocol-prim-ref-recursion`) — confirmed still present, not addressed by this wave. Both
protocol files therefore keep one honest opaque `chain payload bytes` tail for the genuinely
self-recursive `IfcValue::Aggregate`/`TypedValue` case, exactly mirroring md/xml/dxf's own
identical documented boundary.

## Verification
`cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc::"` → **74 passed, 0 failed** (includes
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`grammar_conformance_law`, `committed_facet_files_parse`, `op_text_binary_roundtrip_law`
(`encode_op`/`decode_op` round-trip), `diff_codec_text_binary_roundtrip_law`
(`encode_diff`/`decode_diff` round-trip), plus every `semio` model artifact test that consumes
ifc4's serializers/deserializers — none broke). Full `cargo test -p semio-s-plugin-stdio --lib`
compiled clean (0 errors; only pre-existing unrelated warnings across the crate).

`git status`/`stat` confirmed the only files this session wrote are the 4 listed above; several
sibling files under `🏗️ifc/` (engine, snapshot grammar/protocol, demo fixture) show as modified in
`git status` but predate this session's start (timestamps ~19:21-19:32, before this agent began) —
another concurrent session's in-progress work, not touched or reverted here.

## Files touched
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`

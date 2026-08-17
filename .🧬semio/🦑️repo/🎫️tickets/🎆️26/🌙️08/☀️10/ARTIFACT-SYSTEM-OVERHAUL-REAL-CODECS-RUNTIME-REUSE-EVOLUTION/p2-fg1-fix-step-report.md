# P2-FG1-Fix: step — real binary-frame upgrade for `OpBinary`/`DiffCodec`

## Scope
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/` only —
`🧬️mutations/🦀️component.rs` (+ its `💾️binary/📡️component.protocol.semio`) and
`🔺️diff/🦀️component.rs` (+ its `💾️binary/📡️component.protocol.semio`). No other file touched.

## What was wrong (confirmed by direct read before starting)
Both `StepMutation::OpBinary` and `StepDiff::DiffCodec` were still on the F6-era
"binary = `print_op()`/`print_diff()`'s text bytes verbatim" shortcut — no format/tag header, no
field structure at all — despite md/xml/dxf doing the real upgrade this same wave for equally (or
more) recursive types.

## What changed

### `🔺️diff/🦀️component.rs`
- New `#region 🔖️BinaryPrimitives`: `write_str_bin`/`read_str_bin`, `write_f64_bin`/`read_f64_bin`,
  `write_option_bin`/`read_option_bin`, `write_str_list_bin`/`read_str_list_bin` — `pub(crate)`,
  reused by the mutations sibling (same intra-artifact-reuse split md/dxf use).
- New `#region 🔖️ValueBinaryCodecs`: real recursive binary twins of every `enc_*`/`dec_*` in
  `#region 🔖️ValueCodecs` — `enc_value_bin`/`dec_value_bin` (`StepValue`, 9 variants, tag bytes
  0-8 matching declaration order; `Aggregate`/`TypedValue` recurse via plain Rust function
  recursion — no `Prim::Ref` involved since this is hand-written, not DSL-derived),
  `enc_complex_bin`, `enc_entity_bin`, `enc_file_description_bin`, `enc_file_name_bin`,
  `enc_file_schema_bin`, `enc_step_snapshot_bin` (needed by `SetSnapshot`'s `OpBinary`) — every one
  of these is a FULL field-by-field binary frame (varint-length-prefixed strings, 8-byte LE for
  `f64`, LEB128 varint for `u64`/`i64`), not a header+opaque-tail shortcut — `StepFileDescription`/
  `StepFileName`/`StepFileSchema`/`StepEntity`/`StepComplexType` are all genuinely flat records,
  confirmed against their actual struct shape before writing the codec.
- New `#region 🔖️DiffValueBinaryCodecs`: `enc_args_diff_bin`/`enc_entity_diff_bin`/
  `enc_entities_diff_bin` — real recursive binary twins of the text collection-triple codecs,
  varint-counted `removed`/`modified`/`added` sections.
- `impl protocol::DiffCodec for StepDiff`: `encode_diff`/`decode_diff` now emit/parse a REAL binary
  frame — `format u8` + `flags u8` (4-bit presence mask: bit0=`file_description`,
  bit1=`file_name`, bit2=`file_schema`, bit3=`entities` — `StepDiff` has four independently
  optional top-level fields, same shape dxf's own `DxfDiff` upgraded to this wave) followed by each
  PRESENT field's real field-by-field binary payload. Replaces the old
  `Ok(self.print_diff().into_bytes())` shortcut.

### `🧬️mutations/🦀️component.rs`
- Imports extended to pull in the new `pub(crate)` binary primitives from the diff sibling
  (`enc_value_bin`/`dec_value_bin`/`enc_entity_bin`/`dec_entity_bin`/`enc_step_snapshot_bin`/
  `dec_step_snapshot_bin`/`enc_file_description_bin`/…/`write_str_bin`/`read_str_bin`) — same
  reuse split the TEXT `OpText` impl already uses against `StepDiff`'s text primitives.
- `impl protocol::OpBinary for StepMutation`: `encode_op`/`decode_op` now emit/parse a REAL binary
  frame — `format u8` + `tag u8` (the `StepMutation` variant ordinal, 0-10, same order
  `print_step_mutation`'s keyword match uses) followed by the variant's own field-by-field binary
  payload (`InsertEntity`'s `index`+`entity`, `SetEntityArg`'s `id`+`arg_index`+`value`, etc.).
  Replaces the old `Ok(self.print_op().into_bytes())` shortcut.

### `.protocol.semio` files (both `🧬️mutations/💾️binary/` and `🔺️diff/💾️binary/`)
Rewritten from the old "text-native, no header at all" description to `header fixed 2` (`format
u8` + `tag u8` / `flags u8`) + `chain payload bytes`, with a doc comment explaining: (a) why the
generic `format u8 | ordinal varint | record body` layout doesn't fit (no `DslField` impl for
`StepValue`/`StepEntity`/`StepSnapshot`, confirmed by the `#[derive(dsl::DslOps)]`/
`#[derive(dsl::DslDiff)]` rejections already documented in the Rust files), and (b) why the payload
stays an opaque `chain` at the protocol-DIALECT layer even though it's genuinely field-by-field
structured (and round-trip-tested) at the Rust layer: `Prim::Ref` — the only construct that could
describe `StepValue`'s `Aggregate`/`TypedValue` self-recursion declaratively — unconditionally
errors during `walk_protocol` (`protocol-prim-ref-recursion`), the same wall md/dxf/every prior
pilot with a self-recursive value type hit and worked around identically. Only the two
`.protocol.semio` files were touched — `.ksy`/`.spicy`/`.abnf`/`.g4`/etc. left alone, matching what
md/dxf did this same wave (confirmed by checking their actual diff against `git status` before
writing mine).

## Verification
`cargo test -p semio-s-plugin-stdio --lib "artifacts::step"` → **106 passed, 0 failed** (full
crate `cargo check` also passes — one transient failure from a concurrent session's in-progress
`ifc` edit, `store::pack_rt::write_varint_i64` vs. the correct crate-root `store::write_varint_i64`
re-export, resolved on its own before I finished; not this artifact's code, not touched).

Tests exercised, all green, including the mandated laws:
- `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
  `grammar_conformance_law`
- `op_text_binary_roundtrip_law` (mutations) — every `StepMutation` variant, `print_op`/`parse_op`
  AND `encode_op`/`decode_op` round-trip, incl. `InsertEntity`'s bare `StepEntity` payload and
  every `StepValue` tag incl. the recursive `Aggregate`/`TypedValue` cases.
- `diff_codec_text_binary_roundtrip_law` (diff) — `print_diff`/`parse_diff` AND
  `encode_diff`/`decode_diff` round-trip over the empty diff, a genuine `between()` result
  exercising every top-level field + all three `entities`/`args` collection-triple flavors +
  `StepEntityDiff.complex`, and its reverse direction.
- `mutation_diff_law_covers_every_variant`, `inverse_law_mutation_level_round_trips_every_variant`,
  `absorb_law_holds_over_curated_ops`, `between_roundtrip_law`, `inverse_law`,
  `field_sweep_covers_every_mutable_field` (all pre-existing, unaffected, still green).

## Deviations from the brief
None. Both `OpBinary` and `DiffCodec` upgraded to real field-by-field binary frames as instructed
(step's flat records — `StepFileDescription`/`StepFileName`/`StepFileSchema`/`StepEntity`/
`StepComplexType` — got FULL field-by-field encoding, not header+opaque-tail; only `StepValue`'s
own `Aggregate`/`TypedValue` self-recursion stays opaque at the `.protocol.semio` DECLARATIVE
layer, exactly per the brief's own carve-out for "genuinely recursive value args" — the Rust code
itself recurses fully, real and round-trip-tested, same honest-boundary split md/dxf established
this same wave).

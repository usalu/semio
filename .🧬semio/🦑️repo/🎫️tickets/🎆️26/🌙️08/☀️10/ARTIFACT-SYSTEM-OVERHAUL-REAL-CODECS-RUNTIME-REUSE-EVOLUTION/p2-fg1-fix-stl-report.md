# P2-FG1-FIX — stl: real binary frames for OpBinary/DiffCodec

## Scope
Artifact: `stdio.stl` (standard `ascii`, subset `any`). Upgraded the two F6-era text-as-binary
shortcuts identified by the wave's own census — both `impl protocol::OpBinary for StlMutation`
(`🧬️mutations/🦀️component.rs`) and `impl protocol::DiffCodec for StlDiff`
(`🔺️diff/🦀️component.rs`) were still `print_*(self).into_bytes()` — to real, field-by-field
binary frames, following this wave's own `md`/`xml`/`dxf` reference implementations (read in
full before writing any code).

## Why STL could go further than md/xml/dxf
`StlDiff`/`StlMutation`'s whole field tree (`StlTriangleDiff`, `StlTrianglesDiff`, `StlTriangle`)
is genuinely FLAT — no type in this tree ever references `StlDiff`/`StlMutation`/itself. Unlike
`md`'s `MdBlockDiff` (a self-recursive enum needing an opaque payload chain for the `Prim::Ref`
gap), STL has zero self-recursion. Every level down to the individual `f64` is real fixed/varint
binary in the Rust code — `write_f64_bin`/`enc_vec3_bin`/`enc_vertices_bin`/`enc_triangle_bin`/
`enc_triangle_diff_bin`/`enc_triangles_diff_bin`/`enc_snapshot_bin` — never a text-as-binary
shortcut and never an opaque byte-chain at the Rust layer.

The ONE place that still terminates in an opaque `chain payload bytes` at the **protocol-dialect**
layer (not the Rust layer) is `StlTrianglesDiff`'s `removed`/`modified`/`added` — these are
variable-length **vectors of records**, which hits the same `protocol-array-of-records`
`walk_protocol` gap this wave's own `dxf` upgrade independently documents (confirmed by grepping
every `.protocol.semio` file in the repo: none use the grammar's `array-prim`/`record`-block
constructs anywhere — they are unexercised, not proven safe to invent here). Filed as a
`mechanism_gaps` entry, not a shortcut — the payload itself is genuinely structured varint binary
at the Rust layer, exactly matching how `dxf`'s own upgrade (also flat at the top level, also
array-of-records inside) treated the identical shape.

## Files changed
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — added `#region 🔖️BinaryPrimitives` (`write_str_bin`/`read_str_bin`, `write_f64_bin`/
  `read_f64_bin`, `write_option_bin`/`read_option_bin`), `#region 🔖️ValueBinaryCodecs`
  (`enc_vec3_bin`/`dec_vec3_bin`, `enc_vertices_bin`/`dec_vertices_bin`, `enc_triangle_bin`/
  `dec_triangle_bin`), `#region 🔖️DiffValueBinaryCodecs` (`enc_triangle_diff_bin`/
  `dec_triangle_diff_bin`, `enc_triangles_diff_bin`/`dec_triangles_diff_bin`); rewrote
  `DiffCodec::encode_diff`/`decode_diff` to a real `format u8 | flags u8 | [solid_name] |
  [triangles]` frame (flags = 2-bit presence mask, since `StlDiff` has two independently optional
  top-level fields, unlike `md`'s single `has_value` byte).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `#region 🔖️OpBinaryCodec` (`enc_snapshot_bin`/`dec_snapshot_bin`, reusing `diff`'s
  `pub(crate)` binary primitives); rewrote `OpBinary::encode_op`/`decode_op` to a real
  `format u8 | tag u8 | variant payload` frame (`tag` = `StlMutation`'s declaration-order ordinal,
  0=`NoMutation` .. 6=`SetTriangleVertices`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
  — replaced the `chain payload utf8` text-as-binary placeholder with a real
  `header fixed 2 { field format u8; field flags u8 } + chain payload bytes` frame description,
  documenting the presence-mask header fields as real fields and citing the one remaining
  `protocol-array-of-records` gap for the opaque tail.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
  — same treatment: `header fixed 2 { field format u8; field tag u8 } + chain payload bytes`,
  citing the identical gap for `SetSnapshot`'s `Vec<StlTriangle>`.

No other files touched. `⚙️engine/🦀️component.rs`'s conformance-law tests were NOT modified — they
already reference `diff::binary::COMPONENT_PROTOCOL_SEMIO`/`mutations::binary::
COMPONENT_PROTOCOL_SEMIO` (which `include_str!` the `.protocol.semio` files edited above), so they
picked up the new frame description automatically.

## Verification
`cargo test -p semio-s-plugin-stdio --lib "artifacts::stl"` — **34 passed, 0 failed** (full run,
including `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`, `mutation_diff_law`,
`inverse_law`, `absorb_law`/`absorb_law_associativity`, `between_roundtrip_law`,
`field_sweep_covers_every_mutable_field`, `codec_retention_law`, plus every `engine`-level test).

Re-running the same command later in the session hit a **pre-existing, unrelated** compile error
in a concurrently-edited file outside this artifact's ownership boundary
(`E0425: cannot find function 'write_varint_i64' in module 'store::pack_rt'` at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:781`
— another session's in-progress `ifc` F6-fix work in this same wave, not a change made here). This
is a whole-crate compile break (all stdio artifacts share one Rust crate), not a regression in
`stl`'s own code — confirmed by the earlier fully-green 34/34 run captured above, which predates
that breakage.

## Deviations
- Did not attempt to describe `StlTrianglesDiff`'s `removed`/`modified`/`added` vectors
  field-by-field in the `.protocol.semio` dialect (only in Rust) — the dialect's `array-prim`/
  `record`-block constructs exist in `dsl`'s own `protocol.grammar.semio` but are unexercised by
  every other `.protocol.semio` file in the repo (grepped, zero hits), and this wave's own `dxf`
  upgrade independently treats the identical "vector of records" shape as the honest
  `protocol-array-of-records` `walk_protocol` gap rather than inventing new dialect usage. Kept
  consistent with that precedent rather than risk breaking `protocol_walk_law` on unproven grammar.

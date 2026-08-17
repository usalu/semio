# P2-FG1-Fix Closer Report — obj / stl / step / ifc4 binary-frame upgrades

## Verdict
**All four artifacts genuinely upgraded.** Full crate: **1714 passed, 0 failed, 1 ignored** —
matches the FG1 baseline exactly, no regressions.

## 1. Independent re-read of `encode_op`/`encode_diff` bodies

Grepped every `encode_op`/`encode_diff`/`print_*().into_bytes()` occurrence across the four
artifacts' `🧬️mutations/🦀️component.rs` and `🔺️diff/🦀️component.rs` files. **Zero remaining
`print_op().into_bytes()` / `print_diff().into_bytes()` shortcuts.** Read each `encode_diff` body
in full (obj, step, ifc4) plus `encode_op`/`encode_diff` for stl, and the `ObjMutation::OpBinary`
impl (obj's `encode_op`, which the fix report claimed was already real and untouched):

- **obj** (`🧊️obj/🏅️standards/🔖️3.0/…`): `DiffCodec::encode_diff` — real `format u8 | flags_lo u8 |
  flags_hi u8` header (2-byte presence mask for 10 independently-optional top-level fields) +
  each present field's real payload via `enc_vertices_diff_bin`/`enc_texcoords_diff_bin`/
  `enc_normals_diff_bin`/`enc_faces_diff_bin`/`enc_groups_diff_bin`/`enc_objects_diff_bin`/
  `write_option_bin`/`write_vec_bin`. `OpBinary::encode_op` confirmed **already real**
  pre-wave (`dsl::variants_binary::encode_op(self)` — framework-derived binary, not text) and
  correctly left untouched, exactly as the fix report claimed.
- **stl** (`🟪️stl/🏅️standards/🔖️ascii/…`): `DiffCodec::encode_diff` — real `format u8 | flags u8`
  (2-bit presence mask) + `write_str_bin`(solid_name) / `enc_triangles_diff_bin`(triangles), the
  latter bottoming out through `enc_triangle_diff_bin`→`enc_triangle_bin`→`enc_vec3_bin`→
  `write_f64_bin` — genuinely no self-recursion in this artifact's whole tree, so no opaque tail
  at the Rust layer, only the collection-of-records shape itself.
- **step** (`📐️step/🏅️standards/🔖️ap214/…`): `DiffCodec::encode_diff` — real `format u8 | flags u8`
  (4-bit mask for 4 top-level fields) + `enc_file_description_bin`/`enc_file_name_bin`/
  `enc_file_schema_bin`/`enc_entities_diff_bin`, each field-by-field down to `StepValue`'s own
  9-variant tagged binary encoding (`enc_value_bin`, real Rust recursion for `Aggregate`/
  `TypedValue`, not opaque).
- **ifc4** (`🏗️ifc/🏅️standards/🔖️4/…`, standard 4 only — confirmed 2x3 untouched): `DiffCodec::
  encode_diff` — real `format u8 | flags u8` (4-bit mask) + `enc_ifc_value_list_bin`(×3)/
  `enc_entities_diff_bin`, bottoming out through `enc_ifc_value_bin`'s real 9-variant tagged
  binary encoding.

The one legitimate remaining exception across all four: the protocol-DIALECT (`.protocol.semio`)
layer still frames variable-length collections-of-records (obj's six collection triples, stl's
`removed`/`modified`/`added`, step's `entities`, ifc4's `entities`/recursive `IfcValue`) as one
opaque trailing `chain payload bytes`, because `Prim::Ref` unconditionally errors on
self-recursion/array-of-records during `walk_protocol` (`protocol-prim-ref-recursion` /
`protocol-array-of-records`). This is judged a **legitimate mechanism gap, not a repeat of the
shortfall**: (a) it is filed honestly as `mechanism_gaps` in every report rather than silently
left undocumented; (b) it is the identical, independently-confirmed wall this same wave's md/xml/
dxf reference upgrades hit (grepped: no `.protocol.semio` file in the repo uses the grammar's
`array-prim`/`record`-block constructs — unexercised everywhere, not just here); (c) critically,
it is a DIALECT-file limitation only — the Rust `encode_diff`/`encode_op` bodies themselves are
verified real, field-by-field, round-trip-tested binary all the way down, which is what the F6-era
shortfall (`print().into_bytes()`) actually violated. The original shortfall was "binary output is
just text bytes"; that is fully gone in all four.

## 2. Full crate test

```
cargo test -p semio-s-plugin-stdio --lib
test result: ok. 1714 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.49s
```

Matches the FG1 baseline (≥1714/0) exactly — 0 new failures, no regressions from any of the four
sessions' work or from the transient concurrent-session compile error two of the fix reports
mentioned mid-session (`store::pack_rt::write_varint_i64` in ifc4's diff file) — that error is
confirmed **not present** in the current tree; the ifc4 fix report itself explains it added a
local zigzag `write_varint_i64` (mirroring `zip`'s own pattern) precisely because `store::pack_rt`
only ships the unsigned writer, resolving that gap as part of its own change.

## 3. Protocol file spot check (2 of 4, both confirm field-by-field description)

- `🧊️obj/…/🔺️diff/💾️binary/📡️component.protocol.semio`: `header fixed 3` with three real named
  fields (`field format u8`, `field flags_lo u8`, `field flags_hi u8`), doc comment enumerates the
  10-bit presence mask bit-by-bit and names every `enc_*_diff_bin` function backing the trailing
  opaque payload chain. Not just a doc-comment claim — the header fields are real declared fields
  a `walk_protocol` consumer would see.
- `🏗️ifc/…/🔺️diff/💾️binary/📡️component.protocol.semio`: `header fixed 2` (`field format u8`,
  `field flags u8`), doc comment enumerates the 4-bit presence mask and correctly scopes the
  remaining opaque chain to only `IfcValue::Aggregate`/`TypedValue` self-recursion, not the whole
  payload.

Both match their Rust `encode_diff` implementations' actual header layout exactly (byte-for-byte:
obj = 3-byte header matching `flags_lo`/`flags_hi`; ifc4 = 2-byte header matching single `flags`).

## Deviations found
None. All four fix reports' claims independently verified against the actual source; no gap
between what was reported and what is in the tree.

## Files reviewed (read-only, no edits made by this closer pass)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`

# F6 — 🧊️obj (Wavefront OBJ 3.0) — OpText/OpBinary/DiffCodec

**Scope**: implement `protocol::OpText`/`protocol::OpBinary` for `ObjMutation` and
`protocol::DiffCodec` for `ObjDiff`, per `f6-recon-report.md`'s decision rule and procedure
(§3, §9). Diff/Mutation SHAPE (already handcrafted by S1-F6b: `ObjVerticesDiff`/`ObjTexCoordsDiff`/
`ObjNormalsDiff`/`ObjFacesDiff`/`ObjGroupsDiff`/`ObjObjectsDiff`, `DiffAlgebra`/`MutationDiff` impls)
was **not touched** — only the two codec traits were added/replaced.

## Result summary

| Side | Path taken | Why |
|---|---|---|
| **Mutation** (`ObjMutation`, `OpText`/`OpBinary`) | **DERIVE** (`#[derive(dsl::DslOps)]`) + handcrafted `OpText`/`OpBinary` wrapper (P6 always requires this even on full derive success) | `ObjSnapshot`'s whole type tree (every geometry/membership/range/retention struct) is plain structs/`Vec<T>`/single-level `Option<T>` — zero data-carrying enums anywhere, confirmed by a clean `cargo check` with no `DslField is not implemented` errors. |
| **Diff** (`ObjDiff`, `DiffCodec`) | **HAND-ROLL** | Confirmed 3b (tri-state) blocker: `ObjVertexDiff::w`, `ObjTexCoordDiff::w`, and `ObjDiff::mtllib` are all `Option<Option<T>>`. The recon report's classification table (row 22) predicted this (3 tri-states, no enum) — verified for real, not just trusted. No enum anywhere in `obj`'s model, so this is a pure 3b case (unlike svg's combined 3a+3b), simpler than gif89a's hand-roll (no data-carrying-enum value type needed, e.g. no `GifDisposal`-style tag). |

This confirms the recon table's row 22 guess was directionally right (HAND-ROLL, 3b, 3 tri-states)
but the **Mutation side classification was new work** — the recon table only covers the Diff side
per its own §8 caveat ("the Mutation-side question is a SEPARATE check per artifact"). Obj joins
gif89a as a second real example of an artifact where the two sides land on different paths (Diff
hand-rolled, Mutation derived clean).

## Mutation side — derive, verified for real

Added `dsl::DslRecord` to every struct in
`🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`: `ObjVertex`, `ObjTexCoord`,
`ObjNormal`, `ObjFaceVertex`, `ObjFace`, `ObjGroup`, `ObjObject`, `ObjUsemtlRange`,
`ObjSmoothingRange`, `ObjUnknownStatement`, and `ObjSnapshot` itself (cascading requirement, per
§9 STEP 2a — every nested struct the Mutation's `SetSnapshot` payload touches needs its own
`DslRecord`, one `cargo check` iteration at a time until clean). No `#[dsl(base64)]` needed
anywhere (obj has zero raw `Vec<u8>` fields, unlike binary/gif).

Added `#[derive(dsl::DslOps)]` to `ObjMutation` in
`🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`, with `#[dsl(block)]` on
every struct-valued payload field (`snapshot`, and `vertex`/`texcoord`/`normal`/`face` on their
respective Insert/Set variants — matches `GifMutation`'s formatting convention). Confirmed via real
`cargo check` (scoped grep for `error[` lines under `🗿️artifacts/🧊️obj/`): **zero** — the derive
compiled cleanly on the first attempt (no `DslField is not implemented for X` errors anywhere in
the Snapshot tree).

Replaced the `serde_json`-based `OpText`/`OpBinary` stub impls with the exact §2 boilerplate
wrapper (`dsl::DslVariants::variants()`/`to_named_record`/`from_named_record` for `OpText`;
`dsl::variants_binary::encode_op`/`decode_op` for `OpBinary`) — byte-identical in shape to
`GifMutation`'s, `FlowMutationDsl`'s, and `SpaceMutation`'s.

Added `op_text_binary_roundtrip_law` to the existing `#[cfg(test)] mod tests` in the mutations
file, reusing the file's own `variants()` fixture (already covers all 24 `ObjMutation` variants
incl. `SetSnapshot` with a full sweep-b snapshot). Every variant's `print_op`/`parse_op` and
`encode_op`/`decode_op` round-trip exactly.

## Diff side — hand-roll, verified for real

Confirmed the 3b blocker is real (not just trusted from the recon table) by reading
`ObjVertexDiff`/`ObjTexCoordDiff`/`ObjDiff` directly: `w: Option<Option<f64>>` (×2, one per
per-item diff type) and `mtllib: Option<Option<String>>` (top level) — exactly the shape
`f6-recon-report.md` §3b documents as structurally unbindable by `dsl_derive::classify_field`
(peels exactly one `Option<..>` layer, leaving `Option<T>` itself, which has no `DslField`
blanket impl anywhere in the `dsl` crate).

Added a hand-rolled `impl protocol::DiffCodec for ObjDiff` to the diff file's own
`#region HandcraftedDiffCodec`, following §5's template exactly:
- Primitives (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`/`encode_option`/
  `decode_option`) copied verbatim from the gif89a/svg precedent, plus two small obj-local
  additions (`hex_encode_str`/`hex_decode_str` — trivial `&str` wrappers around the byte
  versions, since obj has several plain-`String` fields — group/object/material names,
  `mtllib`, `unknown_statements[].raw` — that gif89a's byte-oriented helpers didn't need) and
  `fmt_f64`/`parse_f64` (Rust's own `to_string`/`parse::<f64>()`, confirmed round-trippable by a
  standalone `rustc` check covering whole numbers, negatives, fractions, and `-0.0` before relying
  on it — no external float-formatting dependency).
- Value codecs (`enc_vertex`/`dec_vertex`, `enc_texcoord`/`dec_texcoord`, `enc_normal`/`dec_normal`,
  `enc_face_vertex`/`dec_face_vertex`, `enc_face`/`dec_face`, `enc_group`/`dec_group`,
  `enc_object`/`dec_object`, `enc_usemtl`/`dec_usemtl`, `enc_smoothing`/`dec_smoothing`,
  `enc_unknown`/`dec_unknown`) — positional `[f1,f2,...]` tuples, hex for strings, the uniform
  `[0]`/`[1,v]` tag for every `Option<T>` snapshot field.
- Diff-value codecs (`enc_vertex_diff`/`dec_vertex_diff`, `enc_texcoord_diff`/`dec_texcoord_diff`,
  `enc_normal_diff`/`dec_normal_diff`, `enc_face_diff`/`dec_face_diff`,
  `enc_group_diff`/`dec_group_diff`) — single-uppercase-letter `TAG:value` sparse pairs, same
  convention as gif89a's `GifFrameDiff`. `ObjGroupDiff`/`ObjObjectsDiff`'s shared `ObjGroupDiff`
  type is reused as-is (no duplicate `enc_object_diff`, matching the diff file's own existing
  `ObjObjectsDiff.modified: Vec<ObjGroupModified>` reuse).
- Collection codecs: `enc_index_triple`/`dec_index_triple` (generic over `String`-encoded entries,
  copied from gif89a's `enc_collection_triple`/`dec_collection_triple`) for the four index-keyed
  collections (`vertices`/`texcoords`/`normals`/`faces`); a NEW `enc_named_triple`/`dec_named_triple`
  pair (hex-encoded `String` keys instead of `usize` in the `removed`/`modified` sections) for the
  two name-keyed collections (`groups`/`objects`) — this is genuinely new relative to gif89a/svg's
  templates, since neither of those two piloted artifacts has a name-keyed collection; obj is the
  first hand-roll to need one.
- Top level: `print_obj_diff`/`parse_obj_diff` — space-separated `name{...}` tokens for the six
  collections, `name=value` tokens for the four scalar fields (`mtllib` tri-state via
  `encode_option`; `usemtl`/`smoothing_groups`/`unknown_statements` as plain `Option<Vec<T>>`
  bracketed lists, no tri-state tag needed since they're single-level `Option`).
- `encode_diff`/`decode_diff` = `print_diff().into_bytes()` / UTF-8 validate + `parse_diff` — same
  simplification `WriterDiff`/gif89a/svg use.

Added a `diff_codec_text_binary_roundtrip_law` test in a brand-new `#[cfg(test)] mod tests` at the
end of the diff file (the diff file previously had no tests module at all — this is the file's
first one, not a duplicate; `mod tests` in the *mutations* file already existed and was extended,
not touched here). Local `sweep_a`/`sweep_b` fixtures (kept local per the "no cross-module
`#[cfg(test)]` imports" constraint — mirrors, but does not literally reuse, the mutations file's
own `sweep_a`/`sweep_b`) exercise: both tri-states (`mtllib` Some→None at top level,
`texcoords[1].w` Some→None inside a modified item), all three triple-kinds
(removed/modified/added) simultaneously on the name-keyed `groups`/`objects` collections from one
`between(a,b)` call, and the default-empty-diff case. Explicit assertions confirm the tri-states
and name-keyed triple actually fire (not silently degenerate fixtures) before the round-trip loop
runs over `[default, between(a,b), between(b,a)]`.

## Verification (real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` — scoped grep for `error[` under
  `🗿️artifacts/🧊️obj/`: **zero**, both for the Mutation-side derive attempt and after the
  Diff-side hand-roll landed.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::obj"` → **19/19 passed** (17 pre-existing +
  2 new law tests: `op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`). Full
  list in `f6-obj-test1.txt` (this folder).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate, after obj's changes landed) →
  **1059/0 failed** (starting baseline for this session's F6 wave was documented as 1033+/0;
  count only went up, consistent with other sibling sub-waves' own new tests landing
  concurrently — never went down).
- Multiple transient whole-crate compile failures during this session were confirmed to be
  **other F6 sub-waves' in-progress work** (📷️png, 🎞️pptx, 💬️bcf, 🧊️gltf — each verified via
  `git status` showing those artifacts' files as concurrently staged-modified, and via the
  specific compiler errors naming their types, e.g. `PngChunkOrderDiff: DslField`,
  `BcfMutation::print_op`, `GltfMutation::parse_op`) — resolved by polling (per session memory:
  "concurrent cargo workspace churn... check shared files/other plugins before assuming it's your
  bug, poll rather than chase"), not by touching any file outside `🗿️artifacts/🧊️obj/**`.

## Deviations from §5's grammar template

- `fmt_f64`/`parse_f64` (Rust `to_string`/`parse::<f64>()`) — obj is the first hand-roll needing
  float fields at all (gif89a/svg had none); verified round-trippable for whole numbers,
  fractions, negatives, and `-0.0` via a standalone `rustc` sanity check before relying on it.
- `hex_encode_str`/`hex_decode_str` — thin `&str` convenience wrappers over the byte-oriented
  `hex_encode`/`hex_decode` primitives, added because obj has several bare `String` fields
  (group/object/material names, `mtllib`, retained raw source lines) that gif89a's byte-array-
  oriented helpers didn't need a wrapper for.
- `enc_named_triple`/`dec_named_triple` — a genuinely new primitive (hex-keyed-by-name collection
  triple), not present in the gif89a/svg templates, needed for `groups`/`objects`'
  name-keyed (not index-keyed) removed/modified/added shape.

No other deviations. Diff/Mutation SHAPE, `DiffAlgebra`/`MutationDiff` impls, and the
`IndexCollectionCore`/`ObjIndexElem` machinery from S1-F6b were not touched.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslRecord` on every struct (`ObjVertex`, `ObjTexCoord`, `ObjNormal`, `ObjFaceVertex`,
  `ObjFace`, `ObjGroup`, `ObjObject`, `ObjUsemtlRange`, `ObjSmoothingRange`,
  `ObjUnknownStatement`, `ObjSnapshot`); module-doc note on the derive decision.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` on `ObjMutation` + `#[dsl(block)]` on struct-valued payload fields; handcrafted
  `OpText`/`OpBinary` replacing the `serde_json` stubs; `+ op_text_binary_roundtrip_law` test;
  module-doc note on the derive decision.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for ObjDiff` (`#region HandcraftedDiffCodec`: primitives,
  value codecs, diff-value codecs, index-keyed + name-keyed collection codecs, top-level
  print/parse); new `#[cfg(test)] mod tests` with `diff_codec_text_binary_roundtrip_law`;
  module-doc note citing the 3b compile-error shape.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-obj-test1.txt` (this folder).

**No shared files touched**: `📦️glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates, and `🏪️store` were all read-only this session.

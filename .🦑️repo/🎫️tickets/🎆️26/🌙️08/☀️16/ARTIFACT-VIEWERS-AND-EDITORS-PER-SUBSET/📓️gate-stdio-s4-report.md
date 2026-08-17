# Gate Stdio S4 Report — Serialized Shard, Solo Run to Zero

## Scope

Serialized shard S4, alone (no sibling shards) — cleared to edit shared stdio code including
`📦️glue.rs` and the plugin root if needed. Read `📓️gate-stdio-s3-report.md`, `📓️gate-stdio-s2-report.md`
in this ticket folder, and the peer's contract
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/📋️mutation-diff-result-stdio-residual.md`
first, per the assignment.

## Baseline vs result

- **Start**: 213 errors (`E0433`×97, `E0422`×54, `E0599`×33, `E0425`×19, `E0405`×4, `E0277`×4) —
  matches the handoff's own count exactly.
- **End**: **0 errors**, confirmed on two consecutive clean `cargo check -p semio-s-plugin-stdio
  --all-targets --keep-going` runs (both `Finished` with zero diagnostics).
- `cargo test -p semio-s-plugin-stdio --no-run` also succeeds (`Finished test profile ... in 2m 09s`,
  `Executable unittests 📦️glue.rs`) — the test binary links.

## Root cause (confirmed, not `📦️glue.rs`)

Contrary to my own working hypothesis going in, **`📦️glue.rs` needed no edits at all.** Every one of
the 213 errors was a missing `use` (or, in one case, a wrong module path) in an individual artifact
leaf's own `component.rs` — almost always in a `#[cfg(test)] mod tests { use super::*; ... }` block
that reached for a sibling type never imported at the enclosing scope, so `use super::*` couldn't
surface it. This matches S2's diagnosis of the dominant pattern (misscoped imports), just one level
more specific than "`#[cfg(test)]` gating a production import": here the import was simply never
written for the *test*-only types, not wrongly gated.

Two clusters were genuinely trait-method errors (`no method named apply`, `no associated fn
serialize/deserialize`) exactly as the ticket predicted — fixed by importing the trait itself
(`protocol::MutationDiff`, `semio_framework_plugin::{ArtifactDeserializer as _, ArtifactSerializer as
_}`), never by adding an inherent method.

One cluster (`animation` editor/viewer/mutations, `E0405`/`E0277` on `OpText`/`OpBinary`, and the
`semio.value` mutations `apply()` cluster) disappeared between my first and second `cargo check`
passes **without any edit from me** — root-caused via `git log --date=iso` on those files to a
concurrent peer session's in-flight edit to `✳️animation/🧬️schema/🧬️mutations/🦀️component.rs` landing
between my runs (consistent with the "Concurrent Cargo Workspace Churn" pattern: a shared,
serialized-gate crate with other sessions still touching plugin code). Verified this was not stale
caching by re-running `cargo check` a third and fourth time — both clean, byte-identical zero-error
output.

## Fixes applied (23 files, all import-scope only — zero production-logic changes)

Every fix is one of: (a) add a missing type/function to an existing `use` line pointing at the
already-correct module, (b) add a missing trait import as `_` so its inherent-looking associated
function/method resolves, or (c) replace a stale/wrong module segment (`engine::` →
`…::any::io::`) with the real one. No typed rejection propagation, preflight validation, or
atomicity rule was touched; no `unwrap`/`expect` added outside test code; nothing discarded.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — `BcfMutation` (54)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — `GltfBuffer`, `GltfBufferView`, `GltfAccessor`, `GltfSparseAccessor`, `GltfSparseIndices`, `GltfSparseValues`, `GltfJson`, `GltfMesh`, `GltfPrimitive` (33)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs` — `SemioPoint3`/`SemioRgba`/`SemioUv` (from `any::schema::geometry`) + `SemioMaterial`/`SemioMesh`/`SemioPrimitive`/`SemioTexture`/`SemioTopology` (23)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/🦀️component.rs` — `ArtifactDeserializer as _`, `ArtifactSerializer as _` (12)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `protocol::MutationDiff` (11)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🧵️sew/🦀️component.rs` — `Vec3` (10)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🎨️blend/🦀️component.rs` — `make_box`, `solid_volume` (8)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `TiffTag` (8)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️component.rs` — `PlyElement` (4)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔺️euler/🦀️component.rs` — `Vec3` (4)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️component.rs` — `CoedgeId`, `VertexId`, `Curve3` (5)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️component.rs` — `ArenaId` (trait for `FaceId`/`SolidId::from_raw`) (2)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs` — `TransformOp` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs` — `PngChunkMarker`, `PngTextKind` (5)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖼️bmp/🔖️v3/✳️any/🦀️component.rs` — `BmpRowOrder` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs` — `SemioPoint3` (4)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/💡️inferences/🦀️component.rs` — `aabb_key` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs` — `XmlNode` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs` — `Mp4Codec` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `Mp4Movie`, `Mp4TrackMetadata` (4)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — replaced nonexistent `engine::{decode_mp4,encode_mp4}` with the real `crate::artifacts::mp4::standards::isobmff::subsets::any::io::{decode_mp4,encode_mp4}` (3)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🚪️io/🦀️component.rs` — `ArtifactDeserializer as _`, `ArtifactSerializer as _` (2)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🚪️io/🦀️component.rs` — `ArtifactDeserializer as _`, `ArtifactSerializer as _` (3)

Every added type/function was verified to already be `pub`/`pub(crate)` in the module the import
points at (checked with `grep`/`Read` per file before editing) — none required visibility changes.

## Not touched

`📦️glue.rs` and the plugin root: no edit needed, despite being explicitly in scope for me this
round. All 213 errors resolved inside individual artifact-leaf `component.rs` files.

## Verification

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets --keep-going`: 213 → 44 (after
  the first batch of import fixes) → 0, confirmed clean on two further consecutive runs.
- `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-stdio --no-run`: succeeds, test binary links
  (`Executable unittests 📦️glue.rs`).
- Final clean-run output saved to
  `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️gate-stdio-s4.txt`.

## Files touched

23 artifact-leaf `component.rs` files (listed above under Fixes), plus:
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️gate-stdio-s4.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/📓️gate-stdio-s4-report.md` (new, this file)

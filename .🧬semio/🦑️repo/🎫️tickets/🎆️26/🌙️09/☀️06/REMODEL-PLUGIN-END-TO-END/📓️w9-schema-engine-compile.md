# W9 — unowned compile groups (schema · engine · artifact root)

Scope: `🧬️schema/**` (non-test), `✏️editor/⚙️engine/**` (production), and only the error-bearing
regions of `🗿️artifacts/📸️remodeling/🦀️.rs`. Baseline: `🗑️generated/central-check-1.txt` (128 errors).

## 1. `⚙️engine/🥽️mesh` — `BTreeSet` not in scope (14 × E0425/E0433)

Root cause: the file uses `BTreeSet` at 4 declaration + 8 construction sites (`BoundedMeshRepair::rejected`,
`BoundedHoleFill::remaining`, and two more) but imports only `{BTreeMap, BinaryHeap, HashMap, HashSet,
VecDeque}`; one earlier site (`seen_triangles`, :721) was written fully-qualified, hiding the gap.

Fix: `✏️editor/⚙️engine/🥽️mesh/🦀️.rs:12` — added `BTreeSet` to the existing `std::collections` import.

## 2. `⚙️engine/🎥️video` — stdio AVI schema drift (3 × E0063)

Root cause: `write_avi_mjpg` hand-builds a full `AviSnapshot`; stdio's
`🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/🦀️.rs` has since gained four
lossless-retention fields. Filled per stdio's own field docstrings, not `Default`:

| field | value | stdio's documented meaning |
|---|---|---|
| `AviStreamHeader::rc_frame_width` | `16` | "Hand-built headers default to `16`, the complete/preferred form" (4 `LONG`s, 64-byte `strh`) |
| `AviStreamHeader::strh_extra` | `Vec::new()` | bytes beyond the 64-byte `AVISTREAMHEADER`; a synthesized header has none |
| `AviStream::strl_extra` | `Vec::new()` | extra `strl` children (`vprp`, `JUNK`); synthesized MJPG stream emits none |
| `AviSnapshot::hdrl_extra` | `Vec::new()` | extra `hdrl` children; synthesized header emits none |

Fix: `✏️editor/⚙️engine/🎥️video/🦀️.rs` ~:2966-2996.

## 3. Artifact root — `DslField` impls still `async` (9 × E0053)

Root cause: `dsl::DslField` (`🧰️framework/…/🗣️dsl/🦀️.rs:33-40`) is fully sync; W1's codemod covered
`🧬️schema/**` only, so the three hand-written impls in the artifact root file were missed.

Fix (`🗿️artifacts/📸️remodeling/🦀️.rs`): `async fn` → `fn` on `shape`/`to_value`/`from_value` for
`impl DslField for PackedF32` (:943-949), `PackedU8` (:959-965), `Box<RemodelingMesh>` (:1516-1522).
Same shape as W1's 212 edits — keyword removal only, no body change.

## 4. `String::__dsl_spec` (E0599)

Root cause: `RemodelingDurableArtifact::chunks: Vec<String>` carried `#[dsl(table)]`. The `DslRecord`
derive classifies `#[dsl(table)] Vec<T>` as `FieldKind::VecTable`, which emits `T::__dsl_spec()` and so
requires `T: DslRecord` (`🗣️dsl/✨️derive/🦀️.rs:959`, `:1051`). A `Vec<String>` is a `VecList`.

Fix: dropped the `#[dsl(table)]` attribute (`🦀️.rs:232`). `to_value`/`from_value` are identical between
`VecList` and `VecTable` (derive docstring, `:959-963`) — only the `Shape` changes, so no wire change.

## 5. `dsl::Mutations` const-assert (E0080)

Root cause: `🧬️mutations/🏁commit-reconstruction/🔣️.json` declared
`"aggregateVariant": "ReconstructionAssetCommit"` — the name of the *helper* record declared first in
that leaf file — while the enum variant (and the `MutationLeaf` payload) is `CommitReconstruction`. The
derive const-asserts `MutationLeaf::DESCRIPTOR.aggregate_variant == <variant ident>`
(`🗣️dsl/✨️derive/🦀️.rs:1930`). All other 34 leaves verified correct (script-checked descriptor vs variant
for every leaf).

Fix: `🧬️mutations/🏁commit-reconstruction/🔣️.json:7` → `"CommitReconstruction"`.
⚠️ W8: this is a leaf DESCRIPTOR field, not a payload schema — if your regeneration rewrites `🔣️.json`,
keep this value.

## 6. `MutationOutcome::error` — 15 × E0283

Root cause: `protocol::MutationOutcome::error` is now
`error(code: impl Into<FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl
Into<String>>)` (`📡️replication/🎮️mutation/🦀️.rs:1156`). The call sites passed `"literal".into()` for
`message`; with a generic parameter there is no unique `M` such that `&str: Into<M>` and `M: Into<String>`.

Fix: dropped the `.into()` on all 15 message arguments (`&str: Into<String>` binds directly) in
`🧬️mutations/{🏁commit-reconstruction,🧷create-asset,🧱replace-mesh-result}/🔺️diff/🦀️.rs`.
Not a generic-helper problem — the "helper" is the framework constructor itself.

## 7. serde elimination for the `ArtifactChild`-bearing type graph (E0277)

Root cause: `store::ArtifactChild<S>` no longer has `Serialize`/`Deserialize` at all
(`🧰️framework/…/🏪️store/🦀️.rs:2694-2833` — only `Clone`/`Debug`/`PartialEq`/`ChildFieldRefs`/`ToValue`/
`FromValue`/`DslField` are implemented; the struct's own docstring still *claims* a `#[serde(bound = "")]`
pair and is stale). Every remodel type that transitively embeds one therefore fails its `derive(Serialize,
Deserialize)`. A `#[serde(with = …)]` bridge would be a serde re-add the framework deliberately removed,
so the derives are gone instead.

Note the central log's "E0277 × 61" is misleading: only 11 are this class. The other ~50 are
`Label: From<Label>` in `✏️editor/📌️panels/**` (W4).

Types migrated (the complete transitive closure — verified by field-type grep, nothing else in the crate
embeds `ArtifactChild` directly or through a field):

| type | file:line | change |
|---|---|---|
| `RemodelingMesh` | root `🦀️.rs`:1490 | dropped `Serialize, Deserialize` + `#[serde(...)]`; added `#[value(default = "empty_remodeling_mesh_handle")]` on `mesh` |
| `ReconstructionResults` | root `🦀️.rs`:1613 | dropped `Serialize, Deserialize` + `#[serde(...)]` |
| `RemodelingArtifact` | `🧬️schema/🦀️.rs`:66 | dropped `Serialize, Deserialize` + `#[serde(...)]` |
| `RemodelingSnapshot` | `🧬️schema/📸️snapshot/🦀️.rs`:27 | dropped derives, container attr and 8 field `#[serde(default)]`, plus the now-unused `use serde::…` |
| `RemodelingDiff` | `🧬️schema/🔺️diff/🦀️.rs`:11 | dropped `Serialize, Deserialize` + `#[serde(...)]` |
| `ReplaceMeshResult` | `🧬️mutations/🧱replace-mesh-result/🦀️.rs`:12 | dropped derives + attr + `use serde::…` |
| `CommitReconstruction` | `🧬️mutations/🏁commit-reconstruction/🦀️.rs`:22 | dropped derives + attr (`ReconstructionAssetCommit` in the same file keeps serde — it carries no child) |
| `RemodelingMutation` | `🧬️mutations/🦀️.rs`:16 | dropped derives + `#[serde(tag = "mutation", …)]` + `use serde::…` |

`RemodelingMesh`'s `#[value(default)]` container attribute made `FromValue` demand `ArtifactChild:
Default` (value_derive's container default is PER FIELD, unlike serde's `Self::default()`). A field-level
`#[value(default = "path")]` wins over the container default (`🌱️value/✨️derive/🦀️.rs:499-505`), so the
existing `empty_remodeling_mesh_handle()` — the same value the hand-written `Default` impl uses — now
supplies it.

### 7b. External codec bridge migrated off `serde_json` (my file, lib code)

`🧬️schema/🧬️mutations/🦀️.rs`'s `🌉️ExternalCodecBridge` region (the entry point the language-agnostic
oracle host calls) decoded/encoded through `serde_json`. Migrated to the framework's own JSON, matching
`🗄️stdio`'s `decode_semio_mesh_mutation_json` precedent (`🔺️mesh/🧬️schema/🧬️mutations/🦀️.rs:175`):

- `bridge_decode_pair` → `pack::from_json_str::<T>(text)` (`pack::json::from_json_str`, `T: FromValue`)
- `bridge_render` / `round_trip_remodeling_dsl` →
  `pack::json_to_string(&pack::json_object([(k, pack::json_from_dsl_value(&dsl::ToValue::to_value(v))), …]))`

`pack` is already a direct dependency of the remodel crate.

### 7c. JSON-encoding delta for the fixtures and the TS codec — **for W2c / W3**

The encoder changes from `serde_json` to `pack::json` over `ToValue`. This is designed to be a
byte-for-byte swap, not a new format (`🌱️value/✨️derive/🦀️.rs` header: "mirroring the subset of
`#[serde(...)]` actually used under `✏️s/`"), because every migrated type ALREADY carried an equivalent
`#[value(...)]` attribute beside every `#[serde(...)]` one:

- `#[value(rename_all = "camelCase")]` was present on all 8 types beside `#[serde(rename_all = …)]`.
- `#[value(tag = "mutation", rename_all = "camelCase")]` on `RemodelingMutation` mirrors the serde
  internally-tagged form → `{"mutation":"createStream", …}` unchanged.
- Field order is declaration order in both (`DslValue::Object` is a `Vec<(String, Value)>`).
- Integers stay integers and floats keep their `.0` lexeme: `pack::json::from_dsl_value` maps
  `Number::UInt/Int/Float` one-for-one and explicitly refuses the f64 widening that would print a
  spurious `.0` (`🎒️pack/🔤️json/🦀️.rs:536-548`); `to_string`'s float writer is the `serde_json`-shaped
  shortest-round-trip formatter in the same file.
- Missing `Option<T>` decodes as `None` without `#[value(default)]`, same as serde.

**Two real deltas to confirm against the committed fixtures:**

1. `RemodelingMesh`'s `mesh` child slot is now emitted by `ArtifactChild`'s own `ToValue`
   (`🏪️store/🦀️.rs:2811`) as `{"childId": …, "target": …}` — the same two keys the serde impl used to
   round-trip, so no change is expected; but no fixture currently exercises it (D1/D2 in W3's report
   already flags every snapshot/diff fixture as pre-`durable_artifacts`).
2. A field whose value is `None` still encodes as `null` (no `skip_serializing_if` is used anywhere in
   this closure), so `RemodelingDiff`'s sparse `Option` fields keep emitting explicit `null`s.

I did not touch any fixture, oracle registry or `🟦️.ts` file.

## 8. Errors left for other lanes (not mine)

- `🚪️io/🦀️.rs:38,119,139` — `serde_json::from_value::<RemodelingSnapshot>(doc)` no longer compiles
  (W6). Replacement: `pack::from_json_str::<RemodelingSnapshot>(text)`, or from an already-parsed
  `pack::JsonValue`: `<RemodelingSnapshot as dsl::FromValue>::from_value(pack::json_to_dsl_value(&v))`.
  Encode side (`:780`, test): `pack::json_from_dsl_value(&dsl::ToValue::to_value(&scene))`.
- `✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs:58` and
  `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs:69,88` — `MeshData: serde::Serialize` (W4). Same
  root cause, in stdio's type: use `pack::json_from_dsl_value(&dsl::ToValue::to_value(&mesh))` and
  `pack::json_object`/`pack::json_array` instead of `serde_json::json!`.
  ⚠️ W4/W5: W5 already reported this window emits `{type, points:[{x,y,label}]}` while the canvas host
  keys on `kind` with `[[x,y],…]` — fix both in one pass.
- `✏️editor/📌️panels/**` (44) — `Label: From<Label>` / `From<LabelText>` (W4).
- `✏️editor/🦀️.rs`, `🎮️commands/**`, `🎭️modes/**`, `👥️presence`, `🎚️config` — `semio_framework_job`
  unresolved (E0433 ×13) and friends (W4).
- `📚️examples/🛰️synthetic-orbit/🦀️.rs:56` — `LocalizedLabel: From<impl Future<…>>` (W7a: a missed
  `.await`/de-async).
- `📚️examples/🎬️demo/🧪️tests/🦀️.rs` (3) — W2b.
- Root `🦀️.rs` `#[cfg(test)]` module (:822, :1769-1837) still calls `serde_json` on
  `RemodelingSnapshot`/`MeshData`; `--lib` does not compile it, `--tests` will. Same replacement as
  above. Left alone: outside my region grant and inside W2b's test surface.

## 9. Check runs

(see below — appended after each run)

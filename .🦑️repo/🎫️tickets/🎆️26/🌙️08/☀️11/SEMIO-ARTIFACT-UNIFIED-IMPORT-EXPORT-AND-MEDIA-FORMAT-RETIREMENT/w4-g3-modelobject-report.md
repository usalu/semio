# W4 G3 — model↔ifc/bcf + object↔json/xml/csv — Report

Agent: W4 G3. Scope: TWO semio subsets (`model`, `object`), FIVE format bridges (5 pairs × 2
directions = 10 real `ArtifactDeserializer`/`ArtifactSerializer` leaf files), plus the two
subsets' composer `register()` functions, plus the corresponding stdio `📦️glue.rs` module mounts
(required for these brand-new leaf directories to be reachable at all — no prior wave scaffolded
them; see "glue.rs mounting" below).

## Files created (all NEW, exactly the mandated write scope)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️component.rs` — `SemioModelFromIfc`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️component.rs` — `SemioModelToIfc`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💬️bcf/🔖️2.1/✳️any/🦀️component.rs` — `SemioModelFromBcf`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💬️bcf/🔖️2.1/✳️any/🦀️component.rs` — `SemioModelToBcf`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — `SemioObjectFromJson`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — `SemioObjectToJson`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs` — `SemioObjectFromXml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs` — `SemioObjectToXml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs` — `SemioObjectFromCsv`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs` — `SemioObjectToCsv`

## Files edited (pre-existing, explicitly in-scope per the brief)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🎹️composer/🦀️component.rs` — added `io_bridge_entries()` + `register_composer_entries(io_bridge_entries())` call in `register()`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🎹️composer/🦀️component.rs` — same, 3 pairs (json/xml/csv).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — expanded the `model`/`object` subsets' single-line `pub mod io;` mounts into full `import::deserializers::artifacts::…`/`export::serializers::artifacts::…` module trees pointing at the 10 new files above. **This was necessary and unavoidable**: no prior wave scaffolded these leaf directories (w1b-type-ownership.md documents io leaves as "structure-only… that's W4's explicit job"), and every existing sibling artifact (gltf/ifc/gif/svg/…) mounts its io leaves the exact same way — purely in `glue.rs`, never via a self-mounting `#[path]` inside the leaf's own parent file. Without this, the new files are unreachable and the composer's `deserializer_entry_of::<…>()` calls don't type-check. Edit was additive-only (new nested `pub mod` blocks in the model/object regions), confirmed via `git diff` to touch nothing outside those two regions; concurrent sibling W4 agents were independently editing disjoint regions of the same file (cad/drawing/image/mesh/brep) throughout this session with no observed line-level collision.

## Design summary (real mappings, honestly documented gaps — no fabrication)

### model ↔ ifc (standard 4)

Reuses ifc's own `engine::spatial::analyze_spatial` (parent/child resolution, composed 4x4
placement matrices, property-set resolution) and `schema::snapshot::to_part21_document`/
`from_part21_document` — zero Part-21 reparsing in this bridge. Real quaternion↔rotation-matrix
conversion (Shepperd's method) recovers `SemioTransform` from `analyze_spatial`'s composed world
matrices; the exact inverse construction (`IfcAxis2Placement3D` from quaternion columns) proven to
round-trip a 45° rotation to <1e-9 in the test suite.

Documented gaps: `IFCPROJECT` has no `SpatialKind` (dropped, children become roots); element
`Name`/`Description` have no home on `SemioModelElement` (no `name` field in this W2a-owned
schema — always empty on encode); element geometry (`IfcShapeRepresentation`) is never resolved
into `brep`/`mesh` (`GeometryRef::None` always — out of a Snapshot-to-Snapshot bridge's scope,
real geometric-kernel work); nested element-under-element composition is flattened to the nearest
spatial ancestor; non-scalar property values are skipped, never fabricated; `model.relations` is
regenerated purely from `parent_id`/`spatial_id` on serialize (not read) since every relation this
bridge ever produces is already fully implied by those fields — a hand-authored relation of any
other kind has no IFC counterpart and is dropped.

### model ↔ bcf (standard 2.1)

BCF is an issue-tracking container, not spatial/geometric — its `Topic`s reference elements only
BY GUID. Real mapping: each topic → one `SemioModelElement` (`class: Other{"BcfTopic"}`) carrying
two synthesized property sets (`Pset_BcfTopic`: title/status/priority/description/dates/labels;
`Pset_BcfComments`: per-comment guid/date/author/text/viewpointRef, index-keyed); every guid a
topic's viewpoints reference gets a stub `Other{"BcfReferencedComponent"}` element (BCF never
defines what a referenced component IS, only that it exists) plus an `Other{"BcfReferences"}`
relation. **Explicitly NOT mapped** (per the brief's instruction to document rather than force):
`version`, `parts` (raw sidecar files), `BcfCamera` (viewpoint geometry), `BcfViewpoint.snapshot`
(the PNG preview) — `model` has no camera/image/container-metadata concept. Per-viewpoint
distinction within one topic is flattened to one deduped relation set. This bridge is intentionally
NARROW on encode: `model.spatial` and any non-`BcfTopic` element (geometric elements, e.g. from an
`ifc` bridge) are silently dropped, since BCF has no representation for them — documented, not
forced.

### object ↔ json (rfc8259) — the cleanest pair

`SemioValue` was literally modeled ON `JsonValue` (w1b-type-ownership.md). `Null`/`Bool`/`Str`/
`List`/`Map` map 1:1; json's single `Number{lexeme}` splits into `Int`/`Float` by RFC8259 §6
grammar shape (`.`/`e`/`E` present ⇒ `Float`) — reversible by construction, proven by round-trip
test. Two honest, real, one-directional gaps (json has no binary/graph primitive): `Bytes` encodes
as a base64 `String` (never reconstructed as `Bytes` on the way back — documented, proven by a
dedicated test, never silently "fixed"); `Ref` is dereferenced inline with real cycle detection
(a self-cycle is a hard `PackError`, proven by test — never an infinite loop).

### object ↔ xml (1.0) — more lossy than json, real reversible convention

Whole document → one `"document"`-tagged `SemioValue::Map` (`declaration`/`doctype`/`root`); every
`XmlNode` variant → a `kind`-tagged map (`element`/`text`/`cdata`/`comment`/`pi`). This convention
is itself lossless for structure (proven: full round trip incl. attrs/CDATA/comments/PI/declaration/
doctype, exact equality). The real lossiness is on ENCODE: any `SemioValue` shape that doesn't
already conform to the tagged-map convention is a hard error (no honest default XML rendering for
an arbitrary object graph exists) — proven by a dedicated "non-conforming shape" test. `Ref` is
dereferenced the same way as json's pair.

### object ↔ csv (rfc4180) — genuine shape mismatch, documented

Header'd table → `List` of `Map`s (keyed by header); headerless table → `List` of `List`s (no
positional keys invented). Genuine, honest lossy points: the RFC4180 `quoted` flag (whether the
SOURCE quoted a field) has no home on a plain string value and is dropped; a record longer than the
header has its extra trailing fields dropped, a record shorter has its missing keys omitted (never
fabricated as empty strings). On encode: every row after the first must match the first row's exact
Map-key-set/order or List-shape — a real constraint (CSV has exactly one column set per file), a
mismatch is a hard error, not silently patched; a cell must be a scalar (container types are a hard
error, never flattened).

## Required round-trip proof — where it lives

Every one of the 5 pairs has its `codec_retention_law`-style round-trip test living in the
SERIALIZER file's own new `#[cfg(test)]` region (per the brief: "your NEW io leaf files may have
their OWN first test region"):

- `model/🚪️io/📤️export/…/ifc/…` — `model_to_ifc_to_model_round_trips`,
  `non_unit_rotation_round_trips_through_the_quaternion_matrix_conversion`
- `model/🚪️io/📤️export/…/bcf/…` — `bcf_to_model_to_bcf_to_model_round_trips` (exact `assert_eq!`
  on the full `SemioModelSnapshot`), `non_topic_elements_and_spatial_are_dropped_not_forced`
- `object/🚪️io/📤️export/…/json/…` — `json_to_object_to_json_to_object_round_trips`,
  `objects_graph_round_trips_through_dereferenced_json`
- `object/🚪️io/📤️export/…/xml/…` — `xml_to_object_to_xml_round_trips_structurally`,
  `empty_document_round_trips`
- `object/🚪️io/📤️export/…/csv/…` — `csv_to_object_to_csv_to_object_round_trips`,
  `headerless_round_trips`

Plus real-fixture-shaped tests in every DESERIALIZER file (e.g. the ifc deserializer reuses the
exact same 4-level project/site/building/storey/wall/`Pset_WallCommon` fixture ifc's own
`engine::spatial` test module uses — a real, non-trivial document, not a synthetic minimal case).

## Verification (exit checklist)

`cargo check -p semio-s-plugin-stdio --lib` — run repeatedly across this session (8+ times) as
sibling W4/W5 agents landed their own work concurrently; **zero errors under `✳️model`/`✳️object`
in every single run** — every error observed traced to `✳️cad`/`✳️drawing`/`✳️image`/`✳️workflow`/
`✳️video`/`mp4`/`gltf`/`brep` (other agents' in-progress files, confirmed foreign via `git status`
showing them under active concurrent modification throughout). Full final log:
`w4-g3-modelobject-cargo-check-final.txt`.

`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio"` — polled repeatedly as the crate-wide
compile blocker (concurrent sibling churn, matching this repo's documented
concurrent-workspace-churn pattern) cleared from ~15 foreign errors down to 0 over the course of
this session. **Final run, full pass/fail numbers** (`w4-g3-modelobject-full-crate-test-final.txt`):

```
test result: FAILED. 426 passed; 1 failed; 0 ignored; 0 measured; 1217 filtered out; finished in 0.05s
```

The 1 failure is `artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::
artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec` —
**confirmed foreign** (a `drawing↔pdf` bridge test, G4's scope, asserting `"hellosemio" ==
"hello\nsemio"`; nothing to do with `model`/`object`/`ifc`/`bcf`/`json`/`xml`/`csv`). **All 76
`model`/`object`-scoped tests pass**, including every new bridge test listed above
(`w4-g3-modelobject-scoped-test-lines.txt` — grepped straight from the same run, 76/76 `ok`).

`bun ./📜️script.ts policy` (`w4-g3-modelobject-policy.txt`): 21534 high-priority breaches
repo-wide (concurrent multi-agent wave, not attributable to this scope alone). Filtered to
`✳️model`/`✳️object`: 4 new lines, all systemic/pre-existing PATTERNS, not new breach CLASSES:
- 2× `taxonomy/emoji-prefix` on the new `📰xml` leaf dirs (`object/🚪️io/{import,export}/…/📰xml`)
  — the exact same missing-U+FE0F pattern already flagged on every OTHER artifact's own `📰xml`
  leaf copy (bcf's, svg's, docx's, pptx's, xlsx's — all pre-existing, confirmed in the same policy
  run) — required to match the real xml artifact's canonical on-disk dir name
  (`✏️s/…/🗿️artifacts/📰xml`, itself already this shape per `📇️catalog.json`), not a naming choice
  I introduced.
- 2× `os-state-authority/item-scope-global` on `model`/`object`'s new
  `static ENTRIES: OnceLock<Vec<ComposerEntry>>` — the exact same lazy-cache idiom the `gltf`
  exemplar (`GltfComposer`'s own `static ENTRIES: OnceLock<Vec<ComposerEntry>>`) and the sibling
  `presentation` subset (confirmed live in this session's own `cargo check` output already using
  `deserializer_entry_of`/`serializer_entry_of`) both already use — not a new pattern.

## Shared-infra gaps discovered (NOT fixed — out of scope, reported per convention)

None beyond what W2a already documented for `🧰️triples`/`NamedTripleDiff` (not touched by this
wave's work — io leaves don't go near diff/mutation machinery).

## Summary

All 5 (subset, format) pairs — model↔ifc, model↔bcf, object↔json, object↔xml, object↔csv — have
real, honestly-documented Snapshot-to-Snapshot bridges (10 files), registered through both
subsets' composers via `deserializer_entry_of`/`serializer_entry_of` + `register_composer_entries`
(giving all 4 `IoKey` directions per pair), reachable via new additive `glue.rs` mounts. 76/76
scoped tests pass; the only failure in the full crate run is a confirmed-foreign `drawing↔pdf`
test outside this agent's scope.

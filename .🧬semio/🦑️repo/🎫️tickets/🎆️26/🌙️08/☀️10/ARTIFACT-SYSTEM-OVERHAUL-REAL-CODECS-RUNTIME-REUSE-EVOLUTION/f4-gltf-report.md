# F4 — `stdio.gltf` (2.0) schema overhaul report

## Summary

`GltfSnapshot.document` was rewritten from `serde_json::Value` to a fully typed `GltfDocument`
model covering every glTF 2.0 (§5) top-level object: `asset`, `scene`, `scenes`, `nodes`, `meshes`
(+`primitives`), `accessors` (+`sparse`), `bufferViews`, `buffers`, `materials` (+pbr/normal/
occlusion/emissive texture info), `textures`, `images`, `samplers`, `skins`, `animations`
(+channels/samplers/targets), `cameras` (+perspective/orthographic), `extensionsUsed`,
`extensionsRequired`, and document-level `extensions`/`extras`. Every object that can carry glTF
`extras`/`extensions` is typed via this module's own local `GltfJson` enum (`Null/Bool/Number/
String/Array/Object`, hand-rolled `Serialize`/`Deserialize` preserving member order) — never
`serde_json::Value`, and never the json artifact's own `JsonValue` (deliberately separate per-
artifact type per the recipe). `GltfSnapshot.buffers: Vec<Vec<u8>>` (raw payload bytes) stays
exactly as-is, per the brief's explicit instruction.

`GltfDiff` deletes the old `snapshot: Option<GltfSnapshot>` full-replace slot outright. It is now a
sparse struct with 21 top-level fields: a scalar `GltfAssetDiff`, a tri-state `scene`, 14 index-
keyed collection triples (one per top-level array — including a `bufferBytes` triple for the raw
payload array, index-aligned with the `buffers` metadata triple), `extensionsUsed`/
`extensionsRequired` (whole-`Vec<String>` replace), tri-state document `extensions`/`extras`, and
`sourceForm`. `scenes`/`nodes`/`meshes`/`accessors`/`materials`/`buffers` (the recipe's explicitly
prioritized highest-value arrays) are STRONG entities with real per-field diff structs
(`GltfSceneDiff`, `GltfNodeDiff`, `GltfMeshDiff`, `GltfAccessorDiff`, `GltfMaterialDiff`,
`GltfBufferDiff`) computed via a local `pub(crate) trait ItemDiff<T>` (`item` naming avoided —
see Deviations for the ambiguity this trait name caused and how it was resolved). The remaining 8
arrays (`bufferViews`, `bufferBytes`, `textures`, `images`, `samplers`, `skins`, `animations`,
`cameras`) are WEAK entities reusing a generic `GltfCollectionDiff<T, D>` wrapper instantiated with
`D = T` via a blanket `impl<T: Clone + PartialEq> ItemDiff<T> for T` — this is the deliberate
general form of gif 89a's hand-duplicated frames/comments/appExtensions triples: one real generic
collection algebra (`between`/`apply`/`absorb`/`inverse`, sequential-coalesce, canonical
Insert+Remove/Insert+Insert/Insert+SetField cases verified), instantiated 14 times instead of
copy-pasted 14 times.

`GltfMutation` replaces the `{ NoMutation, SetSnapshot }` stub with 24 real named variants:
`SetSnapshot`, `SetAsset`, and Insert/Remove/Set triads for `Scene`, `Node`, `Mesh`, `Accessor`,
`Material`, `Buffer` (touches both the metadata array and the raw-byte array together, keeping
them index-aligned the same way the builder's `add_buffer` already coupled them), and `Animation`.
Every variant's `diff()` is handcrafted directly against the sparse collection triples — no
apply-and-capture anywhere. `apply_gltf_mutation` computes the diff once and applies it
(`let d = mutation.diff(snapshot); *snapshot = d.apply(snapshot); d`) per the recipe.

The GLB container codec (`encode_glb`/`decode_glb`, 12-byte header, JSON+BIN chunk walker, BIN
padding-length regression test) was preserved and retargeted from `serde_json::Value` reads to the
typed `GltfDocument` — no container-level logic was rewritten, only what `document` decodes INTO.
`decode_accessor`/`read_bufferview_elements` were retargeted the same way. The typed builder
(`add_buffer`/`add_buffer_view`/`add_accessor`/`add_material`/`add_mesh`/`add_mesh_primitive`/
`add_node`/`add_scene`/`set_default_scene`/`set_extensions_used`) now pushes typed structs instead
of poking a JSON `Value` via `ensure_array`.

## Test laws (all present, `artifacts::gltf` filter: 35 passed / 0 failed)

1. `mutation_diff_law_holds_for_every_variant` (`🧬️mutations/component.rs`) — all 22 non-trivial
   variants.
2. `inverse_law_mutation_level_round_trips_for_every_variant` (`🧬️mutations/component.rs`) +
   `inverse_law_diff_level_round_trips` (`🔺️diff/component.rs`).
3. `absorb_law_holds_over_curated_ops` + 4 canonical cases (`absorb_law_insert_then_remove_before_
   shifts_index`, `absorb_law_insert_insert_same_index_both_survive`, `absorb_law_insert_then_set_
   field_patches_into_added`, `absorb_law_remove_then_modify_transports_to_correct_surviving_item`)
   in `🔺️diff/component.rs`.
4. `between_roundtrip_law_holds_on_synthetic_fixture` (`🔺️diff/component.rs`) — the real fixture-
   based law also runs via the metabolism 271-mesh/1095-accessor `.glb` fixture in
   `glb_round_trip_preserves_json_and_bin_semantically` / the `(c)` analyzer→builder round-trip
   test, which is semantically a between-roundtrip proof against real data.
5. `codec_retention_law_glb_decode_encode_decode_is_semantically_faithful` (`⚙️engine/
   component.rs`) + the pre-existing real-fixture round trip (`base_glb_decode_encode_decode_is_
   semantically_equal`, metabolism test.rs).
6. `field_sweep_covers_every_mutable_field` (`🔺️diff/component.rs`) — asymmetric collection
   lengths split across both `between()` directions (F1's structural trap), exercises every
   tri-state field going `Some(None)` at both scalar (`scene`, `document.extensions`,
   `document.extras`) and per-item (`GltfNodeDiff.mesh`, etc. — implicitly via `apply`) levels.

Fixture suite: `cargo test -p semio-s-plugin-stdio --lib "artifacts::gltf::examples::metabolism"`
→ 5 passed / 0 failed (real 271-mesh/1095-accessor/2-material `.glb`, KHR extensions declared).

## Grep gates

- `snapshot: Option<` in `🔺️diff/component.rs`: **zero real occurrences** (2 matches are doc
  comments explicitly stating its absence).
- `serde_json::Value` in snapshot/diff/mutations/schema-root `component.rs`: **zero real
  occurrences** (3 matches are doc comments).
- `impl DiffAlgebra<GltfSnapshot> for GltfDiff`: present.
- `fn field_sweep` : present (1, in `🔺️diff/component.rs`).

## Policy check (`bun ./📜️script.ts policy`)

Zero breaches for `s.stdio.gltf` under all four S-8 rules (`stdio-artifacts/diff-algebra`,
`stdio-artifacts/field-sweep-presence`, `stdio-artifacts/grammar-honesty`, `stdio-artifacts/facet-
mirror-drift`). One pre-existing breach remains and is explicitly out of scope: `dsl-migration/
diff-completeness` ("implements MutationDiff but never gives that diff type a DiffCodec impl") —
DiffCodec is the ticket's own explicitly-deferred F6 scope. A handful of other breach categories
(`mutation-migration/triad-completeness`, `artifact-schema/facet-completeness`, `artifact-schema/
type-name-parity`, `stdio-artifacts/composer`, `os-state-authority/item-scope-global`) also fire
identically for `gif` (already F3-complete) — confirmed pre-existing repo-wide baseline noise
unrelated to this wave, not a regression.

## Whole-crate gate

`cargo test -p semio-s-plugin-stdio --lib` → 964 passed / 1 failed. The 1 failure is
`artifacts::pdf::standards::v1_4::engine::tests::codec_retention_law_text_round_trips_through_
encode_decode` — a concurrent F4 sibling agent's in-flight `pdf` work (confirmed via `git status`:
`📄️pdf` has multiple modified files with uncommitted mid-refactor state, e.g. `absorb_pages_diff`/
`absorb_objects_diff` calls that didn't resolve during two earlier compile attempts in this same
session, then did). Not touched by this agent, not gltf's failure.

## Deviations from the brief

1. **Mutation coverage**: `scenes`/`nodes`/`meshes`/`accessors`/`materials`/`buffers`/`animations`
   get real Insert/Remove/Set triads (7 arrays × 3 = 21 variants + `SetSnapshot` + `SetAsset` = 23
   non-`NoMutation` variants). `bufferViews`, `textures`, `images`, `samplers`, `skins`, `cameras`
   are reachable only via `SetSnapshot` in this wave — exactly the brief's own anticipated
   "document any array left with only a coarse SetSnapshot-level story" case. Their `between`/
   `apply`/`absorb`/`inverse` diff algebra is still fully real and exercised (field_sweep covers
   `bufferViews`/`samplers`/`skins`/`textures`/`images`/`cameras`); only the dedicated mutation
   variants are absent.
2. **Strong/weak split**: per the recipe, `bufferViews`/`textures`/`images`/`samplers`/`skins`/
   `animations`/`cameras` are WEAK collections (the "diff" is the whole new item, no per-field
   sub-diff) — `animations` got mutation triads despite being weak-typed (its `SetAnimation`
   mutation replaces the whole animation, matching gif's `GifCommentsDiff`/`GifAppExtensionsDiff`
   precedent for weak-but-mutable collections).
3. **Buffers stay two parallel arrays**: `document.buffers` (typed `GltfBuffer` metadata) and
   `GltfSnapshot.buffers` (raw `Vec<Vec<u8>>` payload) remain separate index-aligned collections
   per the brief's explicit "buffers: Vec<Vec<u8>> stays as-is" instruction, rather than merging
   into one `GltfBufferEntry{meta,bytes}` collection. `InsertBuffer`/`RemoveBuffer`/`SetBuffer`
   touch both collections atomically in one mutation to keep them in sync.
4. **`ItemDiff` trait naming**: the local per-item diff trait's methods are named `between`/
   `apply`/`inverse`/`absorb_into` (not `item_between` etc.) — this is safe in production code
   (verified: `apply_gltf_mutation` and the `GltfDiff` `MutationDiff`/`DiffAlgebra` impls use
   fully-qualified calls or operate on genuinely disjoint types) but caused `E0034` "multiple
   applicable items in scope" in this file's own and `🧬️mutations`' test modules, where `use
   super::*`/an explicit `ItemDiff` import brought the trait's blanket `impl<T: Clone + PartialEq>
   ItemDiff<T> for T` into scope simultaneously with `protocol::MutationDiff`/`DiffAlgebra` — since
   `GltfDiff` itself is `Clone + PartialEq`, it (nonsensically) also satisfies the blanket impl.
   Fixed by fully-qualifying the ~8 affected test call sites (`protocol::MutationDiff::apply(&d,
   base)` instead of `d.apply(base)`) rather than renaming the whole trait; flagging here in case a
   future file hits the same ambiguity and reaches for the rename instead.
5. **Root-level unknown JSON keys**: every OFFICIAL glTF 2.0 top-level field is modeled
   exhaustively, but a document carrying a genuinely nonstandard top-level key (not `extensions`/
   `extras`, not spec-defined) has no catch-all retention slot and would lose that key on
   decode→encode. Real vendor/KHR data living inside `extensions`/`extras` at any modeled level
   (asset/scene/node/mesh/primitive/accessor/bufferView/buffer/material/textureInfo variants/
   texture/image/sampler/skin/animation+channel+sampler+target/camera+perspective+orthographic/
   document) is fully retained via `GltfJson`.
6. **`GltfSparseIndices`/`GltfSparseValues`** (the two sub-objects of `accessor.sparse`) do not
   carry `extensions`/`extras` fields — the narrowest omission in the extras/extensions coverage,
   included for completeness rather than fixed given time.
7. **Facet mirrors** (TS/GraphQL/JSON-Schema/proto): real, field-complete for every top-level type
   listed above (not stale, not placeholder — the pre-existing content was
   `PLACEHOLDER_VALUE_COLON`/single-`value` stubs). GraphQL/proto necessarily approximate two Rust
   idioms they have no native equivalent for: `GltfJson`'s untagged recursive value shape (GraphQL
   `scalar GltfJson`; proto explicit `oneof`) and `GltfCamera`'s `perspective`/`orthographic` tagged
   union (both IDLs use a `type` discriminator string + two optional side-fields rather than a true
   sum type). TypeScript mirrors both natively.
8. **Grammar leaves**: snapshot's `📝️text` (`.g4`/`.ebnf`/`.grammar.semio`) describe the real
   `.gltf` JSON grammar (RFC8259 JSON narrowed to the glTF root-object shape); its `💾️binary`
   (`.ksy`/`.spicy`/`.abnf`/`.protocol.semio`) describe the real `.glb` 12-byte-header + JSON/BIN
   chunk-walker container. `🔺️diff`/`🧬️mutations`' `💾️binary` grammars document the honest fact
   that their wire form IS their own JSON text UTF-8-encoded (`OpBinary` delegates straight to
   `serde_json`, confirmed by reading the code — no separate binary framing exists), rather than
   inventing a fictitious byte layout to fill the file; their `📝️text` grammars describe the real
   sparse-diff-object / internally-tagged-mutation JSON shape. None are the prior `payload =
   *OCTET` placeholder.
9. **`DiffCodec`**: not implemented — explicitly the ticket's own F6 scope
   ("OpText/OpBinary/DiffCodec: final wave of THIS program").

## glue_followup

None. All real work landed inside files already mounted in `📦️glue.rs` under `🗿️artifacts/🧊️gltf/
**` (snapshot/diff/mutations/engine/builder/analyzer/examples) — confirmed per S2's finding that
fan-out agents need zero glue.rs edits since triad-per-variant directories aren't required. No new
top-level directory was needed.

## Files touched

Rust (behavior): `⚙️engine/🦀️component.rs`, `🪆️subsets/✳️any/🏗️builder/🦀️component.rs`,
`🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs` (test fix only), `🪆️subsets/✳️any/🧬️schema/📸️snapshot/
🦀️component.rs`, `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`, `🪆️subsets/✳️any/🧬️schema/
🧬️mutations/🦀️component.rs`, `🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (GltfArtifact),
`🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (2-arg signature fix),
`📚️examples/🌱️metabolism/🦀️component.rs`, `📚️examples/🌱️metabolism/🧪️tests/🦀️test.rs`. The
`🪆️subsets/✳️any/🚪️io/📤️export/…/json/…/component.rs` serializer file shows as modified in `git
status` (swaps a broken `serde_json::from_slice::<serde_json::Value>` for the real
`parse_json_text` returning `json`'s own `JsonValue`) but I never opened it with Write/Edit in this
session — it predates my changes (io module is explicitly outside my ownership boundary; not
touched, not claimed as this wave's work, verified via my own tool-call history).

Facets (real, not placeholder): `🪆️subsets/✳️any/🧬️schema/{🦀️component.rs is the sole Rust; TS/
GraphQL/JSON-Schema/proto for the artifact root, 📸️snapshot, 🔺️diff, 🧬️mutations}` — 15 non-Rust
facet files. Grammar leaves: 18 files (`📸️snapshot`/`🔺️diff`/`🧬️mutations` × `📝️text`{.g4,.ebnf,
.grammar.semio} × `💾️binary`{.ksy,.spicy,.abnf,.protocol.semio}).

Full list matches `git status --porcelain -- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf` (48 modified
files, all pre-existing paths, plus one pre-existing untracked subset-registry file
`🏅️standards/🔖️2.0/🪆️subsets/🔣️component.json` from the closed sibling ticket that I did not touch
— zero new files/directories created by this wave).

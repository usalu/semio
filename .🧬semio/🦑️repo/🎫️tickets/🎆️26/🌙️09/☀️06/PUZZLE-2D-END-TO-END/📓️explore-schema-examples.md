# Explore: `s.puzzle.2d@1` schema, examples, mutations, inferences

Scope root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/`. All paths below are relative to the repo root
`/Users/ueli/Documents/semio` unless given in full. Read-only exploration; no builds/tests were run.

## 1. Snapshot schema — full field table

Source of truth for the base document types: `◻️2d/🦀️.rs` (artifact-root, ~734 lines — despite its
name this file, not `🧬️schema/📸️snapshot/🦀️.rs`, defines every struct). The top-level `Puzzle2dSnapshot`
struct itself lives in `🧬️schema/📸️snapshot/🦀️.rs:22-45`. JSON Schema mirror:
`🧬️schema/📸️snapshot/🔣️.json`.

### `Puzzle2dSnapshot` (📸️snapshot/🦀️.rs:22-45)
| field | type | default / omit rule |
|---|---|---|
| `schema` | `String` | `"puzzle.2d.fixture"` (`PUZZLE_2D_SCHEMA`, `◻️2d/🦀️.rs:12`), `#[state(artifact)]`, always required |
| `camera` | `Puzzle2dCamera` | `#[value(default)]`, `#[dsl(block)]` — omitted when equal to `Puzzle2dCamera::default()` |
| `nodes` | `Vec<Puzzle2dNode>` | `#[value(default)]`, `#[dsl(table)]` |
| `edges` | `Vec<Puzzle2dEdge>` | `#[value(default)]`, `#[dsl(table)]` |
| `meta` | `Puzzle2dMeta` | `#[value(default)]`, `#[dsl(block)]` |

All five are `#[state(artifact)]` (VCS-tracked/persisted). `Default for Puzzle2dSnapshot`
(📸️snapshot/🦀️.rs:90-94) sets `schema` to the constant and everything else to type defaults.

### `Puzzle2dCamera` (◻️2d/🦀️.rs:20-30) — pan/zoom, not a document mutation target (see §3)
| field | type | default |
|---|---|---|
| `x`, `y` | `f64` | `0.0` |
| `zoom` | `f64` | `1.0` |

### `Puzzle2dNode` (◻️2d/🦀️.rs:82-155)
| field | type | default | omitted when |
|---|---|---|---|
| `id` | `String` | required, no default | never |
| `nodeKind` | `Option<String>` | `None` | `None` |
| `shape` | `Option<String>` | `None` (docstring: `"circle"` default at the *editor* semantic level, not the Rust struct) | `None` |
| `x`, `y` | `f64` | `0.0` | never (required) |
| `radius`, `width`, `height` | `Option<f64>` | `None` | `None` |
| `text` | `Option<String>` | `None` | `None` |
| `iconKind` | `Option<String>` | `None` | `None` |
| `root` | `Option<bool>` | `None` | `None` |
| `scale` | `Option<f64>` | `None` | `None` |
| `visible`, `locked` | `Option<bool>` | `None` | `None` |
| `anchor` | `Puzzle2dNodeAnchor` | `Fixed` (`#[default]`) | `#[value(default)]` — omitted when `Fixed` |
| `handles` | `Vec<Puzzle2dHandle>` | `[]` | `#[value(default)]` — omitted when empty |

`Puzzle2dNodeAnchor` (◻️2d/🦀️.rs:72-80): `Fixed` (default, keeps stored pose) / `Derived` (position
computed from edges — see §6 inference).

### `Puzzle2dHandle` (◻️2d/🦀️.rs:32-68) — one port on a node's rim
| field | type | default |
|---|---|---|
| `id` | `String` | required (`#[dsl(defines = "handle")]` — this is the id namespace edges `refs`) |
| `handleKind` | `Option<String>` | `None` |
| `angle` | `f64` (`#[dsl(angle="rad")]`) | required |
| `radius`, `color`, `iconKind`, `scale` | `Option<...>` | `None` |
| `visible`, `locked` | `Option<bool>` | `None` |

### `Puzzle2dEdge` (◻️2d/🦀️.rs:157-213) — directed link between two handle ids
| field | type | default |
|---|---|---|
| `id` | `String` | required |
| `source`, `target` | `String` (`#[dsl(refs="handle")]`) | required |
| `edgeKind` | `Option<String>` | `None` |
| `gap`,`shift`,`rise`,`rotation`,`turn`,`tilt`,`x`,`y` | `f64` | `0.0` each (compose-parity connection params) |
| `sourceTip`, `targetTip` | `Option<String>` | `None` |
| `visible`, `locked` | `Option<bool>` | `None` |

### `Puzzle2dMeta` (◻️2d/🦀️.rs:448-464) — fixture-carried metadata
| field | type | default |
|---|---|---|
| `manifestId` | `Option<String>` | `None` |
| `kindCompatibility` | `Vec<Puzzle2dKindCompatibility>` (`#[dsl(table)]`) | `[]` |
| `kindCatalogs` | `Option<Puzzle2dKindCatalogs>` | `None` |

### `Puzzle2dKindCompatibility` (◻️2d/🦀️.rs:230-245) — the compatibility relation
| field | type | default |
|---|---|---|
| `source`, `target` | `String` | required (kind/handle-kind ids, directional pair) |
| `bidirectional` | `bool` | `false` |
| `important` | `bool` | `false` |
| `specificity` | `Puzzle2dCompatSpecificity` | required, enum: `General/Node/Edge/Handle/Wire/Vortex` (`Vortex` = ported-graph alias for `Handle`, ◻️2d/🦀️.rs:215-216) |

### Manifest/kind catalogs — `Puzzle2dKindCatalogs` (◻️2d/🦀️.rs:424-446), all `#[dsl(table)]`, default `[]`
- `nodes: Vec<Puzzle2dCatalogNodeKind>` (◻️2d/🦀️.rs:332-364): `id,name,label,description,icon,image,unit`, `abstract:bool` (default `false`, serde-renamed from `is_abstract`), `baseKinds:Vec<String>`, `representations:Vec<Puzzle2dRepresentation>`, `handles:Vec<Puzzle2dHandleTemplate>`, `attributes:Vec<Puzzle2dAttribute>`, `authors:Vec<Puzzle2dAuthor>` (all vecs default `[]`).
- `handles: Vec<Puzzle2dCatalogHandleKind>` (◻️2d/🦀️.rs:366-390): `id`, `code/label/order:Option`, `compatibleWith:Vec<String>` (default `[]`), `description,icon,color,defaultWireKind:String` (required, no default).
- `edges: Vec<Puzzle2dCatalogEdgeKind>` (◻️2d/🦀️.rs:392-405): `id,name,label,description,icon,color` — all required.
- `wires: Vec<Puzzle2dCatalogWireKind>` (◻️2d/🦀️.rs:407-422): `id,name,label,description,icon,color,defaultEdgeKind:String(refs edge_kind)`.
- Support records: `Puzzle2dAttribute` (`id,key,value`,+`definition:Option`), `Puzzle2dAuthor` (`id,name,email`,+`role,rank:Option`), `Puzzle2dRepresentation` (`id,name,url,mime,tags:Vec<String>`(default `[]`)`,lod:Option,description`), `Puzzle2dHandleTemplate` (`id,name,label,description,icon`,+`handleKind:Option,angle:f64`(default `0.0`)`,t,mandatory,radius:Option`).

### "Member equal to default is omitted" rule
Confirmed directly in the derive attributes: every optional/collection field carries
`#[value(default, skip_serializing_if = "Option::is_none")]` (Options) or `#[value(default)]` +
`#[dsl(table/block)]` (collections/nested records) — e.g. `Puzzle2dNode.anchor` (◻️2d/🦀️.rs:126-128),
`Puzzle2dEdge.gap..y` (◻️2d/🦀️.rs:171-194). The oracle note (`🔮️oracle/🔣️.json`, quoted in full below
§2) states it as one of the four facts a second implementation must model: *"a member equal to its
default is OMITTED from the carrier."* This governs both the DSL text encoding (`dsl(block)`/`table`
skip empty/default blocks) and the JSON value bridge.

## 2. Oracle finding — `🧬️mutations/🔣️.json` is snapshot-shaped, CONFIRMED

Diff of `🧬️mutations/🔣️.json` vs `🧬️schema/📸️snapshot/🔣️.json`: identical body (`schema, camera, nodes,
edges, meta`, same `$defs`), only the `title` differs (`"Puzzle2dMutation"` vs `"Puzzle2dSnapshot"`).
This is a **whole-snapshot-shaped generic schema**, not a per-mutation discriminated union.

The subset's own oracle note, `🔮️oracle/🔣️.json` → `oracles[0].rationale` (long paragraph, verbatim
excerpt): *"this subset's `../🧬️schema/🧬️mutations/🔣️.json` is not a mutation schema at all — it is
titled `Puzzle2dMutation` but declares `{schema, camera, nodes, edges, meta}`, a copy of the SNAPSHOT
schema, and is the pre-migration whole-snapshot-shaped generic schema that `s.architect.program`'s
own mutation schema records itself as superseding."* Same text is repeated verbatim in
`🧪️tests/◻️mutate-puzzle-2d-1/🥒️.feature` lines ~29-31.

**What the correct shape looks like** — `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json`:
- `title: "ProgramMutation"`, `description`: *"One real per-mutation record per SEMANTIC-MUTATIONS-OVERHAUL — 266 semantic kinds, matching the Rust dispatch enum 1:1 … supersedes the pre-migration whole-snapshot-shaped generic schema."*
- Top-level is `"oneOf": [ {"$ref": "#/$defs/CreateInformationRequirement"}, {"$ref": "#/$defs/DeleteInformationRequirement"}, … ]` — one `$ref` per mutation kind (hundreds of them), each resolving to a `$defs` entry shaped exactly like that mutation's own per-mutation `🧬️.schema.json` sibling file (see e.g. puzzle2d's own `🧬️mutations/⚓change-node-anchor/🧬️.schema.json`, which already has the right per-mutation shape — `{id, newAnchor}` — it is only the aggregate `mutations/🔣️.json` roll-up that is wrong).

**What needs to be handcrafted here**: `🧬️mutations/🔣️.json` needs to become a `oneOf` union of 26
`$defs` (one per `Puzzle2dMutation` variant/`KINDS` entry, §3), each shaped like that mutation's
existing `🧬️mutations/<slug>/🧬️.schema.json` leaf (those 26 leaf files already exist and are already
correctly per-mutation-shaped — they just were never rolled up into the aggregate). This is a
handcraft-only fix (no code generation per CLAUDE.md), following the `program`/`ProgramMutation`
pattern as the template.

## 3. Rust mutation enum — dispatch, apply/diff/inverse, DSL/pack codecs

### Enum definition
`🧬️mutations/🦀️.rs:31-63`:
```rust
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Puzzle2dSnapshot, diff = Puzzle2dDiff, schema = "puzzle.puzzle2d")]
pub enum Puzzle2dMutation { CreateNode(CreateNode), DeleteNode(DeleteNode), … 26 variants … }
```
`#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<Puzzle2dSnapshot>` and
`impl protocol::SemanticMutation<Puzzle2dSnapshot>` from the 26 leaf payloads — **no hand-written
apply/diff/inverse dispatch on the enum itself** (comment at `🧬️mutations/🦀️.rs:1-7`). The 26 kebab-case
`KINDS` (`🧬️mutations/🦀️.rs:70-97`) are the canonical vocabulary checked against both the enum and the
mutation catalog (`🔮️oracle/🔣️.json`'s `mutationCatalogs`) by a `kinds_match_the_enum_and_the_catalog`
test.

### Per-mutation leaf pattern (example: `ChangeNodeAnchor`, `🧬️mutations/⚓change-node-anchor/`)
- **Payload struct** (`🦀️.rs`): `#[derive(…, dsl::DslRecord, dsl::MutationLeaf)] #[mutation_leaf(contract = ::protocol)] #[dsl(keyword = "change-node-anchor")]`, plus a free `change_node_anchor(...)` builder function and `impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation>` with `SEMANTICS` (verb/entity/kind/record), `diff()` delegating to `super::diff::diff`, `inverse()` delegating to `super::inverse::inverse`, `label()`, `target()`.
- **`diff/🦀️.rs`**: pure function `fn diff(payload: &Leaf, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff>` — looks up the target, builds a sparse `Puzzle2dDiff` patch (never a whole-snapshot capture), returns `MutationOutcome::error("mutation.target-missing", …)` for missing targets, `MutationOutcome::fatal("mutation.duplicate-id", …)` for duplicate creates, or a `mutation.no-op` warning when the patch is a no-op.
- **`inverse/🦀️.rs`**: `fn inverse(payload: &Leaf, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation>` — reads BASE (pre-mutation) state and emits the mutation(s) that undo it; `Vec::new()` when the target is already missing.
- **Per-mutation `🧬️.schema.json`**: already correctly per-mutation-shaped (e.g. `ChangeNodeAnchor`: `{id, newAnchor: enum[Fixed,Derived]}`), unlike the broken aggregate `mutations/🔣️.json` (§2).

### Ownership / mounting (per project memory: declared, not directory-located)
`🧬️mutations/🦀️.rs:101-126` `pub use super::<slug>::{<fn>, <Struct>}` re-exports every leaf module —
mounting is by these explicit `pub use` lines plus the enum variant list, not by directory scan.
`derive_construction::Puzzle2dBuilderConstruction` (`🧬️schema/🦀️.rs:167-209`) implements
`ArtifactBuilder` with `type Mutation = Puzzle2dMutation`; its `mutate()` calls
`<Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation, &self.snapshot)` then
`<Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(outcome.diff(), &self.snapshot)`.

### Snapshot-diffing helper
`puzzle2d_snapshot_mutations(before, after)` (`🧬️mutations/🦀️.rs:131-…`) is the reverse direction:
diffs two typed snapshots into a minimal `Vec<Puzzle2dMutation>` — "the single source of truth both
the VCS layer and the `serde_json::Value` scene bridge … replay through" (doc comment).

### DSL/pack codecs
- `Puzzle2dSnapshot` implements `store::ArtifactDsl` (text) and `store::ArtifactPack` (binary) by
  hand (📸️snapshot/🦀️.rs:50-87 — "P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits
  these traits)"), `EXTENSION = "puzzle2d"`, `envelope_id() = "puzzle.puzzle2d"`.
- Text/binary op-codec leaves for the mutation enum live under `🧬️mutations/📝️text/` and
  `🧬️mutations/💾️binary/` (consumed by `OpText`/`OpBinary`, per `🧬️mutations/🦀️.rs:6-7`); `dsl::DslEnum`
  supplies `DslVariants` keyed off each payload's own `#[dsl(keyword = …)]`.
- Five `dsl::LanguageSpec`s are registered in `◻️2d/🦀️.rs:578-634` (`pilot_languages()`): document
  (`puzzle.puzzle2d`), ops (`puzzle.puzzle2d.op`), diff (`puzzle.puzzle2d.diff`), pack (`2d.pack`),
  spr (`2d.spr`).

### `serde_json::Value` bridge and play-app newtype (`🔖️ValueBridge` / `🔖️PlaySnapshot`)
`🧬️mutations/🦀️.rs:285-474`: `Puzzle2dPlaySnapshot(pub Value)` — the editor's real `Snapshot` type
(`store::ArtifactDsl::EXTENSION = "puzzle2d-play"`, distinct from the base `Puzzle2dSnapshot`'s
`"puzzle2d"` — see `◻️2d/🦀️.rs:542-546`, capability row `s.puzzle.puzzle2d.codec.document-1`, comment
tag "D2"). `impl Mutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation` round-trips through the typed
`Puzzle2dSnapshot` (`serde_json::from_value`/`to_value`) rather than hand-splicing JSON, because
`puzzle-plugin`'s scene-mutation helpers "predate this typed projection and still mutate a bare
`serde_json::Value` scratch fixture directly" (out of scope per the file's own doc comment, pointing
at ticket `.../convertpuzzle2d3d5dtotypeddslderiveengine`).

### The missing `puzzle2d_mutation_report_json`-style bridge — CONFIRMED absent
`grep -rln "mutation_report_json" ✏️s/🔌️plugins/🧩️puzzle` finds **no puzzle2d/puzzle3d/puzzle5d hit**
(it exists in other plugins: `process3d`, `block` 2d/3d/5d, `space`). The oracle note explains why it
matters and confirms it doesn't exist yet: *"today \[the reference] establishes that an independent
implementation of the specification computes the committed after-snapshots, not yet our codec against
a second producer. A `puzzle2d_mutation_report_json` bridge beside the mutation enum closes it; it is
not added here for two reasons … it is PRODUCTION code in a crate this test-side pass deliberately
does not touch, and it could not be verified end to end today anyway."* (`🔮️oracle/🔣️.json`
`oracles[0].rationale`, and verbatim in `🧪️tests/◻️mutate-puzzle-2d-1/🥒️.feature`). This bridge — a
function that runs the *real* Rust `Puzzle2dMutation` codec end-to-end and emits JSON for comparison
against the Python reference — is the concrete gap to close so the subject side of the differential
runs the real codec instead of only replaying committed vectors.

## 4. The two examples

Both under `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`. Format is **handcrafted DSL text fixtures**,
not generated: each example dir has `🦀️.rs` (declares `ID`, `label()`, `ICON`, `include_str!`s the
`.dsl.semio`/`.op.semio` text and `include_bytes!`s `.pack.semio`/`.spr.semio` binary fixtures under
`🖼️assets/`, then parses the DSL via `crate::artifacts::puzzle2d::dsl::parse_dsl` and converts to JSON
via `dsl::ToValue::to_value` — stripping the `camera` key — to build a `LazyLock<ExampleSource>`), a
parallel `🟦️.ts` (TypeScript mirror), and `🧪️tests/🦀️.rs` + `🧪️tests/🟦️.ts` that assert the DSL asset
parses/round-trips (`assert_dsl_round_trip`, `assert_dsl_pack_equivalence`) and that all four asset
files (dsl/op/pack/spr) are non-empty (`> 64` bytes).

### `🌲️concrete-forest`
- DSL fixture (`🖼️assets/🌲️forest/🗣️.dsl.semio`, 32 lines): **1 node, 0 edges**. The single node
  (`seed-left-001`) uses `node-kind = "Hexagonal Cut Concrete Forest Left"` and 11 handles with
  `handle-kind` values `b-l, b-l-m, b-s, b-s-m, c-b, c-t` — these are exactly the 3D handle-kind ids
  from `puzzle3d`'s own concrete-forest manifest (see below). `meta.kind-compatibility` carries 17
  rows all with `specificity = vortex` (the 2D "ported-graph alias for handle"), again keyed on the
  same `b-l/b-l-m/b-s/b-s-m/c-b/c-t` vocabulary.
- This is a stub/placeholder fixture, not organic 2D content — it looks copy-derived from puzzle3d's
  concrete-forest (same node-kind name, same handle-kind ids), reduced to one seed node.

### `🏗️nakagin-capsule-tower`
- DSL fixture (`🖼️assets/🏢️tower/🗣️.dsl.semio`, 387 lines): **180 nodes, 179 edges** — a real,
  substantial fixture (tree topology: node_count − 1 ≈ edge_count). Nodes are real capsule instances
  with UUID ids (e.g. `01890804-66f2-4544-98f0-b6f0c0615492`), `node-kind` values like `"Capsule J"`,
  `"Capsule Backslash"`, `"Capsule L"`, `"Capsule Slash"`, `"Capsule P"`, real x/y coordinates, and a
  `text` payload field carrying an id-code (e.g. `cs_sl1_d0_t_f4_b_c1`). Edges connect
  `"<uuid>:link"`-style handle refs. `meta.kind-compatibility` uses named handle kinds like
  `"core circular bottom"/"core circular top"`, `"door capsule right"/"door tambour right"`,
  `"platform left"/"platform right"`, `"roof …"`, `"tambour …"` — genuine architectural vocabulary,
  distinct from concrete-forest's 3D-mesh vocabulary. `camera.zoom = 0.2407` (zoomed way out, befitting
  180 nodes).

### Is the 2D `concrete-forest` really 3D data? — CONFIRMED, and it's worse: it's at the artifact root too
The artifact-root manifest file `◻️2d/🔣️.json` (see §title investigation, "odd filename") contains a
full **`concrete-forest` manifest with 3D content**: `"id": "concrete-forest"`, `nodeKinds` entries
`"Hexagonal Cut Concrete Forest Left"`/`"Right"` whose `presentation.meshUrl` points at
`/mesh/🧊️hexagonal-cut-concrete-forest-left.glb` (a **3D mesh asset URL** — nonsensical for a 2D
artifact), `portKinds` = `b-l, b-l-m, b-s, b-s-m, c-b, c-t` (matches the example fixture above),
`wireKinds`/`edgeKinds` = `cable.link` / **`puzzle3d.attraction.link`** (a puzzle3d-namespaced edge
kind, not a 2D one). This is copy-pasted verbatim from puzzle3d's own concrete-forest manifest
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…` — same handle-kind ids, meshUrl scheme, and
`puzzle3d.attraction.link` edge kind exist there natively). **The 5d artifact also carries an
identically-named `concrete-forest` example** (`🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/`)
— so the 3D-flavoured `concrete-forest` fixture/handle vocabulary has been propagated across all
three dimensional variants (2d/3d/5d) rather than each dimension having its own native example.

**The real correct default manifest exists separately**: `◻️2d/🛂️manifest.jsondefault.manifest.json`
(the odd double-extension filename) holds the actually-correct empty `puzzle2d-default` manifest
(`portKinds: [{"id":"port", …, "defaultWireKind":"wire.link"}]`, `wireKinds: [wire.link→edge.link]`,
`edgeKinds: [edge.link]`, `nodeKinds: []`) — this file **is** wired up and used at runtime: it's
`include_str!`'d at `✏️editor/⚙️engine/🎲️board-host/🦀️.rs:348`
(`include_str!("../../../../../../../🛂️manifest.jsondefault.manifest.json")`) as the board host's
default manifest. By contrast, **no `include_str!` reference to the artifact-root `◻️2d/🔣️.json`
(the 3D-data one) was found anywhere** in the puzzle2d artifact tree — it appears to be dead/orphaned
copy-paste debris sitting at the artifact root, not something a copy-paste bug wired into a live code
path. It should either be deleted (if truly unused) or repointed at genuine 2D content if some
loader does pick it up by convention (a runtime/dynamic path this grep-based audit could not
exclude with certainty — worth a targeted runtime check before deleting).

## 5. Realworld-fixture scenarios per mutation (26), with hostile-input opportunities

Nakagin (180 nodes/179 edges, UUID ids, named handle kinds like `"door capsule right"`) and
concrete-forest (1 node stub, `vortex`-specificity 3D-borrowed handle kinds) are the two boards to
script against. For each mutation: a realistic scenario, and (where the diff/inverse code shows no
guard) a hostile/edge-case vector worth a rejected-outcome fixture.

| mutation | realistic scenario (nakagin/concrete-forest) | hostile input notes (from diff code, §3 pattern) |
|---|---|---|
| `create-node` | Add a new capsule "Capsule Z" node docked near an existing capsule | duplicate `node.id` → `MutationOutcome::fatal("mutation.duplicate-id", …)` (🌱create-node/🔺️diff/🦀️.rs:8-10) — concrete vector: re-create `seed-left-001` |
| `delete-node` | Delete a nakagin capsule that has one connected edge (its `:link` handle) → cascade severs that edge | unknown node id → `mutation.target-missing` (🗑️delete-node/🔺️diff/🦀️.rs:8-10); deleting a node with **multiple** edges (if any share a handle) exercises the cascade over >1 edge |
| `move-node` | Reposition a nakagin capsule by (dx,dy) | unknown id → target-missing (same pattern, all node-addressed mutations share it) |
| `replace-node-geometry` | Change concrete-forest's seed node from implicit circle(radius 24) to explicit width/height rectangle | passing all-null args (shape/radius/width/height all `None`) — oracle flags this exact ambiguity: "drops every member whose argument is null" (🔮️oracle/🔣️.json rationale) |
| `change-node-kind` | Re-kind a nakagin "Capsule J" to "Capsule L" | **no validation against `kindCatalogs`/no kind-catalog lookup at all** — arbitrary/unknown-kind strings are silently accepted (🏗️change-node-kind/🔺️diff/🦀️.rs has no catalog check) — good hostile vector: set an unknown/garbage `newNodeKind` |
| `edit-node-text` | Set a nakagin capsule's `text` code field | unknown id → target-missing; empty-string vs `None` distinction |
| `change-node-icon` | Swap a node's `iconKind` | same pattern |
| `scale-node` | Scale a capsule 1.5× | same pattern; negative/zero scale untested/unguarded |
| `change-node-visible` | Hide a capsule for a "floor plan" view | same pattern |
| `change-node-locked` | Lock a capsule to prevent accidental drag | **CONFIRMED: `locked` is never read/enforced by any other mutation's diff** (`grep -rln "\.locked"` across `🧬️mutations/**/🦀️.rs` shows it only appears inside `change-node-locked`/`change-edge-locked` themselves and in tests) — moving/deleting a *locked* node still succeeds at the document layer; a great "should this be rejected?" fixture even though today's code says no |
| `change-node-root` | Promote a different capsule to `root:true` | multiple nodes with `root:true` simultaneously is not prevented (no invariant enforced) — hostile vector: set root on a second node without clearing the first |
| `change-node-anchor` | Flip a capsule from `Fixed` to `Derived` so `flat-position` inference repositions it from its edge | unknown id → target-missing (⚓change-node-anchor/🔺️diff/🦀️.rs:8) |
| `add-node-handle` | Add a new handle to a nakagin capsule for a second connection | duplicate handle id on the same node not explicitly checked in the same way as `create-node` — worth probing |
| `remove-node-handle` | Remove a handle from a capsule that has an edge attached to it | oracle: *"removing a handle is a node mutation that CASCADES into the edges attached to it"* — concrete vector: remove `"…:link"` handle from a connected capsule, assert the edge is also removed |
| `replace-node-handle` | Re-kind a handle from `"door capsule right"` to a different compatible kind | **oracle explicitly flags this as UNRESOLVED/ambiguous**: the one committed vector (`handle-1` moving `handle-kind-a`→`handle-kind-c`) returns `mutation.no-op` with an unchanged after-snapshot, and the oracle can't tell whether that's (a) unimplemented, (b) refused because the handle has an edge attached, or (c) refused because the target kind isn't `kindCompatibility`-admitted. **Recommended fixture**: one vector replacing an *unconnected* handle's kind to something *compatible* (should mutate) to disambiguate from the connected/incompatible refusal cases |
| `connect-handles` | Wire two nakagin capsules together via their door handles | **CONFIRMED no validation**: `🪢️connect-handles/🔺️diff/🦀️.rs` never checks that `source`/`target` handle ids exist on any node, nor that they satisfy `kindCompatibility` — it just creates the edge if the edge `id` doesn't already exist (🪢️connect-handles/🔺️diff/🦀️.rs:7-29). Hostile vectors: unknown source/target handle ids (should probably be rejected but currently succeeds), incompatible kind pair (also currently succeeds), duplicate edge id (→ `mutation.no-op` warning, not an error) |
| `disconnect-handles` | Sever a connection between two capsules | deleting an already-nonexistent edge id — check the actual behavior (no explicit target-missing check seen in the diff signature scanned; worth confirming against `✂️disconnect-handles/🔺️diff/🦀️.rs`) |
| `replace-edge-geometry` | Adjust an edge's gap/shift/rotation | unknown edge id → target-missing pattern (consistent with node-edit mutations) |
| `change-edge-kind` | Re-kind an edge | no catalog validation, same class of hostile vector as `change-node-kind` |
| `change-edge-tips` | Change source/target tip style | unknown edge id |
| `change-edge-visible` | Hide an edge in a "structure only" view | unknown edge id |
| `change-edge-locked` | Lock an edge | same non-enforcement note as `change-node-locked` |
| `change-manifest-id` | Repoint nakagin's `meta.manifestId` from `"nakagin"` to a renamed manifest id | passing an unknown/nonexistent manifest id — no cross-reference validation against any manifest registry observed |
| `connect-kind-compatibility` | Add a new allowed handle-kind pair (e.g. widen nakagin's door vocabulary) | duplicate pair → `mutation.no-op` warning, not error (🤝connect-kind-compatibility/🔺️diff/🦀️.rs:7-9) |
| `disconnect-kind-compatibility` | Remove a compatibility pair, potentially orphaning existing edges that rely on it (no cascade check!) | nonexistent pair → `mutation.target-missing` (💔disconnect-kind-compatibility/🔺️diff/🦀️.rs:7-9); **hostile vector**: disconnect a pair that existing edges currently rely on — the mutation succeeds with **no cascade/validation** against edges already using that pairing, unlike `delete-node`'s edge cascade |
| `replace-kind-catalogs` | Install/replace nakagin's full node/handle/edge/wire kind catalogs | oracle explicitly flags: *"the committed vector INSTALLS a catalogue where the before-snapshot had none, so undoing it requires removing the member, and nothing states whether the verb accepts a null argument."* Verified from the code itself (📚replace-kind-catalogs/🔺️diff/🦀️.rs, ↩️inverse/🦀️.rs) that a `None` `new_catalogs` **is** accepted (no guard) and inverse correctly restores `None` — so this ambiguity is resolvable purely by adding an explicit fixture, not a code fix |

General cross-cutting hostile patterns confirmed across the mutation family:
- **Missing target** → `protocol::MutationOutcome::error("mutation.target-missing", …)` for
  node/edge/kind-compatibility-addressed mutations (see ⚓change-node-anchor, 🗑️delete-node,
  🏗️change-node-kind, 💔disconnect-kind-compatibility diffs).
- **Duplicate id on create** → `protocol::MutationOutcome::fatal("mutation.duplicate-id", …)` (only
  `create-node`/`connect-handles`(warn, not fatal)/`connect-kind-compatibility`(warn) checked directly).
- **No-op** (new value equals old) → `protocol::MutationOutcome::new(default diff).absorb_messages([warn("mutation.no-op", …)])` — a genuinely different but functionally-equivalent value could also trip this, worth testing.
- **No kind-catalog referential integrity**: `change-node-kind`/`change-edge-kind` never check the new
  kind id against `meta.kindCatalogs` — any string is accepted.
- **No lock enforcement**: `locked` fields exist on nodes/edges/handles but are read by nothing except
  their own setter mutations.
- **`retained-jobs/🔣️.json`** (`🗄️retained-jobs/🔣️.json`) independently documents a `hostileSourceMutations`
  vector list for the bounded-first-step tool-command layer (not the document-mutation layer) —
  `forceLayoutOldReducer`, `*CursorRemoved` variants, `addNodeUnboundedReducer`, all `expected: "rejected"`
  — plus a `vectors` array of malformed/zero/oversize/stale-generation/cancel/fault probes for the
  `addNode`/`forceLayout`/`setActiveExample` retained tool commands specifically (distinct from, and a
  useful structural template for, the 26-mutation document-level hostile vectors above).

## 6. `💡️inferences` — what exists, what doesn't

`🧬️schema/💡️inferences/` is the **fourth schema family** alongside snapshot/diff/mutations (per
ticket `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`, doc comment at
`💡️inferences/🦀️.rs:1-10`). Directory shape mirrors `🧬️mutations/`: the family-root `🦀️.rs` is the
sole mounting point, each named inference gets its own `<emoji><slug>/` child.

**Puzzle2d has exactly ONE inference implemented: `flat-position`** (`💡️inferences/🎛️flat-position/`).
- `Puzzle2dInference { flat_position: Puzzle2dFlatPosition }` (`💡️inferences/🦀️.rs:24-27`),
  `impl protocol::Inference<Puzzle2dSnapshot>` computing it via `compute_flat_position`.
- `Puzzle2dFlatPosition { positions: BTreeMap<String, Puzzle2dFlatPositionXy{x,y}> }`
  (`🎛️flat-position/🦀️.rs:21-31`) — resolved `(x,y)` per node id after a BFS layout pass.
- Computed by re-running `fastened_layout_snapshot` (defined at the inference-family root,
  `💡️inferences/🦀️.rs:114-214`, rehomed from the deleted `⚙️engine/📐️layout`) on a snapshot clone:
  `Fixed`-anchor nodes keep stored coordinates; `Derived`-anchor nodes get theirs walked from the
  connecting edge's `gap/shift/rise/rotation/turn/tilt/x/y` params, reusing puzzle3d's
  `DIAGRAM_HORIZONTAL_SCALE`/`DIAGRAM_RADIUS` constants for parity (`💡️inferences/🦀️.rs:121`, imported
  from `crate::artifacts::puzzle3d::schema::inferences::flatten`).
- **Uncached by design**: `impl ArtifactInferrer for Puzzle2dBuilder` uses the default `infer_cached`
  passthrough — "no `InferredField`/incremental caching is needed here", explicitly because it's "a
  plain whole-snapshot BFS pass" (doc comment, `💡️inferences/🦀️.rs:48-51`), same rationale as jack's
  `🎛️flat-position`/`🧭️topology` and puzzle3d's own sibling.

**Relation to 3d/5d and the fill/suggestions precompute**: puzzle3d's own inference family
(`🧊️3d/…/🧬️schema/💡️inferences/`) has **two** named inferences — `📍️flat-position` (same concept,
puzzle2d's own name differs by emoji: `🎛️` vs `📍️`) **and** `🗜️flatten` (an extra one 2d doesn't have).
No `fill`/`suggestions`-precompute inference exists in puzzle2d's `💡️inferences/` family at all — the
`fill_count`/`brush_candidates_json`/`node_kind_weights_json`/`handle_kind_weights_json` fields visible
on `Puzzle2dArtifact` (`🧬️schema/🦀️.rs:41,55-57`) are `#[state(config)]` (ephemeral/session UI state),
not `#[derived]` inference-schema fields — so any fill/suggestion precompute for puzzle2d, if it
exists, lives in the editor/engine layer (`✏️editor/⚙️engine/🖌️brush/`, `✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/`
— seen in the file listing but not read in this pass), not in the schema/inference family that this
ticket's schema exploration covers.

## Other file-path notes

- `◻️2d/🏅️standards/🔖️1/🦀️.rs` — standard declaration (`terra-fleet-trinity-recipe` recipe), mounts
  subset `any`; documents `mimes: ["application/vnd.semio.puzzle2d+json"]` as a "documented synthesis"
  (no real MIME registration exists) vs. `extensions: ["puzzle2d-play"]` which is real
  (carried from `definition()`'s `s.puzzle2d.codec.document-1` capability row).
- `🚪️io/🦀️.rs` — registers import/export stdio kinds (`dwg,dxf,json,pdf,png,svg,txt`) and the
  `Puzzle2dComposerComposition` (`ArtifactComposition`) that dispatches to per-format
  deserializers under `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/<format>/…` for composing a
  `Puzzle2dSnapshot` from any supported source dialect.
- `🗄️retained-jobs/🔣️.json` — bounded first-step command fixture for exactly three tool ids:
  `addNode`, `forceLayout`, `setActiveExample` (not the 26 document mutations) — its own
  `hostileSourceMutations` and `vectors` arrays are a ready-made template for tool-command-layer
  hostile fixtures, separate from the document-mutation ones in §5.

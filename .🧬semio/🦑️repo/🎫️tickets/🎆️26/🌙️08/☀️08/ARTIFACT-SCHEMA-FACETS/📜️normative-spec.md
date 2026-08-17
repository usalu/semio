# 📜️ Normative Spec — Artifact Schema Facets

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. This document is the **only** contract every wave agent
reads. If disk and this document disagree, this document wins and the disk is a breach.

Greenfield rules apply: no legacy support, no adapters, no deprecations, no migration scripts, no
codegen. Every leaf listed here is **handcrafted**. Cross-format consistency is a *policy scanner +
runtime test* property, never a generator property.

---

## 1. Glossary

| Term | Definition |
| --- | --- |
| **Artifact** | The data of an app. Its *schema* is every field it has, whatever the field's durability. |
| **Snapshot** | All persisted data of one complete artifact, with no version history. Replaces the noun `Projection` everywhere except the `🛢️db/📽️projection` read-model module, which keeps the word for its own unrelated meaning. |
| **Diff** | Every change that can be applied to an artifact, as a sparse field delta. |
| **Mutation** | A declarative document change that constructs a Diff from its arguments and yields an Inverse. |
| **Facet** | One `🧬️schema` directory. There are exactly three per artifact: artifact-level, snapshot, diff. |
| **Format** | One of the five languages a facet is expressed in. Registered in `schemaFormats`. |
| **Leaf** | One file inside a facet: `<facet>/<formatLeafFilename>`. 3 facets × 5 formats = 15 leaves per artifact, 810 repo-wide. |
| **State class** | The kernel's `StateClass` — `Persistent`, `SharedUi`, `LocalUi`, `Preview`, `Effect` — declared per field in all five formats. |

---

## 2. Facet layout on disk

Per artifact under `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/`:

```
<artifact>/
  🦀️component.rs 🟦️component.ts        artifact root leaves (existing, keep)
  🧬️schema/                            NEW facet — every field, any state class
    🦀️component.rs
    🟦️component.ts
    🔗️component.graphql
    🔣️component.json
    🛰️component.proto
  📸️snapshot/                          NEW facet dir
    🧬️schema/                          NEW facet — persistent fields only
      🦀️component.rs 🟦️component.ts 🔗️component.graphql 🔣️component.json 🛰️component.proto
    🎒️pack/                            MOVED here verbatim from the artifact root
      📡️component.protocol.semio 🦀️component.rs 🟦️component.ts
  🔺️diff/                              KEPT — grammar + DiffCodec stay at its root
    📖️component.grammar.semio 🦀️component.rs 🟦️component.ts
    🧬️schema/                          NEW facet — every applicable change
      🦀️component.rs 🟦️component.ts 🔗️component.graphql 🔣️component.json 🛰️component.proto
  🧬️mutations/ 🗣️dsl/ 🔧️op/ 📡️spr/ ⚙️engine/ 📚️examples/    unchanged
```

`🎒️pack` moves under `📸️snapshot` because a pack encodes exactly the snapshot and nothing else.
No `🎒️pack` may remain directly under an artifact root — that is a policy breach.

The three facet paths, relative to the artifact root, are canonical and referred to below as
`artifact` / `snapshot` / `diff`:

```
🧬️schema
📸️snapshot/🧬️schema
🔺️diff/🧬️schema
```

---

## 3. `schemaFormats` registry

Added to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` as its **own
top-level key**. It is *not* an entry in `taxonomyLeafFilenames` and *not* an entry in `ecosystems`:
`validateTaxonomy` cross-asserts `taxonomyLeafFilenames[lang] === ecosystems[lang].leafFilename`, and
GraphQL / JSON Schema / Protobuf are wire formats, not package ecosystems.

```json
"schemaFormats": {
  "🦀️rust":       { "leafFilename": "🦀️component.rs",      "extension": ".rs",      "fieldCasing": "snake" },
  "🟦️typescript": { "leafFilename": "🟦️component.ts",      "extension": ".ts",      "fieldCasing": "camel" },
  "🔗️graphql":    { "leafFilename": "🔗️component.graphql", "extension": ".graphql", "fieldCasing": "camel" },
  "🔣️jsonschema": { "leafFilename": "🔣️component.json",    "extension": ".json",    "fieldCasing": "camel" },
  "🛰️protobuf":   { "leafFilename": "🛰️component.proto",   "extension": ".proto",   "fieldCasing": "snake" }
}
```

Adding a sixth format later is one JSON entry plus one field extractor in the policy region. That is
the whole extension point.

Emoji budget check (verified against every tracked path segment):

- `🛰` is unused repo-wide → free for protobuf.
- `🔗` already means GraphQL (`🔗️schema.graphql`, `🔗️graphql`).
- `🔣` already means JSON (`🔣️taxonomy.json`, `🔣️json`).
- `🧬` already prefixes semio data and `🧬️mutations`; `📸` already prefixes the `📸️remodel` plugin.
  Reuse at a different namespace level is the established convention — exactly as `🔺️` serves both
  the `🔺️diff` facet and the diff example-asset prefix.

### Normative format

Within a facet the **`🔣️component.json` JSON Schema leaf is normative**; the other four are mirrors
of it. When a field's shape is ambiguous, the JSON Schema decides.

Taxonomy carries this as a new key (it cannot go into `artifactSpecFilenames`, whose values
`validateTaxonomy` requires to end in `.semio`):

```json
"artifactSchemaSpecFilenames": {
  "🧬️schema": "🔣️component.json",
  "📸️snapshot/🧬️schema": "🔣️component.json",
  "🔺️diff/🧬️schema": "🔣️component.json"
}
```

---

## 4. Type naming

Per artifact with prefix `X` (table in §10):

| Facet | Type name |
| --- | --- |
| `🧬️schema` | `XArtifact` |
| `📸️snapshot/🧬️schema` | `XSnapshot` |
| `🔺️diff/🧬️schema` | `XDiff` |

`XSnapshot` **replaces** today's heterogeneous snapshot names — `XProjection`, `XDocument`,
`XFixture`, `XSpec`, `XDefinition`, `XDeck`, `Program`, `Document`. The bare `Document` that all
fifteen norm artifacts share today is the clearest reason to do this: after the rename each norm
artifact has a unique, greppable snapshot type.

The same three names appear verbatim in all five leaves of the facet — Rust `struct`, TS `interface`,
GraphQL `type`, JSON Schema `title`, proto `message`. Nested helper types keep the `X` prefix
(`XObject`, `XPaintLayer`, `XObjectPatch`) and are declared in whichever facet owns them, re-exported
by the others.

### Proto package and JSON Schema `$id`

```
package semio.s.<plugin_key>.<artifact_key>.<facet>;
```

where `<facet>` is `artifact` | `snapshot` | `diff`, e.g. `semio.s.lowpoly.lowpoly.snapshot`.

```
"$id": "https://semio.tech/schema/s/<plugin_key>/<artifact_key>/<facet>.json"
```

---

## 5. State classes

`StateClass` is the kernel enum at
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs` (region `🔖️StateClass`).
Every field of every facet declares exactly one, in the format's own idiom:

| Format | Annotation | Values |
| --- | --- | --- |
| JSON Schema (normative) | `"x-semio-state": "persistent"` on the property | `persistent`, `shared-ui`, `local-ui`, `preview`, `effect` |
| Rust | `#[state(persistent)]` field attribute, read by the `schema::ArtifactSchema` derive | `persistent`, `shared_ui`, `local_ui`, `preview`, `effect` |
| TypeScript | `/** @state persistent */` JSDoc immediately above the property | kebab, as JSON |
| GraphQL | `@state(class: PERSISTENT)` on the field | `PERSISTENT`, `SHARED_UI`, `LOCAL_UI`, `PREVIEW`, `EFFECT` |
| Protobuf | `// @state persistent` leading comment on the field | kebab, as JSON |

The GraphQL directive is declared once, in a shared SDL preamble owned by the framework `🧬️schema`
module, never repeated per artifact:

```graphql
enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT }
directive @state(class: StateClass!) on FIELD_DEFINITION
```

### Where the fields come from

The artifact schema is the **union** of what is spread across three places today plus engine-derived
values:

| Source today | State class |
| --- | --- |
| the persisted projection / document / fixture | `Persistent` |
| `DocumentApp::Config` fields shared between collaborators | `SharedUi` |
| `DocumentApp::Config` fields private to one client (camera, chrome, tool options) | `LocalUi` |
| `DocumentApp::Draft` — in-flight gesture state | `Preview` |
| fire-and-forget engine outputs (exports, toasts, media writes) | `Effect` |

Lowpoly is the worked example: its artifact leaf documents that "active object, selection, utilities,
camera, brush live in the plugin's app config, never here". Those become `SharedUi`/`LocalUi` fields
of `LowpolyArtifact`; `schema` and `objects` stay `Persistent` and are the entire `LowpolySnapshot`.

---

## 6. Field casing and type mapping

Canonical field identity is the **camelCase JSON Schema property name**. Per format:

- `🔣️jsonschema`, `🟦️typescript`, `🔗️graphql` → camelCase, identical spelling.
- `🦀️rust`, `🛰️protobuf` → `snake_case` of the canonical name. Rust structs carry
  `#[serde(rename_all = "camelCase")]` so the wire form stays canonical.

Scalar mapping table — all five columns are required to agree:

| Canonical | JSON Schema | Rust | TypeScript | GraphQL | Protobuf |
| --- | --- | --- | --- | --- | --- |
| string | `{"type":"string"}` | `String` | `string` | `String` | `string` |
| bool | `{"type":"boolean"}` | `bool` | `boolean` | `Boolean` | `bool` |
| int32 | `{"type":"integer","format":"int32"}` | `i32` | `number` | `Int` | `int32` |
| uint32 | `{"type":"integer","format":"uint32","minimum":0}` | `u32` | `number` | `Int` | `uint32` |
| int64 | `{"type":"integer","format":"int64"}` | `i64` | `number` | `Int` | `int64` |
| float32 | `{"type":"number","format":"float"}` | `f32` | `number` | `Float` | `float` |
| float64 | `{"type":"number","format":"double"}` | `f64` | `number` | `Float` | `double` |
| bytes | `{"type":"string","contentEncoding":"base64"}` | `Vec<u8>` | `string` | `String` | `bytes` |
| json blob | `{"type":"string","contentMediaType":"application/json"}` | `String` | `string` | `String` | `string` |

Cardinality mapping — this is what "identical optionality and cardinality" means to the scanner:

| Canonical | JSON Schema | Rust | TypeScript | GraphQL | Protobuf |
| --- | --- | --- | --- | --- | --- |
| required scalar `T` | listed in `required` | `T` | `f: T` | `f: T!` | `T f = n;` |
| optional scalar `T` | absent from `required` | `Option<T>` | `f?: T` | `f: T` | `optional T f = n;` |
| list of `T` | `{"type":"array","items":T}` | `Vec<T>` | `f: T[]` | `f: [T!]!` | `repeated T f = n;` |
| fixed list of `T`×k | `array` + `minItems`/`maxItems` = k | `[T; k]` | tuple `f: [T, …]` | `f: [T!]!` | `repeated T f = n;` |
| map `string → T` | `{"type":"object","additionalProperties":T}` | `BTreeMap<String, T>` | `Record<string, T>` | `f: [XEntry!]!` | `map<string, T> f = n;` |

Proto field numbers are assigned in canonical field order starting at `1`, never reused, never
reordered. That ordering is also the `📡️component.protocol.semio` field order for the pack — the
proto leaf and the pack protocol must not disagree.

---

## 7. Facet contents

### 7.1 `🧬️schema` — `XArtifact`

Every field the artifact has, each with a state class. Rust:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schema::ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub objects: Vec<LowpolyObject>,
    #[state(shared_ui)]  pub active_object_id: Option<String>,
    #[state(local_ui)]   pub selection: LowpolySelection,
}
```

The derive emits `fn field_states() -> &'static [(&'static str, StateClass)]` keyed by canonical
camelCase names, and `fn artifact_schema_id() -> &'static str`.

### 7.2 `📸️snapshot/🧬️schema` — `XSnapshot`

Exactly the `Persistent` fields of `XArtifact`, same names, same types, same cardinality, same order.
Not a subset "roughly" — the policy scanner compares the two sets for equality. `XSnapshot` is the
type that `📸️snapshot/🎒️pack`, `🗣️dsl` and the `store::Document*` traits operate on, and the type
`ArtifactEngine::Snapshot` / `DocumentApp::Snapshot` resolve to.

### 7.3 `🔺️diff/🧬️schema` — `XDiff`

A **sparse field delta**, not a list of mutations. One entry per mutable artifact field, absent
meaning "unchanged":

```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, schema::ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyDiff {
    #[state(persistent)] pub artifact: Option<Box<LowpolyArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub objects: Option<LowpolyObjectsDelta>,
    #[state(shared_ui)]  pub active_object_id: Option<Option<String>>,
}
```

Rules:

- Field `f` of state class other than `effect` **must** have a diff entry. `effect` fields must not.
  That is the "diff coverage" scanner.
- The diff entry's name equals the artifact field's canonical name.
- Scalar/record field `f: T` → `f: Option<T>`. A nullable field `f: Option<T>` → `f: Option<Option<T>>`
  so "set to null" is expressible; JSON Schema encodes that as `{"oneOf":[{"type":"null"}, T]}`.
- Collection field `f: Vec<T>` where `T: Identified` → `f: Option<XFDelta>` with
  `{ added: Vec<T>, removed: Vec<Id>, patched: Vec<(Id, TPatch)>, reordered: Option<Vec<Id>> }`.
- Map field → `f: Option<BTreeMap<K, Option<V>>>`, inner `None` meaning "remove key".
- One extra field `artifact: Option<Box<XArtifact>>` for whole-artifact replacement. When present it
  wins over every other entry. (Puzzle's `document` and cad's `scene` fields are this, renamed.)

`XDiff` implements the kernel's `MutationDiff<XSnapshot>` by applying **only** its `persistent`
entries, and additionally exposes `fn apply_to_artifact(&self, artifact: &XArtifact) -> XArtifact`
which applies all of them, so the `ArtifactEngine` can drive full artifact state from one diff type.
`absorb` merges field-wise: later `Some` wins, collection deltas concatenate, a later `artifact`
replacement clears everything before it.

---

## 8. Kernel changes

Inside `semio-framework-os-kernel` only (W3 owns this; W6 sweeps the rest):

- `DocumentApp::Projection` → `DocumentApp::Snapshot`.
- `ArtifactEngine::Projection` → `ArtifactEngine::Snapshot`, and `fn projection()` → `fn snapshot()`.
- `ArtifactEngine` gains `type Artifact` and `fn artifact(&self) -> &Self::Artifact`. The engine owns
  the full artifact state; `snapshot()` returns its persisted part.
- `DocumentStore<P, Mutation>` type parameter renamed, `DocumentVcs::initial_projection` →
  `initial_snapshot`, `Mutation<P>` / `MutationDiff<P>` docs reworded.
- `🛢️db/📽️projection` keeps the word `Projection`. It is a read-model, a different concept, and the
  rename is precisely what disambiguates the two.

## 9. Framework `🧬️schema` module

`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` already owns a `SchemaCatalog` over `schemars` +
`jsonschema`. It gains, in new regions of the **existing** files:

- `ArtifactSchemaDescriptor { id, artifact: FacetLeaves, snapshot: FacetLeaves, diff: FacetLeaves }`
  where `FacetLeaves` holds the five `include_str!`'d leaf bodies.
- `ArtifactSchemaRegistry` with `register(descriptor)` / `iter()`.
- The `ArtifactSchema` derive macro (`#[state(..)]`, `#[artifact_schema(id = ..)]`).
- The shared GraphQL `@state` preamble constant from §5.
- A TypeScript twin exporting the same descriptor shape.
- **One table-driven test in the existing test region**: for every registered artifact, serialise
  `XSnapshot::default()`, validate it against that artifact's snapshot `🔣️component.json`, and assert
  `XSnapshot::field_states()` matches the `x-semio-state` values in the JSON. No new test file. This
  is the runtime proof; the scanners are the static proof.

---

## 10. Artifact table — keys, prefixes, current snapshot type

`key` is the Rust module name under `crate::artifacts::`. `prefix` is `X` in §4. `current` is the type
that `XSnapshot` replaces.

| plugin / artifact | key | prefix | current snapshot type |
| --- | --- | --- | --- |
| ✒️writer / ✒️writer | `writer` | `Writer` | `WriterProjection` |
| ➗️mathematical / ➗️mathematical | `mathematical` | `Mathematical` | `MathProjection` |
| 🌀️procedural / 🌀️procedural2d | `procedural2d` | `Procedural2d` | `Procedural2dDocument` |
| 🌀️procedural / 🧊️procedural3d | `procedural3d` | `Procedural3d` | `Procedural3dDocument` |
| 🌊️flow / 🌊️flow | `flow` | `Flow` | `FlowFixture` |
| 🌍️gis / 🏔️gisterrain | `gisterrain` | `GisTerrain` | `Gis3dTerrainDocument` |
| 🌍️gis / 🗺️gismap | `gismap` | `GisMap` | `GisMapDocument` |
| 🌿️vcs / 🌿️vcs | `vcs` | `Vcs` | `VcsDemoProjection` |
| 🎞️animate / 🎬️present | `present` | `Present` | `PresentDeck` |
| 🎥️shooting / 🎥️shooting | `shooting` | `Shooting` | `ShootingFixture` |
| 🎪️demonstrator / 🎪️playground | `playground` | `Playground` | `PlaygroundDocument` |
| 🎬️sequence / 🎬️sequence | `sequence` | `Sequence` | `SequenceFixture` |
| 🏗️fem / ◻2d | `fem2d` | `Fem2d` | `Fem2dDocument` |
| 🏗️fem / 🧊️3d | `fem3d` | `Fem3d` | `Fem3dDocument` |
| 🏛️architect / 🏛️program | `program` | `Program` | `Program` |
| 🏭️process / 🧊️process3d | `process3d` | `Process3d` | `Process3dDocument` |
| 💠️lowpoly / 💠️lowpoly | `lowpoly` | `Lowpoly` | `LowpolyProjection` |
| 💡️reasoning / 🔌️wires | `wires` | `Wires` | `MindmapWiresDocument` |
| 📋️forms / 📋️forms | `forms` | `Forms` | `FormSpec` |
| 📏️layout / 📏️layout | `layout` | `Layout` | `LayoutDocument` |
| 📐️cad / 📐️cad | `cad` | `Cad` | `CadProjection` |
| 📕️norm / 📓️iso16757 | `iso16757` | `Iso16757` | `Document` |
| 📕️norm / 📔️vdi3805 | `vdi3805` | `Vdi3805` | `Document` |
| 📕️norm / 📕️din4108 | `din4108` | `Din4108` | `Document` |
| 📕️norm / 📗️din16798 | `din16798` | `Din16798` | `Document` |
| 📕️norm / 📘️en1990 … 📘️en1999 | `en1990`…`en1999` | `En1990`…`En1999` | `Document` |
| 📕️norm / 📙️din18599 | `din18599` | `Din18599` | `Document` |
| 📖️playbook / 📖️playbook | `playbook` | `Playbook` | `PlaybookSpec` |
| 📜️imperative / 📜️imperative | `imperative` | `Imperative` | `ImperativeDocument` |
| 📸️remodel / 📸️remodel | `remodel` | `Remodel` | `RemodelProjection` |
| 🔋️energy / 🔋️model | `model` | `EnergyModel` | `EnergyModelDocument` |
| 🔱️trinity / ♻️rewrite | `rewrite` | `Rewrite` | `RewriteRuleDocument` |
| 🔱️trinity / 🔌️jack | `jack` | `Jack` | `TrinityGraphDocument` |
| 🕸️dag / 🕸️dag | `dag` | `Dag` | `DagDocument` |
| 🖍️draw / 🖍️draw | `draw` | `Draw` | `DrawDocument` |
| 🖨️raster / 🖨️raster | `raster` | `Raster` | `RasterProjection` |
| 🗒️note / 🗒️note | `note` | `Note` | `NoteDocument` |
| 🧩️puzzle / ◻2d | `puzzle2d` | `Puzzle2d` | `Puzzle2dProjection` |
| 🧩️puzzle / 🖐️5d | `puzzle5d` | `Puzzle5d` | `Puzzle5dPlayProjection` |
| 🧩️puzzle / 🧊️3d | `puzzle3d` | `Puzzle3d` | `Puzzle3dProjection` |
| 🧱️block / ◻2d | `block2d` | `Block2d` | `Block2dDefinition` |
| 🧱️block / 🖐️5d | `block5d` | `Block5d` | `Block5dDefinition` |
| 🧱️block / 🧊️3d | `block3d` | `Block3d` | `Block3dDefinition` |
| 🪐️space / 🏠️home | `home` | `SHome` | `SHomeDocument` |
| 🪵️sourcing / 🗂️curate | `curate` | `Curate` | `SourcingDocument` |

54 artifacts. `🎪️playground` and `🔋️model` are the two whose facet set is incomplete today and must
be brought to the full shape rather than merely renamed.

---

## 11. Policy rules (root `📜️script.ts`, region `🔧️PolicyRuleArtifactSchemas`)

Modelled on `🔧️PolicyRuleMutationArtifactEngines`: plain `policy*Breaches(repoRoot)` functions (there
are no `PolicyRule` classes) returning `BreachRecord[]` with `id` / `summary` / `kind` / `scope` /
`priority` / `reason` / `solution`, aggregated by `policyArtifactSchemaBreaches(repoRoot)`, registered
in `export const policy` and hooked into `VerifyScript.runGate()` next to
`policyHandcraftedSpecP3Breaches`.

There is deliberately **no new nx target and no launch entry**: `policy` is not a root
`📋️project.json` target, it is the synthetic `breach-script_ts:lint` project that `🟨️nx-plugin.mjs`
derives from `export const policy`, and `.vscode/launch.json` carries zero verification entries by
design.

| Rule | Assertion |
| --- | --- |
| facet completeness | all three facet dirs exist, each holding all five `schemaFormats` leaves |
| field parity | all five leaves of one facet declare the identical canonical field set, with identical optionality and cardinality per §6 |
| state-class parity | the snapshot facet's field set equals exactly the `persistent` fields of the artifact facet |
| diff coverage | every non-`effect` artifact field has a diff entry; no `effect` field does; plus the `artifact` replacement entry exists |
| type-name parity | `XArtifact` / `XSnapshot` / `XDiff` spelled identically in all five leaves of their facet |
| pack relocation | no `🎒️pack` directly under an artifact root |
| normative leaf | each facet carries the `artifactSchemaSpecFilenames` leaf |

Field extractors, one per format: Rust `pub` struct fields, TS interface members, GraphQL type fields,
JSON Schema `properties`, proto message fields. Each normalises to the canonical camelCase name plus
`{ optional, cardinality, scalar }` so the five sets are directly comparable. These extractors *are*
the compiler this design does not have; they must be strict, not heuristic.

Two existing walkers assume artifact children are **flat** and need one more level, driven by the new
`snapshotChildDirs` / `diffChildDirs`: `validateTaxonomyTree` in the plugin registry script and
`policyTaxonomyDirsBreaches` in root `📜️script.ts`.

---

## 12. Taxonomy diff (W1)

```
artifactComponentDirs   += 🧬️schema, 📸️snapshot     −= 🎒️pack   (now nested)
artifactChildDirs       += 🧬️schema, 📸️snapshot     −= 🎒️pack
snapshotChildDirs        = ["🧬️schema", "🎒️pack"]              NEW, mirrors mutationChildDirs
diffChildDirs            = ["🧬️schema"]                        NEW
taxonomyLeafParentDirs  += 🧬️schema
artifactSpecFilenames    : rekey "🎒️pack" → "📸️snapshot/🎒️pack"
artifactSchemaSpecFilenames                                    NEW, §3
schemaFormats                                                  NEW, §3
```

Four consumers must move with it:

1. `validateTaxonomy` in the discovery library `🟦️component.ts` — new clauses for `schemaFormats`,
   `snapshotChildDirs`, `diffChildDirs`, `artifactSchemaSpecFilenames`, mirroring the existing
   `mutationChildDirs` clause.
2. `validateTaxonomyTree` in the plugin registry `📜️script.ts` — nested walk.
3. `policyTaxonomyDirsBreaches` in root `📜️script.ts` — nested walk (W2 owns this edit).
4. `assert_taxonomy_components` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
   — currently **hardcoded and already stale** (its `EXAMPLE_KINDS` still names the plural dirs
   `🗣️dsls` / `🎒️packs` that `forbiddenExamplePluralDirs` now bans). Convert it to read the taxonomy
   JSON rather than adding a seventh hardcoded entry.

---

## 13. Per-plugin wiring

Each plugin's `📦️packages/🦀️rust/📦️glue.rs`:

- re-`#[path]`s `artifacts::<key>::pack` to `artifacts::<key>::snapshot::pack`,
- mounts `artifacts::<key>::schema`, `artifacts::<key>::snapshot::schema`,
  `artifacts::<key>::diff::schema`.

`📦️packages/🟦️typescript/📦️index.ts` mirrors it. `rustEntryPathRules` in the taxonomy documents that
`#[path]` resolution is **cumulative**; the new nesting adds one `../` level inside
`pub mod snapshot { … }`. Each fan-out agent owns exactly one plugin's glue file, so this is
conflict-free — except where a crate is split across agents, in which case the W5 glue integrator
makes the single edit at the end of the wave.

---

## 14. Wave protocol

- Every wave writes `🧪wave<N>-report.md` into this ticket folder: what it changed, which gate it ran,
  the gate's verbatim tail, and anything it deliberately left for a later wave.
- Ownership is disjoint **by tree**, never by feature across trees. There is no repo-enforced file
  lock; ownership is convention plus these reports.
- No modifying git commands, ever. Edit existing files; never create a parallel "fixed" copy.
- Temporary probes, logs and dumps go in this ticket folder and stay there.
- macOS Rust link steps need `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
- W4's finished lowpoly leaves are appended to this document verbatim as §15. Fan-out agents diff
  against §15 rather than improvising.

---

## 15. Pilot leaves — lowpoly, verbatim
Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS` wave W4. Fan-out agents MUST diff their leaves against this section.
Key `lowpoly`, prefix `Lowpoly`, schema id `s.lowpoly.lowpoly`.

### 15.1 `artifact` facet

#### `artifact/🦀️component.rs`

```rust
//! 🧬️ Lowpoly artifact schema — every field of the artifact with its state class.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolySelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full lowpoly artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub objects: Vec<LowpolyObject>,
    #[state(shared_ui)] pub active_object_id: Option<String>,
    #[state(shared_ui)] pub selection: LowpolySelection,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub paint_utility: String,
    #[state(shared_ui)] pub active_paint_layer: u32,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub show_edges: bool,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub world_camera_position_x: f64,
    #[state(local_ui)] pub world_camera_position_y: f64,
    #[state(local_ui)] pub world_camera_position_z: f64,
    #[state(local_ui)] pub world_camera_target_x: f64,
    #[state(local_ui)] pub world_camera_target_y: f64,
    #[state(local_ui)] pub world_camera_target_z: f64,
    #[state(local_ui)] pub world_camera_fov: f64,
    #[state(local_ui)] pub utility_params_json: String,
    #[state(local_ui)] pub paint_color_r: u32,
    #[state(local_ui)] pub paint_color_g: u32,
    #[state(local_ui)] pub paint_color_b: u32,
    #[state(local_ui)] pub paint_color_a: u32,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_object_id: Option<String>,
    #[state(preview)] pub hovered_target_object_id: Option<String>,
    #[state(preview)] pub hovered_target_mode: Option<String>,
    #[state(preview)] pub hovered_target_id: Option<u32>,
    #[state(preview)] pub stroke_drag_active: bool,
    #[state(preview)] pub transform_drag_active: bool,
    #[state(preview)] pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LowpolyArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA.into(),
            objects: Vec::new(),
            active_object_id: None,
            selection: crate::artifacts::lowpoly::LowpolySelection::default(),
            selected_object_ids: Vec::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            active_utility_id: "move".into(),
            show_edges: true,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            world_camera_position_x: 18.0,
            world_camera_position_y: -18.0,
            world_camera_position_z: 12.0,
            world_camera_target_x: 0.0,
            world_camera_target_y: 0.0,
            world_camera_target_z: 0.0,
            world_camera_fov: 45.0,
            utility_params_json: String::new(),
            paint_color_r: 255,
            paint_color_g: 64,
            paint_color_b: 64,
            paint_color_a: 255,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            engagement_input: String::new(),
            locale: "en-US".into(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
            stroke_drag_active: false,
            transform_drag_active: false,
            preview_seq: 0,
        }
    }
}

impl LowpolyArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::lowpoly::LowpolySnapshot {
        crate::artifacts::lowpoly::LowpolySnapshot {
            schema: self.schema.clone(),
            objects: self.objects.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::lowpoly::LowpolySnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            objects: snapshot.objects,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::lowpoly::LowpolySnapshot) {
        self.schema = snapshot.schema;
        self.objects = snapshot.objects;
    }
}
//#endregion 🔖️Conversions


//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.lowpoly.lowpoly` — fifteen handcrafted schema leaves.
pub fn lowpoly_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
```

#### `artifact/🟦️component.ts`

```typescript
/** 🧬️ Lowpoly artifact schema — every field with its state class. */

export interface LowpolyArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  objects: LowpolyObject[];
  /** @state shared-ui */
  activeObjectId?: string;
  /** @state shared-ui */
  selection: LowpolySelection;
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  paintUtility: string;
  /** @state shared-ui */
  activePaintLayer: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  showEdges: boolean;
  /** @state local-ui */
  sunEnabled: boolean;
  /** @state local-ui */
  sunAzimuth: number;
  /** @state local-ui */
  sunElevation: number;
  /** @state local-ui */
  sunIntensity: number;
  /** @state local-ui */
  sunColor: string;
  /** @state local-ui */
  worldCameraPositionX: number;
  /** @state local-ui */
  worldCameraPositionY: number;
  /** @state local-ui */
  worldCameraPositionZ: number;
  /** @state local-ui */
  worldCameraTargetX: number;
  /** @state local-ui */
  worldCameraTargetY: number;
  /** @state local-ui */
  worldCameraTargetZ: number;
  /** @state local-ui */
  worldCameraFov: number;
  /** @state local-ui */
  utilityParamsJson: string;
  /** @state local-ui */
  paintColorR: number;
  /** @state local-ui */
  paintColorG: number;
  /** @state local-ui */
  paintColorB: number;
  /** @state local-ui */
  paintColorA: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionModeDefault: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredObjectId?: string;
  /** @state preview */
  hoveredTargetObjectId?: string;
  /** @state preview */
  hoveredTargetMode?: string;
  /** @state preview */
  hoveredTargetId?: number;
  /** @state preview */
  strokeDragActive: boolean;
  /** @state preview */
  transformDragActive: boolean;
  /** @state preview */
  previewSeq: number;
}

export interface LowpolySelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}

export interface LowpolySelection {
  targets: LowpolySelectionTargets;
  keys: string[];
  mode: string;
  ids: number[];
}

export interface LowpolyTransform {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface LowpolyPaintLayer {
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  pixels: string;
}

export interface LowpolyObject {
  id: string;
  name: string;
  transform: LowpolyTransform;
  smoothShading: boolean;
  meshJson: string;
  paintLayers: LowpolyPaintLayer[];
}
```

#### `artifact/🔗️component.graphql`

```graphql
# 🧬️ Lowpoly artifact schema — every field with its state class.

type LowpolyArtifact {
  schema: String! @state(class: PERSISTENT)
  objects: [LowpolyObject!]! @state(class: PERSISTENT)
  activeObjectId: String @state(class: SHARED_UI)
  selection: LowpolySelection! @state(class: SHARED_UI)
  selectedObjectIds: [String!]! @state(class: SHARED_UI)
  paintUtility: String! @state(class: SHARED_UI)
  activePaintLayer: Int! @state(class: SHARED_UI)
  activeUtilityId: String! @state(class: SHARED_UI)
  showEdges: Boolean! @state(class: LOCAL_UI)
  sunEnabled: Boolean! @state(class: LOCAL_UI)
  sunAzimuth: Float! @state(class: LOCAL_UI)
  sunElevation: Float! @state(class: LOCAL_UI)
  sunIntensity: Float! @state(class: LOCAL_UI)
  sunColor: String! @state(class: LOCAL_UI)
  worldCameraPositionX: Float! @state(class: LOCAL_UI)
  worldCameraPositionY: Float! @state(class: LOCAL_UI)
  worldCameraPositionZ: Float! @state(class: LOCAL_UI)
  worldCameraTargetX: Float! @state(class: LOCAL_UI)
  worldCameraTargetY: Float! @state(class: LOCAL_UI)
  worldCameraTargetZ: Float! @state(class: LOCAL_UI)
  worldCameraFov: Float! @state(class: LOCAL_UI)
  utilityParamsJson: String! @state(class: LOCAL_UI)
  paintColorR: Int! @state(class: LOCAL_UI)
  paintColorG: Int! @state(class: LOCAL_UI)
  paintColorB: Int! @state(class: LOCAL_UI)
  paintColorA: Int! @state(class: LOCAL_UI)
  selectionMethod: String! @state(class: LOCAL_UI)
  selectionModeDefault: String! @state(class: LOCAL_UI)
  engagementInput: String! @state(class: LOCAL_UI)
  locale: String! @state(class: LOCAL_UI)
  hoveredObjectId: String @state(class: PREVIEW)
  hoveredTargetObjectId: String @state(class: PREVIEW)
  hoveredTargetMode: String @state(class: PREVIEW)
  hoveredTargetId: Int @state(class: PREVIEW)
  strokeDragActive: Boolean! @state(class: PREVIEW)
  transformDragActive: Boolean! @state(class: PREVIEW)
  previewSeq: Int! @state(class: PREVIEW)
}

type LowpolySelectionTargets {
  mesh: Boolean!
  vertex: Boolean!
  edge: Boolean!
  face: Boolean!
}

type LowpolySelection {
  targets: LowpolySelectionTargets!
  keys: [String!]!
  mode: String!
  ids: [Int!]!
}

type LowpolyTransform {
  position: [Float!]!
  rotation: [Float!]!
  scale: [Float!]!
}

type LowpolyPaintLayer {
  name: String!
  visible: Boolean!
  opacity: Float!
  blendMode: String!
  pixels: String!
}

type LowpolyObject {
  id: String!
  name: String!
  transform: LowpolyTransform!
  smoothShading: Boolean!
  meshJson: String!
  paintLayers: [LowpolyPaintLayer!]!
}
```

#### `artifact/🔣️component.json`

```json
{
  "$id": "https://semio.tech/schema/s/lowpoly/lowpoly/artifact.json",
  "title": "LowpolyArtifact",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema",
    "objects",
    "selection",
    "selectedObjectIds",
    "paintUtility",
    "activePaintLayer",
    "activeUtilityId",
    "showEdges",
    "sunEnabled",
    "sunAzimuth",
    "sunElevation",
    "sunIntensity",
    "sunColor",
    "worldCameraPositionX",
    "worldCameraPositionY",
    "worldCameraPositionZ",
    "worldCameraTargetX",
    "worldCameraTargetY",
    "worldCameraTargetZ",
    "worldCameraFov",
    "utilityParamsJson",
    "paintColorR",
    "paintColorG",
    "paintColorB",
    "paintColorA",
    "selectionMethod",
    "selectionModeDefault",
    "engagementInput",
    "locale",
    "strokeDragActive",
    "transformDragActive",
    "previewSeq"
  ],
  "properties": {
    "schema": {
      "type": "string",
      "x-semio-state": "persistent"
    },
    "objects": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/LowpolyObject"
      },
      "x-semio-state": "persistent"
    },
    "activeObjectId": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "selection": {
      "$ref": "#/$defs/LowpolySelection",
      "x-semio-state": "shared-ui"
    },
    "selectedObjectIds": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "x-semio-state": "shared-ui"
    },
    "paintUtility": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "activePaintLayer": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "shared-ui"
    },
    "activeUtilityId": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "showEdges": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "sunEnabled": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "sunAzimuth": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "sunElevation": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "sunIntensity": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "sunColor": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "worldCameraPositionX": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraPositionY": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraPositionZ": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraTargetX": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraTargetY": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraTargetZ": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraFov": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "utilityParamsJson": {
      "type": "string",
      "contentMediaType": "application/json",
      "x-semio-state": "local-ui"
    },
    "paintColorR": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "paintColorG": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "paintColorB": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "paintColorA": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "selectionMethod": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "selectionModeDefault": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "engagementInput": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "locale": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "hoveredObjectId": {
      "type": "string",
      "x-semio-state": "preview"
    },
    "hoveredTargetObjectId": {
      "type": "string",
      "x-semio-state": "preview"
    },
    "hoveredTargetMode": {
      "type": "string",
      "x-semio-state": "preview"
    },
    "hoveredTargetId": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "preview"
    },
    "strokeDragActive": {
      "type": "boolean",
      "x-semio-state": "preview"
    },
    "transformDragActive": {
      "type": "boolean",
      "x-semio-state": "preview"
    },
    "previewSeq": {
      "type": "integer",
      "format": "int64",
      "x-semio-state": "preview"
    }
  },
  "$defs": {
    "LowpolyObject": {
      "title": "LowpolyObject",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id",
        "name",
        "transform",
        "smoothShading",
        "meshJson",
        "paintLayers"
      ],
      "properties": {
        "id": {
          "type": "string"
        },
        "name": {
          "type": "string"
        },
        "transform": {
          "$ref": "#/$defs/LowpolyTransform"
        },
        "smoothShading": {
          "type": "boolean"
        },
        "meshJson": {
          "type": "string",
          "contentMediaType": "application/json"
        },
        "paintLayers": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LowpolyPaintLayer"
          }
        }
      }
    },
    "LowpolyTransform": {
      "title": "LowpolyTransform",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "position",
        "rotation",
        "scale"
      ],
      "properties": {
        "position": {
          "type": "array",
          "items": {
            "type": "number",
            "format": "float"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "rotation": {
          "type": "array",
          "items": {
            "type": "number",
            "format": "float"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "scale": {
          "type": "array",
          "items": {
            "type": "number",
            "format": "float"
          },
          "minItems": 3,
          "maxItems": 3
        }
      }
    },
    "LowpolyPaintLayer": {
      "title": "LowpolyPaintLayer",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "name",
        "visible",
        "opacity",
        "blendMode",
        "pixels"
      ],
      "properties": {
        "name": {
          "type": "string"
        },
        "visible": {
          "type": "boolean"
        },
        "opacity": {
          "type": "number",
          "format": "float"
        },
        "blendMode": {
          "type": "string"
        },
        "pixels": {
          "type": "string",
          "contentEncoding": "base64"
        }
      }
    },
    "LowpolySelection": {
      "title": "LowpolySelection",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "targets",
        "keys",
        "mode",
        "ids"
      ],
      "properties": {
        "targets": {
          "$ref": "#/$defs/LowpolySelectionTargets"
        },
        "keys": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "mode": {
          "type": "string"
        },
        "ids": {
          "type": "array",
          "items": {
            "type": "integer",
            "format": "uint32",
            "minimum": 0
          }
        }
      }
    },
    "LowpolySelectionTargets": {
      "title": "LowpolySelectionTargets",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "mesh",
        "vertex",
        "edge",
        "face"
      ],
      "properties": {
        "mesh": {
          "type": "boolean"
        },
        "vertex": {
          "type": "boolean"
        },
        "edge": {
          "type": "boolean"
        },
        "face": {
          "type": "boolean"
        }
      }
    }
  }
}
```

#### `artifact/🛰️component.proto`

```protobuf
syntax = "proto3";
package semio.s.lowpoly.lowpoly.artifact;

// 🧬️ Lowpoly artifact schema — every field with its state class.

message LowpolyArtifact {
  // @state persistent
  string schema = 1;
  // @state persistent
  repeated LowpolyObject objects = 2;
  // @state shared-ui
  optional string active_object_id = 3;
  // @state shared-ui
  LowpolySelection selection = 4;
  // @state shared-ui
  repeated string selected_object_ids = 5;
  // @state shared-ui
  string paint_utility = 6;
  // @state shared-ui
  uint32 active_paint_layer = 7;
  // @state shared-ui
  string active_utility_id = 8;
  // @state local-ui
  bool show_edges = 9;
  // @state local-ui
  bool sun_enabled = 10;
  // @state local-ui
  double sun_azimuth = 11;
  // @state local-ui
  double sun_elevation = 12;
  // @state local-ui
  double sun_intensity = 13;
  // @state local-ui
  string sun_color = 14;
  // @state local-ui
  double world_camera_position_x = 15;
  // @state local-ui
  double world_camera_position_y = 16;
  // @state local-ui
  double world_camera_position_z = 17;
  // @state local-ui
  double world_camera_target_x = 18;
  // @state local-ui
  double world_camera_target_y = 19;
  // @state local-ui
  double world_camera_target_z = 20;
  // @state local-ui
  double world_camera_fov = 21;
  // @state local-ui
  string utility_params_json = 22;
  // @state local-ui
  uint32 paint_color_r = 23;
  // @state local-ui
  uint32 paint_color_g = 24;
  // @state local-ui
  uint32 paint_color_b = 25;
  // @state local-ui
  uint32 paint_color_a = 26;
  // @state local-ui
  string selection_method = 27;
  // @state local-ui
  string selection_mode_default = 28;
  // @state local-ui
  string engagement_input = 29;
  // @state local-ui
  string locale = 30;
  // @state preview
  optional string hovered_object_id = 31;
  // @state preview
  optional string hovered_target_object_id = 32;
  // @state preview
  optional string hovered_target_mode = 33;
  // @state preview
  optional uint32 hovered_target_id = 34;
  // @state preview
  bool stroke_drag_active = 35;
  // @state preview
  bool transform_drag_active = 36;
  // @state preview
  int64 preview_seq = 37;
}

message LowpolySelectionTargets {
  bool mesh = 1;
  bool vertex = 2;
  bool edge = 3;
  bool face = 4;
}

message LowpolySelection {
  LowpolySelectionTargets targets = 1;
  repeated string keys = 2;
  string mode = 3;
  repeated uint32 ids = 4;
}

message LowpolyTransform {
  repeated float position = 1;
  repeated float rotation = 2;
  repeated float scale = 3;
}

message LowpolyPaintLayer {
  string name = 1;
  bool visible = 2;
  float opacity = 3;
  string blend_mode = 4;
  bytes pixels = 5;
}

message LowpolyObject {
  string id = 1;
  string name = 2;
  LowpolyTransform transform = 3;
  bool smooth_shading = 4;
  string mesh_json = 5;
  repeated LowpolyPaintLayer paint_layers = 6;
}
```

### 15.2 `snapshot` facet

#### `snapshot/🦀️component.rs`

```rust
//! 🧬️ Lowpoly snapshot schema — persistent fields only.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "lowpoly", layout = "lines")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub objects: Vec<LowpolyObject>,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for LowpolySnapshot {
    const EXTENSION: &'static str = "lowpoly";
    fn envelope_id() -> &'static str { "lowpoly.lowpoly" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for LowpolySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs

/// 🏗️ Builds a single-object snapshot from mesh JSON.
pub fn snapshot_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolySnapshot {
    LowpolySnapshot {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject {
            id: object_id.into(),
            name: object_name.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json: mesh_json.into(),
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        }],
    }
}
//#endregion 🔖️Snapshot

impl Default for LowpolySnapshot {
    fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
```

#### `snapshot/🟦️component.ts`

```typescript
/** 🧬️ Lowpoly snapshot schema — persistent fields only. */

export interface LowpolySnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  objects: LowpolyObject[];
}

export interface LowpolySelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}

export interface LowpolySelection {
  targets: LowpolySelectionTargets;
  keys: string[];
  mode: string;
  ids: number[];
}

export interface LowpolyTransform {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface LowpolyPaintLayer {
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  pixels: string;
}

export interface LowpolyObject {
  id: string;
  name: string;
  transform: LowpolyTransform;
  smoothShading: boolean;
  meshJson: string;
  paintLayers: LowpolyPaintLayer[];
}
```

#### `snapshot/🔗️component.graphql`

```graphql
# 🧬️ Lowpoly snapshot schema — persistent fields only.

type LowpolySnapshot {
  schema: String! @state(class: PERSISTENT)
  objects: [LowpolyObject!]! @state(class: PERSISTENT)
}

type LowpolySelectionTargets {
  mesh: Boolean!
  vertex: Boolean!
  edge: Boolean!
  face: Boolean!
}

type LowpolySelection {
  targets: LowpolySelectionTargets!
  keys: [String!]!
  mode: String!
  ids: [Int!]!
}

type LowpolyTransform {
  position: [Float!]!
  rotation: [Float!]!
  scale: [Float!]!
}

type LowpolyPaintLayer {
  name: String!
  visible: Boolean!
  opacity: Float!
  blendMode: String!
  pixels: String!
}

type LowpolyObject {
  id: String!
  name: String!
  transform: LowpolyTransform!
  smoothShading: Boolean!
  meshJson: String!
  paintLayers: [LowpolyPaintLayer!]!
}
```

#### `snapshot/🔣️component.json`

```json
{
  "$id": "https://semio.tech/schema/s/lowpoly/lowpoly/snapshot.json",
  "title": "LowpolySnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema",
    "objects"
  ],
  "properties": {
    "schema": {
      "type": "string",
      "x-semio-state": "persistent"
    },
    "objects": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/LowpolyObject"
      },
      "x-semio-state": "persistent"
    }
  },
  "$defs": {
    "LowpolyObject": {
      "title": "LowpolyObject",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id",
        "name",
        "transform",
        "smoothShading",
        "meshJson",
        "paintLayers"
      ],
      "properties": {
        "id": {
          "type": "string"
        },
        "name": {
          "type": "string"
        },
        "transform": {
          "$ref": "#/$defs/LowpolyTransform"
        },
        "smoothShading": {
          "type": "boolean"
        },
        "meshJson": {
          "type": "string",
          "contentMediaType": "application/json"
        },
        "paintLayers": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/LowpolyPaintLayer"
          }
        }
      }
    },
    "LowpolyTransform": {
      "title": "LowpolyTransform",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "position",
        "rotation",
        "scale"
      ],
      "properties": {
        "position": {
          "type": "array",
          "items": {
            "type": "number",
            "format": "float"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "rotation": {
          "type": "array",
          "items": {
            "type": "number",
            "format": "float"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "scale": {
          "type": "array",
          "items": {
            "type": "number",
            "format": "float"
          },
          "minItems": 3,
          "maxItems": 3
        }
      }
    },
    "LowpolyPaintLayer": {
      "title": "LowpolyPaintLayer",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "name",
        "visible",
        "opacity",
        "blendMode",
        "pixels"
      ],
      "properties": {
        "name": {
          "type": "string"
        },
        "visible": {
          "type": "boolean"
        },
        "opacity": {
          "type": "number",
          "format": "float"
        },
        "blendMode": {
          "type": "string"
        },
        "pixels": {
          "type": "string",
          "contentEncoding": "base64"
        }
      }
    }
  }
}
```

#### `snapshot/🛰️component.proto`

```protobuf
syntax = "proto3";
package semio.s.lowpoly.lowpoly.snapshot;

// 🧬️ Lowpoly snapshot schema — persistent fields only.

message LowpolySnapshot {
  // @state persistent
  string schema = 1;
  // @state persistent
  repeated LowpolyObject objects = 2;
}

message LowpolySelectionTargets {
  bool mesh = 1;
  bool vertex = 2;
  bool edge = 3;
  bool face = 4;
}

message LowpolySelection {
  LowpolySelectionTargets targets = 1;
  repeated string keys = 2;
  string mode = 3;
  repeated uint32 ids = 4;
}

message LowpolyTransform {
  repeated float position = 1;
  repeated float rotation = 2;
  repeated float scale = 3;
}

message LowpolyPaintLayer {
  string name = 1;
  bool visible = 2;
  float opacity = 3;
  string blend_mode = 4;
  bytes pixels = 5;
}

message LowpolyObject {
  string id = 1;
  string name = 2;
  LowpolyTransform transform = 3;
  bool smooth_shading = 4;
  string mesh_json = 5;
  repeated LowpolyPaintLayer paint_layers = 6;
}
```

### 15.3 `diff` facet

#### `diff/🦀️component.rs`

```rust
//! 🧬️ Lowpoly diff schema — sparse field delta over the artifact.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolySelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the lowpoly artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::lowpoly::schema::LowpolyArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub objects: Option<LowpolyObjectsDelta>,
    #[state(shared_ui)] pub active_object_id: Option<Option<String>>,
    #[state(shared_ui)] pub selection: Option<crate::artifacts::lowpoly::LowpolySelection>,
    #[state(shared_ui)] pub selected_object_ids: Option<LowpolyStringList>,
    #[state(shared_ui)] pub paint_utility: Option<String>,
    #[state(shared_ui)] pub active_paint_layer: Option<u32>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub show_edges: Option<bool>,
    #[state(local_ui)] pub sun_enabled: Option<bool>,
    #[state(local_ui)] pub sun_azimuth: Option<f64>,
    #[state(local_ui)] pub sun_elevation: Option<f64>,
    #[state(local_ui)] pub sun_intensity: Option<f64>,
    #[state(local_ui)] pub sun_color: Option<String>,
    #[state(local_ui)] pub world_camera_position_x: Option<f64>,
    #[state(local_ui)] pub world_camera_position_y: Option<f64>,
    #[state(local_ui)] pub world_camera_position_z: Option<f64>,
    #[state(local_ui)] pub world_camera_target_x: Option<f64>,
    #[state(local_ui)] pub world_camera_target_y: Option<f64>,
    #[state(local_ui)] pub world_camera_target_z: Option<f64>,
    #[state(local_ui)] pub world_camera_fov: Option<f64>,
    #[state(local_ui)] pub utility_params_json: Option<String>,
    #[state(local_ui)] pub paint_color_r: Option<u32>,
    #[state(local_ui)] pub paint_color_g: Option<u32>,
    #[state(local_ui)] pub paint_color_b: Option<u32>,
    #[state(local_ui)] pub paint_color_a: Option<u32>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(local_ui)] pub selection_mode_default: Option<String>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub hovered_object_id: Option<Option<String>>,
    #[state(preview)] pub hovered_target_object_id: Option<Option<String>>,
    #[state(preview)] pub hovered_target_mode: Option<Option<String>>,
    #[state(preview)] pub hovered_target_id: Option<Option<u32>>,
    #[state(preview)] pub stroke_drag_active: Option<bool>,
    #[state(preview)] pub transform_drag_active: Option<bool>,
    #[state(preview)] pub preview_seq: Option<i64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyObjectsDelta {
    pub added: Vec<LowpolyObject>,
    pub removed: Vec<String>,
    pub patched: Vec<LowpolyObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched object entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyObjectPatchEntry {
    pub id: String,
    pub patch: LowpolyObjectPatch,
    #[serde(default)]
    pub paint_layers: Option<LowpolyPaintLayersDelta>,
}

/// 🖌️ Paint-layer sub-delta under an object patch.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayersDelta {
    pub added: Vec<LowpolyIndexedPaintLayer>,
    pub removed: Vec<u32>,
    pub patched: Vec<LowpolyIndexedPaintLayerPatch>,
    pub strokes: Vec<LowpolyPaintStrokeAt>,
}

/// ➕️ Paint layer at index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyIndexedPaintLayer {
    pub index: u32,
    pub layer: LowpolyPaintLayer,
}

/// 🩹 Paint layer metadata patch at index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyIndexedPaintLayerPatch {
    pub index: u32,
    pub patch: LowpolyPaintLayerPatch,
}

/// 🖌️ Pixel runs on one layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyPaintStrokeAt {
    pub layer_index: u32,
    pub runs: Vec<PixelRun>,
}

/// 🩸 Contiguous RGBA run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixelRun {
    pub offset: u32,
    #[serde(with = "pixel_run_bytes_base64")]
    pub bytes: Vec<u8>,
}

mod pixel_run_bytes_base64 {
    use base64::Engine;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD.decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}

/// 🩹 Paint-layer metadata patch.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub blend_mode: Option<String>,
}
//#endregion 🔖️DeltaHelpers
```

#### `diff/🟦️component.ts`

```typescript
/** 🧬️ Lowpoly diff schema — sparse field delta. */

export interface LowpolyDiff {
  /** @state persistent */
  artifact?: LowpolyArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  objects?: LowpolyObjectsDelta;
  /** @state shared-ui */
  activeObjectId?: string | null;
  /** @state shared-ui */
  selection?: LowpolySelection;
  /** @state shared-ui */
  selectedObjectIds?: LowpolyStringList;
  /** @state shared-ui */
  paintUtility?: string;
  /** @state shared-ui */
  activePaintLayer?: number;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  showEdges?: boolean;
  /** @state local-ui */
  sunEnabled?: boolean;
  /** @state local-ui */
  sunAzimuth?: number;
  /** @state local-ui */
  sunElevation?: number;
  /** @state local-ui */
  sunIntensity?: number;
  /** @state local-ui */
  sunColor?: string;
  /** @state local-ui */
  worldCameraPositionX?: number;
  /** @state local-ui */
  worldCameraPositionY?: number;
  /** @state local-ui */
  worldCameraPositionZ?: number;
  /** @state local-ui */
  worldCameraTargetX?: number;
  /** @state local-ui */
  worldCameraTargetY?: number;
  /** @state local-ui */
  worldCameraTargetZ?: number;
  /** @state local-ui */
  worldCameraFov?: number;
  /** @state local-ui */
  utilityParamsJson?: string;
  /** @state local-ui */
  paintColorR?: number;
  /** @state local-ui */
  paintColorG?: number;
  /** @state local-ui */
  paintColorB?: number;
  /** @state local-ui */
  paintColorA?: number;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  selectionModeDefault?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  hoveredObjectId?: string | null;
  /** @state preview */
  hoveredTargetObjectId?: string | null;
  /** @state preview */
  hoveredTargetMode?: string | null;
  /** @state preview */
  hoveredTargetId?: number | null;
  /** @state preview */
  strokeDragActive?: boolean;
  /** @state preview */
  transformDragActive?: boolean;
  /** @state preview */
  previewSeq?: number;
}

export interface LowpolyStringList {
  values: string[];
}

export interface LowpolyObjectsDelta {
  added: LowpolyObject[];
  removed: string[];
  patched: LowpolyObjectPatchEntry[];
  reordered?: string[];
}

export interface LowpolyObjectPatchEntry {
  id: string;
  patch: LowpolyObjectPatch;
}

export interface LowpolyObjectPatch {
  name?: string;
  smoothShading?: boolean;
  transform?: LowpolyTransform;
  meshJson?: string;
  paintLayers?: LowpolyPaintLayersDelta;
}

export interface LowpolyPaintLayersDelta {
  added: LowpolyIndexedPaintLayer[];
  removed: number[];
  patched: LowpolyIndexedPaintLayerPatch[];
  strokes: LowpolyPaintStrokeAt[];
}

export interface LowpolyIndexedPaintLayer {
  index: number;
  layer: LowpolyPaintLayer;
}

export interface LowpolyIndexedPaintLayerPatch {
  index: number;
  patch: LowpolyPaintLayerPatch;
}

export interface LowpolyPaintLayerPatch {
  name?: string;
  visible?: boolean;
  opacity?: number;
  blendMode?: string;
}

export interface LowpolyPaintStrokeAt {
  layerIndex: number;
  runs: PixelRun[];
}

export interface PixelRun {
  offset: number;
  bytes: string;
}

export interface LowpolyArtifact {
  schema: string;
  objects: LowpolyObject[];
}

export interface LowpolySelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}

export interface LowpolySelection {
  targets: LowpolySelectionTargets;
  keys: string[];
  mode: string;
  ids: number[];
}

export interface LowpolyTransform {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface LowpolyPaintLayer {
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  pixels: string;
}

export interface LowpolyObject {
  id: string;
  name: string;
  transform: LowpolyTransform;
  smoothShading: boolean;
  meshJson: string;
  paintLayers: LowpolyPaintLayer[];
}
```

#### `diff/🔗️component.graphql`

```graphql
# 🧬️ Lowpoly diff schema — sparse field delta.

type LowpolyDiff {
  artifact: LowpolyArtifact @state(class: PERSISTENT)
  schema: String @state(class: PERSISTENT)
  objects: LowpolyObjectsDelta @state(class: PERSISTENT)
  activeObjectId: String @state(class: SHARED_UI)
  selection: LowpolySelection @state(class: SHARED_UI)
  selectedObjectIds: LowpolyStringList @state(class: SHARED_UI)
  paintUtility: String @state(class: SHARED_UI)
  activePaintLayer: Int @state(class: SHARED_UI)
  activeUtilityId: String @state(class: SHARED_UI)
  showEdges: Boolean @state(class: LOCAL_UI)
  sunEnabled: Boolean @state(class: LOCAL_UI)
  sunAzimuth: Float @state(class: LOCAL_UI)
  sunElevation: Float @state(class: LOCAL_UI)
  sunIntensity: Float @state(class: LOCAL_UI)
  sunColor: String @state(class: LOCAL_UI)
  worldCameraPositionX: Float @state(class: LOCAL_UI)
  worldCameraPositionY: Float @state(class: LOCAL_UI)
  worldCameraPositionZ: Float @state(class: LOCAL_UI)
  worldCameraTargetX: Float @state(class: LOCAL_UI)
  worldCameraTargetY: Float @state(class: LOCAL_UI)
  worldCameraTargetZ: Float @state(class: LOCAL_UI)
  worldCameraFov: Float @state(class: LOCAL_UI)
  utilityParamsJson: String @state(class: LOCAL_UI)
  paintColorR: Int @state(class: LOCAL_UI)
  paintColorG: Int @state(class: LOCAL_UI)
  paintColorB: Int @state(class: LOCAL_UI)
  paintColorA: Int @state(class: LOCAL_UI)
  selectionMethod: String @state(class: LOCAL_UI)
  selectionModeDefault: String @state(class: LOCAL_UI)
  engagementInput: String @state(class: LOCAL_UI)
  locale: String @state(class: LOCAL_UI)
  hoveredObjectId: String @state(class: PREVIEW)
  hoveredTargetObjectId: String @state(class: PREVIEW)
  hoveredTargetMode: String @state(class: PREVIEW)
  hoveredTargetId: Int @state(class: PREVIEW)
  strokeDragActive: Boolean @state(class: PREVIEW)
  transformDragActive: Boolean @state(class: PREVIEW)
  previewSeq: Int @state(class: PREVIEW)
}

type LowpolyStringList {
  values: [String!]!
}

type LowpolyObjectsDelta {
  added: [LowpolyObject!]!
  removed: [String!]!
  patched: [LowpolyObjectPatchEntry!]!
  reordered: [String!]
}

type LowpolyObjectPatchEntry {
  id: String!
  patch: LowpolyObjectPatch!
}

type LowpolyObjectPatch {
  name: String
  smoothShading: Boolean
  transform: LowpolyTransform
  meshJson: String
}

type LowpolyArtifact {
  schema: String!
  objects: [LowpolyObject!]!
}

type LowpolySelection {
  mode: String!
  ids: [Int!]!
  keys: [String!]!
}

type LowpolyTransform {
  position: [Float!]!
  rotation: [Float!]!
  scale: [Float!]!
}

type LowpolyObject {
  id: String!
  name: String!
}
```

#### `diff/🔣️component.json`

```json
{
  "$id": "https://semio.tech/schema/s/lowpoly/lowpoly/diff.json",
  "title": "LowpolyDiff",
  "type": "object",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "artifact": {
      "title": "LowpolyArtifact",
      "type": "object",
      "x-semio-state": "persistent"
    },
    "schema": {
      "type": "string",
      "x-semio-state": "persistent"
    },
    "objects": {
      "$ref": "#/$defs/LowpolyObjectsDelta",
      "x-semio-state": "persistent"
    },
    "activeObjectId": {
      "oneOf": [
        {
          "type": "null"
        },
        {
          "type": "string"
        }
      ],
      "x-semio-state": "shared-ui"
    },
    "selection": {
      "$ref": "#/$defs/LowpolySelection",
      "x-semio-state": "shared-ui"
    },
    "selectedObjectIds": {
      "$ref": "#/$defs/LowpolyStringList",
      "x-semio-state": "shared-ui"
    },
    "paintUtility": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "activePaintLayer": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "shared-ui"
    },
    "activeUtilityId": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "showEdges": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "sunEnabled": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "sunAzimuth": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "sunElevation": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "sunIntensity": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "sunColor": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "worldCameraPositionX": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraPositionY": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraPositionZ": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraTargetX": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraTargetY": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraTargetZ": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "worldCameraFov": {
      "type": "number",
      "format": "double",
      "x-semio-state": "local-ui"
    },
    "utilityParamsJson": {
      "type": "string",
      "contentMediaType": "application/json",
      "x-semio-state": "local-ui"
    },
    "paintColorR": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "paintColorG": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "paintColorB": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "paintColorA": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "selectionMethod": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "selectionModeDefault": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "engagementInput": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "locale": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "hoveredObjectId": {
      "oneOf": [
        {
          "type": "null"
        },
        {
          "type": "string"
        }
      ],
      "x-semio-state": "preview"
    },
    "hoveredTargetObjectId": {
      "oneOf": [
        {
          "type": "null"
        },
        {
          "type": "string"
        }
      ],
      "x-semio-state": "preview"
    },
    "hoveredTargetMode": {
      "oneOf": [
        {
          "type": "null"
        },
        {
          "type": "string"
        }
      ],
      "x-semio-state": "preview"
    },
    "hoveredTargetId": {
      "oneOf": [
        {
          "type": "null"
        },
        {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        }
      ],
      "x-semio-state": "preview"
    },
    "strokeDragActive": {
      "type": "boolean",
      "x-semio-state": "preview"
    },
    "transformDragActive": {
      "type": "boolean",
      "x-semio-state": "preview"
    },
    "previewSeq": {
      "type": "integer",
      "format": "int64",
      "x-semio-state": "preview"
    }
  },
  "$defs": {
    "LowpolyStringList": {
      "title": "LowpolyStringList",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "values"
      ],
      "properties": {
        "values": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "LowpolyObjectsDelta": {
      "title": "LowpolyObjectsDelta",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "added",
        "removed",
        "patched"
      ],
      "properties": {
        "added": {
          "type": "array",
          "items": {
            "type": "object"
          }
        },
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "patched": {
          "type": "array",
          "items": {
            "type": "object"
          }
        },
        "reordered": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    }
  }
}
```

#### `diff/🛰️component.proto`

```protobuf
syntax = "proto3";
package semio.s.lowpoly.lowpoly.diff;

// 🧬️ Lowpoly diff schema — sparse field delta.

message LowpolyDiff {
  // @state persistent
  optional LowpolyArtifact artifact = 1;
  // @state persistent
  optional string schema = 2;
  // @state persistent
  optional LowpolyObjectsDelta objects = 3;
  // @state shared-ui
  optional string active_object_id = 4;
  // @state shared-ui
  optional LowpolySelection selection = 5;
  // @state shared-ui
  optional LowpolyStringList selected_object_ids = 6;
  // @state shared-ui
  optional string paint_utility = 7;
  // @state shared-ui
  optional uint32 active_paint_layer = 8;
  // @state shared-ui
  optional string active_utility_id = 9;
  // @state local-ui
  optional bool show_edges = 10;
  // @state local-ui
  optional bool sun_enabled = 11;
  // @state local-ui
  optional double sun_azimuth = 12;
  // @state local-ui
  optional double sun_elevation = 13;
  // @state local-ui
  optional double sun_intensity = 14;
  // @state local-ui
  optional string sun_color = 15;
  // @state local-ui
  optional double world_camera_position_x = 16;
  // @state local-ui
  optional double world_camera_position_y = 17;
  // @state local-ui
  optional double world_camera_position_z = 18;
  // @state local-ui
  optional double world_camera_target_x = 19;
  // @state local-ui
  optional double world_camera_target_y = 20;
  // @state local-ui
  optional double world_camera_target_z = 21;
  // @state local-ui
  optional double world_camera_fov = 22;
  // @state local-ui
  optional string utility_params_json = 23;
  // @state local-ui
  optional uint32 paint_color_r = 24;
  // @state local-ui
  optional uint32 paint_color_g = 25;
  // @state local-ui
  optional uint32 paint_color_b = 26;
  // @state local-ui
  optional uint32 paint_color_a = 27;
  // @state local-ui
  optional string selection_method = 28;
  // @state local-ui
  optional string selection_mode_default = 29;
  // @state local-ui
  optional string engagement_input = 30;
  // @state local-ui
  optional string locale = 31;
  // @state preview
  optional string hovered_object_id = 32;
  // @state preview
  optional string hovered_target_object_id = 33;
  // @state preview
  optional string hovered_target_mode = 34;
  // @state preview
  optional uint32 hovered_target_id = 35;
  // @state preview
  optional bool stroke_drag_active = 36;
  // @state preview
  optional bool transform_drag_active = 37;
  // @state preview
  optional int64 preview_seq = 38;
}

message LowpolyStringList {
  repeated string values = 1;
}

message LowpolyObjectsDelta {
  repeated LowpolyObject added = 1;
  repeated string removed = 2;
  repeated LowpolyObjectPatchEntry patched = 3;
  repeated string reordered = 4;
}

message LowpolyObjectPatchEntry {
  string id = 1;
  LowpolyObjectPatch patch = 2;
}

message LowpolyObjectPatch {
  optional string name = 1;
  optional bool smooth_shading = 2;
  optional LowpolyTransform transform = 3;
  optional string mesh_json = 4;
}

message LowpolyArtifact {
  string schema = 1;
  repeated LowpolyObject objects = 2;
}

message LowpolySelection {
  string mode = 1;
  repeated uint32 ids = 2;
  repeated string keys = 3;
}

message LowpolyTransform {
  repeated float position = 1;
  repeated float rotation = 2;
  repeated float scale = 3;
}

message LowpolyObject {
  string id = 1;
  string name = 2;
}
```

### 15.4 `LowpolyDiff` runtime (`🔺️diff/🦀️component.rs`)

```rust
//! 🔺️ Lowpoly artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::lowpoly::diff::schema::{
    LowpolyDiff, LowpolyObjectPatchEntry, LowpolyObjectsDelta, LowpolyPaintLayersDelta, LowpolyPaintStrokeAt,
    PixelRun as SchemaPixelRun,
};
use crate::artifacts::lowpoly::schema::LowpolyArtifact;
use crate::artifacts::lowpoly::{apply_paint_layers_delta, LowpolySnapshot};
use protocol::MutationDiff;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Apply
impl LowpolyDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &LowpolyArtifact) -> LowpolyArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = apply_objects_delta(&next.objects, delta);
        }
        if let Some(value) = &self.active_object_id {
            next.active_object_id = value.clone();
        }
        if let Some(value) = &self.selection {
            next.selection = value.clone();
        }
        if let Some(list) = &self.selected_object_ids {
            next.selected_object_ids = list.values.clone();
        }
        if let Some(value) = &self.paint_utility {
            next.paint_utility = value.clone();
        }
        if let Some(value) = self.active_paint_layer {
            next.active_paint_layer = value;
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = self.show_edges {
            next.show_edges = value;
        }
        if let Some(value) = self.sun_enabled {
            next.sun_enabled = value;
        }
        if let Some(value) = self.sun_azimuth {
            next.sun_azimuth = value;
        }
        if let Some(value) = self.sun_elevation {
            next.sun_elevation = value;
        }
        if let Some(value) = self.sun_intensity {
            next.sun_intensity = value;
        }
        if let Some(value) = &self.sun_color {
            next.sun_color = value.clone();
        }
        if let Some(value) = self.world_camera_position_x {
            next.world_camera_position_x = value;
        }
        if let Some(value) = self.world_camera_position_y {
            next.world_camera_position_y = value;
        }
        if let Some(value) = self.world_camera_position_z {
            next.world_camera_position_z = value;
        }
        if let Some(value) = self.world_camera_target_x {
            next.world_camera_target_x = value;
        }
        if let Some(value) = self.world_camera_target_y {
            next.world_camera_target_y = value;
        }
        if let Some(value) = self.world_camera_target_z {
            next.world_camera_target_z = value;
        }
        if let Some(value) = self.world_camera_fov {
            next.world_camera_fov = value;
        }
        if let Some(value) = &self.utility_params_json {
            next.utility_params_json = value.clone();
        }
        if let Some(value) = self.paint_color_r {
            next.paint_color_r = value;
        }
        if let Some(value) = self.paint_color_g {
            next.paint_color_g = value;
        }
        if let Some(value) = self.paint_color_b {
            next.paint_color_b = value;
        }
        if let Some(value) = self.paint_color_a {
            next.paint_color_a = value;
        }
        if let Some(value) = &self.selection_method {
            next.selection_method = value.clone();
        }
        if let Some(value) = &self.selection_mode_default {
            next.selection_mode_default = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.hovered_object_id {
            next.hovered_object_id = value.clone();
        }
        if let Some(value) = &self.hovered_target_object_id {
            next.hovered_target_object_id = value.clone();
        }
        if let Some(value) = &self.hovered_target_mode {
            next.hovered_target_mode = value.clone();
        }
        if let Some(value) = &self.hovered_target_id {
            next.hovered_target_id = *value;
        }
        if let Some(value) = self.stroke_drag_active {
            next.stroke_drag_active = value;
        }
        if let Some(value) = self.transform_drag_active {
            next.transform_drag_active = value;
        }
        if let Some(value) = self.preview_seq {
            next.preview_seq = value;
        }
        next
    }
}

/// 🧩 Applies an identified-collection delta to a snapshot object list.
pub fn apply_objects_delta(
    objects: &[crate::artifacts::lowpoly::LowpolyObject],
    delta: &LowpolyObjectsDelta,
) -> Vec<crate::artifacts::lowpoly::LowpolyObject> {
    let mut next = objects.to_vec();
    for id in &delta.removed {
        next.retain(|object| &object.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(object) = next.iter_mut().find(|object| object.id == entry.id) {
            use protocol::Patchable;
            object.apply_patch(&entry.patch);
            if let Some(paint) = &entry.paint_layers {
                apply_paint_layers_delta(object, paint);
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|object| (object.id.clone(), object)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(object) = by_id.remove(id) {
                ordered.push(object);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

impl MutationDiff<LowpolySnapshot> for LowpolyDiff {
    fn apply(&self, snapshot: &LowpolySnapshot) -> LowpolySnapshot {
        if let Some(replacement) = &self.artifact {
            return LowpolySnapshot { schema: replacement.schema.clone(), objects: replacement.objects.clone() };
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = apply_objects_delta(&next.objects, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(active_object_id);
        take!(selection);
        take!(selected_object_ids);
        take!(paint_utility);
        take!(active_paint_layer);
        take!(active_utility_id);
        take!(show_edges);
        take!(sun_enabled);
        take!(sun_azimuth);
        take!(sun_elevation);
        take!(sun_intensity);
        take!(sun_color);
        take!(world_camera_position_x);
        take!(world_camera_position_y);
        take!(world_camera_position_z);
        take!(world_camera_target_x);
        take!(world_camera_target_y);
        take!(world_camera_target_z);
        take!(world_camera_fov);
        take!(utility_params_json);
        take!(paint_color_r);
        take!(paint_color_g);
        take!(paint_color_b);
        take!(paint_color_a);
        take!(selection_method);
        take!(selection_mode_default);
        take!(engagement_input);
        take!(locale);
        take!(hovered_object_id);
        take!(hovered_target_object_id);
        take!(hovered_target_mode);
        take!(hovered_target_id);
        take!(stroke_drag_active);
        take!(transform_drag_active);
        take!(preview_seq);
        match (&mut self.objects, other.objects) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (dst, Some(src)) => *dst = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Constructors
/// 🏗️ Objects-add field delta.
pub fn diff_objects_add(index: usize, item: crate::artifacts::lowpoly::LowpolyObject, base: &LowpolySnapshot) -> LowpolyDiff {
    let mut order: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
    let id = item.id.clone();
    let at = index.min(order.len());
    order.insert(at, id);
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: vec![item],
            removed: Vec::new(),
            patched: Vec::new(),
            reordered: Some(order),
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Objects-remove field delta.
pub fn diff_objects_remove(id: String) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: Vec::new(),
            removed: vec![id],
            patched: Vec::new(),
            reordered: None,
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Objects-move field delta.
pub fn diff_objects_move(id: &str, to_index: usize, base: &LowpolySnapshot) -> LowpolyDiff {
    let mut order: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
    if let Some(from) = order.iter().position(|existing| existing == id) {
        let moved = order.remove(from);
        let at = to_index.min(order.len());
        order.insert(at, moved);
    }
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: Vec::new(),
            removed: Vec::new(),
            patched: Vec::new(),
            reordered: Some(order),
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Objects-patch field delta.
pub fn diff_objects_patch(id: String, patch: crate::artifacts::lowpoly::LowpolyObjectPatch) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            added: Vec::new(),
            removed: Vec::new(),
            patched: vec![LowpolyObjectPatchEntry { id, patch, paint_layers: None }],
            reordered: None,
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Add-paint-layer field delta.
pub fn diff_add_paint_layer(object_id: String, index: usize, layer: crate::artifacts::lowpoly::LowpolyPaintLayer) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    added: vec![crate::artifacts::lowpoly::diff::schema::LowpolyIndexedPaintLayer {
                        index: index as u32,
                        layer,
                    }],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Remove-paint-layer field delta.
pub fn diff_remove_paint_layer(object_id: String, index: usize) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    removed: vec![index as u32],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Patch-paint-layer field delta.
pub fn diff_patch_paint_layer(
    object_id: String,
    index: usize,
    patch: crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayerPatch,
) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    patched: vec![crate::artifacts::lowpoly::diff::schema::LowpolyIndexedPaintLayerPatch {
                        index: index as u32,
                        patch,
                    }],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Paint-stroke field delta.
pub fn diff_paint_stroke(object_id: String, layer_index: usize, runs: Vec<SchemaPixelRun>) -> LowpolyDiff {
    LowpolyDiff {
        objects: Some(LowpolyObjectsDelta {
            patched: vec![LowpolyObjectPatchEntry {
                id: object_id,
                patch: crate::artifacts::lowpoly::LowpolyObjectPatch::default(),
                paint_layers: Some(LowpolyPaintLayersDelta {
                    strokes: vec![LowpolyPaintStrokeAt { layer_index: layer_index as u32, runs }],
                    ..LowpolyPaintLayersDelta::default()
                }),
            }],
            ..LowpolyObjectsDelta::default()
        }),
        ..LowpolyDiff::default()
    }
}

/// 🏗️ Whole snapshot replacement via schema+objects (clears then adds).
pub fn diff_replace_snapshot(before: &LowpolySnapshot, after: &LowpolySnapshot) -> LowpolyDiff {
    LowpolyDiff {
        schema: (before.schema != after.schema).then(|| after.schema.clone()),
        objects: Some(LowpolyObjectsDelta {
            added: after.objects.clone(),
            removed: before.objects.iter().map(|object| object.id.clone()).collect(),
            patched: Vec::new(),
            reordered: Some(after.objects.iter().map(|object| object.id.clone()).collect()),
        }),
        ..LowpolyDiff::default()
    }
}
//#endregion 🔖️Constructors

```

### 15.5 Glue `#[path]` convention (leaf-prefixed + grouping `#[path = "."]`)

This crate uses the **leaf-prefixed** convention: grouping modules reset with `#[path = "."]` so nested `snapshot` / `diff` keep the same `../../` leaf prefix (no extra `../`).

```rust
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🗿️artifacts/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/💠️lowpoly/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/💠️lowpoly/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/💠️lowpoly/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod objects_add {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️objects-add/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️objects-add/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️objects-add/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod objects_remove {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️objects-remove/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️objects-remove/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️objects-remove/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod objects_move {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/↔️objects-move/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/↔️objects-move/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/↔️objects-move/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod objects_patch {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹objects-patch/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹objects-patch/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹objects-patch/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod add_paint_layer {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️add-paint-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️add-paint-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️add-paint-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_paint_layer {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️remove-paint-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️remove-paint-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️remove-paint-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod patch_paint_layer {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹patch-paint-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹patch-paint-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹patch-paint-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod paint_stroke {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖌️paint-stroke/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖌️paint-stroke/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖌️paint-stroke/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/💠️lowpoly/🗣️dsl/🦀️component.rs"]
        pub mod dsl;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/💠️lowpoly/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/💠️lowpoly/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/💠️lowpoly/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🎨️paint/🦀️component.rs"]
            pub mod paint;
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🧵️media/🦀️component.rs"]
            pub mod media;
            pub use media::{lowpoly_document_from_mesh, lowpoly_mesh_from_document, mesh_data_from_transfer, mesh_document_from_mesh, mesh_from_mesh_document};
            pub use paint::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from, stamp_brush};
        }
    }
}
//#endregion 🗿️Artifacts
```

### 15.6 TypeScript index mirror

```typescript
/** lowpoly facet WASM facades */
export * as lowpoly_schema from "../../🗿️artifacts/💠️lowpoly/🧬️schema/🟦️component.ts";
export * as lowpoly_snapshot_schema from "../../🗿️artifacts/💠️lowpoly/📸️snapshot/🧬️schema/🟦️component.ts";
export * as lowpoly_diff from "../../🗿️artifacts/💠️lowpoly/🔺️diff/🟦️component.ts";
export * as lowpoly_diff_schema from "../../🗿️artifacts/💠️lowpoly/🔺️diff/🧬️schema/🟦️component.ts";
export * as lowpoly_dsl from "../../🗿️artifacts/💠️lowpoly/🗣️dsl/🟦️component.ts";
export * as lowpoly_pack from "../../🗿️artifacts/💠️lowpoly/📸️snapshot/🎒️pack/🟦️component.ts";
export * as lowpoly_op from "../../🗿️artifacts/💠️lowpoly/🔧️op/🟦️component.ts";
export * as lowpoly_mutations from "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🟦️component.ts";
export * as lowpoly_spr from "../../🗿️artifacts/💠️lowpoly/📡️spr/🟦️component.ts";
```

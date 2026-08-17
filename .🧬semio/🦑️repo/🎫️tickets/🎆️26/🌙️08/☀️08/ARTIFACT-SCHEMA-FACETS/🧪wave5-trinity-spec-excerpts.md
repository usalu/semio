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


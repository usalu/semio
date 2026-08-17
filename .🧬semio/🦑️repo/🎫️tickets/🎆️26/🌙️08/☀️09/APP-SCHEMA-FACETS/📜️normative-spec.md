# 📜️ Normative Spec — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`. Every agent on this ticket reads **only this document** plus the
sections of the sibling artifact spec it cites. Do not improvise; if this document does not answer a
question, the answer is in the artifact spec at
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/📜️normative-spec.md`.

## 1. What is being added

Every app gets **two** handcrafted schema facets:

| Facet | Meaning | State class of every field |
| --- | --- | --- |
| `🎚️config/🧬️schema` | all local app state, unshared | `local-ui` |
| `👥️presence/🧬️schema` | shared live, ephemeral state | `shared-ui` |

This is the same pattern as the artifact facets, so the following sections of the artifact spec apply
**verbatim** and are not restated here:

- **§3 `schemaFormats` registry** — the same five leaves (`🦀️component.rs`, `🟦️component.ts`,
  `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`), and `🔣️component.json` is normative
  within each facet; the other four mirror it.
- **§5 state-class annotation syntax** per format (`"x-semio-state"`, `#[state(...)]`,
  `/** @state ... */`, `@state(class: ...)`, `// @state ...`).
- **§6 field casing and scalar type mapping** — Rust/proto snake, TS/GraphQL/JSON camel.

Canonical state-class spellings are kebab-lowercase: `persistent`, `shared-ui`, `local-ui`, `preview`,
`effect`.

## 2. Facet ownership — read this before touching any file

An app's config facet lives at the directory that **defines the app's `type Config`**, not blindly under
the app. This matters because `📕️norm`'s 15 apps share one `NormConfig` declared at the *plugin* level.
Duplicating one schema 15 times would be a lie about the code, so the rule is:

> The config owner of an app is the `🎚️config` dir that declares the type named by that app's
> `type Config = …;` binding. The presence owner is the same directory.

Consequences:

- 53 apps resolve to **39 owners**: 38 app-level `🎚️config` dirs and one plugin-level
  `✏️s/🔌️plugins/📕️norm/🎚️config` serving all 15 norm apps.
- There are exactly **39 distinct config types**, one per owner — the mapping is a bijection.
- The scanners derive the expected type name from the app's `type Config` binding, never from a
  hand-maintained table. A shared config can therefore never drift out of the table, because there is
  no table.

The authoritative owner list is generated into `🧪owner-table.json` in this ticket folder. Regenerate it
rather than editing it by hand.

## 3. Facet layout on disk

```
<owner>/                          e.g. ✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🎚️config
  🦀️component.rs                  KEPT — XConfig + XConfigMutation, unchanged in purpose
  🧬️schema/                       NEW
    🦀️component.rs  🟦️component.ts  🔗️component.graphql  🔣️component.json  🛰️component.proto

<owner-parent>/👥️presence/        NEW facet, sibling of 🎚️config
  🦀️component.rs                  NEW — XPresence + XPresenceMutation
  🧬️schema/                       NEW
    🦀️component.rs  🟦️component.ts  🔗️component.graphql  🔣️component.json  🛰️component.proto
```

`👥️presence` sits beside `🎚️config` under the same parent: under the app for the 38 app-level owners,
under `✏️s/🔌️plugins/📕️norm/` for norm.

`👥️` already means presence in this repo — `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/👥️presence/`
predates this ticket — so the prefix is consistent, not new.

## 4. The `🧮️config` consolidation

`appChildDirs` currently lists **both** `🎚️config` and `🧮️config`, and three owners use the duplicate:

- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🧮️config` → `CadConfig`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🧮️config` → `LayoutConfig`
- `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🧮️config` → `LowpolyConfig`

All three move to `🎚️config`, `🧮️config` is removed from the taxonomy, and a policy rule forbids its
return. Each move is one directory rename plus the one `#[path]` line in that plugin's `📦️glue.rs`.

`🕸️wasm` (one app) is likewise a duplicate of `🌉️wasm` (16 apps) and is consolidated the same way.

## 5. Type naming

For an owner whose config type is `XConfig`:

| Facet | Type name |
| --- | --- |
| `🎚️config/🧬️schema` | `XConfig` — the existing type, unchanged |
| `👥️presence/🧬️schema` | `XPresence` |

`XPresence` is formed by replacing the trailing `Config` with `Presence`: `LowpolyConfig` →
`LowpolyPresence`, `NormConfig` → `NormPresence`, `SourcingCurateConfig` → `SourcingCuratePresence`.

Mutation types are `XConfigMutation` (exists) and `XPresenceMutation` (new).

Proto package and JSON Schema `$id` follow artifact spec §4's rule with `app` in place of `artifact`:
`$id` is `https://semio.tech/schema/app/<plugin>/<owner-slug>/<facet>.json`, proto package
`semio.app.<plugin>.<owner_slug>.<facet>`, both emoji-stripped and snake/lower as that section requires.

## 6. What goes in each facet

### 6.1 `🎚️config/🧬️schema` — `XConfig`

The field set is **exactly** the fields of the existing `XConfig` struct in the owner's
`🦀️component.rs`. This facet documents what already exists; it must not invent or drop fields. Every
field carries `local-ui`, because config is by definition the app's unshared local state.

The normative JSON Schema is therefore a faithful transcription of the Rust struct, using artifact spec
§6's casing and scalar mapping.

### 6.2 `👥️presence/🧬️schema` — `XPresence`

This facet is new state, so its field set is a **design decision** constrained as follows. Every field
is `shared-ui`. The facet holds only what other humans must see live and what is meaningless to persist:

- the peer's selection within this app's surface, typed — this is what replaces today's untyped
  `PresencePeer.selection_json`;
- the peer's cursor / hover target within this app's surface, where the app has one;
- the peer's viewport or camera, where the app has a canvas;
- the peer's transient activity — the mode or tool it currently has active, and whether it is mid-drag
  or mid-edit.

It must **not** hold: anything already in `XConfig` (that is local, by definition), anything persisted
(that belongs to the artifact snapshot), identity of the peer (`actor`, `label`, `user_id`, `role` stay
on the app-agnostic `PresencePeer`), or connection bookkeeping.

Derive the fields from that app's real surface by reading its `🎮️commands`, `🎭️modes` and `📌️panels`
dirs — a presence schema that mentions a selection kind the app cannot express is wrong. When an app has
no shareable live state at all, the facet still exists and `XPresence` is an empty struct with no
fields; say so in its docstring rather than inventing state.

## 7. Kernel changes

`DocumentApp` gains a typed presence pair beside the existing config pair, in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`:

```rust
type Presence: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack;
type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
```

with `NoPresence`/`NoPresenceMutation` defaults mirroring the existing `NoConfig`/`NoConfigMutation`, so
an app with an empty presence schema needs no boilerplate.

Presence becomes load-bearing rather than decorative: `PresencePeer.selection_json: Option<String>` in
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs` is replaced by
`presence_pack: Option<Vec<u8>>`, the app's `Presence` encoded through `DocumentPack`.
`encode_presence_peer`/`decode_presence_peer` carry it as a length-prefixed byte string in the same flag
slot the JSON string used, so the frame layout keeps its shape and the hub stays a blind relay.
`ViewModel.presence_peers_json` keeps its name and its JSON-to-the-renderer contract — the difference is
that the JSON is now produced by serialising a typed `XPresence` instead of being passed through
unvalidated.

## 8. Framework `🧬️schema` module

`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` already owns `ArtifactSchemaDescriptor`,
`ArtifactSchemaRegistry` and the `ArtifactSchema` derive. Add the app twin in a new region, modelled on
it exactly:

- `AppSchemaDescriptor` — owner id plus `include_str!` handles for all five leaves of both facets.
- `AppSchemaRegistry` + `register_app_schema_descriptor`, and registration of all 39 owners in one
  central place (the artifact wave's W7 lesson: one catalog, not 39 scattered registries).
- One table-driven test in the existing test module that, for every registered owner, serialises a
  default `XConfig` and a default `XPresence`, validates each against its handcrafted `🔣️component.json`,
  and asserts `field_states()` matches `x-semio-state`. No new test file.

Reuse the existing shared GraphQL `@state` preamble; do not declare a second one.

## 9. Policy rules — root `📜️script.ts`, region `🔧️PolicyRuleAppSchemas`

Placed immediately after `🔧️PolicyRuleArtifactSchemas` and modelled on it exactly: same `BreachRecord`
shape, aggregated by one exported `policyAppSchemaBreaches(repoRoot)`, added to `VerifyScript.runGate()`
beside `policyArtifactSchemaBreaches`.

The five per-format field extractors are **reused unchanged** — they already select the declared type by
name via `policyFindSchemaDeclaration`, which is what makes a shared `NormConfig` and helper types that
precede the facet type both safe.

| Rule | Statement |
| --- | --- |
| `app-schema/facet-completeness` | both facet dirs exist for every owner, each with all five `schemaFormats` leaves and the normative `🔣️component.json` |
| `app-schema/field-parity` | all five leaves of one facet declare the identical canonical field set with identical optionality and cardinality; JSON Schema is truth. Protobuf `map` optionality is exempt (proto3 forbids `optional` on a map field) |
| `app-schema/config-fidelity` | the config facet's field set equals the fields of the owner's real `XConfig` struct — the facet may not drift from the code it documents |
| `app-schema/state-purity` | every config-facet field is `local-ui`; every presence-facet field is `shared-ui` |
| `app-schema/type-name-parity` | `XConfig` / `XPresence` named identically across all five leaves, with `XPresence` derived from the owner's `type Config` binding |
| `app-schema/config-relocation` | no `🧮️config` and no `🕸️wasm` anywhere under `✏️s/🔌️plugins` |

## 10. Taxonomy diff

In `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:

- `appChildDirs`: add `👥️presence`; remove `🧮️config` and `🕸️wasm`.
- new `appComponentDirs: ["⚙️engine", "🎮️commands", "🎚️config", "👥️presence"]` — the completeness set
  every owner-bearing app must carry, mirroring `artifactComponentDirs`. `🎭️modes`, `📌️panels`,
  `🗣️terminology`, `🌉️wasm`, `📚️examples` stay structural-only in `appChildDirs`.
- new `configChildDirs: ["🧬️schema"]` and `presenceChildDirs: ["🧬️schema"]`, following the
  `snapshotChildDirs`/`diffChildDirs` precedent.
- `taxonomyLeafParentDirs`: add `🎚️config` and `👥️presence`.
- new `appSchemaSpecFilenames: {"🎚️config/🧬️schema": "🔣️component.json", "👥️presence/🧬️schema": "🔣️component.json"}`,
  the app twin of `artifactSchemaSpecFilenames`.

Three twins must move with it: `validateTaxonomy` in the discovery library `🟦️component.ts` (extend the
`SchemaFacetContract` region and reuse `artifactFacetPathIsDeclared`'s tree walk), `validateTaxonomyTree`
in the registry `📜️script.ts`, and `assert_taxonomy_components` in the plugin Rust component.

## 11. Per-plugin wiring

Each plugin's `📦️glue.rs` mounts the new modules: `apps::<a>::config::schema`,
`apps::<a>::presence`, `apps::<a>::presence::schema` — and for norm, `config::schema` /
`presence::schema` at plugin level. Where a `🧮️config` dir was renamed, the existing `#[path]` for
`config` changes with it. The TS glue mirrors it. Each agent owns exactly one plugin's glue file, so this
is conflict-free.

## 12. Wave protocol

Every agent, without exception:

1. reads this spec and the pilot leaves in §13 before writing anything;
2. writes leaves for **its owners only** and touches no shared file other than its own plugin's
   `📦️glue.rs`;
3. gates with, in order — `cargo check -p <crate>`, `cargo test -p <crate> --lib`, and the app-schema
   policy scoped to its plugin:

```
bun -e 'const m = await import("./📜️script.ts");
const b = m.policyAppSchemaBreaches(process.cwd()).filter(x => x.scope.includes("<plugin-dir>"));
console.log(b.length); for (const x of b) console.log(x.kind, x.summary);'
```

Do **not** gate with `bun ./📜️script.ts policy` and grep for your plugin. That command reports 1173
pre-existing breaches from unrelated rule families, and during the artifact ticket every fan-out agent
that grepped it mistook unrelated output for a pass. Call the scanner directly, as above.

On macOS, Rust link steps need `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.

## 13. Pilot leaves — lowpoly, verbatim
The pilot fills this section with its ten finished leaves before the fan-out starts. Fan-out agents diff their output against these rather than improvising.

Owner: `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/`.

Runtime presence document + mutation (sibling of the presence schema facet):

### `👥️presence/🦀️component.rs`

```rust
//! 👥️ Lowpoly presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::DocumentPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of lowpoly view state (selection, hover, camera, active utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "lowpoly.presence")]
#[dsl(layout = "lines")]
pub struct LowpolyPresence {
    pub selection_mode: String,
    pub selection_ids: Vec<u32>,
    pub selection_targets_mesh: bool,
    pub selection_targets_vertex: bool,
    pub selection_targets_edge: bool,
    pub selection_targets_face: bool,
    pub selected_object_ids: Vec<String>,
    pub hovered_object_id: Option<String>,
    pub hovered_target_object_id: Option<String>,
    pub hovered_target_mode: Option<String>,
    pub hovered_target_id: Option<u32>,
    pub world_camera_position: [f64; 3],
    pub world_camera_target: [f64; 3],
    pub world_camera_fov: f64,
    pub active_utility_id: String,
    pub paint_utility: String,
}

impl Default for LowpolyPresence {
    fn default() -> Self {
        Self {
            selection_mode: "object".into(),
            selection_ids: Vec::new(),
            selection_targets_mesh: true,
            selection_targets_vertex: false,
            selection_targets_edge: false,
            selection_targets_face: false,
            selected_object_ids: Vec::new(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
            world_camera_position: [2.5, 2.0, 2.5],
            world_camera_target: [0.0, 0.0, 0.0],
            world_camera_fov: 50.0,
            active_utility_id: String::new(),
            paint_utility: "brush".into(),
        }
    }
}

impl protocol::MutationDiff<LowpolyPresence> for LowpolyPresence {
    fn apply(&self, _base: &LowpolyPresence) -> LowpolyPresence {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::DocumentDsl for LowpolyPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
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
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl DocumentPack for LowpolyPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum LowpolyPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: LowpolyPresence,
    },
}

impl Mutation<LowpolyPresence> for LowpolyPresenceMutation {
    type Diff = LowpolyPresence;

    fn diff(&self, _base: &LowpolyPresence) -> LowpolyPresence {
        match self {
            Self::Snapshot { presence } => presence.clone(),
        }
    }

    fn inverse(&self, base: &LowpolyPresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for LowpolyPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() {
                    line[keyword.len()..].trim_start()
                } else {
                    ""
                };
                let record = dsl::parse(
                    body,
                    &spec_fn(),
                    &dsl::ParseOptions {
                        limits: dsl::Limits::default(),
                        mode: dsl::SourceMode::Inline,
                    },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for LowpolyPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
```

### Schema facet leaves

#### `🎚️config/🧬️schema/🦀️component.rs`

```rust
//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly.config")]
pub struct LowpolyConfig {
    #[state(local_ui)] pub active_object_id: String,
    #[state(local_ui)] pub selection_mode: String,
    #[state(local_ui)] pub selection_ids: Vec<u32>,
    #[state(local_ui)] pub selection_targets_mesh: bool,
    #[state(local_ui)] pub selection_targets_vertex: bool,
    #[state(local_ui)] pub selection_targets_edge: bool,
    #[state(local_ui)] pub selection_targets_face: bool,
    #[state(local_ui)] pub selection_keys: Vec<String>,
    #[state(local_ui)] pub paint_utility: String,
    #[state(local_ui)] pub active_paint_layer: u32,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub selected_object_ids: Vec<String>,
    #[state(local_ui)] pub hovered_object_id: Option<String>,
    #[state(local_ui)] pub hovered_target_object_id: Option<String>,
    #[state(local_ui)] pub hovered_target_mode: Option<String>,
    #[state(local_ui)] pub hovered_target_id: Option<u32>,
    #[state(local_ui)] pub utility_params_json: String,
    #[state(local_ui)] pub paint_color_r: u8,
    #[state(local_ui)] pub paint_color_g: u8,
    #[state(local_ui)] pub paint_color_b: u8,
    #[state(local_ui)] pub paint_color_a: u8,
    #[state(local_ui)] pub world_camera_position: [f64; 3],
    #[state(local_ui)] pub world_camera_target: [f64; 3],
    #[state(local_ui)] pub world_camera_fov: f64,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub show_edges: bool,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}
```

#### `🎚️config/🧬️schema/🟦️component.ts`

```typescript
/** 🧬️ LowpolyConfig */
export interface LowpolyConfig {
  /** @state local-ui */
  activeObjectId: string;
  /** @state local-ui */
  selectionMode: string;
  /** @state local-ui */
  selectionIds: number[];
  /** @state local-ui */
  selectionTargetsMesh: boolean;
  /** @state local-ui */
  selectionTargetsVertex: boolean;
  /** @state local-ui */
  selectionTargetsEdge: boolean;
  /** @state local-ui */
  selectionTargetsFace: boolean;
  /** @state local-ui */
  selectionKeys: string[];
  /** @state local-ui */
  paintUtility: string;
  /** @state local-ui */
  activePaintLayer: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionModeDefault: string;
  /** @state local-ui */
  selectedObjectIds: string[];
  /** @state local-ui */
  hoveredObjectId?: string;
  /** @state local-ui */
  hoveredTargetObjectId?: string;
  /** @state local-ui */
  hoveredTargetMode?: string;
  /** @state local-ui */
  hoveredTargetId?: number;
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
  worldCameraPosition: number[];
  /** @state local-ui */
  worldCameraTarget: number[];
  /** @state local-ui */
  worldCameraFov: number;
  /** @state local-ui */
  engagementInput: string;
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
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
}
```

#### `🎚️config/🧬️schema/🔗️component.graphql`

```graphql
type LowpolyConfig {
  activeObjectId: String! @state(class: LOCAL_UI)
  selectionMode: String! @state(class: LOCAL_UI)
  selectionIds: [Int!]! @state(class: LOCAL_UI)
  selectionTargetsMesh: Boolean! @state(class: LOCAL_UI)
  selectionTargetsVertex: Boolean! @state(class: LOCAL_UI)
  selectionTargetsEdge: Boolean! @state(class: LOCAL_UI)
  selectionTargetsFace: Boolean! @state(class: LOCAL_UI)
  selectionKeys: [String!]! @state(class: LOCAL_UI)
  paintUtility: String! @state(class: LOCAL_UI)
  activePaintLayer: Int! @state(class: LOCAL_UI)
  selectionMethod: String! @state(class: LOCAL_UI)
  selectionModeDefault: String! @state(class: LOCAL_UI)
  selectedObjectIds: [String!]! @state(class: LOCAL_UI)
  hoveredObjectId: String @state(class: LOCAL_UI)
  hoveredTargetObjectId: String @state(class: LOCAL_UI)
  hoveredTargetMode: String @state(class: LOCAL_UI)
  hoveredTargetId: Int @state(class: LOCAL_UI)
  utilityParamsJson: String! @state(class: LOCAL_UI)
  paintColorR: Int! @state(class: LOCAL_UI)
  paintColorG: Int! @state(class: LOCAL_UI)
  paintColorB: Int! @state(class: LOCAL_UI)
  paintColorA: Int! @state(class: LOCAL_UI)
  worldCameraPosition: [Float!]! @state(class: LOCAL_UI)
  worldCameraTarget: [Float!]! @state(class: LOCAL_UI)
  worldCameraFov: Float! @state(class: LOCAL_UI)
  engagementInput: String! @state(class: LOCAL_UI)
  showEdges: Boolean! @state(class: LOCAL_UI)
  sunEnabled: Boolean! @state(class: LOCAL_UI)
  sunAzimuth: Float! @state(class: LOCAL_UI)
  sunElevation: Float! @state(class: LOCAL_UI)
  sunIntensity: Float! @state(class: LOCAL_UI)
  sunColor: String! @state(class: LOCAL_UI)
  activeUtilityId: String! @state(class: LOCAL_UI)
  locale: String! @state(class: LOCAL_UI)
}
```

#### `🎚️config/🧬️schema/🔣️component.json`

```json
{
  "$id": "https://semio.tech/schema/app/lowpoly/lowpoly/config.json",
  "title": "LowpolyConfig",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "activeObjectId",
    "selectionMode",
    "selectionIds",
    "selectionTargetsMesh",
    "selectionTargetsVertex",
    "selectionTargetsEdge",
    "selectionTargetsFace",
    "selectionKeys",
    "paintUtility",
    "activePaintLayer",
    "selectionMethod",
    "selectionModeDefault",
    "selectedObjectIds",
    "utilityParamsJson",
    "paintColorR",
    "paintColorG",
    "paintColorB",
    "paintColorA",
    "worldCameraPosition",
    "worldCameraTarget",
    "worldCameraFov",
    "engagementInput",
    "showEdges",
    "sunEnabled",
    "sunAzimuth",
    "sunElevation",
    "sunIntensity",
    "sunColor",
    "activeUtilityId",
    "locale"
  ],
  "properties": {
    "activeObjectId": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "selectionMode": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "selectionIds": {
      "type": "array",
      "items": {
        "type": "integer",
        "minimum": 0
      },
      "x-semio-state": "local-ui"
    },
    "selectionTargetsMesh": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "selectionTargetsVertex": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "selectionTargetsEdge": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "selectionTargetsFace": {
      "type": "boolean",
      "x-semio-state": "local-ui"
    },
    "selectionKeys": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "x-semio-state": "local-ui"
    },
    "paintUtility": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "activePaintLayer": {
      "type": "integer",
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
    "selectedObjectIds": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "x-semio-state": "local-ui"
    },
    "hoveredObjectId": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "hoveredTargetObjectId": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "hoveredTargetMode": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "hoveredTargetId": {
      "type": "integer",
      "minimum": 0,
      "x-semio-state": "local-ui"
    },
    "utilityParamsJson": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "paintColorR": {
      "type": "integer",
      "minimum": 0,
      "maximum": 255,
      "x-semio-state": "local-ui"
    },
    "paintColorG": {
      "type": "integer",
      "minimum": 0,
      "maximum": 255,
      "x-semio-state": "local-ui"
    },
    "paintColorB": {
      "type": "integer",
      "minimum": 0,
      "maximum": 255,
      "x-semio-state": "local-ui"
    },
    "paintColorA": {
      "type": "integer",
      "minimum": 0,
      "maximum": 255,
      "x-semio-state": "local-ui"
    },
    "worldCameraPosition": {
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 3,
      "maxItems": 3,
      "x-semio-state": "local-ui"
    },
    "worldCameraTarget": {
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 3,
      "maxItems": 3,
      "x-semio-state": "local-ui"
    },
    "worldCameraFov": {
      "type": "number",
      "x-semio-state": "local-ui"
    },
    "engagementInput": {
      "type": "string",
      "x-semio-state": "local-ui"
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
      "x-semio-state": "local-ui"
    },
    "sunElevation": {
      "type": "number",
      "x-semio-state": "local-ui"
    },
    "sunIntensity": {
      "type": "number",
      "x-semio-state": "local-ui"
    },
    "sunColor": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "activeUtilityId": {
      "type": "string",
      "x-semio-state": "local-ui"
    },
    "locale": {
      "type": "string",
      "x-semio-state": "local-ui"
    }
  }
}
```

#### `🎚️config/🧬️schema/🛰️component.proto`

```protobuf
syntax = "proto3";
package semio.app.lowpoly.lowpoly;
message LowpolyConfig {
  // @state local-ui
  string active_object_id = 1;
  // @state local-ui
  string selection_mode = 2;
  // @state local-ui
  repeated uint32 selection_ids = 3;
  // @state local-ui
  bool selection_targets_mesh = 4;
  // @state local-ui
  bool selection_targets_vertex = 5;
  // @state local-ui
  bool selection_targets_edge = 6;
  // @state local-ui
  bool selection_targets_face = 7;
  // @state local-ui
  repeated string selection_keys = 8;
  // @state local-ui
  string paint_utility = 9;
  // @state local-ui
  uint32 active_paint_layer = 10;
  // @state local-ui
  string selection_method = 11;
  // @state local-ui
  string selection_mode_default = 12;
  // @state local-ui
  repeated string selected_object_ids = 13;
  // @state local-ui
  optional string hovered_object_id = 14;
  // @state local-ui
  optional string hovered_target_object_id = 15;
  // @state local-ui
  optional string hovered_target_mode = 16;
  // @state local-ui
  optional uint32 hovered_target_id = 17;
  // @state local-ui
  string utility_params_json = 18;
  // @state local-ui
  uint32 paint_color_r = 19;
  // @state local-ui
  uint32 paint_color_g = 20;
  // @state local-ui
  uint32 paint_color_b = 21;
  // @state local-ui
  uint32 paint_color_a = 22;
  // @state local-ui
  repeated double world_camera_position = 23;
  // @state local-ui
  repeated double world_camera_target = 24;
  // @state local-ui
  double world_camera_fov = 25;
  // @state local-ui
  string engagement_input = 26;
  // @state local-ui
  bool show_edges = 27;
  // @state local-ui
  bool sun_enabled = 28;
  // @state local-ui
  double sun_azimuth = 29;
  // @state local-ui
  double sun_elevation = 30;
  // @state local-ui
  double sun_intensity = 31;
  // @state local-ui
  string sun_color = 32;
  // @state local-ui
  string active_utility_id = 33;
  // @state local-ui
  string locale = 34;
}
```

#### `👥️presence/🧬️schema/🦀️component.rs`

```rust
//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly.presence")]
pub struct LowpolyPresence {
    #[state(shared_ui)] pub selection_mode: String,
    #[state(shared_ui)] pub selection_ids: Vec<u32>,
    #[state(shared_ui)] pub selection_targets_mesh: bool,
    #[state(shared_ui)] pub selection_targets_vertex: bool,
    #[state(shared_ui)] pub selection_targets_edge: bool,
    #[state(shared_ui)] pub selection_targets_face: bool,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_target_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_target_mode: Option<String>,
    #[state(shared_ui)] pub hovered_target_id: Option<u32>,
    #[state(shared_ui)] pub world_camera_position: [f64; 3],
    #[state(shared_ui)] pub world_camera_target: [f64; 3],
    #[state(shared_ui)] pub world_camera_fov: f64,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub paint_utility: String,
}
```

#### `👥️presence/🧬️schema/🟦️component.ts`

```typescript
/** 🧬️ LowpolyPresence */
export interface LowpolyPresence {
  /** @state shared-ui */
  selectionMode: string;
  /** @state shared-ui */
  selectionIds: number[];
  /** @state shared-ui */
  selectionTargetsMesh: boolean;
  /** @state shared-ui */
  selectionTargetsVertex: boolean;
  /** @state shared-ui */
  selectionTargetsEdge: boolean;
  /** @state shared-ui */
  selectionTargetsFace: boolean;
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  hoveredObjectId?: string;
  /** @state shared-ui */
  hoveredTargetObjectId?: string;
  /** @state shared-ui */
  hoveredTargetMode?: string;
  /** @state shared-ui */
  hoveredTargetId?: number;
  /** @state shared-ui */
  worldCameraPosition: number[];
  /** @state shared-ui */
  worldCameraTarget: number[];
  /** @state shared-ui */
  worldCameraFov: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  paintUtility: string;
}
```

#### `👥️presence/🧬️schema/🔗️component.graphql`

```graphql
type LowpolyPresence {
  selectionMode: String! @state(class: SHARED_UI)
  selectionIds: [Int!]! @state(class: SHARED_UI)
  selectionTargetsMesh: Boolean! @state(class: SHARED_UI)
  selectionTargetsVertex: Boolean! @state(class: SHARED_UI)
  selectionTargetsEdge: Boolean! @state(class: SHARED_UI)
  selectionTargetsFace: Boolean! @state(class: SHARED_UI)
  selectedObjectIds: [String!]! @state(class: SHARED_UI)
  hoveredObjectId: String @state(class: SHARED_UI)
  hoveredTargetObjectId: String @state(class: SHARED_UI)
  hoveredTargetMode: String @state(class: SHARED_UI)
  hoveredTargetId: Int @state(class: SHARED_UI)
  worldCameraPosition: [Float!]! @state(class: SHARED_UI)
  worldCameraTarget: [Float!]! @state(class: SHARED_UI)
  worldCameraFov: Float! @state(class: SHARED_UI)
  activeUtilityId: String! @state(class: SHARED_UI)
  paintUtility: String! @state(class: SHARED_UI)
}
```

#### `👥️presence/🧬️schema/🔣️component.json`

```json
{
  "$id": "https://semio.tech/schema/app/lowpoly/lowpoly/presence.json",
  "title": "LowpolyPresence",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "selectionMode",
    "selectionIds",
    "selectionTargetsMesh",
    "selectionTargetsVertex",
    "selectionTargetsEdge",
    "selectionTargetsFace",
    "selectedObjectIds",
    "worldCameraPosition",
    "worldCameraTarget",
    "worldCameraFov",
    "activeUtilityId",
    "paintUtility"
  ],
  "properties": {
    "selectionMode": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "selectionIds": {
      "type": "array",
      "items": {
        "type": "integer",
        "minimum": 0
      },
      "x-semio-state": "shared-ui"
    },
    "selectionTargetsMesh": {
      "type": "boolean",
      "x-semio-state": "shared-ui"
    },
    "selectionTargetsVertex": {
      "type": "boolean",
      "x-semio-state": "shared-ui"
    },
    "selectionTargetsEdge": {
      "type": "boolean",
      "x-semio-state": "shared-ui"
    },
    "selectionTargetsFace": {
      "type": "boolean",
      "x-semio-state": "shared-ui"
    },
    "selectedObjectIds": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "x-semio-state": "shared-ui"
    },
    "hoveredObjectId": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "hoveredTargetObjectId": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "hoveredTargetMode": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "hoveredTargetId": {
      "type": "integer",
      "minimum": 0,
      "x-semio-state": "shared-ui"
    },
    "worldCameraPosition": {
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 3,
      "maxItems": 3,
      "x-semio-state": "shared-ui"
    },
    "worldCameraTarget": {
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 3,
      "maxItems": 3,
      "x-semio-state": "shared-ui"
    },
    "worldCameraFov": {
      "type": "number",
      "x-semio-state": "shared-ui"
    },
    "activeUtilityId": {
      "type": "string",
      "x-semio-state": "shared-ui"
    },
    "paintUtility": {
      "type": "string",
      "x-semio-state": "shared-ui"
    }
  }
}
```

#### `👥️presence/🧬️schema/🛰️component.proto`

```protobuf
syntax = "proto3";
package semio.app.lowpoly.lowpoly;
message LowpolyPresence {
  // @state shared-ui
  string selection_mode = 1;
  // @state shared-ui
  repeated uint32 selection_ids = 2;
  // @state shared-ui
  bool selection_targets_mesh = 3;
  // @state shared-ui
  bool selection_targets_vertex = 4;
  // @state shared-ui
  bool selection_targets_edge = 5;
  // @state shared-ui
  bool selection_targets_face = 6;
  // @state shared-ui
  repeated string selected_object_ids = 7;
  // @state shared-ui
  optional string hovered_object_id = 8;
  // @state shared-ui
  optional string hovered_target_object_id = 9;
  // @state shared-ui
  optional string hovered_target_mode = 10;
  // @state shared-ui
  optional uint32 hovered_target_id = 11;
  // @state shared-ui
  repeated double world_camera_position = 12;
  // @state shared-ui
  repeated double world_camera_target = 13;
  // @state shared-ui
  double world_camera_fov = 14;
  // @state shared-ui
  string active_utility_id = 15;
  // @state shared-ui
  string paint_utility = 16;
}
```

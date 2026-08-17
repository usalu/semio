# 🧪 Wave-5 report — `semio-s-plugin-flow` / artifact `flow`

## Artifact

| | |
|---|---|
| Plugin folder | `✏️s/🔌️plugins/🌊️flow/` |
| Crate | `semio-s-plugin-flow` |
| Artifact key | `flow` |
| Prefix | `Flow` |
| Schema id | `s.flow.flow` |
| Former snapshot type | `FlowFixture` (plugin-local; **not** `flow::FlowFixture`) |
| New snapshot type | `FlowSnapshot` |

## Field inventory (`FlowArtifact`)

| Field | State class | Notes |
|---|---|---|
| `schema` | persistent | Document schema string (`flow.fixture` today) |
| `camera` | persistent | `CameraJson` {x,y,zoom} |
| `widgets` | persistent | `Vec<Widget>` (opaque JSON string in non-Rust leaves) |
| `synapses` | persistent | `Vec<SynapseSpec>` |
| `layout` | persistent | `BTreeMap<String, WidgetLayout>` |
| `selectedNodeIds` | shared-ui | From `FlowConfig` selection |
| `selectedEdgeIds` | shared-ui | |
| `selectedHandleIds` | shared-ui | |
| `previewOffNodeIds` | shared-ui | |
| `lodMode` | local-ui | |
| `proximityDistance` | local-ui | |
| `gridVisible` | local-ui | |
| `gridSnapEnabled` | local-ui | |
| `gridFactor` | local-ui | |
| `catalogueSectionsJson` | local-ui | JSON blob |
| `automationEnabledJson` | local-ui | JSON blob |
| `contributionsJson` | local-ui | JSON blob |
| `generationJson` | local-ui | JSON blob |
| `locale` | local-ui | |

`FlowSnapshot` = exactly the five persistent fields. Config live camera is not duplicated as a second artifact field; the document camera is the persistent one.

## Diff shape (`FlowDiff`)

Sparse optional entries for every non-effect artifact field, plus whole-replacement `artifact?: FlowArtifact` that wins over field entries.

- Identified collections `widgets` / `synapses` → `added` / `removed` / `patched` / optional `reordered`
- `layout` → `FlowLayoutMapDelta { entries: map id → WidgetLayout | null }`
- Selection lists → `FlowStringList { values }` (optional-list record wrapper)
- Scalars (`schema`, `camera`, lod/grid/locale/json blobs, …) → optional field of the same type

Implements `MutationDiff<FlowSnapshot>` over persistent entries and `apply_to_artifact` over all entries. `absorb` merges field-wise. Mutations under `🧬️mutations/` (including renamed `📄set-snapshot`) construct this delta; inverses still round-trip.

## Pack relocation

`🗿️artifacts/🌊️flow/🎒️pack/` → `🗿️artifacts/🌊️flow/📸️snapshot/🎒️pack/`. Protocol segment is `Snapshot` (kind 3).

## Glue convention

Cumulative `#[path]` nesting already used by this crate; extended one level:

- `artifacts::flow::schema`
- `artifacts::flow::snapshot::{schema, pack}`
- `artifacts::flow::diff::{component, schema}`
- `artifacts::flow::mutations::set_snapshot`
- `extern crate semio_framework_schema as schema`

TypeScript `📦️index.ts` mirrors the same nesting.

## Document codecs / envelope

`FlowSnapshot` DocumentDsl/DocumentPack use envelope id **`flow.flow`** with a **JSON body**. They do **not** call `flow::FlowFixture::{print_dsl,encode_pack}` because the framework type still derives envelope id `flow`, which `SemioEnvelope::from_envelope_id` rejects (`plugin.artifact` required). Parse still accepts framework DSL bodies as a read fallback. Example fixture rewritten to a real round-tripping `semio flow.flow.dsl v1` + JSON document.

Config envelope id: `flow.config`.

## Engine

`FlowEngine` owns a real `FlowArtifact` (`type Artifact = FlowArtifact`) and returns the persisted subset from `snapshot()`. Host bridging uses `from_fixture` / `to_fixture` against framework `flow::FlowFixture`.

## Disambiguating the two `FlowFixture` types

| | Plugin (this work) | Framework (`semio-framework-os-flow`) |
|---|---|---|
| Former / current name | was plugin `FlowFixture` → **`FlowSnapshot`** | still **`flow::FlowFixture`** |
| Crate | `semio-s-plugin-flow` | `semio-framework-os-flow` |
| How told apart | Path under `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/…`; type now named `FlowSnapshot`; bridge methods `from_fixture`/`to_fixture` | Import path `flow::FlowFixture`; used by `FlowHost`, framework VCS/DSL, procedural plugins |
| Role | Persisted plugin document / `DocumentApp` snapshot | Kernel host document (isomorphic fields) |

Neither is a test fixture; the old `*Fixture` name was the persisted document.

## App / command fixes applied preemptively

- `DocumentApp::handle` already takes `DraftView` + `EngineHandles`
- Tests use `store::os_store::test_support::*`
- Removed duplicate `#[dsl(keyword = …)]` on command payloads (enum `app_commands!` already supplies the wire keyword) — fixed double-printed `add-widget add-widget`
- Updated `optional_field_rows_keep_their_pre_migration_bytes` SetGridVisible binary ordinals `0x17` → `0x18` (one variant inserted earlier in the enum vs pre-merge goldens)
- Mutations serde tag `mutation`; `ViewModel` naming already in place

## Gates (verbatim tails)

### `cargo check -p semio-s-plugin-flow`

```
warning: `semio-s-plugin-flow` (lib) generated 9 warnings (run `cargo fix --lib -p semio-s-plugin-flow` to apply 9 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.06s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### `cargo test -p semio-s-plugin-flow --lib`

```
test result: ok. 84 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
```

### `bun ./📜️script.ts policy 2>&1 | rg -i 'flow'`

```
(empty — 0 lines)
```

Confirmed with direct `policyArtifactSchemaBreaches(root)` → **0** flow-scoped breaches after TS leaf reorder (facet type first).

## Fixup items (shared surfaces — out of scope)

1. **`semio-framework-os-flow`**: `FlowFixtureDsl` still uses `#[dsl(extension = "flow")]` without `id = "flow.flow"`, so framework `DocumentDsl::print_dsl` / `DocumentPack::encode_pack` panic on `SemioEnvelope::from_envelope_id("flow", …)`. Plugin codecs deliberately do not call them.
2. Framework example `📚️examples/🌊️default.flow` referenced by framework tests appears missing from the tree (path resolves under OS examples and was absent when checked).

## Not validated

- TypeScript/vitest package tests (none gated for this crate in the brief)
- Runtime UI / wasm host behaviour beyond lib tests
- Framework `semio-framework-os-flow` test suite (out of scope; expected red on envelope print until fixup)

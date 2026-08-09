# Wave 5 Report — Energy (`semio-s-plugin-energy`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🔋️energy/**` plus this ticket folder.

| Artifact | Key | Prefix | Schema id | Old → new snapshot |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/🔋️model/` | `model` | `EnergyModel` | `s.energy.model` | `EnergyModelDocument` → `EnergyModelSnapshot` |

Headless library plugin (no DocumentApp). Facet set was structurally incomplete; built the full taxonomy shape from scratch rather than a rename-only pass.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `schema` | persistent | document / DSL envelope id (`energy.model`) |
| `modelJson` | persistent | opaque JSON of `crate::Model` (building inputs) |
| `resultsJson` | preview | opaque JSON of `crate::Results` — recomputed by `sim::Engine::run` |

Snapshot facet = exactly the persistent fields (`schema`, `modelJson`).
Artifact facet = snapshot ∪ preview.
No shared-ui / local-ui / effect fields (no play app / view model).

## 2. Simulation-result state classification

`resultsJson` is **preview**, never persistent — same judgement as FEM `solverResultsJson`:

- Produced by `EnergyModelEngine::run_simulation` via `crate::sim::Engine::run(&Model, &SimulationConfig)`.
- Fully determined by persisted `modelJson` + run config; caching it in the snapshot would duplicate authority with the model and drift on reload.
- Appears on `EnergyModelArtifact` so the engine can surface last-run results without inventing a parallel type; `MutationDiff` applies only persistent entries.

## 3. Diff-delta shape

`EnergyModelDiff` sparse field delta (lowpoly / FEM pattern):

- `artifact: Option<Box<EnergyModelArtifact>>` — whole replacement wins
- persistent: `schema`, `modelJson`
- preview: `resultsJson`
- helpers: `diff_set_snapshot` / `diff_set_model_json` / `diff_set_results_json`
- `MutationDiff<EnergyModelSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes; `absorb` merges field-wise

`SetDocument` → `SetSnapshot { snapshot }` (folder `📄set-document` → `📄set-snapshot`).

## 4. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as note / lowpoly):

- `artifacts::model::schema`
- `artifacts::model::snapshot::{schema, pack}`
- `artifacts::model::diff::{component, schema}` (`pub use super::schema::*`)
- `artifacts::model::{op, dsl, spr, engine, mutations}`

Also nested plugin-root BEM domains: `⚙️engine/<domain>/🦀️component.rs` (was flat `⚙️engine/🦀️<domain>.rs`) so taxonomy leaf naming passes.

TS `📦️index.ts` re-exports the three schema facet types. Cargo depends on `semio-framework-os-kernel`, `semio-framework-schema`, `serde_json`.

## 5. Components created from scratch

Previously present (stub): artifact root, `⚙️engine` (3-line stub), `🔧️op` (re-export only), `🧬️mutations` (+ no-mutation / set-document), `📚️examples`.

**Created:**

- `🧬️schema/` — all five leaves
- `📸️snapshot/🧬️schema/` — all five leaves (`EnergyModelSnapshot`)
- `📸️snapshot/🎒️pack/` — rust/ts/protocol (no prior pack existed)
- `🔺️diff/` — runtime + grammar + ts + all five schema leaves
- `🗣️dsl/` — rust/ts/grammar
- `📡️spr/` — rust/ts/protocol
- `🔧️op` grammar + ts; real OpText/OpBinary impl
- `⚙️engine` TS leaf; real `EnergyModelArtifact` engine
- Mutation TS leaves for `🫙no-mutation` and `📄set-snapshot`
- Mutations grammar

**Moved / renamed:**

- `EnergyModelDocument` → `EnergyModelSnapshot` in snapshot schema (out of mutations)
- `📄set-document` → `📄set-snapshot`
- 50 plugin-root engine domain files nested under `⚙️engine/<name>/🦀️component.rs`

## 6. Gates (verbatim tails)

### `cargo check -p semio-s-plugin-energy`

```
warning: `semio-s-plugin-energy` (lib) generated 1 warning (run `cargo fix --lib -p semio-s-plugin-energy` to apply 1 suggestion)
    Finished `dev` profile [unoptimized] target(s) in 39.84s
```

### `cargo test -p semio-s-plugin-energy --lib`

```
test result: ok. 244 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### `bun ./📜️script.ts policy 2>&1 | rg -i 'energy'`

```
(empty — 0 lines)
```

Direct `policyArtifactSchemaBreaches(root)` filtered for energy / `s.energy.model` → **0** breaches.

### `bun nx run @semio-tech/plugin-registry:check 2>&1 | rg -F '🔋️energy'`

```
(empty — 0 lines)
```

Note: `rg -i 'energy'` on the same registry output still matches an unrelated CAD finding (`📐️cad` … `aec-building-energy`), outside this plugin tree.

## 7. Shared-framework blockers

None that blocked energy facet delivery. Notes for fixup:

1. Repo MCP `ticket_*` / `repo://goals` tools were unavailable in this Cursor session; work used the existing ticket folder without MCP close.
2. Concurrent `rg -i energy` noise from CAD `aec-building-energy` extension (other agent / other plugin).

## 8. Not validated

- Full `bun nx run workspace:verify-gate`
- TypeScript vitest (index re-exports only; example asset length smoke only)
- Interactive UI / playground (library plugin, no DocumentApp)
- End-to-end BEM run through `EnergyModelEngine::run_simulation` on a full Model fixture (domain `sim::` tests already green; artifact engine stores results as preview JSON)

## 9. Files touched (high level)

- `✏️s/🔌️plugins/🔋️energy/⚙️engine/<domain>/🦀️component.rs` (50 nested moves)
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/**` (full facet tree)
- `✏️s/🔌️plugins/🔋️energy/📦️packages/{🦀️rust/{Cargo.toml,📦️glue.rs},🟦️typescript/📦️index.ts}`
- `✏️s/🔌️plugins/🔋️energy/🔌️plugin/🦀️component.rs`
- Ticket: `🧪wave5-energy-*.{md,txt}` probes/logs + this report

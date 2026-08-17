# Wave 5 Report — Norm DIN / ISO five-artifact slice (`semio-s-plugin-norm`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `iso16757`, `vdi3805`, `din4108`, `din16798`, `din18599` under `✏️s/🔌️plugins/📕️norm/**` (not glue/Cargo/TS index).

| Artifact | Key | Prefix | Schema id | Snapshot type |
| --- | --- | --- | --- | --- |
| `📓️iso16757` | `iso16757` | `Iso16757` | `s.norm.iso16757` | `Iso16757Snapshot` |
| `📔️vdi3805` | `vdi3805` | `Vdi3805` | `s.norm.vdi3805` | `Vdi3805Snapshot` |
| `📕️din4108` | `din4108` | `Din4108` | `s.norm.din4108` | `Din4108Snapshot` |
| `📗️din16798` | `din16798` | `Din16798` | `s.norm.din16798` | `Din16798Snapshot` |
| `📙️din18599` | `din18599` | `Din18599` | `s.norm.din18599` | `Din18599Snapshot` |

Shared UI on every artifact facet: `selectedCheckIndex` (`Option<u32>`) from `NormConfig`.

## 1. Field inventory (persistent + shared-ui)

### Din4108

Persistent: `category`, `layers` (`LayerDocument[]`), `climate` (`ClimateZoneDe`), `airtightnessN50`, `psiTimesLSum`, `rhInt`, `catalogId`, `materialId`, `airtightnessClass`, `tIntC`, `solarAbsorptance`, `irradianceWM2`, `moistureMuExterior`, `moistureMuInterior`, `envelopeAreaM2`, `bb2DetailsConform`, `applicationType`, `declaredApplicationClass`. Shared-ui: `selectedCheckIndex`.

### Din16798

Persistent: full DIN EN 16798 DSL document (occupancy, comfort, ventilation, HR, cooling, duct leakage, … — all fields on former `Document`). Shared-ui: `selectedCheckIndex`.

### Din18599

Persistent: `BalancingInputs` (`useClass`, `heatedAreaM2`, `occupants`, `hT`, `hV`, `climate` (`MonthlyClimate`), `internalGainsWM2`, `solarGainsKwh`, `systemLossesKwh`, `renewableKwh`, `annualLimitKwh`, `energyCarrier`, `referenceQPKwh`). Shared-ui: `selectedCheckIndex`.

### Iso16757

Persistent: `catalogue`, `dictionary`, `geometry`, `selection`, `partNumberRule`, `partNumberInputs` (`BTreeMap<String, CatalogueValue>`), `scriptLimits`, `exchangeProcess`. Shared-ui: `selectedCheckIndex`.

### Vdi3805

Persistent: `manufacturerFile`, `catalog`, `editionProfile`, `correctionAsOf`, `strictMode`, `index`, `geometry`, `curves`, `limits`. Shared-ui: `selectedCheckIndex`.

Snapshot facet = persistent fields only (equality with artifact persistent slice).

## 2. Diff shape (`XDiff`)

Sparse delta per artifact:

- `artifact: Option<Box<XArtifact>>` — whole replacement wins
- one optional entry per persistent document field (scalars / object blobs / `BTreeMap` maps / `layers` as `Din4108LayerList` where applicable)
- `selectedCheckIndex: Option<Option<u32>>` (shared-ui)

Runtime: `apply_to_artifact`, `MutationDiff<XSnapshot>`, `absorb`, `diff_set_snapshot` in each `🔺️diff/🦀️component.rs` (pattern from `En1990Diff`).

Mutations / engines / apps still largely on legacy `Document` + `SetDocumentMutation` aliases — integrator + follow-up should align with `En1995Mutation::SetSnapshot` pattern.

## 3. Glue / Cargo / index (integrator — verbatim targets)

For each key `K` / folder `🗿️artifacts/<emoji>K/` replace bare pack mount:

```rust
        #[path = "../../🗿️artifacts/<emoji>K/🎒️pack/🦀️component.rs"]
        pub mod pack;
```

with:

```rust
        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/<emoji>K/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
            #[path = "."]
            pub mod schema {
                #[path = "../../🗿️artifacts/<emoji>K/📸️snapshot/🧬️schema/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "../../🗿️artifacts/<emoji>K/🧬️schema/🦀️component.rs"]
        pub mod schema;
```

Emoji folder mapping:

- `iso16757` → `📓️iso16757`
- `vdi3805` → `📔️vdi3805`
- `din4108` → `📕️din4108`
- `din16798` → `📗️din16798`
- `din18599` → `📙️din18599`

Register descriptors (add to each artifact `⚙️engine/🦀️component.rs` `register_*` or plugin setup):

```rust
schema::register_artifact_schema(crate::artifacts::din4108::schema::din4108_artifact_schema_descriptor());
schema::register_artifact_schema(crate::artifacts::iso16757::schema::iso16757_artifact_schema_descriptor());
schema::register_artifact_schema(crate::artifacts::vdi3805::schema::vdi3805_artifact_schema_descriptor());
schema::register_artifact_schema(crate::artifacts::din16798::schema::din16798_artifact_schema_descriptor());
schema::register_artifact_schema(crate::artifacts::din18599::schema::din18599_artifact_schema_descriptor());
```

`Cargo.toml` / `index.ts`: mirror `gis` — export `snapshot::pack`, `snapshot::schema`, `schema`, diff runtime unchanged path.

## 4. Gate tails (verbatim)

### `bun ./📜️script.ts policy 2>&1 | rg -i 'iso16757|vdi3805|din4108|din16798|din18599'`

```
(empty — no lines matched)
```

### `bun -e policyArtifactSchemaBreaches` (five keys)

```
breach count 0
```

### `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-norm`

```
    Checking semio-s-plugin-norm v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust)
error: couldn't read `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/././../../🗿️artifacts/📕️din4108/🎒️pack/🦀️component.rs`: No such file or directory (os error 2)
  --> ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs:74:9
   |
74 |         pub mod pack;
   |         ^^^^^^^^^^^^^

error: could not compile `semio-s-plugin-norm` (lib) due to 1 previous error
```

**Expected until glue integrator** moves `pack` to `📸️snapshot/🎒️pack` in `📦️glue.rs` (pack relocated on disk for all five artifacts). Further compile errors from `Document` → `XSnapshot` runtime migration are expected after glue is wired.

## 5. Not validated

- Full `cargo check` / `cargo test --lib` after glue + `Document`/`SetSnapshot` runtime sweep (engines, `NormFamily`, apps, `spr`, `set-document` → `set-snapshot`).
- Runtime UI / playground.
- TypeScript vitest for norm package.

## 6. Ticket tooling

- `🧪wave5-norm-din-iso-generate.py` — leaf generation + pack move
- `🧪wave5-norm-din-iso-fix-snapshots.py`, `🧪wave5-norm-din-iso-fix-graphql.py`, `🧪wave5-norm-din-iso-fix-maps.py` — parity fixups

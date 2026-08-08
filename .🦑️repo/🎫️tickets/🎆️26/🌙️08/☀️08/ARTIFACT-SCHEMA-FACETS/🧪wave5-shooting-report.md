# Wave 5 Report — Shooting (`semio-s-plugin-shooting`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🎥️shooting/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/🎥️shooting/` | `shooting` | `Shooting` | `s.shooting.shooting` | `ShootingFixture` → `ShootingSnapshot` |

App: `🎥️shooting` ↔ `shooting` (`type Snapshot = ShootingSnapshot`). Draft lane is `NoDraft`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | former `ShootingFixture.schema` |
| `assets` | persistent | identified `Vec<ShootingAsset>` |
| `savedCameras` | persistent | identified `Vec<ShootingSavedCamera>` |
| `scene` | persistent | `ShootingSceneLighting` record |
| `shots` | persistent | identified `Vec<ShootingShot>` |
| `activeShotId` | persistent | document selection of active shot |
| `activeAssetId` | persistent | document selection of active asset |
| `selectedShotIds` | shared-ui | `ShootingConfig` |
| `selectedAssetIds` | shared-ui | `ShootingConfig` |
| `activeUtilityId` | shared-ui | `ShootingConfig` (gumball utility) |
| `defaultShotFormat` | local-ui | sticky shot-format default |
| `defaultShotShape` | local-ui | sticky shot-shape default |
| `defaultAssetFormat` | local-ui | sticky asset-format default |
| `selectionMethod` | local-ui | marquee method |
| `centerModel` | local-ui | viewport fit toggle |
| `fitRevision` | local-ui | fit re-trigger counter |
| `cameraDraftLabel` | local-ui | in-progress save-camera label |
| `camera` | local-ui | free/live viewport camera |
| `locale` | local-ui | BCP-47 |
| `hoveredAssetId` | preview | hover highlight |

Snapshot facet = the seven persistent fields exactly.

## 2. Diff-delta shape

`ShootingDiff` sparse field delta:

- `artifact: Option<Box<ShootingArtifact>>` — whole replacement wins
- persistent: `schema`, `assets` / `savedCameras` / `shots` as `Option<Shooting*Delta>` (`added`/`removed`/`patched`/`reordered`), `scene: Option<ShootingSceneLighting>`, `activeShotId`, `activeAssetId`
- shared-ui: `selectedShotIds` / `selectedAssetIds` as `Option<ShootingStringList>`, `activeUtilityId`
- local-ui: defaults, selection method, center/fit/camera draft/camera/locale
- preview: `hoveredAssetId: Option<Option<String>>`

`MutationDiff<ShootingSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise. `📄set-fixture` → `📄set-snapshot` (`SetSnapshot { snapshot }`).

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as lowpoly / gis):

- `artifacts::shooting::schema`
- `artifacts::shooting::snapshot::{schema, pack}`
- `artifacts::shooting::diff::{component, schema}` (`pub use super::schema::*`)

TypeScript `📦️packages/🟦️typescript/📦️index.ts` mirrors pack under snapshot plus the three schema facet exports. Dependency: `semio-framework-schema` (`extern crate … as schema`).

## 4. Other structural changes

- Fifteen handcrafted leaves (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `ShootingFixture` removed; `ShootingSnapshot` lives in snapshot schema and is re-exported from the artifact root
- Document schema constant `SHOOTING_DOCUMENT_SCHEMA = "shooting.shooting"`; config envelope `shooting.config`
- Engine owns real `ShootingArtifact` + `ShootingSnapshot`; `ArtifactEngine::{Artifact, Snapshot, artifact, snapshot}`
- Example `.dsl.semio` is a real round-tripping base-icon document (not a stub)
- `DocumentApp` / views / tests: `Projection` → `Snapshot`, `.projection` → `.snapshot`, `store::os_store::test_support::*`

## 5. Gate tails (verbatim)

### cargo check

```
warning: `semio-s-plugin-shooting` (lib) generated 7 warnings (run `cargo fix --lib -p semio-s-plugin-shooting` to apply 7 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 10.28s
```

### cargo test --lib

```
test result: ok. 95 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### policy `| rg -i shooting`

(empty)

Direct confirm: `policyArtifactSchemaBreaches(root)` filtered for `shooting` → **0** breaches.

## 6. Shared-surface blockers

None that blocked completion. Repo MCP (`ticket_*` / `repo://goals`) was unavailable in this agent session; work proceeded inside the existing ticket folder.

## 7. Not validated

- Full `bun ./📜️script.ts policy` human-readable stdout beyond the shooting filter (CLI can be silent when piped; confirmed via direct `policyArtifactSchemaBreaches`)
- TypeScript vitest package run (nx wrappers are budget-limited; no TS unit surface beyond index re-exports was required for the gates)
- Interactive UI / playground runtime beyond lib tests

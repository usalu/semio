# Wave 5 Report — Note (`semio-s-plugin-note`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🗒️note/**` plus this ticket folder.
Key `note`, prefix `Note`, schema id `s.note.note`. Former snapshot type `NoteDocument` → `NoteSnapshot`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | document |
| `id` | persistent | document |
| `title` | persistent | document |
| `blocks` | persistent | `Vec<NoteBlockNode>` tree |
| `gridVisible` | persistent | document (grid settings) |
| `gridSpacing` | persistent | document |
| `gridSubdivisions` | persistent | document |
| `gridOpacity` | persistent | document |
| `snapEnabled` | persistent | document |
| `snapGridSpacing` | persistent | document |
| `pencilWidth` | persistent | document |
| `eraserRadius` | persistent | document |
| `assets` | persistent | `BTreeMap` image assets |
| `selectedBlockIds` | shared-ui | `NoteConfig` |
| `activeUtilityId` | shared-ui | `NoteConfig` |
| `engagementInput` | local-ui | `NoteConfig` |
| `cameraX` / `cameraY` / `cameraZoom` | local-ui | flattened from `NoteCamera` |
| `locale` | local-ui | `NoteConfig` |
| `hoveredBlockId` | preview | `NoteConfig` |

Snapshot facet = the twelve persistent document fields exactly (`schema` through `assets`).

## 2. Diff-delta shape

`NoteDiff` sparse field delta:

- `artifact: Option<Box<NoteArtifact>>` — whole replacement wins
- persistent: scalars as `Option<Option<T>>` where needed; `blocks: Option<NoteBlocksDelta>`; `assets: Option<NoteAssetsDelta>`
- shared-ui: `selectedBlockIds: Option<NoteStringList>`, `activeUtilityId`
- local-ui: `engagementInput`, `cameraX/Y/Zoom`, `locale`
- preview: `hoveredBlockId: Option<Option<String>>`

`NoteBlocksDelta`: `added` / `removed` / `patched` / `reordered`. `MutationDiff<NoteSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes. `absorb` merges field-wise. `SetDocument` → `SetSnapshot { snapshot }`.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`:

- `artifacts::note::schema`
- `artifacts::note::snapshot::{schema, pack}`
- `artifacts::note::diff::{component, schema}` (`pub use super::schema::*`)

TypeScript `📦️index.ts` mirrors snapshot pack path and three schema facet exports.

## 4. Other structural changes

- Fifteen handcrafted schema leaves under `🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `NoteEngine` owns `NoteArtifact` + `NoteSnapshot`; `ArtifactEngine` API
- `DocumentApp` uses `Snapshot` / `.snapshot` / `initial_snapshot`; config envelope `note.config`
- `note_artifact_schema_descriptor()` registered from `engine::register()`
- Real `🗣️example.dsl.semio` round-trip fixture; SPR `tag=N` form
- Tests: `InvocationResult.mutations`, `store::os_store::test_support`, canvas events `mutation` tag

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-note

```
warning: `semio-s-plugin-note` (lib) generated 7 warnings (run `cargo fix --lib -p semio-s-plugin-note` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 34.00s
```

### cargo test -p semio-s-plugin-note --lib

```
test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'note'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches(root)` filter for `note` → **0** breaches.

## 6. Shared-surface blockers

- Concurrent workspace breakage on `semio-s-plugin-imperative` (missing `📦️glue.rs`) intermittently blocked `cargo` workspace resolution; note crate gates succeed when workspace loads.
- Repo MCP goals/ticket tools unavailable this session; work used existing ticket folder.

## 7. Not validated

- Full `bun nx run workspace:verify-gate`
- TypeScript vitest package run (index re-exports only)
- Interactive playground / UI smoke beyond lib tests

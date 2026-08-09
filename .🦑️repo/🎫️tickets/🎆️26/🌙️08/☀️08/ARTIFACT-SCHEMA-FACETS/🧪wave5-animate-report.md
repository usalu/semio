# Wave 5 Report — Animate (`semio-s-plugin-animate`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🎞️animate/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/🎬️present/` | `present` | `Present` | `s.animate.present` | `PresentDeck` → `PresentSnapshot` |

App: `🎬️present` ↔ `present` (`type Snapshot = PresentSnapshot`). Config envelope `present.config`. Document envelope `animate.present`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | document schema id |
| `source` | persistent | shared `FigureTileSource` |
| `tiles` | persistent | identified `Vec<FigureTileDraft>` |
| `selectedIds` | shared-ui | `PresentConfig` |
| `engagementInput` | local-ui | `PresentConfig` |
| `locale` | local-ui | `PresentConfig` |

Playback position and Reveal `currentSlide` DOM state live only in `📺️renderer/⚛️react` — not in artifact schema (ephemeral preview).

Snapshot facet = `schema`, `source`, `tiles` exactly.

## 2. Diff-delta shape

`PresentDiff` sparse field delta:

- `artifact: Option<Box<PresentArtifact>>` — whole replacement wins
- persistent: `schema`, `source`, `tiles` as `Option<PresentTilesDelta>` (`added` / `removed` / `patched` / `reordered`)
- shared-ui: `selectedIds` as `Option<PresentStringList>`
- local-ui: `engagementInput`, `locale`

`MutationDiff<PresentSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes. `set-deck` → `set-snapshot` (`SetSnapshot { snapshot }`).

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (sequence / lowpoly pilot):

- `artifacts::present::schema`
- `artifacts::present::snapshot::{schema, pack}`
- `artifacts::present::diff::{component, schema}` (`pub use super::schema::*` in runtime)

`engine::animate` nests engine topic submodules (`animation::animation`, `scene::scene`, …) so existing `engine::animate::animation::…` paths keep working; flat re-exports at `animate` root for `AnimateConfig`, `Scene`, etc.

TypeScript `📦️packages/🟦️typescript/📦️index.ts` exports three schema facets and snapshot pack. Dependency: `semio-framework-schema` (`extern crate … as schema`).

## 4. Other structural changes

- Fifteen handcrafted leaves (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`; protocol segment `Snapshot` kind 3
- `PresentDeck` removed; `PRESENT_DOCUMENT_SCHEMA = "animate.present"`
- `PresentEngine` owns real `PresentArtifact` + `PresentSnapshot`; `ArtifactEngine::{Artifact, Snapshot, artifact, snapshot}`
- Example `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` round-trips (canonical `print_dsl` shape)
- `DocumentApp` / views / tests: `Projection` → `Snapshot`, `.projection` → `.snapshot`, `store::os_store::test_support::*`
- SPR protocol: `schema animate.present.mutation`, records `tag=N`; serde `mutation` tag on enum
- `materialize_document_snapshot` (was `materialize_document_projection`)

## 5. Gate tails (verbatim)

### cargo check

```
warning: `semio-s-plugin-animate` (lib) generated 3 warnings (run `cargo fix --lib -p semio-s-plugin-animate` to apply 3 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 32.32s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test --lib

```
test result: ok. 206 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.59s
```

### policy `| rg -i 'present|animate'`

(empty)

Direct confirm: `policyArtifactSchemaBreaches(root)` with scope containing `🎞️animate` or `🎬️present` → **0** breaches.

## 6. Shared-surface blockers (fixup wave)

| Surface | Impact | Workaround in plugin |
| --- | --- | --- |
| `materialize_document_projection` removed from store | SPR helpers | Use `materialize_document_snapshot` |
| `InvocationResult.document_mutations` → `mutations` | Shell/engagement tests | Assert on `result.mutations` |
| Engine topic files use nested `pub mod X { pub mod X { … } }` | Glue `animate` aggregator | Submodule re-exports + selective flat re-exports |
| `TYPST_FONTS` static missing in text engine | Typst labels | Restored `OnceLock` + static `FONT_BOOK` |

## 7. Not validated

- Full `bun ./📜️script.ts policy` stdout (CLI silent when piped; confirmed via `policyArtifactSchemaBreaches` scoped to animate)
- TypeScript vitest / nx package scripts
- WASM playground / interactive UI beyond lib tests
- Repo MCP ticket tools

# Wave 5 Report — Layout (`semio-s-plugin-layout`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/📏️layout/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/📏️layout/` | `layout` | `Layout` | `layout.layout` | `LayoutDocument` → `LayoutSnapshot` |

App: `📏️layout` ↔ `layout` (`type Snapshot = LayoutSnapshot`). Config envelope `layout.config`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` … `dataFieldsJson` | persistent | `LayoutSnapshot` document (grid, styles, stories, links, parent pages, spreads, pages, print target, data fields) |
| `selectedIds` | shared-ui | `LayoutConfig` |
| `activePageId`, `engagementInput`, blueprint/preview cameras, `dropPreview`, `locale` | local-ui | `LayoutConfig` |
| `hoveredId` | preview | `LayoutConfig` |

Snapshot facet = the thirteen persistent fields exactly (matches `LayoutArtifact::to_snapshot`).

## 2. Diff-delta shape

`LayoutDiff` sparse field delta: `artifact: Option<Box<LayoutArtifact>>` wins; persistent collections use `LayoutPagesDelta` / `LayoutStoriesDelta` / `LayoutLinksDelta` / frame ops via `pages_replace_delta`; `SetDataFields` sparse; shared-ui / local-ui / preview entries mirror artifact. `MutationDiff<LayoutSnapshot>` on persistent fields; `apply_to_artifact` on all classes. Mutations under `🧬️mutations/` build deltas; SPR baselines use `mutation` tag and `tag=N` form.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with `extern crate semio_framework_schema as schema`:

- `artifacts::layout::schema`
- `artifacts::layout::snapshot::{schema, pack}`
- `artifacts::layout::diff::{component, schema}`

TypeScript `📦️packages/🟦️typescript/📦️index.ts` exports snapshot pack + three schema facets. Pack protocol segment `layout.layout`; path `📸️snapshot/🎒️pack/`.

## 4. Other structural changes

- Fifteen handcrafted leaves (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- `LayoutDocument` removed; `LAYOUT_DOCUMENT_SCHEMA = "layout.layout"`
- `LayoutArtifactEngine` + `register_artifact_schema()`; `default_document()` / demo DSL fixture (2 pages, master inheritance, preflight link, hit-test frames)
- `🗣️example.dsl.semio` regenerated from `print_dsl(default_document())` — round-trips
- `DocumentApp` / tests: `.snapshot`, `DraftView` + `EngineHandles` in kits, `store::os_store::test_support::*`, `ViewModel`
- Op wire baselines: removed duplicate `#[dsl(keyword)]` on rows where wire keyword already matches (`canvas-pointer-move`, `add-frame`, `set-camera` struct keyword)

## 5. Gate tails (verbatim)

### cargo check

```
warning: `semio-s-plugin-layout` (lib) generated 22 warnings (run `cargo fix --lib -p semio-s-plugin-layout` to apply 14 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.13s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test --lib

```
test artifacts::layout::engine::scene::tests::png_cpu_export_writes_valid_rgba_png ... ok
test artifacts::layout::engine::scene::tests::scene_png_from_display_list_writes_a_valid_png ... ok

test result: ok. 116 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.56s
```

### policy (`rg -i layout`)

```
```

(Confirmed `policyArtifactSchemaBreaches(root)` filtered for layout: **0** breaches.)

## 6. Shared-surface blockers (fixup wave)

None for layout gates. During workspace compile, corrected a wrong `semio-framework-schema` path in `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/Cargo.toml` (outside assigned tree) so `cargo check` could resolve the workspace.

## 7. Not validated

- Repo MCP (`ticket_close`, `repo://goals`) unavailable in this agent session.
- Layout TypeScript package vitest (if any) not run separately.
- Full `bun ./📜️script.ts policy` without filter (layout slice is clean).

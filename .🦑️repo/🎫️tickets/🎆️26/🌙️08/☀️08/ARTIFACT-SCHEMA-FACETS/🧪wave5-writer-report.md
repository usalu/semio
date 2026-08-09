# Wave 5 Report — Writer (`semio-s-plugin-writer`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/✒️writer/**` plus this ticket folder.
Key `writer`, prefix `Writer`, schema id `s.writer.writer`. Former snapshot type `WriterProjection` → `WriterSnapshot`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | snapshot / document |
| `id` | persistent | snapshot |
| `languageId` | persistent | snapshot |
| `uri` | persistent | snapshot |
| `text` | persistent | snapshot (DSL `lang_from`) |
| `selectedAstIds` | shared-ui | `WriterConfig` |
| `editorSelection` | shared-ui | `WriterConfig` |
| `editorSettings` | shared-ui | `WriterConfig` |
| `formatSignal` | local-ui | `WriterConfig` |
| `lintSignal` | local-ui | `WriterConfig` |
| `revision` | local-ui | `WriterConfig` |
| `engagementInput` | local-ui | `WriterConfig` |
| `cameraX` / `cameraY` / `cameraZoom` | local-ui | flattened camera |
| `locale` | local-ui | `WriterConfig` |
| `treeHoveredAstId` | preview | `WriterConfig` |
| `editorHoverOffset` | preview | `WriterConfig` |

Snapshot facet = the five persistent fields (`schema`, `id`, `languageId`, `uri`, `text`).

## 2. Diff-delta shape (text-sequence)

`WriterDiff` sparse field delta:

- `artifact: Option<Box<WriterArtifact>>` — whole replacement wins
- persistent scalars: `schema`, `id`, `languageId`, `uri` as `Option<String>`
- **`text: Option<WriterTextDelta>`** — not `added`/`removed`/`patched` on characters or lines
- shared-ui: `selectedAstIds: Option<WriterStringList>`, `editorSelection: Option<Option<WriterEditorSelection>>`, `editorSettings`
- local-ui: signals, engagement, camera, locale
- preview: `treeHoveredAstId`, `editorHoverOffset` as `Option<Option<…>>`

`WriterTextDelta`:

- `replacement: Option<String>` — whole-buffer replace (`SetText`, format/lint paths)
- `edits: Vec<WriterTextRangeEdit>` — honest substring edits (`start`, `end`, `insert`) for incremental typing without pretending the document is an identified collection

**Why:** Writer’s persisted body is a single text sequence (Trinity/Jack DSL). Collection deltas misrepresent edits and fight kernel `MergeStrategyKind::TextSequence`. Range edits + optional replacement match how the engine and LSP actually change text.

`SetDocument` → `SetSnapshot { snapshot }`. `MutationDiff<WriterSnapshot>` + `apply_to_artifact`. Diff wire codec: JSON via `DiffCodec` (no `DslDiff` on full `WriterDiff`).

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`:

- `artifacts::writer::schema`
- `artifacts::writer::snapshot::{schema, pack}`
- `artifacts::writer::diff::{component, schema}` (`pub use super::schema::*`)

TypeScript `📦️index.ts` mirrors snapshot pack path and three schema facet exports.

## 4. Other structural changes

- Fifteen handcrafted schema leaves under `🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `WriterEngine` registers `writer_artifact_schema_descriptor()` from `engine::register()`
- `DocumentApp`: `Snapshot`, `initial_snapshot`, `.snapshot`; config envelope `writer.config`
- `ViewModel` (not `ViewState`); tests use `DraftView` + `EngineHandles`, `store::os_store::test_support`
- SPR / canvas: serde tag `mutation`, baseline `tag=N`
- Real `🗣️example.dsl.semio` + `🗣️dag-example.dsl.semio` round-trip fixtures
- Ticket helper: `🧪wave5-writer-leaves-gen.py` (initial leaf pass; leaves hand-fixed for policy parity)

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-writer

```
warning: `semio-s-plugin-writer` (lib) generated 5 warnings (run `cargo fix --lib -p semio-s-plugin-writer` to apply 5 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 7.91s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test -p semio-s-plugin-writer --lib

```
test result: ok. 91 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'writer'

```
(empty — rg exit 1)
```

Direct: `policyArtifactSchemaBreaches(root)` → **0** writer breaches.

## 6. Shared-surface blockers

None for writer. Stale `🔺️diff` folders under individual mutations (`✍️set-text`, `📄set-snapshot`) remain outside glue and were not expanded; safe to delete in a hygiene pass.

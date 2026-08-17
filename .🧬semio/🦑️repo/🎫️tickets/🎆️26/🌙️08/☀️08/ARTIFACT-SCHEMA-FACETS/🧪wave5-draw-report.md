# Wave 5 Report — Draw (`semio-s-plugin-draw`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🖍️draw/**` plus this ticket folder.
Key `draw`, prefix `Draw`, schema id `s.draw.draw`. Former snapshot type `DrawDocument` → `DrawSnapshot`.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `schema` | persistent | document schema id |
| `id` | persistent | document id |
| `title` | persistent | optional |
| `layers` | persistent | `Vec<DrawLayerNode>` tree |
| `assets` | persistent | required `BTreeMap` (empty default; was `Option`) |
| `artboard` | persistent | optional |
| `selectedIds` | shared-ui | from `DrawConfig` |
| `activeUtilityId` | shared-ui | from `DrawConfig` |
| `engagementInput` | local-ui | from `DrawConfig` |
| `cameraX` / `cameraY` / `cameraZoom` | local-ui | flattened from `DrawCamera` |
| `locale` | local-ui | from `DrawConfig` |
| `hoveredId` | preview | from `DrawConfig` |
| effect | — | none |

Snapshot facet = exactly the six persistent fields. No `DocumentApp::Draft` (`NoDraft`).

## 2. Diff-delta shape

`DrawDiff` sparse field delta:

- `artifact: Option<Box<DrawArtifact>>` — whole replacement wins
- persistent: `schema`, `id`, `title: Option<Option<String>>`, `layers: Option<DrawLayersDelta>`, `assets: Option<DrawAssetsDelta>`, `artboard: Option<Option<DrawArtboard>>`
- shared-ui: `selectedIds: Option<DrawStringList>`, `activeUtilityId`
- local-ui: `engagementInput`, `cameraX/Y/Zoom`, `locale`
- preview: `hoveredId: Option<Option<String>>`

`DrawLayersDelta`: `added` / `removed` / `patched` / `reordered`. Nested tree adds/reorders that need a parent fall back to whole-snapshot replacement via `diff_from_snapshot`. Layer patches carry base fields plus JSON blobs for transform/fill/stroke/trace params.

`MutationDiff<DrawSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as existing draw glue). Nested:

- `artifacts::draw::schema`
- `artifacts::draw::snapshot::{schema, pack}`
- `artifacts::draw::diff::{component, schema}` (runtime `pub use super::schema::*;`)

TypeScript `📦️index.ts` mirrors pack under snapshot plus schema exports.

## 4. Other structural changes

- Pack moved: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `DrawDocument` removed; `DrawSnapshot` defined in snapshot schema, re-exported from artifact root
- `📄set-document` → `🖼️set-snapshot` (`SetSnapshot { snapshot }`)
- Engine owns real `DrawArtifact` + cached `DrawSnapshot` (`type Artifact = DrawArtifact`, never aliased to Snapshot)
- `DocumentApp` / views / stores use `Snapshot` / `.snapshot` / `initial_snapshot` / `snapshot_json`
- Descriptor `draw_artifact_schema_descriptor()` registered from `engine::register()`
- FSM left alone (its `Snapshot` is statechart persistence, not document state)
- Gesture `DrawSession` kept across dispatches via `thread_local` so multi-step canvas gestures work
- Restored corrupted `🗣️example.dsl.semio` from prior history; `DrawConfig` envelope id set to `draw.config`

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-draw

```
warning: `semio-framework-os` (lib) generated 10 warnings (run `cargo fix --lib -p semio-framework-os` to apply 8 suggestions)
    Checking semio-s-plugin-draw v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 8.07s
```

### cargo test -p semio-s-plugin-draw --lib

```
test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'draw'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter: **draw artifact-schema breaches: 0**.

## 6. Shared-surface notes

- Kernel crate-root glob-exports both `os_dsl::test_support` and `os_store::test_support`; bare `store::test_support::*` resolves to the DSL helper set. Draw tests now call `store::os_store::test_support::*` (plugin-side workaround).
- No other shared framework blocker for this crate’s gates.

## 7. Not validated

- Full `bun nx run workspace:verify-gate` (out of scope; other artifacts still incomplete).
- Runtime UI / playground smoke beyond lib tests.
- TypeScript vitest package tests (none required by brief for this crate’s gates).

# Wave 5 Report — Forms (`semio-s-plugin-forms`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/📋️forms/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/📋️forms/` | `forms` | `Forms` | `s.forms.forms` | `FormSpec` → `FormsSnapshot` |

App: `📋️forms` ↔ `forms` (`type Snapshot = FormsSnapshot`). Draft lane is `NoDraft`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | document schema id (`forms.form`) |
| `id` | persistent | document id |
| `version` | persistent | document version |
| `title` | persistent | optional title |
| `steps` | persistent | identified `Vec<FormStep>` (playbook step/block domain) |
| `selectedIds` | shared-ui | `FormsConfig` |
| `currentStepIndex` | local-ui | `FormsConfig` (Try wizard step) |
| `tryValuesJson` | local-ui | `FormsConfig` |
| `locale` | local-ui | `FormsConfig` |
| `contributionsJson` | local-ui | `FormsConfig` |

Snapshot facet = the five persistent fields exactly (`schema`, `id`, `version`, `title`, `steps`).

## 2. Diff-delta shape

`FormsDiff` sparse field delta:

- `artifact: Option<Box<FormsArtifact>>` — whole replacement wins
- persistent: `schema`, `id`, `version`, `title: Option<Option<String>>`, `steps: Option<FormsStepsDelta>` (`added` / `removed` / `patched` / `reordered`)
- shared-ui: `selectedIds: Option<FormsStringList>`
- local-ui: `currentStepIndex`, `tryValuesJson`, `locale`, `contributionsJson`

Block-level step edits that cannot be expressed as step title/description patches fall back to whole-snapshot replacement via `diff_set_snapshot`. `MutationDiff<FormsSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise.

`FormMutation` is a forms-local enum (same variants as playbook); `diff_from_mutation` derives `FormsDiff` from apply + sparse diff.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`:

- `artifacts::forms::schema`
- `artifacts::forms::snapshot::{schema, pack}`
- `artifacts::forms::diff::{component, schema}` (`pub use super::schema::*`)

TypeScript `📦️packages/🟦️typescript/📦️index.ts` mirrors snapshot pack path and three schema facet exports. Added `extern crate semio_framework_schema as schema` and `semio-framework-schema` dependency.

## 4. Other structural changes

- Fifteen handcrafted leaves under `🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `FormSpec` removed; `FormsSnapshot` in snapshot schema, re-exported from artifact root
- `FormsEngine` + `register_artifact_schema()` with `forms_artifact_schema_descriptor()`
- `DocumentApp` / views: `Projection` → `Snapshot`, `.projection` → `.snapshot`, `initial_snapshot`
- `FormsConfig` envelope id `forms.config` via `#[dsl(id = "forms.config")]`
- DSL/pack codecs on `FormsSnapshot` use `PlaybookSpec` record spec with `forms.form` envelope (playbook kernel types unchanged)
- Restored real `.forms` fixture text (building-component, default, onboarding) for round-trip laws
- Tests: `store::os_store::test_support::*`, `DocumentApp::export_media` associated fn syntax

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-forms

```
warning: `semio-s-plugin-forms` (lib) generated 5 warnings (run `cargo fix --lib -p semio-s-plugin-forms` to apply 5 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 25.20s
    Blocking waiting for file lock on package cache
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test -p semio-s-plugin-forms --lib

```
test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'forms'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter: **forms artifact-schema breaches: 0**.

## 6. Shared-surface blockers

- Playbook kernel `PlaybookSpec` / `PlaybookMutation` remain in `📖️playbook`; forms owns `FormsSnapshot`, `FormMutation`, and `FormsDiff` in-plugin. DSL/pack body encoding still shares `PlaybookSpec::__dsl_spec()` until playbook is split for forms-only extension.
- Repo MCP (`ticket_*` / `repo://goals`) was not used this session; work stayed in the assigned plugin tree and ticket folder.

## 7. Not validated

- Full `bun ./📜️script.ts policy` human-readable stdout beyond the forms filter (CLI silent when piped; confirmed via `policyArtifactSchemaBreaches`)
- TypeScript vitest package run (nx budget; index re-exports only)
- Interactive UI / playground runtime beyond lib tests

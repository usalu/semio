# Wave 5 Report — Playbook (`semio-s-plugin-playbook`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/📖️playbook/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/📖️playbook/` | `playbook` | `Playbook` | `s.playbook.playbook` | kernel `PlaybookSpec` (plugin doc) → `PlaybookSnapshot` |

App: `📖️playbook` ↔ `playbook` (`type Snapshot = PlaybookSnapshot`). Draft lane is `NoDraft`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | document schema id (persisted value `playbook.program` from kernel default) |
| `id` | persistent | document id |
| `version` | persistent | document version |
| `title` | persistent | optional playbook title |
| `steps` | persistent | identified `Vec<PlaybookStep>` (kernel domain) |
| `selectedIds` | shared-ui | `PlaybookConfig` selection |
| `locale` | local-ui | `PlaybookConfig` BCP-47 |
| `contributionsJson` | local-ui | `PlaybookConfig` extension contributions cache |

Snapshot facet = the five persistent fields exactly (`schema`, `id`, `version`, `title`, `steps`).

## 2. Diff-delta shape

`PlaybookDiff` sparse field delta:

- `artifact: Option<Box<PlaybookArtifact>>` — whole replacement wins
- persistent: `schema`, `id`, `version`, `title: Option<Option<String>>`, `steps: Option<PlaybookStepsDelta>` (`added`/`removed`/`patched`/`reordered`); block patches nest `PlaybookBlocksDelta` inside step patches
- shared-ui: `selectedIds` as `Option<PlaybookStringList>`
- local-ui: `locale`, `contributionsJson`

`MutationDiff<PlaybookSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise. Cross-step `MoveBlock` uses whole-artifact replacement via kernel `apply_playbook_edit_mutation`. `set-snapshot` (`SetSnapshot { snapshot }`) replaces the persisted document.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as lowpoly / shooting):

- `artifacts::playbook::schema`
- `artifacts::playbook::snapshot::{schema, pack}`
- `artifacts::playbook::diff::{component, schema}` (`pub use super::schema::*` in diff runtime)

TypeScript `📦️packages/🟦️typescript/📦️index.ts` mirrors pack under snapshot plus the three schema facet exports. Dependency: `semio-framework-schema` (`extern crate semio_framework_schema as schema`).

## 4. Two “playbook” surfaces (disambiguation)

| Surface | Path / crate | Types | Role |
| --- | --- | --- | --- |
| Framework kernel module | `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/` (re-exported `flow::playbook` in plugin glue) | `PlaybookSpec`, `PlaybookMutation`, `apply_playbook_edit_mutation`, store schema constant `playbook.program` | Shared step/block domain + kernel mutations; **out of wave-5 plugin scope** |
| Plugin artifact | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/` | `PlaybookSnapshot`, `PlaybookArtifact`, `PlaybookDiff`, facet id `s.playbook.playbook`, DSL/pack envelope `playbook.playbook` | Persisted document the `PlaybookPlayApp` edits |

The plugin snapshot bridges with `from_kernel` / `to_kernel` / `as_kernel`; facet rename applies only to the plugin artifact type. DSL/pack use `playbook.playbook` envelope while the persisted `schema` field remains `playbook.program` until the kernel wave aligns envelope derivation.

## 5. Procedural extension (`🧩️extensions/🌀️procedural/`)

`ModuleApp` implements `DocumentApp` with `type Snapshot = ModuleRenderPayload` and document schema `playbook.module.procedural.payload`. That payload is an ephemeral render/session bundle (flow fixture + answers + preview state), **not** the `playbook` artifact document and **not** an `ArtifactEngine` facet.

Decision: keep `ModuleRenderPayload` as its own snapshot type for the module app; do **not** rename it to `PlaybookSnapshot` or add fifteen artifact-schema leaves. Wave-5 `Snapshot` API migration (`initial_snapshot`, `doc.snapshot`, `DraftView` + `EngineHandles` in tests) applies; no `PlaybookArtifact` / `PlaybookDiff` coupling.

## 6. Other structural changes

- Fifteen handcrafted leaves (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`; protocol envelope segment `Snapshot`
- `PlaybookEngine` owns real `PlaybookArtifact` + `PlaybookSnapshot`; `ArtifactEngine::{Artifact, Snapshot, artifact, snapshot}`
- `PlaybookConfig` envelope id `playbook.config`
- Example `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — round-tripping empty default snapshot (`print_dsl` / `parse_dsl`)
- `DocumentApp` / views / tests: `Projection` → `Snapshot`, `store::os_store::test_support::*`
- SPR baselines: serde tag `mutation`, `tag=N` form

## 7. Gate tails (verbatim)

### cargo check

```
    Finished `dev` profile [unoptimized] target(s) in 9.44s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test --lib

```
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### policy (`bun ./📜️script.ts policy 2>&1 | rg -i playbook`)

```
(empty — no lines)
```

Direct confirmation: `policyArtifactSchemaBreaches(root)` filtered for `playbook` → **0** breaches.

## 8. Fixup / not validated

- Repo MCP ticket open/close not run (namespace unavailable in this session).
- Procedural extension tests still import `store::test_support` in one envelope round-trip test (not required for lib gate).
- Kernel `PlaybookSpec::print_dsl` envelope id remains derived from kernel schema (`playbook.program`); plugin snapshot intentionally overrides envelope to `playbook.playbook` for the artifact codec only.

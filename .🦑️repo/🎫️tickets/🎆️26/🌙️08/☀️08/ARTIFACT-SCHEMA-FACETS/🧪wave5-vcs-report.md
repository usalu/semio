# Wave 5 Report — VCS (`semio-s-plugin-vcs`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🌿️vcs/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/🌿️vcs/` | `vcs` | `Vcs` | `s.vcs.vcs` | `VcsDemoProjection` → `VcsSnapshot` |

App: `vcs-play` ↔ `vcs` (`type Snapshot = VcsSnapshot`). Draft lane is `NoDraft`.

## Plugin vs framework VCS

- **This wave (plugin):** `✏️s/🔌️plugins/🌿️vcs/` — demo document artifact, `VcsSnapshot` / `VcsArtifact`, play app, plugin-local engine. All edits confined here.
- **Out of scope (framework):** `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/` — `DocumentVcs`, checkpoints, alternatives, envelope replay. Already uses `initial_snapshot` naming; not renamed in this agent.

## History vs snapshot (§1)

Version **history** (checkpoints, edits, alternatives) lives only on `DocumentEnvelope.vcs` via framework `DocumentVcs` — **not** in the artifact snapshot.

| Field | State | Classification |
| --- | --- | --- |
| `schema` | persistent | document schema id (`vcs.vcs`) |
| `title` | persistent | demo document title |
| `counter` | persistent | demo scalar |
| `notes` | persistent | demo notes |
| `status` | persistent | demo status string |
| `tags` | persistent | demo tag list (content, not VCS graph) |
| `selectedCheckpointIds` | shared-ui | UI multi-select of checkpoint ids in history **view** — not persisted history |
| `locale` | local-ui | from `VcsDemoConfig` |

Snapshot facet = six persistent fields exactly.

## Diff-delta shape

`VcsDiff` sparse field delta:

- `artifact: Option<Box<VcsArtifact>>` — whole replacement wins
- persistent: `schema`, `title`, `counter`, `notes`, `status`, `tags: Option<VcsTagsDelta>` (`added` / `removed` strings)
- shared-ui: `selectedCheckpointIds: Option<VcsStringList>`
- local-ui: `locale`

`MutationDiff<VcsSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes. `absorb` merges field-wise. `SetSnapshot { snapshot }` added; `diff_set_snapshot` for whole-document replacement.

## Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`:

- `artifacts::vcs::schema`
- `artifacts::vcs::snapshot::{schema, pack}`
- `artifacts::vcs::diff::{component, schema}` (`pub use super::schema::*` in runtime diff)

TypeScript `📦️index.ts` mirrors three schema facets, diff runtime, pack under snapshot.

## Other structural changes

- Fifteen handcrafted leaves (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`; protocol schema `vcs.vcs`
- `VcsDemoProjection` removed; `VcsSnapshot` in snapshot schema, re-exported from artifact root
- Document schema / envelope: `vcs.vcs`; config envelope `vcs.config`
- Engine owns `VcsArtifact` + `VcsSnapshot`; `ArtifactEngine::{Artifact, Snapshot, artifact, snapshot}`
- `DocumentApp` / views / tests: `Snapshot`, `.snapshot`, `initial_snapshot`, `store::os_store::test_support::*`
- Example `🗣️example.dsl.semio` round-trips demo document (not a stub)
- Descriptor `vcs_artifact_schema_descriptor()` registered from `engine::register_artifact_schema()`

## Gate tails (verbatim)

### cargo check -p semio-s-plugin-vcs

```
warning: `semio-framework-plugin` (lib) generated 16 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Checking semio-s-plugin-vcs v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 20.85s
```

### cargo test -p semio-s-plugin-vcs --lib

```
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'vcs'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches(root)` filter for `vcs`: **0** breaches.

## Shared-surface notes

- Repo MCP (`ticket_*`, `repo://goals`) unavailable in this session; work used existing ticket folder.
- No plugin-scope blocker after `semio-framework-schema` path correction (`🧰️framework/🔨️modules/🧬️schema/…`, not under `🛍️products/💻️os`).

## Not validated

- Full `bun nx run workspace:verify-gate`
- Interactive UI / playground beyond lib tests
- TypeScript vitest package run (index re-exports only)

# Wave 5 Report — Architect Fanout (Artifact Schema Facets)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W5 owns `✏️s/🔌️plugins/🏛️architect/**` plus this ticket folder. Artifact key `program`, prefix `Program`.

## 1. What changed

### Fifteen facet leaves

| Facet | Dir | Type |
| --- | --- | --- |
| artifact | `🗿️artifacts/🏛️program/🧬️schema/` (5 formats) | `ProgramArtifact` |
| snapshot | `🗿️artifacts/🏛️program/📸️snapshot/🧬️schema/` (5) | `ProgramSnapshot` |
| diff | `🗿️artifacts/🏛️program/🔺️diff/🧬️schema/` (5) | `ProgramDiff` |

### Structural moves

- `🎒️pack` → `📸️snapshot/🎒️pack/`
- `🗄️registers` → `🧬️schema/🗄️registers/`
- `🧱️kernel` → `🧬️schema/🧱️kernel/`
- `🧬️mutations/📦️set-program` → `🧬️mutations/🖼️set-snapshot` (`SetProgram` → `SetSnapshot { snapshot: Box<ProgramSnapshot> }`)

### Rename

Bare `Program` document type replaced by `ProgramSnapshot` / `ProgramArtifact` / `ProgramDiff` (no aliases). Snapshot defined in `snapshot::schema` with handcrafted `DocumentDsl` / `DocumentPack`; envelope id `architect.program`.

### Diff as sparse field delta

`ProgramDiff` is a sparse field delta (not a mutation list):

- `artifact: Option<Box<ProgramArtifact>>` for whole replacement
- optional entry per artifact field (collection fields use `Program*Delta`)
- `MutationDiff<ProgramSnapshot>` applies persistent entries; `apply_to_artifact` applies all
- Mutations build deltas via `diff_*` helpers; per-mutation `🔺️diff/` folders hold thin wrappers around `ProgramDiff`

### Engine / app

- Engine owns `ProgramArtifact` + `ProgramSnapshot` (`type Artifact = ProgramArtifact`)
- Registers schema descriptor via `OnceLock<Mutex<ArtifactSchemaRegistry>>`
- `DocumentApp::Snapshot = ProgramSnapshot`; views use `.snapshot`
- Testkit `drive`/`render` updated for `DraftView` + `EngineHandles` arity

### Glue

Leaf-prefixed + grouping `#[path = "."]`. Nested `snapshot { schema + pack }` and `diff { component + schema }`. Diff runtime: `pub use super::schema::*;`.

### Registers / kernel decision

Pre-existing `taxonomy/dirs` breach: `🗄️registers` and `🧱️kernel` lived under the artifact root (not in `artifactChildDirs`). **Relocated under `🧬️schema/`** as domain types for the schema facet. Taxonomy JSON was **not** edited.

### EntityId

`EntityId::new_serial` restored to process-local monotonic serials (material arg kept for call-site clarity). Content-addressed blake3 collided when sample/default creators passed constant materials (`"element","element"`), breaking validation, CSV, adjacency upsert, and undo tests.

### SPR wire baseline

`ProgramMutation` serde tag is `mutation` (repo-wide). SPR byte pins updated from legacy `operation` / `to` to current `mutation` / `to_index`.

### Bundled DSL example

`🗣️example.dsl.semio` regenerated from `sample_plugin().print_dsl()` with envelope `architect.program.dsl`.

## 2. Field inventory (`ProgramArtifact`, 81 fields)

**Persistent** (= snapshot, 70): `schema`, `meta`, `project`, 66 register `Vec`s (`stakeholders`…`benchmarks` + `traces`), `governance`.

**SharedUi** (4): `selected_ids`, `active_register`, `adjacency_kind_filter?`, `active_report_json`.

**LocalUi** (7): `search_query`, `search_history_json`, `last_result_json`, `last_analysis_json`, `graph_camera_x/y/zoom`.

**Preview / Effect:** none.

## 3. Gate tails (verbatim)

### cargo check -p semio-s-plugin-architect

```
warning: `semio-s-plugin-architect` (lib) generated 76 warnings (run `cargo fix --lib -p semio-s-plugin-architect` to apply 76 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 17.39s
```

### cargo test -p semio-s-plugin-architect --lib

```
test result: ok. 248 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'architect'

```
(empty — no lines matched; CLI often silent when stdout is a pipe)
```

Direct probes (ticket `🧪wave5-architect-policy-gates.log`):

- `policyArtifactSchemaBreaches` filter architect/program: **0**
- `taxonomy/dirs` filter architect: **0**

## 4. Shared-surface notes / blockers

- Root `policy` CLI prints nothing useful when stdout is a pipe — use `policy()` / `policyArtifactSchemaBreaches(root)` via `bun -e`.
- `DocumentApp::handle` now takes `DraftView` + `EngineHandles`; plugin testkits must pass `NoDraft::default()` + `EngineHandles::empty()`.
- Prefer `semio_framework_os_kernel::os_store::test_support` over bare `store::test_support` (ambiguous alias).
- Mutation serde tag is `mutation` repo-wide; SPR baselines that still pin `operation` need updating inside each plugin.
- `EntityId::new_serial` must mint unique ids for constant materials — content-addressed-only breaks sample fixtures that call `new_serial(prefix, prefix)` twice.

## 5. Logs in ticket folder

- `🧪wave5-architect-check-final.log`
- `🧪wave5-architect-test-6.log`
- `🧪wave5-architect-policy-final.log` / `🧪wave5-architect-policy-gates.log`
- `🧪wave5-architect-policy-cli-rg.log`

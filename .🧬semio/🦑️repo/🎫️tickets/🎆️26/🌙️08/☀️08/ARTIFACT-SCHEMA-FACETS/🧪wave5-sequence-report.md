# Wave 5 Report — Sequence (`semio-s-plugin-sequence`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🎬️sequence/**` plus this ticket folder.

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/🎬️sequence/` | `sequence` | `Sequence` | `s.sequence.sequence` | `SequenceFixture` → `SequenceSnapshot` |

App: `🎬️sequence` ↔ `sequence` (`type Snapshot = SequenceSnapshot`). Config envelope `sequence.config`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `schema` | persistent | document schema id |
| `steps` | persistent | identified `Vec<SequenceStep>` |
| `edges` | persistent | identified `Vec<SequenceEdge>` |
| `selectedStepIds` | shared-ui | `SequenceConfig` |
| `lastRunJson` | local-ui | last `run` scope JSON |
| `orientation` | local-ui | canvas layout orientation |
| `camera` | local-ui | session viewport camera |
| `locale` | local-ui | BCP-47 |

Snapshot facet = the three persistent fields exactly (`schema`, `steps`, `edges`).

## 2. Diff-delta shape

`SequenceDiff` sparse field delta:

- `artifact: Option<Box<SequenceArtifact>>` — whole replacement wins
- persistent: `schema`, `steps` / `edges` as `Option<SequenceStepsDelta>` / `SequenceEdgesDelta` (`added` / `removed` / `patched` / `reordered`)
- shared-ui: `selectedStepIds` as `Option<SequenceStringList>`
- local-ui: `lastRunJson`, `orientation`, `camera`, `locale`

`MutationDiff<SequenceSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise. `set-fixture` → `set-snapshot` (`SetSnapshot { snapshot }`).

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (lowpoly / shooting pilot):

- `artifacts::sequence::schema`
- `artifacts::sequence::snapshot::{schema, pack}`
- `artifacts::sequence::diff::{component, schema}` (`pub use super::schema::*`)

TypeScript `📦️packages/🟦️typescript/📦️index.ts` exports the three schema facets and snapshot pack. Dependency: `semio-framework-schema` (`extern crate … as schema`).

## 4. Other structural changes

- Fifteen handcrafted leaves (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`; protocol segment `Snapshot` kind 3
- `SequenceFixture` removed; `SEQUENCE_DOCUMENT_SCHEMA = "sequence.sequence"`
- `SequenceEngine` owns real `SequenceArtifact` + `SequenceSnapshot`; `ArtifactEngine::{Artifact, Snapshot, artifact, snapshot}`
- Example `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` round-trips (not a stub)
- `DocumentApp` / views / tests: `Projection` → `Snapshot`, `.projection` → `.snapshot`, `store::os_store::test_support::*`
- Lib tests bootstrap imperative extensions (math, text, effect, control catalogue) via `ensure_imperative_modules_for_tests()` because `imperative_module_registry()` is empty without synced contributions
- DAG node sizing: local `sequence_computation_node_width/height` (200×24) — `dag::computation_node_width/height` not exported from infinite canvas crate

## 5. Gate tails (verbatim)

### cargo check

```
warning: `semio-s-plugin-sequence` (lib) generated 10 warnings (run `cargo fix --lib -p semio-s-plugin-sequence` to apply 10 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 27.35s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test --lib

```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
```

### policy `| rg -i 'sequence'`

(empty — note: naive `sequence` also matches unrelated `consequenceClass` breaches elsewhere; scope filter `🎬️sequence` confirms **0** breaches)

Direct confirm: `policyArtifactSchemaBreaches(root)` with `scope` containing `🎬️sequence` → **0** breaches.

## 6. Shared-surface blockers (fixup wave)

| Surface | Impact | Workaround in plugin |
| --- | --- | --- |
| `dag::computation_node_width` / `height` not re-exported from `semio-framework-os-infinite` | Cannot call `fit_node_size` from sequence engine | Fixed 200×24 row-height helpers |
| Imperative registry requires synced `contributions_json` + optional `register_native_imperative_module` | Unit tests had empty registry (math/text ports, `run`) | `#[cfg(test)]` dev-deps on imperative extension crates + one-shot bootstrap in `SequenceHost::from_snapshot` |

Production hosts should still push real imperative contributions; test bootstrap is not a substitute for the full playground graph.

## 7. Not validated

- Full `bun ./📜️script.ts policy` stdout (CLI silent when piped; confirmed via `policyArtifactSchemaBreaches`)
- TypeScript vitest / nx package scripts
- WASM playground / interactive UI beyond lib tests
- Repo MCP `ticket_close` (session had no MCP ticket tools)

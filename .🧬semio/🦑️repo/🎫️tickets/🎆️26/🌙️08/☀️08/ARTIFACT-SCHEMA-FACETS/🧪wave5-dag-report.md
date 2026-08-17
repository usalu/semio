# Wave 5 — DAG (`semio-s-plugin-dag`)

## Summary

Completed artifact-schema-facets refactor for plugin `✏️s/🔌️plugins/🕸️dag/`: fifteen handcrafted schema leaves, `DagSnapshot` / `DagArtifact` / `DagDiff`, pack under `📸️snapshot/🎒️pack/`, `set-snapshot` mutation, plugin-owned `DocumentDsl`/`DocumentPack` mirror (envelope `dag.dag`), round-tripping demo DSL, engine registration, and glue wiring. All three gates green.

## Disambiguation: plugin DAG vs framework `dag`

| Concern | Plugin (`semio-s-plugin-dag`) | Framework (`semio-framework-os-infinite` / `infinite_board_port_directed_dag`) |
| --- | --- | --- |
| Persistent document | `DagSnapshot`, schema id `s.dag.dag`, document schema `dag.dag` | Kernel `DagDocument`, internal schema constant `dag.fixture` for legacy kernel paths |
| DSL envelope | `dag.dag` via `DagSnapshotDsl` in `📸️snapshot/🧬️schema` | `DagDocumentDsl` (not used for plugin print; kernel still parses bundled example body) |
| Mutations / diff | `DagMutation` / `DagDiff` with `serde(tag = "mutation")` | Kernel `DagMutation` bridged at op wire boundary only |
| Layout / canvas helpers | Consumed via `extern crate infinite_canvas as infinite_board_port_directed_dag` | `fit_node_size`, `DagFixture`, `DagHost`, etc. — not renamed |

No framework crate edits; local DSL mirror duplicates kernel derive shape only inside the plugin.

## Field inventory (artifact / snapshot / diff)

- **Snapshot (`DagSnapshot`)**: `schema`, `nodes`, `edges` (persistent).
- **Artifact (`DagArtifact`)**: snapshot fields plus `camera`, selection/UI fields per `🧬️schema` leaves.
- **Diff (`DagDiff`)**: optional `artifact`, collection deltas for `nodes`/`edges`, `set_nodes` / `set_edges` list wrappers, UI fields.

## Glue convention

- `📦️glue.rs`: `artifacts::dag::schema`, `snapshot::{schema, pack}`, `diff::schema`, `mutations::set_snapshot`, `engine::DagEngine`, `register_artifact_schema()`.
- `DagConfig`: `#[dsl(id = "dag.config")]`.
- App: `DocumentApp` uses `Snapshot`, `SetSnapshot`, `DraftView` + `EngineHandles` in tests.

## Example fixture

`📚️examples/.../🗣️example.dsl.semio` — regenerated from `print_dsl` (two computation nodes + edge). Parsed by kernel `DagFixture::default()` include and by plugin `default_snapshot()`.

## Gate tails (verbatim)

### `cargo check -p semio-s-plugin-dag`

```
warning: `semio-s-plugin-dag` (lib) generated 2 warnings (run `cargo fix --lib -p semio-s-plugin-dag` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 5.59s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### `cargo test -p semio-s-plugin-dag --lib`

```
test artifacts::dag::dsl::tests::example_fixture_dsl_round_trips ... ok
test artifacts::dag::snapshot::pack::tests::pack_round_trips_and_agrees_with_dsl ... ok

test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### `bun ./📜️script.ts policy | rg -i 'dag'`

```
(empty — no dag-related policy lines)
```

Direct check: `policyArtifactSchemaBreaches` filtered to dag scope → **0** breaches.

## Not validated

- Repo MCP `ticket_close` (not invoked in this agent turn).
- Vitest / WASM UI smoke.
- Full monorepo `cargo test` outside `semio-s-plugin-dag`.

## Files touched (production)

Under `✏️s/🔌️plugins/🕸️dag/`: artifact root, `🧬️schema/*`, `📸️snapshot/{🧬️schema,🎒️pack}`, `🔺️diff/{🧬️schema,🦀️component}`, `🧬️mutations` (`set-snapshot`), `⚙️engine`, `📦️glue.rs`, `Cargo.toml`, apps/commands/panels/config, `📡️spr`, grammars, `📚️examples/.../🗣️example.dsl.semio`, TS `📦️index.ts`.

Ticket folder: `🧪wave5-dag-schema-gen.py`, `🧪wave5-dag-extract-mirror.py`, `🧪wave5-dag-dsl-mirror.rs.fragment`, this report.

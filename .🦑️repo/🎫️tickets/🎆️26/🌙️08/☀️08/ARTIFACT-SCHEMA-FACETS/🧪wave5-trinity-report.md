# Wave 5 — Trinity Artifact Schema Facets

Ticket: `2026/08/08/ARTIFACT-SCHEMA-FACETS`  
Plugin: `✏️s/🔌️plugins/🔱️trinity/`  
Crate: `semio-s-plugin-trinity`

## Status

Completed for both artifacts. Leaves are handcrafted (no generator). Gates green inside the trinity tree.

## App ↔ artifact type mapping

| App surface | Old name | New artifact type | Role |
|---|---|---|---|
| Rewrite persisted document | `RewriteRuleModel` / `RewriteRuleDocument` | `RewriteSnapshot` | Persistent document / pack / DSL body |
| Rewrite full union | *(config fields lived only on `RewriteConfig`)* | `RewriteArtifact` | Snapshot + shared_ui + local_ui |
| Rewrite sparse delta | `RewriteRuleDiff` | `RewriteDiff` | Field-level `MutationDiff` |
| Jack persisted document | `GraphFixture` / `TrinityGraphDocument` | `JackSnapshot` | Persistent document / pack / DSL body |
| Jack full union | *(config fields lived only on `JackConfig`)* | `JackArtifact` | Snapshot + shared_ui + local_ui |
| Jack sparse delta | `TrinityGraphDiff` | `JackDiff` | Field-level `MutationDiff` |

App config types remain app-owned:

- `RewriteConfig` / `RewriteConfigMutation` — Before-pane camera, selection, LOD, epochs, locale
- `JackConfig` / `JackConfigMutation` — `viewportCamera` (live camera; distinct from persistent seed `camera`), query/result, engagement inputs, selection, LOD, locale

`DocumentApp` uses `type Snapshot = RewriteSnapshot | JackSnapshot`, `initial_snapshot()`, `whole_document_operation(snapshot)`, and `app.snapshot()` / `doc.snapshot`.

## Field inventories

### `RewriteSnapshot` (persistent) — schema id `s.trinity.rewrite`

- `before_fixture_json: String`
- `lhs_json: String`
- `rhs_json: String`
- `parameter_bindings: BTreeMap<String, PropertyValue>`
- `rule_layout: BTreeMap<String, LayoutPoint>`

### `RewriteArtifact` (= snapshot + UI)

Persistent: same as snapshot.

Shared UI:

- `selected_node_ids`
- `active_hover_var`
- `active_select_var`
- `lod_mode_by_window`

Local UI:

- `before_pane_camera`
- `reorganize_epoch`
- `hover_epoch`
- `select_epoch`
- `locale`

### `RewriteDiff` (sparse)

Optional whole-artifact replacement (`artifact`) plus optional per-field entries mirroring every artifact field. Map fields use `BTreeMap<K, Option<V>>` tombstones. Lists use `RewriteStringList`.

### `JackSnapshot` (persistent) — schema id `s.trinity.jack`

- `schema: String` (document schema `trinity.graph`)
- `name: String`
- `manifest_id: Option<String>`
- `manifest: Manifest`
- `camera: Camera` (seed-only persistent camera)
- `nodes: Vec<Node>`
- `edges: Vec<Edge>`
- `root_node_id: Option<String>`

### `JackArtifact` (= snapshot + UI)

Persistent: same as snapshot.

Shared UI:

- `selected_node_ids`
- `active_fixture_id`
- `jack_query`
- `lod_mode_by_window`

Local UI:

- `viewport_camera` (live viewport; avoids clash with persistent `camera`)
- `jack_result_json`
- `editor_engagement_input`
- `graph_engagement_input`
- `results_engagement_input`
- `reorganize_epoch`
- `editor_selection: Option<JackEditorSelection>`
- `revision`
- `locale`

### `JackDiff` (sparse)

Optional whole-artifact replacement (`artifact`) plus scalar optionals; `nodes` / `edges` use identified-collection deltas (`added` / `removed` / `patched` / `reordered`).

## Fifteen handcrafted leaves each

For both `🗿️artifacts/♻️rewrite/` and `🗿️artifacts/🔌️jack/`:

| Facet | Leaves |
|---|---|
| `🧬️schema/` | `.rs` `.ts` `.json` `.graphql` `.proto` |
| `📸️snapshot/🧬️schema/` | `.rs` `.ts` `.json` `.graphql` `.proto` |
| `🔺️diff/🧬️schema/` | `.rs` `.ts` `.json` `.graphql` `.proto` |

Pack relocated to `📸️snapshot/🎒️pack/` (protocol + RS + TS).

## Engines / glue / TS

- Both engines own a real `XArtifact` + cached `XSnapshot`, with `artifact()` / `snapshot()` / `apply` / `inverse`.
- `register()` + `register_artifact_schema()` hooked from `register_trinity_exports()` in glue.
- Glue exposes `schema`, `diff::{component,schema}`, `snapshot::{schema,pack}` for jack + rewrite.
- TS `📦️packages/🟦️typescript/📦️index.ts` re-exports schema / snapshot schema / diff / diff schema / pack / dsl / op / spr for both artifacts.
- Document codecs still register under `TRINITY_GRAPH_SCHEMA` (`trinity.graph`) and `REWRITE_RULE_SCHEMA` (`trinity.rewrite.rule`).
- Semio envelope ids: `trinity.jack`, `trinity.rewrite`, config `trinity.jackcfg` / `trinity.rewritecfg`.

## Diff shape notes

- Rewrite mutations remain LWW `SetState` → `diff_set_state` / whole-snapshot replacement via `artifact`.
- Jack mutations apply directly on the snapshot (no `vcs::apply_mutation` recursion); `Mutation::diff` currently builds whole-snapshot replacement via `diff_set_snapshot` for most ops (`SetFixture` included). Sparse node/edge helpers exist for future tightening.

## Gate tails (verbatim)

### `cargo check -p semio-s-plugin-trinity`

```
warning: `semio-s-plugin-trinity` (lib) generated 49 warnings (run `cargo fix --lib -p semio-s-plugin-trinity` to apply 43 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.73s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### `cargo test -p semio-s-plugin-trinity --lib`

```
test artifacts::jack::dsl::component::tests::dsl_round_trip_mini_and_bundled_fixtures ... ok
test artifacts::rewrite::engine::component::tests::rewrite_rule_parameter_substitution ... ok
test executor::tests::run_create_edge ... ok

test result: ok. 174 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
```

### `bun ./📜️script.ts policy 2>&1 | rg -i 'trinity'`

```
(empty — no trinity artifact-schema policy hits)
```

Logs: `🧪wave5-trinity-check-final.log`, `🧪wave5-trinity-test-5.log`, `🧪wave5-trinity-policy.log`.

## Shared-framework / out-of-tree notes

- Concurrent framework sweep renamed DocumentApp projection→snapshot; trinity was updated in-tree.
- Early check failures outside `✏️s/🔌️plugins/🔱️trinity/**` were not fixed here (per brief). Final trinity crate check/test are green with no remaining out-of-tree blockers for this crate.
- Restored handcrafted nakagin / rewrite example DSL fixtures from git history (stubs had been left under `📚️examples/🎬️demo/`); apps now `include_str!` the jack artifact example instead of the demo-session `.cmd` file.

## Layout summary

```
🗿️artifacts/♻️rewrite/
  🧬️schema/                 → RewriteArtifact
  📸️snapshot/🧬️schema/     → RewriteSnapshot
  📸️snapshot/🎒️pack/       → pack codecs
  🔺️diff/🧬️schema/         → RewriteDiff
  ⚙️engine/                 → owns RewriteArtifact

🗿️artifacts/🔌️jack/
  🧬️schema/                 → JackArtifact
  📸️snapshot/🧬️schema/     → JackSnapshot
  📸️snapshot/🎒️pack/       → pack codecs
  🔺️diff/🧬️schema/         → JackDiff
  ⚙️engine/                 → owns JackArtifact
```

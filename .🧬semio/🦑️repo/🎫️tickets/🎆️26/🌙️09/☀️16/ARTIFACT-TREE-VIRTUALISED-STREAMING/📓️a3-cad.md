# 📓️ A3 — 📐️cad migrated onto the virtualised tree windows

Crate `semio-s-artifact-cad-cad` (no extra feature). Base dir
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/` (`ED` = `…/✳️any/✏️editor`).

## 1. Panels migrated

| Panel | Path | Shape after migration |
|---|---|---|
| document tree | `ED/📌️panels/🗿️artifact/🦀️.rs` | 9 sections → `PanelTreeBuilder::window_section` / `window_section_or_placeholder`; object rows → `tree_window_item` over their primitives; one tree-level `interaction_domain(CAD_PLAY_CONTROLLER_ID, CAD_INTERACTION_DOMAIN)` |
| inspection | `ED/📌️panels/🔍️inspection/🦀️.rs` | the selected-object group (ids block grows with the selection) → `window_section` over `(row id, label, value)` entries; the reference (13 rows), node (2 rows) and summary (3 rows) groups stay plain `section` — fixed, author-bounded lists |
| catalogue | `ED/📌️panels/🛍️catalogue/🦀️.rs` | the typology list → `window_section` over `TYPOLOGY_CATALOG`; every row keeps its own `addObject` binding (an app action, not a domain pick) |

`TreeWindows::for_body(view_state, BODY_KEY)` is built once per body in `CadPlayApp::render_body`
(`ED/🦀️.rs`) and threaded into all three panels, which now take `&TreeWindows<'_>`.

### Domain pick rows
`select_object_action` and its per-row `interactionSelect` argument map are gone. Object rows are now
built from a `TreeItemBuilder` carrying `.granularity(edit::CAD_WORLD_PICK_GRANULARITY)` and **no**
binding at all, keyed by the raw object id; the single `interactionSelect { domainId: "cad" }` lives on
the tree root. Reference rows keep `setReferenceSelection` plus their hide/lock row actions, node rows
keep `setNodeSelection` — unchanged.

## 2. Symbols deleted

`ED/📌️panels/🗿️artifact/🦀️.rs`
- `select_object_action`, `continuation_row`, `paged_section_from`, `section_page`,
  `CAD_SECTION_ROWS`, `SECTIONS`
- imports `panel_continuation_row`, `panel_page_rows`, `PanelRowBudget`
- `document_pane_section` / `artifact_references_section` no longer return
  `(id, label, open, UiFixedList)` + a `PanelRowBudget`; they take and return the `PanelTreeBuilder`

`ED/🦀️.rs`
- `use …commands::panel::set_panel_page` (line 14)
- `CadPlayRuntime.panel_pages` (+ its `Default` and `cad_runtime_from_config` /
  `cad_config_from_runtime` arms), `parse_panel_pages`, `print_panel_pages`
- `app_commands!` row `"setPanelPage" as "set-panel-page"` (~1202) and the `cad_command_from_action`
  arm (~1230)
- `CAD_RETAINED_CONFIG_TOOL_IDS` / `CAD_RETAINED_TOOL_IDS` entries (~1378 / ~1410), the
  `bounded_first_step_tool_proofs!` `tools:` row (~2117, 32 → 31 proof rows),
  `ArtifactToolPublicationContract { tool_id: "setPanelPage", … }` (~1454),
  `ActionDefinition::bounded_catalog("setPanelPage", …)` (~2427),
  `action_interactive_job("setPanelPage", …)` (~2500)
- `cad_config_store_bytes`'s `panel_pages_json` term (~1619)
- the crate-local `ui_node_list` copy and `ui_value_number` (the only caller was the `setPanelPage`
  `page` argument) — the panels import the SDK's `ui_node_list`

`ED/🎚️config/🦀️.rs`
- `CadConfig::panel_pages_json` and `default_panel_pages_json`. The checked-in config schema leaves
  (`ED/🎚️config/🧬️schema/{🔣️.json,🛰️.proto,🔗️.graphql,🟦️.ts,🦀️.rs}`) never carried the field, so the
  Rust struct is now back in sync with them — no schema regeneration was needed.

`ED/🎮️commands/📄️panel/` — whole directory deleted, plus its `pub mod panel;` declaration in
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs`.

## 3. Fixtures, TS and the generated manifest

- `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json`: `routeCount` 38 → 37, `activation.proofRows` 32 → 31,
  `"setPanelPage"` dropped from `admittedRoutes` and its `routes[]` entry removed.
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts`: `"setPanelPage"` dropped from the
  `RetainedAuditScript` `expectedAdmitted` allowlist, both `=== 38` route-count pins → `37`.
- Cross-checked mechanically (script under `🗑️generated/a3-cad/`): `app_commands!` ids (37) ==
  fixture `routes[]` ids in order; `CAD_RETAINED_TOOL_IDS` (31) == `expectedAdmitted` ==
  `admittedRoutes`, in order.
- `✏️s/🔌️plugins/📐️cad/🔣️.json` (4 `setPanelPage` action entries) regenerated through the plugin's own
  target — see §5.

## 4. Tests

`ED/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — the two paging laws
(`set_panel_page_advances_the_structure_section`, `a_stale_panel_page_cursor_clamps_to_the_last_page`)
are replaced by the brief's window laws; `document_lists_every_pane_object_bound_to_the_cad_domain` is
rewritten to assert `window.total` per pane section:

| Test | Law |
|---|---|
| `document_lists_every_pane_object_bound_to_the_cad_domain` | every pane section stamps `window.total == objects.len()`, `offset == 0`, rows ⊆ raw object ids, body JSON has no `.more` |
| `an_oversized_document_stamps_the_full_total_and_materialises_only_its_slice` | (a) 300+ synthetic nodes: `total == entries.len()`, `children.len() <= UI_BUILT_CHILDREN_MAX` and `< total`, no `.more` key, no `"+` label |
| `a_closed_container_stamps_its_total_without_children` | (b) `TreeWindowRequest{open: Some(false)}` → total stamped, zero children; the author-collapsed references section likewise |
| `a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_id` | (c) `TreeWindowRequest{offset: 2, rows: 2}` → exactly `objects[2..4]`, keyed by raw id, `window.offset == 2` |
| `object_rows_are_domain_picks_without_a_row_binding` | (d) the root carries exactly one `interactionSelect`; a pick row has zero bindings and carries `granularity == "object"` |
| `object_tree_item_streams_its_primitive_children_on_expand` | (b) at item level: collapsed → `total == 1`, no children; host-opened → the `cad-primitive:` row is built |

`ED/🧪️tests/🔬️unit/🦀️.rs` — `set_panel_page_records_the_section_cursor_on_the_config_lane` and its
`📄️PanelPaging` region deleted, plus the `CadCommand::SetPanelPage` row in `every_command()`.
Inspection/catalogue tests updated for the new `&TreeWindows` parameter (`TreeWindows::unhosted()`).

## 5. Verification (all foreground, output under `🗑️generated/a3-cad/`)

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-framework-plugin` | clean (wave-1 SDK gate probe) | — |
| `cargo check -p semio-s-artifact-cad-cad --all-targets` | 0 errors | `check.txt` |
| `cargo test -p semio-s-artifact-cad-cad` | **336 passed, 18 failed, 1 ignored** — all 18 are pre-existing, outside this packet (§7); every panel test passes | `test.txt` |
| `cargo check -p semio-s-artifact-cad-cad --target wasm32-wasip2` | 0 errors, 1 pre-existing warning | `wasm.txt` |
| `bun nx run @semio-tech/cad-plugin:describe` | regenerated `✏️s/🔌️plugins/📐️cad/🔣️.json` + `🛂️.descriptor.semio` | `describe.txt` |
| `bun run 📜️script.ts retained-audit` | **cannot run** — pre-existing break (§7) | `retained-audit.txt` |
| `bun nx run @semio-tech/cad-js:test-long` | 160 passed, 93 failed — all in the spatial-kernel TS engine, pre-existing (§7) | `ts-test.txt` |

All 23 `editor::cad::panels::*` tests pass, including the six window laws:

```
a_closed_container_stamps_its_total_without_children ... ok
a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_id ... ok
an_oversized_document_stamps_the_full_total_and_materialises_only_its_slice ... ok
document_lists_every_pane_object_bound_to_the_cad_domain ... ok
object_rows_are_domain_picks_without_a_row_binding ... ok
object_tree_item_streams_its_primitive_children_on_expand ... ok
```

Manifest regeneration notes (for the other app packets): the nx `describe` target OOM-SIGKILLs under
fleet load. What worked: build the two heavy artefacts alone first —
`CARGO_PROFILE_WASM_DEV_DEBUG=false cargo build -p semio-s-plugin-cad --target wasm32-wasip2
--profile wasm-dev -j 3` and `cargo build -p semio-framework-plugin-describe -j 3` — then run
`bun nx run @semio-tech/cad-plugin:describe` with the same env. The resulting `🔣️.json` diff is exactly
the four `setPanelPage` action blocks removed plus the three regenerated `*Sha256` fields.

Mechanical cross-check (run inline, not a checked-in script): `app_commands!` ids (37) equal the
fixture `routes[]` ids in order; `CAD_RETAINED_TOOL_IDS` (31) equals `expectedAdmitted` equals
`admittedRoutes`, in order; the Rust law `retained_route_fixture_matches_the_exact_owner_manifest_and_laws`
(which pins the same joins) passes.

## 6. Pre-existing failures, NOT caused by this packet

Verified against the failure messages: none of these name a path this packet touched, and all 18 Rust
failures were already present in the first run of `cargo test -p semio-s-artifact-cad-cad` before the
one genuine packet failure was fixed (the failing set is byte-identical minus that one test).

1. **Registry-less `new_app` vs the tool-proof catalog** — 12 `editor::cad::component::unit_tests::*`
   failures, all `interactive-job.catalog-authority: tool factory proof rejected tool 'addNode' …
   migrated={}`. `artifact_app_laws::new_app::<A>()` builds `VcsArtifactApp::new` with an EMPTY
   `AppActionRegistry`, so `AppActionRegistry::migrated_tool_ids()` is empty and
   `validate_tool_job_rows` rejects every proof row. Framework-owned
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12576-12629`, `:21651`); the fix is either
   `new_app_with_registry(cad_app_manifest_for_tests)` at those call sites or a framework-side
   exemption for an unregistered app. Matches the known "Registryless testkit::new_app Unusable" note.
2. **Store history/retirement regressions** — `art_cad_demo_tests::demo_subset_integrated_roundtrip`,
   `…mutations::binary::tests::create_shape_model_round_trips_through_store`,
   `…snapshot::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`:
   `apply: ValidationFailed("edit history insertion requires its exact mutation retirement factory")`;
   `…mutations::binary::tests::cad_projection_defaults`: `artifact store reached Drop without its exact
   terminal-empty shallow-shell witness` (`🧰️framework/…/🏪️store/🦀️.rs:18451`, `:21593`).
3. **`cad_document_contract_round_trips_exact_child_identities`** — `assert!(from_json_str::<CadArtifact>
   (&text).is_err())` in `…/🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs:26`.
4. **`cad_document_contract_world_window_runtime_isolates_commands_and_restores_exact_owner`** —
   "CAD window publication timed out".
5. **`retained-audit` cannot run at all** — `📜️script.ts:86` reads
   `…/✳️any/🧬️schema/🔣️.json` `.$defs.CadRetainedJobs`, which no longer exists anywhere in the repo
   (the schema was regenerated at commit `3250e6cb90`, 2026-09-15, and the hand-added `$defs` entry was
   dropped); Ajv then throws `schema must be object or boolean`. Second latent break behind it:
   `semanticValid` compares `.action_interactive_job("<id>", Migrated)` annotations against every
   fixture route, but `setContributions` sets `definition.semantics.execution.interactive_job` by field
   assignment inside `.command({ … })` instead, so `annotationPairs` is one short of `expectedPairs`
   regardless of `setPanelPage`. Both predate this packet and need their own ticket (restoring a `$defs`
   in a generated schema is out of this packet's lease).
6. **`@semio-tech/cad-js` vitest** — 93 failures, all under
   `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts` and the cad JS core-interaction suite
   (`TypeError: undefined is not an object (evaluating 'M.boxModelDiff')`, `interactionCompileCacheClear`).
   No TypeScript was changed by this packet beyond two literal strings in the audit script's allowlist,
   which this suite does not read. Also note the default `test` level's 15 s budget is unreachable under
   fleet load — use `test-long`.

## 7. Not finished

Nothing in the packet's scope is outstanding. `setPanelPage` has zero occurrences left anywhere in
`✏️s/🔌️plugins/📐️cad/` (Rust, fixtures, TS, generated manifest and descriptor), and no `.more` key or
`+N` label is produced by any cad panel.

Wave-1 gate note: `📓️p1-contract.md` and `📓️p3-sdk.md` only appeared after this packet's builds had
already run, so the gate was satisfied by evidence first — the whole `🔖️PanelWindowing` region landed and
re-exported, `ViewModel.tree_windows`/`tree_viewport_rows`/`TreeWindowRequest` in `🛂️manifest`,
`TreeWindow`/`granularity` in the UI contract, and `cargo check -p semio-framework-plugin` clean before any
cad build. Re-checked against `📓️p3-sdk.md` afterwards: every signature this packet calls matches it
verbatim — `window_section(windows, id: &str, label: Option<Label>, default_open, entries, row)`,
`window_section_or_placeholder(…, row, placeholder_label)` (placeholder LAST),
`tree_window_item(windows, item: TreeItemBuilder, id, default_open, entries, row)` (it applies
`.default_open` itself), `interaction_domain(controller_id: &'static str, domain)`, and the SDK
`ui_node_list`. cad never called `ui_history_panel`, so P3's signature change there does not reach it.

P3 flagged that a peer was temporarily holding `UI_BUILT_CHILDREN_MAX`/`UI_VALUE_PAGE_ROWS` at the pre-design
bisect values (`32`/`31`). That bisect is now reverted (`UI_BUILT_CHILDREN_MAX = UI_DOCUMENT_NODES`,
`UI_VALUE_PAGE_ROWS = UI_BUILT_CHILDREN_MAX`). The cad window laws were written to derive from the LIVE
constant (`assert!(nodes > ui::UI_BUILT_CHILDREN_MAX)` over a 300-node container, `children.len() <=
ui::UI_BUILT_CHILDREN_MAX`), so they stay green under either value and need no revisiting.

One dependency was added: `semio-framework-ui-contract = { workspace = true }` on
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml`. `tree_window_item` takes a
`TreeItemBuilder`, and the only constructor for one is `semio_framework_ui_contract::tree_item(Label)`,
which `semio_framework_plugin`'s prelude shadows with its own `(id, label) -> BuiltNode` wrapper. Other
plugin crates (block, flow, remodel, trinity, …) already depend on the contract crate the same way.

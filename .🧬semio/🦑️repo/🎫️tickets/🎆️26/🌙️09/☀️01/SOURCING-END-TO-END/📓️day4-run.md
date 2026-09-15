# 📓️ Day 4 run — 2026-09-15

Resumes [📓️day3-run.md](📓️day3-run.md) and [✅️end-to-end-checklist.md](✅️end-to-end-checklist.md). Repo MCP did not
connect this session, so the ticket folder was managed on disk.

## Boot path, in the order it failed

| # | Symptom | Root cause | Fix |
| --- | --- | --- | --- |
| 1 | `component-dev` fails: `cannot find module semio_framework_tool_run` | `PluginApp` dispatch macro expands to that crate; sourcing never depended on it | `semio-framework-tool-run` added to `📦️packages/🦀️rust/Cargo.toml` |
| 2 | Whole shell replaced by `missing=["1:sourcing-grid"]` | Unfiltered demo grid scene = 34 074 B > 32 768 B fixed surface doc; `MeshWindowKit::render` bypassed the paged `scene_surface` | Framework: `MeshWindowKit::render` → `scene_surface` (World3d lanes). Sourcing: one shared unit-box mesh, every stock kind drawn as scaled box-part instances (`box_parts`, `kind_instances_json`) — frames are 4 box parts |
| 3 | Pool/Curated rows render with empty cells | `TableWindowKit::render` emitted `["Name",…]` + `[["…"]]`; both hosts read `{id,label}` columns and `{id,<col>}` rows | Framework: `TableWindowKit::render` emits the record contract; `app-window-kits` test updated |
| 4 | `setActiveExample demo` → `sourcing.example.unknown` | Picker ids come from the subset `📚️examples` registry (`demo`), handler knew only `demo-stock`/`empty-curation` | Handler + `setActiveExample` arg options resolve through `subset::examples()`; `""` (shell "No example") loads the empty curation |
| 5 | → `coerceWireBytes: unsupported payload undefined` | React `wireEffectToFriendly` read `val.pack`; WIT field is `doc-pack` (`docPack`) | `PluginRuntime`: `val.docPack`; three in-source tests now build the real wire shape |
| 6 | → `loadDocumentArchive: unknown fault` | Guest faults are UTF-8 JSON text, `faultDisplayMessage` only decoded pack records | `faultDisplayMessage` falls back to UTF-8 text (one rule; PluginRuntime's duplicate removed) |
| 7 | → `hydration was rejected: Identity` | App-minted SPR (`empty_document_spr("curation", …)`) lacks the live store's composition dialect/owner | Store: `stamp_document_spr_identity`; `VcsArtifactApp` stamps every published `Effect::LoadDocument` with its own id/schema/dialect/owner |
| 8 | → guest `unreachable` trap, instance poisoned | `begin_persisted_document_store_replacement` refused (no initializer) and the fault branch needed the optional decode bundle → `?` dropped the envelope unretired | Store: `DocumentStoreOwners::retire_envelope`; archive fault paths retire through document store owners. Plugin: generic `bounded_document_store_initialization_job` (port of writer's authority) + sourcing's `build_document_store_initialization_job` |
| 9 | → clean `Fault`: closure `Incomplete` | See **Open** below | — |

## UI reach restored

`c3a79bd4ce` (2026-08-24) stripped the pool/curated windows to plain `TableView`s — no stepper, no remove,
no drag, no filter bar — so no curation command was reachable from the UI.

- Pool/Curated: `sourcing_table` + `sourcing_table_row` + `sourcing_table_action` (editor root) — typed
  cells (stepper → `curationSetCount`, trash → `curationRemove`), `{objectId}` drag payload under
  `SOURCING_DRAG_MIME`, drop actions `dropOnPool`/`dropOnCurated`, `sort_json`.
- Pool filter bar: query input, one toggle per module, typology select, min-availability number input.
- React `TableHost`: read `dropActionJson` (Rust moved `drop_action` → `drop_action_json`; TS type aligned).
- Preview selection: `TableScene` gains `domain_granularity_id` (mirrors `World3dScene`; Rust codec, TS type,
  round-trip test); a `TableHost` row click on a domain-bound table dispatches `interactionSelect` on it;
  sourcing tables bind the `rows` domain (`object` granularity) and `render_with_request_context` renders the
  Preview from `interaction.selection("rows")`.
- UI `Table`: sortable headers were never rendered — header button + `aria-sort`, law extracted to
  `tableSortNextDirectionV1`/`tableSortAriaV1`, fixture `🖱️ui/🧫️fixtures/📊️table-sort-header`, suite registered.

## Verified live on :6081 (React, wasm built 22:06)

Evidence: `input` values / table DOM read back after each action, no `failed|trap` in a console hook.

- Boot: Pool (10 kinds), Curated, Preview, Grid (3D beams, window frames, slabs).
- `+` on a pool row → Curated row appears, both tables show the count; `+` in Curated → 2 in both.
- Trash → row removed. Undo (History panel) → row restored.
- Drag Pool → Curated (`dropOnCurated`) and Curated → Pool (`dropOnPool`).
- Filters: module toggle (slabs only), query `steel`, min availability 15, typology `Steel`; Grid follows filters.
- Sort: Availability header asc → desc, `aria-sort` set.
- Row click → Preview renders the selected kind (Casement frame, then KVH beam); the selection broadcasts
  to the Grid highlight.

## Tests

- `cargo test -p semio-s-artifact-sourcing-curation --lib`: **126 passed, 0 failed, 2 ignored** (was 10 failing).
  Stale harness tests fixed: typed `dispatch` now settles (`context::settle`), 14-row command count,
  `SetFilterModules([])` is a legal retained edit, oracle catalog moved to `🔮️oracles/🔣️.json`, grid/pool/preview
  tests decode laned/nested surfaces.
- `cargo test -p semio-framework-plugin --features artifact-app-testing --lib -- window_kits_tests recursive_document_archive load_document`: 18 passed.
- PluginRuntime in-source `load-document` tests: 3 passed (`vitest-plugin-runtime.config.ts`).
- `📊️table-sort-header`: 5 passed (`vitest-table-sort.config.ts`).
- `semio-framework-ui-scene`: 125 passed (TableScene value round trip with the granularity field).

## Open

1. **Example switch / persisted reload of a curation.** `CurationSnapshot.catalog` is a composed
   `#[child(kind="s.stdio.semio")]` kit child, but the editor runs `VcsArtifactApp<…, NoMembers>` and never
   creates the member, and `Effect::LoadDocument` carries no members. Archive closure validation therefore
   rejects every whole-document load as `Incomplete` (now a clean fault, no trap). Completing it needs
   `SemioMembers` + a genesis/stock-driven kit child, and a member-carrying document load from guest to host
   (WIT/kernel/both hosts). Flow has the same gap (`🚧️ .example_source`). Spec kept as the ignored test
   `example_load_settles_through_the_host_document_archive_door`.
2. `curationSetCount` from the Pool stepper beyond availability and the Grid/Preview Actions menus were not exercised.
3. 263 generated structural-correspondence tests across plugins still read the removed `🔣️oracle.json`.

## Machine notes

- Twelve `wasm-dev` cargo builds (several sessions) deadlocked in `prebuild_lock_exclusive` under
  `-Zfine-grain-locking` for 1h+; no rustc anywhere. Killing the deadlocked set released it;
  `CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false` only starves behind continuous shared holders.
- Vite on :6081 wedged once (listening, not serving) after edit bursts — recycled.

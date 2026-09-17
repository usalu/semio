# 📓️ R2 — re-verification of 🏗️fem 2d/3d, 🔋️energy and the nine A7 list/graph apps

Packet R2 of ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING, run 2026-09-17 after the A4/A5/A7
packets reported green ~7 h earlier, after F2 landed the body-wide node ledger in `TreeWindows`, and
under concurrent peer churn (FEM-3D-*, ENERGY-*, REMODEL tickets active; load average 90–190 on the
shared build dir for the whole run).

Scope: `semio-s-artifact-fem-2d`, `semio-s-artifact-fem-3d`, `semio-s-artifact-energy-model`,
`semio-s-artifact-flow-flow`, `semio-s-artifact-dag-dag`, `semio-s-artifact-vcs-vcs`,
`semio-s-artifact-note-note`, `semio-s-artifact-sequence-sequence`, `semio-s-artifact-writer-writer`,
`semio-s-artifact-animate-presentation`, `semio-s-artifact-imperative-procedure`,
`semio-s-artifact-remodel-remodeling`.

---

## 1. Completeness audit — every Tree in every panel

Method: enumerate every `🦀️.rs` under `📌️panels` for the twelve crates (37 panel files), plus every
tree built OUTSIDE `📌️panels` (grep for `PanelTreeBuilder::new`, `ui::tree(`, bare `tree(`,
`ui::section(`, `section_or_placeholder` across each plugin's `✏️editor`, excluding `🧪️tests`,
`dist/`, `target/`). Then classify every container as **windowed** (streams the document) or
**bounded chrome** (a fixed field form / a closed set of controls, which by §8.1 does not need a
window). Residue grep (`-a`, source extensions only, `dist`/`node_modules`/`target` excluded) for
`paged_panel_section|panel_continuation_row|panel_page_rows|PanelRowBudget|setPanelPage|
set_panel_page|panel_pages|SECTION_ROWS|IDS_ROWS|LIST_ROWS_MAX|CATALOGUE_GROUP_ROWS|
PANEL_RECONCILE_NODE_BUDGET|section_page|artifact_tree_cache` over `*.rs`, `*.ts`, `*.tsx`, `*.json`,
`*.graphql`, `*.proto`, `*.toml`, plus `fn ui_node_list` / `fn fixed_nodes` / `fn paged_` /
`continuation` / `.take(`.

### 1.1 Result: **one real gap**, now fixed

`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` — the flow
generate-mode **Generations** window body (`flow.play.generations`). It is a `PanelTreeBuilder` tree,
but it lives under `🎭️modes/…/🪟️windows/`, not under `📌️panels/`, so wave 2 never touched it. It
pushed every `GenerationPlayState.generations` entry into a `UiFixedList` and handed that to a plain
`.section_or_placeholder(...)`:

```rust
for entry in &generation.generations {
    items.try_push(generation_item(entry, surface_prefix, locale, terminology)?)
        .map_err(|_| generation_error("items"))?;   // ← HARD FAULT past UI_FIXED_LIST_ITEMS
}
```

That is not a `+N` truncation, it is a **render fault**: the 33rd generation fails the whole body with
`ui.generations / fixed UI admission failed at items`. Playbook generation counts are unbounded by
construction (every `addGeneration` appends), so this was reachable.

**Fixed** — the section is now `PanelTreeBuilder::window_section_or_placeholder(windows, …,
&generation.generations, |entry| generation_item(…), <empty label>)`; `render` gained a
`windows: &TreeWindows<'_>` parameter, and both dispatch arms in
`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs` (`render` and `render_with_request_context`) build
`TreeWindows::for_body(view_state, FLOW_PLAY_BODY_GENERATIONS)`. The `Add Generation` row stays in its
own fixed `flow-play-generate.actions` section (fixed chrome beside a content list, per the wave-2
brief). Three window laws added to
`…/🗂️generations/🧪️tests/🔬️unit/🦀️.rs` ((a) 200 generations stamp `total == 200`, materialise less
than the whole list and mint no `.more` / `+` row; (b) `open: Some(false)` stamps the total and builds
zero children; (c) `{offset: 120, rows: 6}` materialises exactly `gen-120 … gen-125` keyed by the raw
generation id).

### 1.2 Result: everything else already windowed — per-app inventory

Legend: **W** = windowed container (`window_section` / `window_section_or_placeholder` /
`tree_window_item`), **F** = bounded chrome that legitimately stays a plain `.section` (a fixed field
form for ONE selected entity, or a closed control set).

| App / body | containers |
|---|---|
| **fem2d + fem3d** `📌️panels/🗿️artifact` (`fem{2,3}d.play.artifact-panel`) | **W×9**: nodes, elements, solids(3d)/regions(2d), supports, load-cases, combinations, materials, sections, analysis. **W nested**: each load-case row nests its loads and each combination row nests its terms through `tree_window_item`. Tree binds `.interaction_domain(FEM{2,3}D_PLAY_CONTROLLER_ID, …)`. **This is the user's "all the nodes in fem"**: `&doc.nodes` is handed to the window whole, so `window.total == doc.nodes.len()` for any document size. |
| **fem2d + fem3d** `📌️panels/🔍️inspection` | **W×3**: multi-selection id list, a load case's loads, a combination's terms. **F×12**: `summary`/`analysis` count blocks and the one selected entity's field form (`node` 4 rows, `element` ≤7, `solid` 10, `material` 6, `section` 6, `support` 8, `load` ≤5, `load-case` 3, `combination`, `actions` ≤3) — each a closed set of controls for a single record, never a document list. The two paths are mutually exclusive (`summary` early-returns). |
| **fem2d + fem3d** `📌️panels/📊️results` | **F only** — the results panel is a FORM (`ui::section` + select/slider/number/toggle/transport rows), not a `Component::Tree`. No document list. |
| **energy** `📌️panels/🗿️artifact` (`energy.model.artifact`) | **W×11**: site, zones, shading, materials, glazing-materials, gas-materials, constructions, loads, controls, hvac, schedules. **W nested ×2 levels**: zone › `ZoneChild{Space,Surface}`, surface › fenestrations, both `tree_window_item`. |
| **energy** `📌️panels/🔍️inspection` | **W×2**: multi-selection header, the construction layer stack (`LayerRow::{Pick,Remove}` over one flat list). **F×13**: surface/fenestration/zone/material/glazing/gas/construction/thermostat/shading field forms, `summary`, `site`, `results`, `actions`. |
| **flow** `🗿️artifact` W×2 (widgets, synapses) · `🛍️catalogue` W over **every** host catalogue section (a `for section in sections` loop) + `extensions.installed` + `extensions.actions` · `🔍️inspection` W×1 · **`🪟️windows/🗂️generations` W×1 (this packet)** | |
| **dag** `🗿️artifact` W×2 (nodes, edges) · `🛍️catalogue` W×1 (node-kinds) · `🔍️inspection` F×3 (multi-select field rows) | |
| **vcs** `🗿️artifact` W×2 (checkpoints, alternatives) · `🔍️inspection` F×1 (document fields) | |
| **note** `🗿️artifact` W×1 (`note-play-blocks.blocks`) + `tree_window_item` at every `Group` level; F×1 (`note-play-blocks.add`, 5 quick-add rows) · `🛍️catalogue` F×1 (6 fixed kinds) · `🔍️inspection` F×1 | |
| **sequence** `🗿️artifact` W×2 (steps, edges) + `tree_window_item` per control-flow SLOT at every depth; F = a control step's closed child set (collapse toggle + 1–2 slots) · `🛍️catalogue` W×1 (`…slots`) + F×1 (5 built-in kinds) · `🔍️inspection` F×1 | |
| **writer** `🗿️artifact` W×1 (`…ast`) + `tree_window_item` at every AST level; the non-`jack` fallback is a 2-row F meta section · `🔍️inspection` W×1 (lint diagnostics, the old `.take(8)` is gone) + F×1 · `🛍️catalogue` F×1 (one row) | |
| **animate** `🗿️artifact` W×1 (tiles) · `🛍️catalogue` F (2 fixed `section`s of templates/figure controls) · `🔍️inspection` F×1 | |
| **imperative** `🗿️artifact` W×1 (steps) · `🛍️catalogue` W×1 (actions) · `🔍️inspection` F×1 | |
| **remodel** (7 panels) `⚙️parameters` W×1 · `🧵️results` W×1 · `🗿️artifact` W×1 (pipeline) · `✅️quality` W×2 (report rows, warnings) · `🎯️calibration` W×2 (cameras, gcps) + F×1 (summary) · `🏃️tracks` W×1 + F×1 (motion status) · `🗂️media` W×1 (imported streams) + F×1 (drop zone/counts) | |

`TreeWindows::for_body(view_state, <BODY_KEY>)` is built per body inside each app's `render` /
`render_body` / `render_with_request_context` `match` — verified by reading every dispatch arm of all
twelve editors. Bodies whose panel has no unbounded list correctly take no `TreeWindows`.

### 1.3 Residue: zero

- `paged_panel_section`, `panel_continuation_row`, `panel_page_rows`, `PanelRowBudget`,
  `setPanelPage` / `set_panel_page`, `panel_pages`, `SECTION_ROWS`, `IDS_ROWS`, `LIST_ROWS_MAX`,
  `CATALOGUE_GROUP_ROWS`, `PANEL_RECONCILE_NODE_BUDGET`, `section_page`, `artifact_tree_cache` —
  **0 hits** across all eleven plugin directories in `*.rs`, `*.ts`, `*.tsx`, `*.json`, `*.graphql`,
  `*.proto`, `*.toml`.
- Each plugin's generated manifest `✏️s/🔌️plugins/<plugin>/🔣️.json` — **0** `setPanelPage` /
  `panelPages`. No regeneration was needed (no action catalogue changed in this packet; the flow fix
  adds no command).
- TS side: **0** `tree.?window|panelPage|SECTION_ROWS|IDS_ROWS|LIST_ROWS|retained-audit` hits (one
  false positive, the word "more" inside a prose comment in energy's `✏️editor/🟦️.ts`).
- Local `fn ui_node_list` / `fn fixed_nodes` / `fn paged_*` copies — **0**. Every `ui_node_list` in
  these crates is the SDK re-export.
- `continuation` — only law names (`window_law_no_continuation`, `…never mints a continuation key`)
  and unrelated engine/job comments.

### 1.4 Deliberately kept, with the reason (each re-read and re-justified)

| Site | Why it is NOT app paging |
|---|---|
| `MARKED_IDS_LIMIT = UI_FIXED_LIST_ITEMS` in fem2d/fem3d/energy `🗿️artifact` `marked_ids()` | `PanelTreeBuilder::selected()/highlighted()` admit into a `UiFixedList<UiText>` (32 hard cap). Unclamped, an 80-id selection FAULTS the render. Not a row list. |
| `SELECT_ITEMS_MAX = 24` in fem2d/fem3d/energy `🔍️inspection` `select_row()` / `thermostat_select()` | `SelectProps::items` is `UiFixedList<SelectItem>` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:310`) — a 32-slot fixed list. **The contract has no windowing for `<select>` options**, so this ticket cannot remove the clamp. ⚠️ It IS a user-visible truncation of "all the nodes in fem": an element's `start`/`end` node picker shows 24 of N nodes. Windowing it needs a contract change (a `window` on `SelectProps` + a host observer), which is out of this ticket's §3 scope. **Recorded as an open limitation, not fixed.** |
| `RESULT_SOURCE_OPTIONS` in fem2d/fem3d `📊️results` `result_sources()` | same shape (`<select>` options), same reason. |
| `SELECT_ITEMS_MAX` in energy `layer_options` / `schedule_options` consumers | same. |

These are the only `.take(` calls left in any panel file of the twelve crates (9 hits total, all
listed above; the nine A7 apps have **zero**).

### 1.5 Finding for F2 (not fixed here — the ledger is F2's)

`TREE_WINDOW_FIXED_NODE_HEADROOM = 16` reserves 16 node records for rows built OUTSIDE the ledger.
Two inspection bodies in this packet's scope build more fixed rows than that in their worst case, so
their windowed containers can be granted more than the body can actually seat:

- **energy `📌️panels/🔍️inspection`**, fenestration selected with a multi-selection: root(1) +
  selection section(1) + `…fenestration` section(1) + **16 fixed field rows** + `…actions`(1+≤3) ≈ 23
  un-ledgered records against a 16-row reserve — ~7 over.
- **fem3d/fem2d `📌️panels/🔍️inspection`**, solid selected: section(1) + 10 fixed rows +
  `…actions`(1+3) ≈ 15 — just inside, but with no margin.

F2 owns `TREE_WINDOW_FIXED_NODE_HEADROOM` and the fem3d House-sized law; this packet did not change
the constant, did not add a House law (the brief forbids duplicating it), and there is no House law in
fem's panel tests as of this run. Recommend F2 either raise the headroom to ~24 or have
`PanelTreeBuilder::section` debit the ledger too.

---

## 2. Files changed by this packet

| Path (relative to `/Users/ueli/Documents/semio`) | Change |
|---|---|
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` | `render(..)` takes `&TreeWindows<'_>`; the generations list is a `window_section_or_placeholder`; the `UiFixedList` accumulation loop is gone; `TreeWindows` imported |
| `…/🗂️generations/🧪️tests/🔬️unit/🦀️.rs` | three window laws added beside the existing add-action law |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | both `FLOW_PLAY_BODY_GENERATIONS` dispatch arms build `TreeWindows::for_body(view_state, FLOW_PLAY_BODY_GENERATIONS)` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | `wide_view()` asks for a realistic 8-row slice instead of 128 (`OPENED_ROWS`, documented); the marked-ids law builds a fresh `TreeWindows` for its second render — §3.4 |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | only the same 4-line fresh-`TreeWindows` fix in the marked-ids law (F2 owns the rest of this file and is editing it live — its `wide_view` was already fixed by F2 and was left alone) |
| `.🧬semio/…/ARTIFACT-TREE-VIRTUALISED-STREAMING/🐚️r2-verify.sh` | the lane driver |

No other file in the twelve crates needed a change. Nothing was reverted, no peer edit was touched.

---

## 3. Verification

All runs foreground, shared build dir, no `CARGO_TARGET_DIR`, env
`DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false`.
Logs under `🗑️generated/r2/`. Machine load 90–190 for the whole session (≈40 peer cargos, 3–5 rustc: the
build dir lock, not the CPU, was the bottleneck — one `cargo check --target wasm32-wasip2` across ten
crates took 21 m 48 s).

### 3.1 `cargo check --target wasm32-wasip2` — **all twelve crates green**

| Command | Result | Log |
|---|---|---|
| `cargo check --target wasm32-wasip2 --keep-going -p energy-model -p flow-flow -p dag-dag -p vcs-vcs -p note-note -p sequence-sequence -p writer-writer -p animate-presentation -p imperative-procedure -p remodel-remodeling` | **rc 0**, `Finished dev profile in 21m 48s`, **0 errors** (51 crates checked, warnings present ⇒ expansion really ran) | `wasm-a7-energy.txt` |
| `cargo check --target wasm32-wasip2 --keep-going -p fem-2d -p fem-3d --features component-app-assembly` | **rc 0**, `Finished dev profile in 5m 38s`, **0 errors** | `wasm-fem.txt` |

### 3.2 Window laws — **46 of 47 green; the one red is the swapped fem3d demo**

Extracted by name from the full runs (a filtered re-run shares the same test binary, so the filter is
just a projection of the same evidence; the fem panel filter was also run standalone).

| Lane | Window laws |
|---|---|
| `cargo test --no-fail-fast -p energy-model -p flow -p dag -p vcs -p note -p sequence -p writer -p animate -p imperative -p remodel` | **42 passed, 0 failed** — every `…stamps…` / `…materialises…` / `…its_slice…` / `…granularity_and_the_tree…` / `…oversized…` law in all ten crates |
| `cargo test --no-fail-fast -p fem-2d -p fem-3d --features component-app-assembly -- panels::` | fem2d **35 passed, 0 failed**; fem3d 18 passed, 17 failed (§3.4) | `laws-fem.txt` |
| fem2d artifact-panel window laws | `an_oversized_document_stamps_every_extent_and_materialises_one_viewport` ✅ · `a_closed_container_stamps_its_extent_and_builds_no_child` ✅ · `a_window_request_materialises_exactly_its_own_range` ✅ · `rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select` ✅ |
| fem3d artifact-panel window laws | the same first three ✅; `rows_declare_their_granularity_…` ❌ — it looks up row `"n1"`, an id the swapped demo no longer contains (§3.4) |
| **flow generations (new, this packet)** | `an_oversized_generation_list_stamps_its_whole_total_and_materialises_at_most_its_slice` ✅ · `a_closed_generation_list_stamps_its_total_and_builds_no_children` ✅ · `a_generation_window_request_materialises_exactly_its_slice_keyed_by_the_generation_id` ✅ |

### 3.3 Full suites

| Crate | Result | Note |
|---|---|---|
| `semio-s-artifact-energy-model` | 6289 passed, **3 failed** | identical to 📓️a5-energy §6.2: the three `sim::tests::p7c*` `RetainedJobPayload` laws. No panel code. |
| `semio-s-artifact-remodel-remodeling` | 1290 passed, **9 failed** | was 37 in A7; all remaining are `interactive-job.catalog-authority` |
| `semio-s-artifact-dag-dag` | 176 passed, 28 failed | ditto |
| `semio-s-artifact-flow-flow` | 131 passed, 95 failed | ditto |
| `semio-s-artifact-note-note` | 332 passed, 57 failed | ditto |
| `semio-s-artifact-sequence-sequence` | 73 passed, 130 failed | ditto + retirement witnesses |
| `semio-s-artifact-imperative-procedure` | 82 passed, 59 failed | ditto |
| `semio-s-artifact-vcs-vcs` | 97 passed, 18 failed | ditto |
| `semio-s-artifact-writer-writer` | window + panel laws pass, then the binary **SIGABRTs** in `io::mutations::binary` ("thread caused non-unwinding panic") | 📓️a7 §5.3, unchanged |
| `semio-s-artifact-animate-presentation` | all four window laws + the panel tests ran; two `tile_editor` scene tests fail | 📓️a7 §5, unchanged |
| `semio-s-artifact-fem-2d` | **1257 passed, 4 failed** (was 1249/12 before this packet's two fixture fixes) | the 4 are pre-existing, §3.4 |
| `semio-s-artifact-fem-3d` | 1087 passed, **42 failed** | 25 outside `panels::`, §3.4 |

Logs: `full-a7-energy.txt`, `full-fem.txt`.

**Every failure above is pre-existing and peer-owned, with one shared cause for the A7 set**: the
framework tool-proof gate refuses the app fixture before any panel is rendered —

```
tool proof catalog must exactly join migrated generated declarations to live concrete factories:
FaultCode("interactive-job.catalog-authority") … tool 'removeWidget' … schema='flow.snapshot'
expected_schema='flow.host_snapshot' … owner_eq=true controller_eq=true schema_eq=false
```

(`🧰️framework/…/🔌️plugin/🦀️.rs:21838`). Every failing panel test in those ten crates is an
`render_body(&mut app, …)` app-fixture test that dies there; **every test that drives `render(...)`
directly — i.e. every window law — passes.** That is exactly 📓️a7-graph-and-list-apps.md §5.1.

### 3.4 fem — what this packet fixed, and what is a peer's moving demo

**Fixed here (regressions genuinely caused by this ticket's new node ledger):**

1. `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (fem2d) — `wide_view()` asked for `rows: 128` on three
   sections. Since F2's ledger, a host request RESERVES `1 + min(rows, UI_BUILT_CHILDREN_MAX)` records
   off `TREE_WINDOW_BODY_NODE_BUDGET` (= `UI_DOCUMENT_NODES − 1 − TREE_WINDOW_FIXED_NODE_HEADROOM` =
   `128 − 1 − 16` = 111) before any container is built, so 3 × 129 = 387 reserved the whole budget and
   every container the host had NOT addressed materialised **zero** rows
   (`a viewport this tall materialises section nodes whole — left: 0, right: 12`). Eight rows is a slice
   a host can actually file (`Σ(1 + rows) ≤ TREE_WINDOW_BODY_NODE_BUDGET`) and still exceeds all three
   sections' counts. **8 fem2d panel laws went green.** fem3d's copy of this fixture had already been
   fixed by F2 in the same file (it now asks for 4).
2. `selected_and_hovered_ids_are_marked_from_the_interaction_snapshot` in **both** fem2d and fem3d —
   the law rendered twice from ONE `TreeWindows`. That value carries the first-paint budget and the node
   ledger in `Cell`s, so the second render ran on an exhausted ledger and returned `Err`
   (`an oversized selection never faults the panel`). Design §5 says "build one `TreeWindows` per body
   per render"; the fixture now does. **Green in both crates.** (`.is_ok()` also became `.expect(..)`
   so the next failure names the fault instead of hiding it.)

**NOT fixed (peer-owned, evidence below):**

- **The fem3d demo document was swapped under its tests.** `demo()` is
  `schema::snapshot::text::fem3d_boot_snapshot()`, which parses `examples::concrete_forest::PRIMARY_TEXT`
  (asset last written 2026-09-16 20:26). The live document is now **20 nodes / 20 elements / 0 solids /
  1 material / 2 sections / 2 supports / 2 load cases × 16 loads / 2 combinations**, keyed
  `lc1b, lc2b, lv0…, l_col1, s_c1, c30, hex30`. The tests still name `n1`, `e1`, `e3`, `s_00`, `n00_g`,
  `steel`. Proof: `a_selected_node_renders_bound_ordinate_inputs_3d` fails with
  `no node keyed fem3d-play-inspection.node.x.input in {…"Solids":"0"…}` — the inspector fell through to
  the document summary because the selected id resolves to no entity. **25 of the 42 fem3d failures are
  outside `📌️panels` entirely** (`commands::patch_{node,element,load,material,section,solid,support}`,
  `interaction::gumball::*`, `interaction::entity_kinds_resolve_every_id_of_the_demo`,
  `standards::…::scene::*`, `fem3d_engine::mesh_preview`, `live_visual`, `viewer::…`) and cannot touch
  panel trees. The remaining 17 panel failures are the same id mismatch, plus
  `demo_document_lists_every_section_with_its_own_count` which now fails as
  `materialises section combinations whole — left: 1, right: 2`: the enlarged demo (9 sections + 82 rows
  + 3 nested containers ≈ 94 records, of which 27 are reserved by `wide_view`) no longer fits one body's
  111-record ledger, so the tail section is starved — correct ledger behaviour, an obsolete assertion.
  **F2 is editing this exact file right now** (it gained a `house()` fixture, a `body_nodes()` helper and
  a lowered `wide_view` while this packet ran), so R2 deliberately did not rewrite those laws — that is
  the House-sized law F2 owns.
- **fem2d's 4 remaining failures** are 📓️a4-fem §4's list, unchanged:
  `analyses::tests::assembly_job_one_fuel_steps_stay_below_eight_milliseconds` and
  `mesh::tests::mesh_job_large_boundary_never_runs_to_completion_in_one_step` (wall-clock budget laws in
  the shared `✏️s/🔨️modules/🏗️fem` engine, failing under a load average of 90–190),
  `editor::fem2d::component::unit_tests::every_route_declares_the_lane_its_handler_emits` and
  `…::two_instances_converge_on_disjoint_edits` (peer classification/convergence work). None is panel code.

---

## 4. Duplicate `node_key` within one body (F2's new loud SDK error)

Container node keys per body, checked for every app in scope:

| App | Container node keys | Unique? |
|---|---|---|
| energy artifact | 11 namespaced section ids + zone/surface ids via `energy_target_id(EntityId)` | ✅ — `EntityId` is documented as "ONE shared space across every `Model` collection" (`…/🕹️interaction/🦀️.rs:92`) |
| note | `note-play-blocks.blocks` / `.add` + `block_tree_row_id` = `note-play-block:{id}` | ✅ namespaced |
| sequence | 2 section ids + `sequence-play-document.slot.{step_id}.{slot_name}` | ✅ namespaced |
| writer | `writer-play-document.ast` + raw AST node ids (path-unique) | ✅ |
| flow / dag / vcs / animate / imperative / remodel | namespaced section ids only (no nested containers) | ✅ |
| **fem2d + fem3d artifact** | 9 namespaced section ids **+ raw `case.id` (load-case rows) + raw `combination.id` (combination rows)** | ⚠️ **two separate id spaces, both un-namespaced, in one body** |

⚠️ **fem is the one residual risk.** `📌️panels/🗿️artifact/🦀️.rs` passes `&case.id` and `&combination.id`
straight to `tree_window_item` as node keys (`🧊️3d` L277 / L293, `◻️2d` L281 / L296). fem validates
duplicate ids **per collection** (`create_node_duplicate_id_is_fatal`, `mutation.duplicate-id`), never
across collections, so a document with a load case and a combination both called `uls` would file two
containers under one node key. **No shipped fem document collides** — I read every example: house
(`dead/live/snow` vs `uls/sls`), concrete-forest (`dead/live` vs `uls/…`), demo (`dead/live` vs `uls`),
and the schema/mutation fixtures (`point/self/drop/wind` vs `uls`) — so nothing is red today.

I did NOT rename these keys: a container's node key must equal its row key for the host's
`data-tree-window-key` addressing, and §8.2 fixes the row key to the raw domain target id that
`interaction_topology` registers, so changing it would break viewport picks. The clean fix is one of
(a) fem validating id uniqueness across load cases and combinations, or (b) `TreeWindow` keys becoming
`(granularity, id)` rather than `id` — both P1/F2 decisions. **Flagged for F2 before the duplicate-key
error lands.**

---

## 5. Not finished / not run

- The two fem3d items above (demo-id mismatch, obsolete `materialises … whole` assertion) are left to the
  peer who owns the demo and to F2, with the evidence in §3.4.
- Wave-3 browser verification is not part of this packet. Note for whoever runs it: the ticket-scoped
  probe `…/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR/🐍️energy-panels-probe.mjs` still asserts `/\.more$/i`
  rows and will fail against a windowed tree (already flagged in 📓️a5-energy §6.5).
- `<select>` option lists are still clamped at 24 (§1.4) — a contract gap, not app paging.
- `TREE_WINDOW_FIXED_NODE_HEADROOM = 16` is under-sized for energy's fenestration inspector (§1.5).

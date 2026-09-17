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
| `.🧬semio/…/ARTIFACT-TREE-VIRTUALISED-STREAMING/🐚️r2-verify.sh` | the lane driver used below |

No other file in the twelve crates needed a change. Nothing was reverted, no peer edit was touched.

---

## 3. Verification

(filled in below — see §3.1/§3.2/§3.3)

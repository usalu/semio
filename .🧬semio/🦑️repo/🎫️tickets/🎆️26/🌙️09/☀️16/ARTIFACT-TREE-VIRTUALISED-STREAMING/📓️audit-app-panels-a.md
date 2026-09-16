# Audit: per-plugin panel-tree builders and paging shapes (set A)

Read-only inventory. All paths relative to repo root `/Users/ueli/Documents/semio`. Scope: 🧩️puzzle (2d/3d/5d), 📐️cad, 🏗️fem (2d/3d), 🔋️energy, 🌀️procedural (generation2d/generation3d), 🏭️process (process3d), 🧱️block (2d/3d/5d), 🎥️shooting, 🖍️draw, 🖨️raster, 🌍️gis, plus the shared `✏️s/🔨️modules/🏗️fem` and `✏️s/🔨️modules/💭️mindmap` modules.

## 0. Framework mechanism being replaced (context for every section below)

Defined once in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`#region 🔖️PanelKit` / `#region 🔖️PanelPaging`), re-exported at line 39485 (`pub use app::{paged_panel_section, panel_continuation_row, panel_page_rows};`):

- `PanelTreeBuilder` (L5784-5872) — namespaced fluent builder: `.section()`/`.section_or_placeholder()` wrap `ui::tree_section().default_open(..).try_children(..)`; `.selected()`/`.highlighted()`; `.interaction_domain()`; `.drop_action()`; `.build()`.
- `panel_page_rows()` (L5888-5890) = `UI_VALUE_PAGE_ROWS.min(ui_value_headroom().rows())` — process-wide interactive-row ceiling for one render.
- `PanelRowBudget` (L5893-5928) — a spendable row counter; `.spend()`, `.remaining()`, `.nested(reserved, closure)` (reserves rows for siblings before running a nested build — the mechanism both CAD's and FEM-3D's `section_quotas()`/`with_quota()` max-min-fair splitters are built on).
- `panel_continuation_row(section_id, omitted)` (L5933-5941) — the stock "+N" row: `tree_item("+{omitted}")`, id `"{section_id}.more"`, icon `"more-horizontal"`. **It never attaches a `Trigger`/action** — by itself it is inert/non-navigable.
- `paged_panel_section()` (L5952-5986) — places up to `section_rows` entries while `PanelRowBudget` allows, appends the continuation row for the remainder.
- A small framework unit-test file is included at `🧪️tests/🔬️app-panel-kit/🦀️.rs` (4 tests) — covers `PanelTreeBuilder`/`tree_item` basics only, no paging-navigation test.

Only two apps in this whole set add an app-owned mechanism to make the "+N" row *navigable* (i.e. an actual `setPanelPage` action + a persisted page cursor): **cad** and **puzzle (2d, 3d)**. Every other app either uses the stock, inert `paged_panel_section`/`panel_continuation_row` (fem 2d/3d, plus per-panel local truncations below), or has no paging at all (block 2d/3d/5d, and per findings below likely several of energy/procedural/process/shooting/draw/raster/gis — see their sections).

Repo-wide (excluding `🗑️generated`) symbol census, used to sanity-check every per-plugin section below:
- `panel_pages` (state field) — only in `📐️cad` (`CadPlayRuntime.panel_pages`, config-lane persisted) and `🧩️puzzle` 2d/3d (`Puzzle{2,3}dWindowTransient.panel_pages` / window-runtime, **window-transient, not config-persisted**).
- `setPanelPage`/`set_panel_page`/`SetPanelPage` (action + handler) — only `📐️cad` (`🎮️commands/📄️panel/🦀️.rs`, module `set_panel_page`) and `🧩️puzzle` 2d/3d (`🎮️commands/📄set-panel-page/🦀️.rs`).
- `setPanelPage` registered in a plugin manifest (`🔣️.json`, `class: "interactive"`, `interactiveJob: "migrated"`, `kind: "view"`, `args: []`, `inPalette: false`) — **only `📐️cad`**. Puzzle's own manifest (`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`) has **zero** `setPanelPage` occurrences even though the 2d/3d editors have a live `"setPanelPage" => set_panel_page::set_panel_page(ctx, args)` arm in `handle_action` — i.e. puzzle's dispatch arm looks orphaned/unreachable from the declared action catalogue (flag for the replacement plan; the puzzle-specific agent's findings below should be read against this).
- `"setPanelPage"` is listed in the host-generic `WINDOW_CONFIG_RAIL_ACTION_IDS` allow-list, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:4654` — a `ReadonlySet<string>` of window-config-lane verbs that force a rail re-take after a `mutationCount: 0` action; this is the only framework-side TS awareness of `setPanelPage` as a concept.
- TS references to `setPanelPage`/`panelPage` repo-wide: `🧰️framework/…/ShellHelpers/🟦️.tsx:4654` (above), `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts:97` (cad's generated action-id allowlist, cross-checked against `CAD_RETAINED_TOOL_IDS`/a `🗄️retained-jobs/🔣️.json` fixture, `routeCount === 38`), and `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧬️schema/🟦️.ts:16` (`panelPages: Record<string, number>` field on puzzle-2d's window schema). Note: the paths named in the ticket brief (`/Users/ueli/Documents/semio/📦️packages/🟦️typescript`, a repo-root `🪟️window/🧬️schema/*.ts`) do not exist — there is no top-level `packages` dir; TS lives nested per-plugin/per-framework-module as above.
- `PANEL_RECONCILE_NODE_BUDGET` — only in `🧩️puzzle` 3D (`.../🧊️3d/.../📌️panels/🗿️artifact/🦀️.rs` + its unit test) — a puzzle-3d-local host-reconcile byte/node budget, not shared by any other app.
- `SECTION_ROWS`/`IDS_ROWS`/`LIST_ROWS_MAX` (app-local page-size constants) found in: cad (`CAD_SECTION_ROWS = 6`), puzzle 2d/3d, fem 2d/3d (`LIST_ROWS_MAX = 8`, inspection-panel nested listings), energy (inspection panel) — see each section.
- `artifact_tree_cached_from` (tree-render memoization) — only in `🧩️puzzle` 3D. No other app in this set memoizes its rendered tree by any key; every other app's `build_*_tree`/`render` rebuilds unconditionally from the live document/config/interaction snapshot each call.

---

## 1. 🧱️block (2d / 3d / 5d)

Panel dirs: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection}` — **only these two panel kinds** exist per artifact; no catalogue/parameters panel.

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| 2d `🗿️artifact/🦀️.rs` | `BLOCK2D_BODY_ARTIFACT` = `"block2d.play.artifact"` (L11) | artifact/document tree | `PanelTreeBuilder::new("block2d-play-document")` (L40) + `.section_or_placeholder()` ×2 (L44-45) | none | none |
| 2d `🔍️inspection/🦀️.rs` | `BLOCK2D_BODY_INSPECTOR` = `"block2d.play.inspector"` (L10) | inspection | `PanelTreeBuilder::new("block2d-play-inspector")` + `.section()` (L62) | none | none |
| 3d `🗿️artifact/🦀️.rs` | `BLOCK3D_BODY_ARTIFACT` = `"block3d.play.artifact"` (L10) | artifact/document tree | `PanelTreeBuilder::new("block3d-play-document")` (L40) + `.section_or_placeholder()` ×2 (L45-46) | none | none |
| 3d `🔍️inspection/🦀️.rs` | `BLOCK3D_BODY_INSPECTOR` = `"block3d.play.inspector"` (L14) | inspection | `PanelTreeBuilder::new("block3d-play-inspector")` + `.section()` (L94) | none | none |
| 5d `🗿️artifact/🦀️.rs` | `BLOCK5D_BODY_ARTIFACT` = `"block5d.play.artifact"` (L9) | artifact/document tree | `PanelTreeBuilder::new("block5d-play-document")` (L38) + `.section_or_placeholder()` ×2 (L42-43) | none | none |
| 5d `🔍️inspection/🦀️.rs` | `BLOCK5D_BODY_INSPECTOR` = `"block5d.play.inspector"` (L15) | inspection | `PanelTreeBuilder::new("block5d-play-inspector")` + `.section()` (L78) | none | none |

**Headline: no paging anywhere in block.** Full-crate greps (all 3 artifacts + shared crate) for `panel_pages`, `setPanelPage`, `set_panel_page`, `PanelPage`, `paged_panel_section`, `panel_continuation_row`, `SECTION_ROWS`, `IDS_ROWS`, `LIST_ROWS_MAX`, `PANEL_RECONCILE_NODE_BUDGET`, `paged_section_from` — **zero hits**. No `📄set-panel-page`/`📄️panel` command directory under any artifact. No tree memoization. All 6 panels render their full, unpaged item lists every render — a net-new capability for virtualisation, not a migration.

**Nesting (identical shape across 2d/3d/5d):** artifact/document panels have exactly 2 flat sibling sections, both `default_open: true`: 2d = `handle-kinds`/`handles` (L44-45); 3d = `representations`/`vortices` (L45-46); 5d = `grip-kinds`/`grips` (L42-43). Each section is a flat list of leaf items (built via a local `icon_item()` helper) — **no `try_children` nesting under item rows**, depth is section→items only (2 levels); empty sections fall back to `section_or_placeholder`'s built-in "(none)" row. Inspection panels: exactly one section (`...-inspector.summary"`, `default_open: true`) of flat field rows (name/label text inputs; 3d adds a `representation` `<select>`; read-only count rows).

**Row bindings/actions:** zero `Trigger::Activate` bindings on any row (`grep` confirms) — object/vortex/grip selection is generic via `.interaction_domain(BLOCK{2,3,5}D_INTERACTION_{HANDLE,VORTEX,GRIP})` (2d L46, 3d L47, 5d L44; explicit code comment: "no per-item click action is declared here anymore (clicks are translated into `interactionSelect` generically)", referencing ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`). Inspector fields dispatch `Trigger::Change`/commit `"blur"` → `patchNodeKind`/`patchObjectKind`/`patchPartKind` with `{field}`; 3d's representation select dispatches `Trigger::Change` → `setActiveRepresentation`. No row actions (hide/lock/delete), no draggable rows (`PanelTreeBuilder.drop_action()` exists framework-side but unused here).

**Tests:** 6 one-assertion smoke tests (`.../📌️panels/{🗿️artifact,🔍️inspection}/🧪️tests/🔬️unit/🦀️.rs` ×3 artifacts) asserting a label/tree-type is present — none pin paging.

**Command dirs** (none named panel-page): 2d has 9 (`patch-node-kind`, `add/remove-handle(-kind)`, compatibility-rule verbs, `edit`); 3d has 24 (`patch-object-kind`, vortex/representation/window verbs, camera, brush); 5d has 7 (`patch-part-kind`, `add/remove-grip(-kind)`, `edit`). Editor `command_from_action`/`render` dispatch: 2d `✏️editor/🦀️.rs` L486/L534-539; 3d L880/L980-992 (+`render_with_request_context` L998); 5d L459/L506-512 — none reference panel paging.

**TS:** no block-specific `setPanelPage`/`panelPage` reference anywhere (block has no top-level `📦️packages/🟦️typescript`; the only repo-wide hits are the framework `ShellHelpers.tsx` allow-list and cad's/puzzle's own files, neither block-related).

---

## 2. 📐️cad

Panels: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue}`.

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` (module `document`) | `CAD_PLAY_BODY_ARTIFACT` = `cad.play.artifact` | artifact tree | `PanelTreeBuilder` + **app-local** `paged_section_from()`/`continuation_row()` layered on the SDK's `PanelRowBudget`/`panel_page_rows` | **App-owned, navigable cursor**: dispatches `setPanelPage`, cursor persisted in `CadConfig::panel_pages_json` → `CadPlayRuntime.panel_pages: BTreeMap<String,u32>` | none (rebuilt every render from `envelope`/`labels`) |
| `🔍️inspection/🦀️.rs` | `CAD_PLAY_BODY_PROPERTIES` = `cad.play.properties` | inspection (form) | raw `tree_item`/`tree_item_desc`/`tree_item_with_action` rows in one `PanelTreeBuilder` section | none (bounded field lists) | none |
| `🛍️catalogue/🦀️.rs` | `CAD_PLAY_BODY_CATALOGUE` = `cad.play.catalogue` | catalogue | `PanelTreeBuilder` + plain `ui_node_list` over static `TYPOLOGY_CATALOG` | none (small/fixed) | none |

**Nesting (artifact tree):** 9 sections, fixed order: `shape`, `shape.references`, `building`, `building.references`, `energy`, `energy.references`, `structure-classic`, `structure-classic.references`, `nodes`. Flat rows (one level); object rows nest primitive children (`cad-primitive:{suffix}:{object.id}:{primitive_id}`) via `item.children.try_push`, `default_open: false` on both objects and primitives; reference sections themselves default **collapsed** (`default_open: false`), objects/nodes sections default **expanded** (`true`). A shared `PanelRowBudget` reserves rows for later sections (`budget.nested(SECTIONS - N, ..)`) so no single pane starves siblings.

**Row bindings/actions:** object row → `Activate` = `select_object_action` → `interactionSelect` (domain `"cad"`, `{domainId, merge:"replace", method:"pick", targets}`); **no row actions** on objects (comment: hide/lock/duplicate/delete row actions were removed as no-ops pending a composed-child dispatch seam, to save argument-arena budget); `draggable: !locked`, `default_open: false`. Reference row → `Activate` = `setReferenceSelection`, **plus** row actions hide/show and lock/unlock via `patchCadPlayReference` (`RowActionPlacement::Row`). Node row → `Activate` = `setNodeSelection` (`nodeIds` list). Catalogue row → `Activate` = `addObject {modelDefinitionId, typology}`. `.interaction_domain(CAD_INTERACTION_DOMAIN)` set on the artifact tree; `.selected()`/`.highlighted()` from `document_tree_selected_ids`/`document_tree_highlighted_ids` (reference-overlay only — object selection is framework-owned by domain).

**Continuation row / setPanelPage — the one fully-wired example in this whole set:**
- `continuation_row()` (`🗿️artifact/🦀️.rs:158-168`) builds `{section}.more`, label `+{omitted}`, args `{page, section}` → dispatches `setPanelPage`; falls back to the SDK's inert `panel_continuation_row` if the argument arena has no credit.
- `paged_section_from()` (same file, L173-212): cursor-aware wrapper reading `envelope.runtime.panel_pages` to start at `page * CAD_SECTION_ROWS` (`CAD_SECTION_ROWS = 6`).
- App struct: `CadPlayRuntime.panel_pages: BTreeMap<String,u32>` (`✏️editor/🦀️.rs:195`), decoded/encoded via `parse_panel_pages`/`print_panel_pages` from `CadConfig::panel_pages_json` (`🎚️config/🦀️.rs:115-120,172`).
- Command: `set_panel_page::SetPanelPage { section: String, page: f64 }`, dsl keyword `"set-panel-page"`, defined at `✏️editor/🎮️commands/📄️panel/🦀️.rs:17-40` (note: the command **directory** is named `📄️panel`, not `📄set-panel-page` as the ticket brief's naming guess assumed — only puzzle 2d/3d use that literal directory name). `handle()` no-ops on empty section / non-finite / negative / unchanged page, else `runtime.panel_pages.insert(section, page)`, emitting a **config-lane-only** mutation (never touches the document).
- Wiring in `✏️editor/🦀️.rs`: `app_commands!` macro entry L1202 (`"setPanelPage" as "set-panel-page" => set_panel_page::SetPanelPage`), parsed in `cad_command_from_action` L1230, listed in `CAD_RETAINED_CONFIG_TOOL_IDS`/`CAD_RETAINED_TOOL_IDS` (~L1378/1410), `ArtifactToolPublicationContract { tool_id: "setPanelPage", lanes: &[Config] }` (L1454), `ActionDefinition::bounded_catalog("setPanelPage", .., ActionKind::View).in_palette(false)` (L2427), `action_interactive_job("setPanelPage", InteractiveJobClassification::Migrated)` (L2500). Manifest entry (`🔣️.json`, 4 near-identical occurrences for the artifact's standard variants): `kind: "view"`, `iconId: "eye"`, `args: []`, `inPalette: false`, `semantics.effects.reads: ["config:{self}"]`, `policy.scopes: ["artifacts.read","shell.observe"]`, `approval: "never"`, `execution.class: "interactive"`, `interactiveJob: "migrated"`, `undo.kind: "none"`.
- **Tests:** `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` → `set_panel_page_advances_the_structure_section` asserts the continuation row dispatches `setPanelPage` and page 1 starts at `objects[CAD_SECTION_ROWS]`; its own comment: *"This is the navigation half of `paged_panel_section` the 2026-09-16 CAD end-to-end report recorded as missing"* — i.e. cad's cursor mechanism was purpose-built 2026-09-16 as a bespoke, single-app addition on top of the framework's stock (non-navigable) `paged_panel_section`. Also `✏️editor/🧪️tests/🔬️unit/🦀️.rs:1794-1808` (`set_panel_page_records_the_section_cursor_on_the_config_lane`) pins the config-lane-only law. TS: `📦️packages/🟦️typescript/📜️script.ts:97` (`RetainedAuditScript`) cross-checks `setPanelPage` is present in the generated command-id list and `CAD_RETAINED_TOOL_IDS` against `🗄️retained-jobs/🔣️.json` (`routeCount === 38`).

---

## 3. 🏗️fem — 2d and 3d

Panels: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection,📊️results}` — a **third** panel kind (`📊️results`, playback/results controls) exists beyond the ticket brief's named four.

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| 2d `🗿️artifact/🦀️.rs` | `fem2d.play.artifact` | artifact tree (outliner) | `PanelTreeBuilder` + **stock** `paged_panel_section`/`PanelRowBudget`/`panel_page_rows` (no app-local cursor); `section_quotas()`/`with_quota()` max-min-fair budget splitter | **inert** `panel_continuation_row` (SDK stock — no action, non-navigable) | none |
| 2d `🔍️inspection/🦀️.rs` | `fem2d.play.inspection` | inspection (form) | raw `ui::section`/`field`/`select`/`button`; app-local `paged_rows()` for nested load/term listings | `LIST_ROWS_MAX = 8` + `PanelRowBudget`; continuation is a plain `text_node("{section}.more","+{n}")` — **no action at all**, not even the SDK helper | none |
| 2d `📊️results/🦀️.rs` | `fem2d.play.results-panel` | parameters/results controls | `ui::column`/`ui::section`/`field` form; not a tree | n/a | n/a (`results_cache_key`/`ResultsCacheKey` exists but is for the *mesh preview*, unrelated to panel paging) |
| 3d `🗿️artifact/🦀️.rs` | `fem3d.play.artifact` | artifact tree (outliner) | same as 2d: stock `paged_panel_section` + `section_quotas()`/`with_quota()` | inert `panel_continuation_row` | none |
| 3d `🔍️inspection/🦀️.rs` | `fem3d.play.inspection` | inspection (form) | raw form controls; app-local `paged_rows()` | `LIST_ROWS_MAX = 8`; inert `text_node` continuation | none |
| 3d `📊️results/🦀️.rs` | `fem3d.play.results-panel` | parameters/results controls | form, not a tree | n/a | n/a |

**Nesting (artifact tree, identical shape 2d & 3d, `SECTIONS = 9`):** `nodes`, `elements`, `solids`(3d)/`regions`(2d), `supports`, `load-cases` (each case row **nests its loads as children**, `default_open(true)` on the case row — matches "load cases > loads" exactly), `combinations` (each combination row nests its terms — 3d keys terms by `case_id`, 2d by index `TREE_NAMESPACE.term.{combination.id}.{index}` since 2d's terms aren't case-keyed), `materials`, `sections`, `analysis` (unpaged, read-only). Row keys are the raw entity id (shared between tree pick and viewport pick). `section_quotas()` performs max-min-fair splitting of one shared page across all 9 sections — "small sections stay whole, only the wide ones truncate," growing a `ceiling` while total demand fits then distributing spare rows to sections furthest from their demand; `with_quota()` reserves `budget.remaining() - quota` for siblings before running a section (built on the framework's `PanelRowBudget::nested`).

**Row bindings/actions:** `entity_row()` → `Activate` = `interactionSelect` (domain `fem2d`/`fem3d`, per-kind granularity constants); **no row actions at all** — 3d's comment states explicitly: two actions/row across a full page "is exactly what exhausts the one-page `UiValue` argument arena and starves every panel rendered beside the tree," so focus/delete moved to the inspection panel's grouped `action_rows()` (one focus + one delete button per *selection*, not per row); `dimmed` flags dangling entities referencing missing entities; no `draggable`. `.interaction_domain(...).selected(marked_ids(...)).highlighted(marked_ids(...))`, `MARKED_IDS_LIMIT = UI_FIXED_LIST_ITEMS` caps a wide selection to its first page rather than refusing render. Inspection panel: per-entity field-row groups; load-case section nests a `paged_rows()` listing of `load_pick_row` buttons (`Activate` → `interactionSelect` at `load` granularity, capped `LIST_ROWS_MAX = 8`); combination section nests `term_row` (`Trigger::Change` → `patchCombination {term:<caseId>}`) + add/remove-term selects; every editable control binds `Trigger::Change` → one `patch*` command `{field, id}`; verbs (focus/delete) live in one `action_rows()` group, not per row.

**No setPanelPage anywhere in fem:** neither 2d nor 3d has a `📄️panel`/`set-panel-page` command dir (2d and 3d each list 29 command dirs — add-*, patch-*, set-result-*, canvas-pointer-*, gumball, etc. — none panel-page); whole-crate grep for `setPanelPage`/`set_panel_page`/`PanelPage`/`panel_pages` = zero hits in both artifacts. Since the "+N" row is already dispatch-less/inert today, **fem's panel trees have no existing navigation semantics to migrate** — a framework-owned virtualisation replacement is a strict win here, not a migration.

**Tests:** 3d's `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` is the most thorough paging test in the whole audited set: `section_quotas_are_max_min_fair`, `an_oversized_document_pages_with_continuation_rows_instead_of_faulting` (60 nodes → `placed+omitted==60`; 40-load wind case pages its own loads), `selected_and_hovered_ids_are_marked_from_the_interaction_snapshot` (caps at `MARKED_IDS_LIMIT`), `an_empty_document_renders_placeholders`, `rows_are_keyed_by_the_raw_entity_id_and_bound_to_the_fem3d_domain`, `load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms`, `the_app_declares_the_artifact_panel_under_its_body_key`. 2d has the mirrored (lighter) test shape at the same relative path, plus `📌️panels/🔍️inspection` and `📌️panels/📊️results` unit tests in both artifacts.

### `✏️s/🔨️modules/🏗️fem` (shared engine module) — no panel trees, engine-only

Confirmed: `find` for `*panel*`/`*editor*` under this tree returns nothing; no file matches `BODY_KEY|ui::tree|PanelTreeBuilder|panel_pages|setPanelPage`. Contents are purely numerical (`⚙️engine/{➗️formulation,🏗️model,🧊️3d,🧮️analyses,📏️elements2d,🕸️mesh,➕️algebra,🧱️elements3d,◻️2d,🔢️sparse}` + their `🧪️tests`) — the FEM solver library the two plugin crates consume. **Out of scope** for the panel-virtualisation replacement.

### `✏️s/🔨️modules/💭️mindmap` — placeholder, no code at all

The entire module is one file, `AGENTS.md` (a technology-bundle description: "A mindmap is a directed graph… A topic is a node… A relationship is an edge"). No `.rs`, no editor, no panels, no UI code whatsoever. **Out of scope** — nothing to audit.

---

## 4. 🧩️puzzle (2d / 3d / 5d)

Panel dirs: 2d and 3d each have 4 panel modules (`⚙️settings`, `🔍️inspection`, `🗿️artifact`, `🛍️catalogue`); **5d has only 3 — no `⚙️settings` dir at all**.

### 4a. 🧊️3d

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `"puzzle.3d.play.artifact"` (L29) | artifact/document outliner | `PanelTreeBuilder::new(ROOT)` (L343) | **Full cursor paging**: own `paged_section_from` (L292-326) wraps the SDK's page-0-only `paged_panel_section`; `SECTION_ROWS = UI_BUILT_CHILDREN_MAX - 1` (L33); `continuation_row_from` (L268-276) emits an **advancing** `setPanelPage{page, section}` Activate binding | `Puzzle3dArtifactTreeKey{fingerprint, label_set, pages}` (`editor/🦀️.rs` L3106-3130, digest over `panel_pages`) — cached in `app.artifact_tree_cache` (L3413-3424) |
| `🛍️catalogue/🦀️.rs` | `"puzzle.3d.play.kinds"` (L21) | placeable-kind catalogue (draggable) | `PanelTreeBuilder` (L152) | **Truncates but never advances** — a real bug: it reuses the artifact panel's `paged_section` which always starts at page 0 (`render()` at L145, called with **no** `pages` arg at `editor/🦀️.rs:8360`); the continuation row still dispatches `setPanelPage`, but the map is never read back, so clicking "+N" here does nothing | none |
| `🔍️inspection/🦀️.rs` | `"puzzle.3d.play.inspector"` (L24) | field inspector | raw `PanelTreeBuilder` + `tree_item_desc`/`tree_item_with_action`, one flat section | Only the multi-select **id list** is paged: `IDS_ROWS = 16` (L27), `ids_page`/`push_ids` (L76-100) reads `panel_pages[IDS_SECTION]` and emits an advancing row; entity fields themselves are never paged | none |
| `⚙️settings/🦀️.rs` | `"puzzle.3d.play.settings"` (L16) | settings | raw `ui::section` + 4 fixed steppers — **not a tree** | none | none |

**Nesting:** artifact panel has 4 top-level sections — `objects` (`default_open: true`), `references`, `target-volumes`, `attractions` (all `false`) (L344-347); each object row nests `vortices` under `try_children` (L176-181, `default_open(false)`), independently paged with the same `paged_section` primitive keyed `puzzle3d-play-document.object.{id}`.

**Row bindings/actions:** every row carries `Trigger::Activate → INTERACTION_SELECT_ACTION_ID` (`{domainId, merge:"replace", method:"pick", targets}`, L102-106); object/reference/target-volume rows additionally carry two `RowAction`s (`RowActionPlacement::Row`) for hide/lock → `setSelectionFlag{entity,ids,flag,value}` (L135-173); `.interaction_domain(PUZZLE3D_INTERACTION_DOMAIN)` on the whole tree (L348). No draggable rows in the outliner (draggability is catalogue-only, MIME `application/x-semio-catalogue-item`, gated on a resolvable `meshUrl`).

**`set-panel-page` command** (`🎮️commands/📄set-panel-page/🦀️.rs`, 15 lines): `ctx.scene.runtime.panel_pages.insert(section, page)` — **no explicit `ui_scope`** (relies on caller's default dirty scope). Registered: enum variant `SetPanelPage = "setPanelPage"` (`editor/🦀️.rs` L2618), `TOOL_JOB_IDS` entry (L2688), dispatch `"setPanelPage" => set_panel_page::set_panel_page(ctx, args)` (L3744), manifest `.view_action("setPanelPage", …)` (L8575, `ActionKind::View`), `ArtifactToolPublicationContract{tool_id:"setPanelPage", lanes:&[WindowConfig]}` (L7290), `.action_interactive_job("setPanelPage", Migrated)` (L8735) — **BUT this action is absent from the plugin's own top-level manifest `🔣️.json`** (0 hits, confirmed independently — see §0 census), i.e. the dispatch arm exists but the action is not declared in the catalogue a real client would read to know it's callable. **`panel_pages` storage:** field lives in **`Puzzle3dWindowConfig`** (`🪟️window/🦀️.rs` L16-41, `#[derive(dsl::DslArtifact)]`, DSL id `s.puzzle.puzzle3d.windowconfig`) — a **persisted per-window struct**, copied into `Puzzle3dRuntime` (`🎚️config/🦀️.rs` L224,261) which panels read as `envelope.runtime.panel_pages`.

**Tests:** `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — `the_outliner_memo_serves_one_key_and_rebuilds_on_a_fixture_or_label_switch` (L42), `the_outliner_pages_a_document_scale_fixture_without_exceeding_the_fixed_page` (L115), `a_document_that_fits_the_page_keeps_every_nested_vortex_row` (L148), `pressing_the_outliner_continuation_reveals_the_next_page` (L343), `nakagin_artifact_panel_fits_reconcile_node_envelope` (L382, exercises `PANEL_RECONCILE_NODE_BUDGET`); `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` — `inspection_ids_page_is_bounded_and_the_continuation_advances` (L159); `📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` — `the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page` (L33, only asserts a continuation row exists, **not** page advancement — consistent with the dead-cursor bug above); `🪟️window/🧪️tests/🔬️unit/🦀️.rs:175-197` round-trips `panel_pages` through the persisted window pack.

### 4b. ◻️2d

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `PUZZLE2D_PLAY_BODY_LAYERS` = `"puzzle2d.play.layers"` | node/edge outliner | `PanelTreeBuilder` + `.section_or_placeholder` (L142-146) | Own `paged_section` (L103-132), `SECTION_ROWS=16` (L27), reads `envelope.runtime.panel_pages` directly (L138); advancing `continuation_row` (L92-100) that **wraps to page 0** after the last page (L124-126) rather than dead-ending | none — rebuilt every render, no cache struct in 2d's editor.rs |
| `🛍️catalogue/🦀️.rs` | `PUZZLE2D_PLAY_BODY_CATALOGUE` = `"puzzle2d.play.catalogue"` | kind catalogue (draggable) | `PanelTreeBuilder` (L100), **no `.interaction_domain()`** (unlike 3d's) | Reuses the artifact panel's cursor-aware `paged_section` — 3 sections, each independently paged and correctly reading `pages` — **this one does advance correctly** | none |
| `🔍️inspection/🦀️.rs` | `PUZZLE2D_PLAY_BODY_PROPERTIES` | field inspector | raw `PanelTreeBuilder`, one flat section | Selected-id list bounded `IDS_ROWS = 8` (L21); past that it prints a **static** `read_only` "+{n}" row (L87-89) — **no action at all**, not even the SDK's inert helper | none |
| `⚙️settings/🦀️.rs` | `PUZZLE2D_PLAY_BODY_SETTINGS` | settings | raw `ui::section` + fixed steppers — not a tree | none | none |

**Nesting:** artifact panel = 2 flat sections, `nodes` (`default_open: true`) / `edges` (`false`) — no nested children (2d has no vortex-style sub-entity). Catalogue = 3 flat sections (`nodes`/`handles`/`edges`).

**Row bindings:** artifact rows carry only `Activate → INTERACTION_SELECT_ACTION_ID` — **no RowAction hide/lock** on outliner rows (unlike 3d); inspection panel instead carries two `flag_row` toggle rows as ordinary tree rows. Catalogue's `"nodes"` rows are draggable via `tree_item_with_action_draggable`; handle/edge rows are plain.

**`set-panel-page` command** (26 lines) differs from 3d by **explicitly setting the dirty scope**: empty section → `UiDirtyScope::None` early-out (L11-14); else inserts and sets `UiDirtyScope::Partial{panel_bodies:[layers, catalogue, properties], …}` (L16-24) — 2d hand-dirties all three tab bodies on every page change, where 3d relies on ambient default dirtying. `ArtifactToolPublicationContract{tool_id:"setPanelPage", lanes:&[WindowTransient]}` (L1373 — **WindowTransient, not WindowConfig** as in 3d). **`panel_pages` storage:** `Puzzle2dWindowTransient` (`🪟️window/🦀️.rs` L170), explicitly retired via `store::artifact_retire_struct!(…, panel_pages)` (L218) — **ephemeral scratch, not persisted config** (architecturally different from 3d's persisted placement, despite sharing the same action id).

**Tests: none reference paging at all** — grepped every `🧪️tests` dir plus the 2900+-line top-level `✏️editor/🧪️tests/🔬️unit/🦀️.rs` for `setPanelPage|panel_pages|SECTION_ROWS|paged_section|continuation` → zero hits. 2d's artifact-panel unit test is 23 lines, tests labels only. **The 2d catalogue panel has no test file at all** — a real coverage gap versus 3d.

**Schema-generation gap (2d only, worth noting for the replacement plan):** 2d's `🪟️window/🧬️schema/🟦️.ts:16` correctly emits `panelPages: Record<string, number>`, but the analogous 3d schema file (`🧊️3d/…/🪟️window/🧬️schema/🟦️.ts`) defines `Puzzle3dWindowConfig` up to `camera` only — **`panel_pages` is completely absent from 3d's generated TS/JSON/GraphQL schema**, even though the Rust struct declares it as a real persisted field. Checked all three sibling schema files (`🔣️.json`, `🔗️.graphql`, `🟦️.ts`) for 3d — none mention it.

### 4c. 🖐️5d — no paging anywhere

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `"puzzle.5d.play.artifact"` | parts/fasteners outliner | `PanelTreeBuilder` + `.section_or_placeholder` | **none** — plain unbounded `for` loops over `parts`/`grips`/`fasteners`, no cap, no continuation row | none |
| `🛍️catalogue/🦀️.rs` | `"puzzle.5d.play.kinds"` | kind catalogue (draggable parts) | `PanelTreeBuilder` | **none** — `.enumerate()` over the full `entries` slice | none |
| `🔍️inspection/🦀️.rs` | `"puzzle.5d.play.inspector"` | inspector (always doc summary) | `PanelTreeBuilder` | n/a | none |
| ⚙️settings | **does not exist** | — | — | — | — |

**Nesting:** artifact panel nests `grips` under each `part` via `try_children`, same shape as 3d's objects→vortices, just unbounded. **Row bindings:** same `Activate → interactionSelect` shape as 2d/3d; **no RowAction anywhere** — the inspection panel's own doc comment states selection was never wired into `ArtifactApp::render` after the FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM ticket, so the inspector is permanently the 4-row document summary (a known, already-flagged framework gap, not introduced by paging). `interaction_domain` is set on the artifact panel but **not** on the catalogue panel. **No `set-panel-page` command dir; zero `panel_pages`/`setPanelPage` hits anywhere in 5d** — the feature was simply never built for this artifact. Tests are 3 single-assertion smoke tests, none touching paging (there is none).

### Puzzle's framework-primitive gap (important for the replacement design)

The cursor semantics (reading `pages: &BTreeMap<String,u32>`, computing an offset, emitting an *advancing* `setPanelPage` row) are **not part of the shared SDK** — the framework's `paged_panel_section` always starts at index 0. Each of puzzle-2d and puzzle-3d re-implements its own `paged_section`/`paged_section_from` on top of the SDK's page-0-only primitive, and cad does the same independently (`paged_section_from`, §2). This triple reimplementation, plus 3d's catalogue panel silently never advancing past page 1 despite emitting a `setPanelPage`-carrying row, is exactly the kind of duplicated/broken app-local logic the design doc's `TreeWindows`/`tree_window_section` replacement is meant to delete outright (see Cross-app patterns, below).

---

## 5. 🔋️energy (`🔋️model` artifact)

Panels: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection}` — only 2 panel kinds, no catalogue.

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `"energy.model.artifact"` (L35) | artifact tree | `PanelTreeBuilder::new(TREE_NAMESPACE)` (L564) | **Stock** `paged_panel_section` throughout (imported L30), inert `panel_continuation_row` — non-navigable | none |
| `🔍️inspection/🦀️.rs` | `"energy.model.inspection"` (L40) | inspection (form) | raw form controls | app-local `LIST_ROWS_MAX = 8` (L46) + `.take(LIST_ROWS_MAX)`, closed with a **static** `read_only_row("…more", "More layers/n")` (L423-432) — no action at all | none |

**Nesting — the deepest 3-level hierarchy found in this whole audit:** 9 sections (`site`, `zones`, `shading`, `materials`, `glazing-materials`, `gas-materials`, `constructions`, `loads`, thermostats/ideal-loads, `schedules`; `default_open: true` only for `site`/`zones`, L565-587). `zones_section` (L240-262) nests **both** `spaces` (flat) **and** `surfaces > windows` (a second nesting level) under each zone row via `try_children`, `default_open: true` on the zone row (L258). Notably, `surfaces_with_windows` is **hand-paged instead of `paged_panel_section`** (L269+) because that helper "reserves exactly ONE row per remaining sibling surface and hands the rest to the current one" — under a tight page this starved the first surface's own windows into an unreachable `…windows.more` marker (a real UX bug found by a browser probe, documented in the code comment at L270-274) — i.e. even the *stock* SDK primitive has a known budget-starvation bug when nesting is 3 levels deep, independent of the navigability question.

**Row bindings/actions:** `entity_row`/`entity_row_marked` (L161-210) bind `Activate → pick_action(granularity, id)` (one argument map per row, no row actions — comment at L160: "It authors ONE argument map (its pick) and no row actions"); selection/hover state is encoded as a **label prefix** (`●`/`○`, `MarkedAs::prefix()`, L165-186) because `PanelTreeBuilder::selected()`/`.highlighted()` "do not paint in this SDK wave" per the code's own doc comment — a distinct workaround from every other plugin in this audit, worth flagging since a framework-owned virtualisation replacement should settle this painting gap too. Site is a read-only row whose `Activate` clears selection (`clear_selection_action`, L228). Schedule rows are read-only (`read_row`, no action, L214-216). `.interaction_domain(ENERGY_MODEL_INTERACTION_DOMAIN)` set once (L588).

**Tests:** `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` has extensive paging tests mirroring fem-3d's shape: `section_quotas_are_max_min_fair_and_never_exceed_the_page`, `an_oversized_document_pages_with_continuation_rows`, `a_wide_selection_marks_only_its_first_page`, `section_demands_count_every_interactive_row_including_the_nested_ones`, `a_marked_surface_keeps_its_windows_under_a_tight_page`, `a_marked_surface_keeps_its_row_when_the_page_cannot_hold_every_wall` (regression tests for the starvation bug above), `windows_nest_under_their_surface_and_surfaces_under_their_zone`, `every_entity_row_picks_into_the_energy_model_domain`. `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` covers form fields per entity kind, none pin the `LIST_ROWS_MAX` truncation's absence of an action.

**No `setPanelPage`/`set-panel-page` command dir; zero hits repo-wide** for energy. No memoization of the rendered tree.

---

## 6. 🌀️procedural — generation2d

Panels: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue}`.

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `GENERATION2D_PLAY_BODY_ARTIFACT` = `"generation2d.play.artifact"` | artifact tree (flow-widget outline) | `PanelTreeBuilder::new("procedural2d-play-document")` + `.section_or_placeholder` | **none** — one flat, unbounded `tree_item` list over `document.host_snapshot.widgets` | none |
| `🔍️inspection/🦀️.rs` | `GENERATION2D_PLAY_BODY_INSPECTION` | inspection | `PanelTreeBuilder`, 3 fixed read-only rows | n/a (fixed-size) | none |
| `🛍️catalogue/🦀️.rs` | `GENERATION2D_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder`, 4 sections | none (small/fixed, 2-5 items/section) | none |

**Nesting:** artifact tree is a single flat section `procedural2d-play-document.widgets` (`default_open: true`), no nesting — item ids are the **raw widget id with no namespace prefix**, deliberately, so they equal the `"graph"` interaction domain's target ids one-for-one for post-render presence stamping (code comment). Catalogue has 4 flat sections: `sources`, `components`, `sinks` (all `default_open: true`), `modes` (`false`) — each a handful of fixed `tree_item_with_action` rows dispatching `addWidget{kind[,neuronKind]}` or `setShowMode{value}`.

**Row bindings:** artifact rows carry **no per-item action at all** — "Clicks/selection are the framework's now" via `.interaction_domain("graph")` (code comment: `_config` param is even unused, "kept for call-site symmetry with inspection"). Inspection has **no `InteractionView`** threaded into `render` at all — a documented, already-flagged framework gap ("the selected-widget-details view degrades to its 'no selection' default until a future wave threads interaction into render"), not something this audit introduces. No row actions, no draggable, no `interaction_domain` on the catalogue.

**Tests:** `📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue}/🧪️tests/🔬️unit/🦀️.rs` exist; none reference paging (there is none to pin).

**No setPanelPage anywhere** — zero hits in generation2d or the plugin's manifest (`✏️s/🔌️plugins/🌀️procedural/🔣️.json`).

---

## 7. 🌀️procedural — generation3d

Panels: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue}` + a top-level test dir `✏️editor/🧪️tests/🔬️mode-panels/` (both `🦀️.rs` and `🟦️.ts`).

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `GENERATION_3D_PLAY_BODY_ARTIFACT` = `"procedural.play.artifact"` | artifact tree (flow graph outline) | delegates to `flow::windows::flow::graph_outline` (shared with the Flow window's own outline, not a local `PanelTreeBuilder` call) | not evaluated by this audit (delegated to a shared flow-module helper outside `📌️panels`) | none |
| `🔍️inspection/🦀️.rs` | `GENERATION_3D_PLAY_BODY_INSPECTION` | inspection | raw `PanelTreeBuilder`, per-widget-kind field rows | n/a (one selection, ≤6 fixed fields) | none |
| `🛍️catalogue/🦀️.rs` | `GENERATION_3D_PLAY_BODY_CATALOGUE` = `"procedural.play.catalogue"` | catalogue (flow operator palette) | `PanelTreeBuilder` + `tree_group` per catalogue section + **stock** `paged_panel_section`/`PanelRowBudget`/`panel_page_rows` per group | **The richest paging shape in the whole audit** — see below | none (recomputed every render; a companion `catalogue_page()` fn does a second, UI-free walk purely to report `{shown, omitted}` for the law-pinning test) |

**Generation3d's catalogue — genuinely different escape valve, worth its own note:** the file's own header comment explains the previous flat build "refused at the 33rd" catalogue entry with hundreds of real operators installed (ticket `26/09/09/PROCEDURAL-3D-END-TO-END §3.1`). The fix groups operators via `tree_group(group_id, title, index==0, items)` (one group per `CatalogueSection`, only the first `default_open: true`), pages **each group** independently at `CATALOGUE_GROUP_ROWS = 31` rows (`UI_BUILT_CHILDREN_MAX - 1`) under a **shared** `PanelRowBudget::new(panel_page_rows())`, and — critically — when the whole page still can't fit every group, appends **one panel-total** inert `panel_continuation_row("procedural-play-catalogue.widgets", omitted)` naming the entire remainder (`shown + omitted == roster`, asserted by test). **The unbounded browse surface is NOT more paging** — it's a separate mechanism entirely: the full operator roster is published on a reserved retained surface (`framework.section.catalogue`) that a **canvas spotlight** UI reads independently, asserted by `the_spotlight_catalogue_offers_every_operator_the_panel_omits`. This is the one plugin in the audit that solves "more than a page exists" by punting to an entirely different, non-tree UI surface rather than by trying to make the panel itself navigable — a third distinct pattern beyond "navigable cursor" (cad/puzzle) and "static inert +N" (fem/energy).

**Row bindings:** catalogue rows dispatch `addWidget{kind}` (`neuron|<neuronKind>` composite kind for neuron operators) — `Activate` only, no row actions, no draggable, no `interaction_domain` on the catalogue tree. Inspection panel's one editable control (an `InputSlider`'s numeric value) is authored as **a child of a tree row**, not a sibling `field` — the code comment explains this was a real, ticket-tracked bug (`26/09/09/PROCEDURAL-3D-END-TO-END`, `react-gap-probe.mjs` step `inspection-edit`): the React host's `collectTreeItems`/`collectTreeItemControls` split means a non-`treeItem` child of a section (rather than of a row) is silently dropped from the DOM — relevant precedent for how carefully any new lazy/windowed contract will need to preserve control-vs-item child placement.

**Tests:** `📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` — `the_catalogue_panel_publishes_its_omitted_count` (asserts exactly one total continuation row and `shown+omitted==roster`, reading the **rendered** JSON body rather than a re-derivation) and a spotlight-roster-parity test. `✏️editor/🧪️tests/🔬️mode-panels/{🦀️.rs,🟦️.ts}` — `every_panel_publishes_its_body_in_generate_mode_as_well_as_edit`, `a_panel_publishes_the_same_body_in_every_mode` — these pin **mode-parity of panel bodies**, not paging; the `.ts` file has zero `setPanelPage`/`panelPage` references (checked directly).

**No `setPanelPage`/`set-panel-page` command anywhere in generation3d** — its "+N" is exactly as inert as fem's/energy's, just materially harder-won (grouped + budget-shared + a dedicated overflow surface) rather than a simple stock `paged_panel_section` call.

---

## 8. 🏭️process — process3d

Panels: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue,🛠️workshop}` — a 4th panel kind (`🛠️workshop`, machine configurator) beyond the ticket brief's named four. A 5th body (`workpiece::PROCESS_3D_PLAY_BODY_MAIN`) is dispatched from the same `render()` match (`✏️editor/🦀️.rs:1343`) but lives **outside** `📌️panels` (the 3D canvas/world body, not a tree) — noted per the brief's ask to flag non-📌️panels trees, though in this case the excluded body isn't a tree at all.

| panel path | BODY_KEY | kind | tree-build mechanism | paging shape | memoization key |
|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | `PROCESS_3D_PLAY_BODY_ARTIFACT` = `"process.play.artifact"` | artifact/document tree (stock + step timeline) | `PanelTreeBuilder::new("process3d-play-document")` | **none** — unbounded `for` loop over `snapshot.step_payloads`, no cap | none |
| `🔍️inspection/🦀️.rs` | `PROCESS_3D_PLAY_BODY_INSPECTION` | inspection (form) | `PanelTreeBuilder`, several fixed sections keyed by selection kind (stock/step/machine) | none (single-selection forms) | none |
| `🛍️catalogue/🦀️.rs` | `PROCESS_3D_PLAY_BODY_CATALOGUE` = `"process.play.catalogue"` | capability catalogue, grouped by source machine-catalog | `PanelTreeBuilder`, one section per catalog (uncataloged/generic machines first, open by default) | **none** — unbounded per-machine capability rows | none |
| `🛠️workshop/🦀️.rs` | `PROCESS_3D_PLAY_BODY_WORKSHOP` = `"process.play.workshop"` | installed-machine configurator | `PanelTreeBuilder`, `machines` section + one section per catalog with something left to install | **none** — unbounded | none |

**Nesting:** artifact panel = 2 flat sections, `stock` (1 row) and `steps` (`default_open: true` both, L82-84) — steps are a flat, ordered list (no nesting); the step index `>= cursor` (unresolved steps) gets a `"pending"` description rather than a different tree structure. Catalogue/workshop group by machine-catalog id (`Vec<(UiText, Vec<&WorkshopMachine>)>`), one section per catalog, flat capability/machine rows inside — no deeper nesting than section→row anywhere in process3d.

**Row bindings/actions — the clearest `RowAction` example in the whole audit:** each step row in the artifact panel carries **two** `RowAction`s: `RowActionPlacement::Row` (eye/eye-off icon, toggles `setStepEnabled{enabled,id}`) and `RowActionPlacement::Menu` (trash icon, `removeStep{id}`) — demonstrating both row-action placement kinds side by side (`📌️panels/🗿️artifact/🦀️.rs:37-64`). Workshop's installed-machine rows carry one `RowActionPlacement::Menu` action (`removeWorkshopMachine{id}`). All rows also carry the generic `.interaction_domain(PROCESS3D_INTERACTION_DOMAIN)` click-to-select (artifact panel L84, workshop panel on its machines section only — catalog-install sections are explicitly left unbound since "their items are install actions, not domain targets"). Catalogue capability rows dispatch `Activate → addStep{capabilityId,machineId}` when valid, or render as a disabled `tree_item_desc` with a validation-failure reason when the current stock doesn't satisfy the capability (dimensional/geometric validation via `stock_validation_context`/`validate_capability`) — a distinct "disabled-with-reason" row pattern not seen elsewhere in this audit.

**Tests:** `📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue,🛠️workshop}/🧪️tests/🔬️unit/🦀️.rs` all exist; none reference paging. (The git-status-visible ticket `📓️fix-2026-09-16-process3d-catalogue-dedupe.md` in the same repo concerns catalogue **representation deduplication**, not paging — unrelated to this audit's scope but worth knowing the catalogue file was touched today for a different reason.)

**No `setPanelPage`/`set-panel-page` command anywhere** — command dirs present: `⏱️cursor`, `☀️sun`, `🌍️world`, `🎛️engagement`, `🎥️camera`, `📤️media`, `🔎️inspector`, `🗿️artifact`, `🛠️workshop`, `🧩️contribution`, `🧰️utility`, `🪜️step`, `🪵️stock` — none panel-page related. Zero repo-wide `panel_pages`/`setPanelPage` hits under process3d.

---

## 9-12. 🎥️shooting, 🖍️draw, 🖨️raster, 🌍️gis — no paging in any of the four

**Headline: none of these four plugins implement any paging at all.** No `paged_panel_section`, `panel_continuation_row`, `PanelRowBudget`, `panel_page_rows`, `SECTION_ROWS`/`IDS_ROWS`/`LIST_ROWS_MAX`, `.take(N)` truncation, `RowAction` usage, `setPanelPage`/`panel_pages`/`PanelPage` (Rust or TS), or hand-rolled "+N"/`.more` rows exist anywhere in these four editor crates. Every panel renders its full, unbounded item list every call via plain `PanelTreeBuilder` (or, once, a bare text node). This is greenfield territory for virtualisation, not a migration.

### 9. 🎥️shooting

Panels: `…/🎥️shooting/…/✏️editor/📌️panels/{🔍️inspection,🗿️artifact,🛍️catalogue}`.

| panel path | BODY_KEY | kind | tree-build mechanism | paging | memo |
|---|---|---|---|---|---|
| `🔍️inspection/🦀️.rs` | `SHOOTING_PLAY_BODY_INSPECTION` | inspection | raw `ui::` field/column/section builders (not `PanelTreeBuilder`) | none | none |
| `🗿️artifact/🦀️.rs` | `SHOOTING_PLAY_BODY_ARTIFACT` | artifact tree | `PanelTreeBuilder`, 2 flat sections (`shots`, `assets`) | none | none |
| `🛍️catalogue/🦀️.rs` | `SHOOTING_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder`, 2 flat sections | none | none |

Shot rows → `setShotSelection{shotIds}`; asset rows → a **manually built** `interactionSelect` dispatch (`domainId:"assets"`) rather than `.interaction_domain(...)`, deliberately, since shot+asset rows share one tree under namespaced ids. Catalogue rows → `addShot`/`addAsset`. Inspection editable fields → `patchShots` on `Trigger::Change`. No draggable, no row actions, no `interaction_domain` call anywhere. No `📄set-panel-page` dir (commands: `☀️scene`, `🎥️camera`, `📄️document`, `📦️asset`, `📷️shot`, `🖨️export`, `🗂️selection`, `🗣️locale`, `🧭️gumball`). No TS refs.

### 10. 🖍️draw

Panels: `…/🖍️draw/…/✏️editor/📌️panels/{🔍️properties,🗂️layers,🛍️catalogue}`.

| panel path | BODY_KEY | kind | tree-build mechanism | paging | memo |
|---|---|---|---|---|---|
| `🔍️properties/🦀️.rs` | `DRAWING_PLAY_BODY_PROPERTIES` | inspection | **no tree** — a single `built_text_node` summary | n/a | none |
| `🗂️layers/🦀️.rs` | `DRAWING_PLAY_BODY_LAYERS` | artifact tree | `PanelTreeBuilder` | none | none |
| `🛍️catalogue/🦀️.rs` | `DRAWING_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder` | none | none |

**Deepest recursive tree in the shooting/draw/raster/gis group:** the layers panel is one flat section holding 5 fixed "add" rows plus the full, unbounded `document.layers` list, where `DrawingLayerNode::Group` recurses arbitrarily via `try_children` and `DrawingLayerNode::Boolean` nests child rows referencing other layers by id — real unbounded-depth nesting, `default_open: true` only for `Group` nodes. Whole tree is `.interaction_domain(DRAWING_INTERACTION_DOMAIN)` bound (code comment explicitly forbids adding a manual per-row action); every layer row is `draggable(true)` (drag_data `application/x-semio-drawing-layer-id`) and `dimmed(!visible)`; boolean-child rows are `draggable(false)`. Catalogue mixes layer-kind rows (draggable, mime `application/x-semio-drawing-layer-kind`) and boolean-operation rows (`combineBoolean{ids:[],operation}`, not draggable) in one flat list. No row actions anywhere. **No dedicated test subdir under any of the 3 panel modules** — panel behavior is exercised only from the editor-crate-level suite instead (a real coverage-location gap vs. other plugins' co-located panel tests). No `📄set-panel-page` dir, no TS refs. (A stray dead pre-migration file exists at the old `✏️editor/🪆️1-any/` path — unrelated to paging, flagged only so it isn't mistaken for the live tree when planning edits.)

### 11. 🖨️raster

Panels: `…/🖨️raster/…/✏️editor/📌️panels/{🎭️masks,🔍️inspection,🗿️artifact,🛍️catalogue}` — a 4th panel kind (masks) beyond the brief's named four.

| panel path | BODY_KEY | kind | tree-build mechanism | paging | memo |
|---|---|---|---|---|---|
| `🎭️masks/🦀️.rs` | `RASTER_PLAY_BODY_MASKS` | derived "masked layers" list | `PanelTreeBuilder::section_or_placeholder`, flattens the layer tree | none | none |
| `🔍️inspection/🦀️.rs` | `RASTER_PLAY_BODY_PROPERTIES` | inspection | `PanelTreeBuilder`, 2 description rows | none | none |
| `🗿️artifact/🦀️.rs` | `RASTER_PLAY_BODY_LAYERS` | artifact tree | `PanelTreeBuilder`, 1 flat section + recursive `Group` nesting | none | none |
| `🛍️catalogue/🦀️.rs` | `RASTER_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder`, 1 flat section, 3 rows | none | none |

Layer tree is `.interaction_domain("layers")` bound, same recursive-`Group` shape as draw's layers panel. Masks panel **deliberately does not bind a domain** — a code comment explains mask row ids live in a different namespace than layer row ids, so binding both to one domain would collide ("dropped rather than shown stale"). No row actions, no `📄set-panel-page` dir, **no dedicated test subdir under any of raster's 4 panel modules** either (same gap pattern as draw — panel behavior tested only editor-crate-wide). No TS refs.

### 12. 🌍️gis (gismap 2D)

Panels: `…/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/📌️panels/{🔍️inspection,🗿️artifact,🛍️catalogue}`.

| panel path | BODY_KEY | kind | tree-build mechanism | paging | memo |
|---|---|---|---|---|---|
| `🔍️inspection/🦀️.rs` | `GIS2D_PLAY_BODY_INSPECTION` | inspection | `PanelTreeBuilder`, base + at most one conditional detail section | none | none |
| `🗿️artifact/🦀️.rs` | `GIS2D_PLAY_BODY_ARTIFACT` | artifact tree | `PanelTreeBuilder`, 1 flat section, 11 fixed rows | none | none |
| `🛍️catalogue/🦀️.rs` | `GIS2D_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder`, same 11-row roster, no `.interaction_domain()` call (unlike the artifact tree) | none | none |

Both artifact and catalogue trees are backed by the **same fixed, hard-coded roster of 11 layer identifiers** (`GIS_MAP_LAYER_IDS`) — individual positions/routes/regions are never enumerated into the tree at all, so there is structurally nothing to page here today (a real content gap, not a paging gap, if per-feature rows are ever wanted). Artifact tree is `.interaction_domain("features")` bound; catalogue rows dispatch `toggleLayerVisibility{layerId}` instead. No row actions, no draggable. No `📄set-panel-page` dir, no TS refs. (Sibling 3D `🏔️gisterrain` artifact under the same plugin was out of the requested scope and not inventoried.)

---

# Cross-app patterns

Across all 17 audited panel-tree surfaces (puzzle ×3 artifacts ×~3-4 panels, cad, fem ×2, energy, generation2d/3d, process3d, block ×3, shooting, draw, raster, gis) plus the two out-of-scope modules (fem engine, mindmap — both confirmed to build no UI at all), the paging landscape resolves into exactly **five** distinct shapes:

1. **Navigable app-owned cursor** (cad; puzzle 2d/3d) — an app-local `paged_section`/`paged_section_from` wraps the SDK's page-0-only `paged_panel_section`, persists a page index per section id in a `BTreeMap<String,u32>` (`panel_pages`), and the "+N" row carries a real `Activate → setPanelPage{section,page}` binding that advances it. Three independent, subtly incompatible reimplementations exist (cad's config-lane-persisted cursor with `page: f64`; puzzle-3d's window-config-persisted cursor with a tree-memo cache key; puzzle-2d's window-**transient** cursor with hand-dirtied multi-body refresh) — plus one confirmed dead spot (puzzle-3d's catalogue panel emits a `setPanelPage` row that is never read back, so it silently caps at page 1 forever) and one manifest-registration asymmetry (puzzle's `handle_action` arm exists but the action isn't declared in puzzle's own plugin manifest, unlike cad's).
2. **Stock, non-navigable `paged_panel_section`** (fem 2d/3d, energy, generation3d's catalogue) — the framework's SDK-level truncation runs, but `panel_continuation_row`'s "+N" carries no action at all; clicking it does nothing today. Generation3d additionally layers a **separate overflow escape valve** on top: a fully-grouped (`tree_group`) page plus a publish-the-whole-roster-to-a-different-surface pattern (`framework.section.catalogue`, browsed by a canvas spotlight) for when even the paged page can't summarize the truncation usefully.
3. **App-local flat truncation with a dead-static "+N"** (fem 2d/3d inspection's `paged_rows()`/`LIST_ROWS_MAX`; energy inspection's `.take(LIST_ROWS_MAX)`; puzzle-2d inspection's `IDS_ROWS` fallback) — a plain `.take(N)` or hand-rolled loop closed by a **read-only** text/tree row stating the omitted count, not even using the SDK's `panel_continuation_row` helper.
4. **No paging, unbounded render** (block ×3 artifacts, shooting, draw, raster, gis, generation2d, process3d, and puzzle-5d) — the majority of surfaces in this audit. Trees range from genuinely flat-and-small (gis's 11 fixed rows, shooting's fixed sections) to structurally unbounded-and-real (process3d's step timeline, draw/raster's recursively-nestable layer trees, puzzle-5d's parts/grips) — these are the surfaces where a document could realistically grow past a viewport today with zero mitigation.
5. **No tree at all** (drawn/raster's `🔍️properties`/`shooting`'s inspection use raw form builders instead of `PanelTreeBuilder`; fem's `📊️results` panels are playback-control forms) — out of scope for tree virtualisation by construction, since there's no tree to window.

**Universal facts that hold across every single plugin audited:**
- **No plugin memoizes its rendered tree** except puzzle-3d's `artifact_tree_cached_from`/`Puzzle3dArtifactTreeKey` (a fingerprint+label-set+pages digest) — every other `render()` rebuilds unconditionally from the live document/config/interaction snapshot on every call, panel-tree or not.
- **`RowAction` (hide/lock/delete-style secondary row controls) is rare and where present is inconsistent**: cad (reference rows: hide/show + lock/unlock), process3d (step rows: enable-toggle as `Row` placement + remove as `Menu` placement — the cleanest example of both placement kinds together), workshop (remove as `Menu`). fem 2d/3d, energy, and puzzle explicitly *removed* per-row actions on purpose to conserve the shared `UiValue` argument arena (documented in-code in three separate places with near-identical reasoning), moving verbs to one grouped `action_rows()` per *selection* instead of per row. Every other plugin (block, shooting, draw, raster, gis, generation2d/3d) never had row actions to begin with.
- **`.interaction_domain(...)` + generic `interactionSelect` on click is the dominant selection idiom** (cad, fem, energy, block, process3d, draw, raster, gis, puzzle, generation2d/3d), replacing an older per-row `Activate` action pattern per the repeated `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM` ticket reference. A few surfaces (shooting, gis-catalogue, puzzle-3d/5d-catalogue) deliberately opt out because rows share one tree across two id-namespaces (shooting) or the catalogue's rows are actions (add-to-document) rather than selectable targets.
- **Nesting depth tops out at 3 levels** (energy: zone > {spaces, surfaces > windows}) and is otherwise 2 levels (object/case/group > child rows) or flat. No plugin nests deeper than that, and only draw/raster's layer trees are recursively self-similar (arbitrary depth via `Group`/`Boolean` nodes) rather than a fixed schema depth.
- **`setPanelPage`/`panel_pages` exist in exactly 3 of the 17 surfaces** (cad, puzzle-2d, puzzle-3d) and nowhere else — fem, energy, generation2d/3d, process3d, block, shooting, draw, raster, and gis have **zero** existing navigation semantics to preserve. For 14 of the 17 audited surfaces, a framework-owned virtualisation replacement is a strict, migration-free win; for the 3 with real cursors, it must additionally retire 3 different persistence models (cad's `CadConfig.panel_pages_json`, puzzle-3d's persisted `WindowConfig.panel_pages`, puzzle-2d's ephemeral `WindowTransient.panel_pages`) plus the puzzle-3d tree-memo cache and its manifest/dispatch/TS-schema wiring.

**Minimal SDK surface every app funnels through today (all under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️PanelKit`/`🔖️PanelPaging`), i.e. exactly what a framework-owned replacement must supersede:**
- `PanelTreeBuilder::{new, section, section_or_placeholder, selected, highlighted, interaction_domain, drop_action, build}` — kept as-is by the design doc (§3.3 of `📓️design-virtualised-tree.md`), gains `.window_section(...)`.
- `PanelRowBudget::{new, spend, remaining, nested}` — the row-credit ledger every hand-rolled app-local pager (cad's `paged_section_from`, puzzle-2d/3d's `paged_section`, fem/energy/generation3d's `section_quotas`/`with_quota`) is independently built on top of; the design doc's `TreeWindows`/`tree_window_section`/`tree_window_item` are meant to be the *single* replacement for all of these call sites.
- `paged_panel_section` / `panel_continuation_row` / `panel_page_rows` — the three functions the design doc's §3.3 explicitly says to delete, along with every app-local `SECTION_ROWS`/`IDS_ROWS`/`LIST_ROWS_MAX` constant and every `setPanelPage` action + `panel_pages` state field found in this audit (cad's `🎮️commands/📄️panel`, puzzle-2d/3d's `🎮️commands/📄set-panel-page`, and their three manifest/schema/TS entries).
- The one framework-level TS touchpoint, `WINDOW_CONFIG_RAIL_ACTION_IDS` in `ShellHelpers.tsx:4654`, which lists `"setPanelPage"` purely to force a rail refresh after a window-config-only mutation — deletable alongside the action per the design doc's §7 note ("`setPanelPage` sits in `WINDOW_CONFIG_RAIL_ACTION_IDS`… delete with the action").

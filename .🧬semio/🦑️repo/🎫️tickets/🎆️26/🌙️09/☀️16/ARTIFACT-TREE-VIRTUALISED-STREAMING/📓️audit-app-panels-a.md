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

*(Sections 4-7 — 🔋️energy, 🌀️procedural generation2d/generation3d, 🏭️process process3d, and 🎥️shooting/🖍️draw/🖨️raster/🌍️gis — pending; appended below as the remaining audit agents complete, followed by Cross-app patterns.)*

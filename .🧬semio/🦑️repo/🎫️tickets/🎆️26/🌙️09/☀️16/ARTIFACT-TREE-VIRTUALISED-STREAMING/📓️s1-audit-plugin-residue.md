# 📓️ S1 Audit — plugin panel-tree windowing residue (2026-09-17)

READ-ONLY audit of every plugin under `✏️s/🔌️plugins/` against 📓️design-virtualised-tree.md §5 (SDK) and §8 (app
migration rule): no `+N`/`…more` continuation rows, no silent truncation, every tree list windowed through
`tree_window_section[_or_placeholder]` / `tree_window_item` / `PanelTreeBuilder::window_section[_or_placeholder]` /
`TreeWindows` / the SDK `ui_node_list`. Rust production sources only; `dist/`, `node_modules/`, `🗑️generated` excluded.

## Verdict summary

**Wave 2 (app migration) is essentially complete.** Of the 32 plugin crates with panel-tree code, **31 are clean**
(every document/registry-scaled list windowed; the only raw, un-windowed `.section(`/`PanelTreeBuilder` calls left
build small, compile-time-bounded field lists — a handful of rows for one selected entity or a static enum — which
carry no `+N`/truncation/hard-fault risk). Zero literal `+N` labels, `"… more"`/`.more` continuation keys, `paged_`,
`panel_page(s)`, `setPanelPage`/`SetPanelPage`, `PANEL_RECONCILE_NODE_BUDGET`, `SECTION_ROWS`/`IDS_ROWS`/
`LIST_ROWS_MAX`/`CATALOGUE_GROUP_ROWS`/any `_ROWS_MAX` constant, `artifact_tree_cache`/`Puzzle3dArtifactTreeKey`, or
a shadow local `fn ui_node_list` were found anywhere in production code. Every `.more`/`continuation`/`setPanelPage`
string still present is inside a **test** asserting the old residue is gone.

**1 plugin is a real offender:**

1. **`📕️norm`** (HIGH) — its shared `app-surface` crate never adopted `TreeWindows`/the windowed SDK at all. The
   "Results" body (`render_report`, fanned out to all ~15 norm-code artifacts: en1990/91/92/93/94/95/96/97/98/99,
   din4108/16798/18599, iso16757, vdi3805) builds one text row per computed check with a bare, unbounded
   `report.checks.iter().enumerate().map(...)` into `ui::column().try_children(...)` — no `TreeWindows`, no window
   call, no cap. A report with enough checks hard-faults the whole render with `ui.fixed-capacity`, the exact "hard
   fail past a fixed row cap" failure mode this migration exists to remove. Zero window-law tests exist for `norm`
   (no `TreeWindowRequest`/`window.total`/`tree_windows` anywhere in the plugin). `norm`'s own `render_inspection` is
   fine (bounded, single-check summary).

**2 plugins have a documented, lower-severity gap** (not `+N`, but incomplete windowing / cap-adjacent):

2. **`🪐️space`** (MEDIUM) — the engine `🔢️parameters` panel (`⚙️engine/🪐️space/📌️panels/🔢️parameters/🦀️.rs:181-291`)
   is a `column` of `Container` sections sliced with `TreeWindows::slice`, but `ContainerProps` carries no window
   carrier field, so `total` can never be stamped back to the host — the panel silently stays pinned at its
   first-paint slice forever (self-documented in a production comment). No `+N` row, but the host never learns the
   full extent as §4/§5 require, and no `TreeWindowRequest` test exists for this panel.
3. **`🏗️fem` (2d & 3d) / `🔋️energy`** (LOW-MEDIUM) — `results`/`inspection` panels cap **dropdown option lists**
   (not tree rows) with `.take(RESULT_SOURCE_OPTIONS = 24)`, `.take(SELECT_ITEMS_MAX = 24)`, `.take(MARKED_IDS_LIMIT)`
   on load-case/combination/entity-reference/marked-id collections that scale with the document. These are
   documented, deliberate `<select>`-arena limits (outside `TreeWindowKit`'s literal remit — no window primitive
   exists for `<select>` today) but share the exact silent-cap shape the ticket targets; worth a follow-up decision
   on whether `<select>` needs a windowing story too.

**Everything else is clean**, including all of `🧩️puzzle` (2d/3d/5d), `🏗️fem` outliner/artifact panels, `📐️cad`,
`🏭️process` (process3d), `🌀️procedural` (generation2d/3d — the generation3d flow-graph outline even carries a
comment naming *this ticket* as the fix for a prior 33-row hard-fault), `🏛️architect`, `📸️remodel`, `🔱️trinity`,
`🧱️block` (2d/3d/5d), `💠️lowpoly`, `📏️layout`, `🌊️flow` (+ its 10 extension crates, which have no tree code at
all), `🕸️dag`, `✒️writer`, `🎥️shooting`, `🌍️gis`, `📋️forms`, `🌿️vcs`, `🖨️raster`, `💡️reasoning`, `🎬️sequence`,
`🗒️note`, `🖍️draw`, `📜️imperative`, `🎞️animate`, `🎪️demonstrator`, `🪵️sourcing`, `📖️playbook`, `➗️mathematical`,
and `🗄️stdio` (all 6 tree-bearing format editor/viewer pairs — json/rfc8259 base+i-json, xml/1.0 base+valid,
zip/2.0 base+iso21320 — delegate to the framework's own `TreeWindowKit::render_windowed` with a real
`TreeWindows::for_body` wired at the dispatch site; the other ~350 stdio panel-bearing formats use `TableWindowKit`/
`TextWindowKit`/`MeshWindowKit`/`ImageWindowKit`, which are not trees and out of scope).

**Pervasive, non-offending pattern worth naming once:** nearly every plugin's `🔍️inspection` panel (and several
`🛍️catalogue` panels) build their few rows with a raw `PanelTreeBuilder::new(id)?.section(...)` instead of
`window_section`, because the content is a small, compile-time-bounded field list for **one** selected
entity/document-summary/static-enum (2–10 rows) — not a list that scales with the document. These are **not**
residue (no `+N`, no truncation, no fault risk) but are technically non-compliant with the literal "every list
section → `window_section`" wording of §8.1. Listed as "Partial" in the table below rather than repeated in prose.

**Window-law test coverage** (tests asserting `TreeWindowRequest`/`window.total`/`tree_windows`, or an explicit
`!json.contains(".more")`/`window_law_no_continuation` law) exists for the document/artifact panel of essentially
every plugin. Coverage is measurably thinner for `🛍️catalogue`/`🔍️inspection` panels in several plugins (noted
per-plugin below), and is **entirely absent** for `📕️norm`.

---

## Table — every panel-tree build site found

Legend: **Yes** = every document-scaled list in this panel goes through a windowed builder. **Partial** = mix of
windowed calls for the scaling list(s) and raw `.section(` for bounded fixed content (by design, not residue).
**No / N/A** = no windowed call, but content is intrinsically bounded (no residue) unless flagged otherwise.

### 🧩️puzzle (2d / 3d / 5d)

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| puzzle-2d | artifact (outliner) | `◻️2d/…/📌️panels/🗿️artifact/🦀️.rs:84-88` | `window_section_or_placeholder` (nodes, edges) | Yes | none |
| puzzle-2d | catalogue | `◻️2d/…/📌️panels/🛍️catalogue/🦀️.rs:98-102` | `window_section_or_placeholder` ×3 | Yes | none |
| puzzle-2d | inspection | `◻️2d/…/📌️panels/🔍️inspection/🦀️.rs:82-87,140,147,174,184` | `window_section` for the unbounded selection-ids list; raw `.section(` for the selected entity's own fixed field group + doc summary | Partial | none (bounded) |
| puzzle-2d | settings | `◻️2d/…/📌️panels/⚙️settings/🦀️.rs:64-76` | raw `ui::section` + `ui_node_list` (3 fixed stepper rows) | N/A | none |
| puzzle-3d | artifact (outliner) | `🧊️3d/…/📌️panels/🗿️artifact/🦀️.rs:143-185` | `window_section` (objects/references/target-volumes/attractions) + `tree_window_item` (nested vortices) | Yes | none |
| puzzle-3d | catalogue | `🧊️3d/…/📌️panels/🛍️catalogue/🦀️.rs:117,144-153` | `window_section` ×4 + `tree_window_item` (nested vortex templates) | Yes | none |
| puzzle-3d | inspection | `🧊️3d/…/📌️panels/🔍️inspection/🦀️.rs:86,167-174,178-184` | `window_section` for ids; raw `.section(` for fixed field group + doc summary | Partial | none (bounded) |
| puzzle-3d | settings | `🧊️3d/…/📌️panels/⚙️settings/🦀️.rs:59-64` | raw `ui::section` + `ui_node_list` (4 fixed rows) | N/A | none |
| puzzle-5d | artifact (outliner) | `🖐️5d/…/📌️panels/🗿️artifact/🦀️.rs:81,96-100` | `window_section_or_placeholder` (parts/fasteners) + `tree_window_item` (grips) | Yes | none |
| puzzle-5d | catalogue | `🖐️5d/…/📌️panels/🛍️catalogue/🦀️.rs:113-118` | `window_section_or_placeholder` ×4 | Yes | none |
| puzzle-5d | inspection | `🖐️5d/…/📌️panels/🔍️inspection/🦀️.rs:33-40` | raw `.section(` + `ui_node_list`, no `TreeWindows` param | N/A | none — selection-based rendering was removed (framework gap noted in a doc comment); only a fixed 4-row summary remains |

### 🏗️fem (2d / 3d)

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| fem-2d | artifact (outliner) | `◻️2d/…/📌️panels/🗿️artifact/🦀️.rs:229-321` | `window_section_or_placeholder` ×7 + `tree_window_item` (load-case loads, combination terms) + `window_section` (analysis) | Yes | none |
| fem-2d | inspection | `◻️2d/…/📌️panels/🔍️inspection/🦀️.rs:403,437,443` + raw `.section(` at 363,364,408,412,416,420,424,448 | `window_section` for ids/case-loads/combination-terms; raw `.section(` for fixed per-entity field groups | Partial | none in-tree; see MEDIUM finding above re: `SELECT_ITEMS_MAX` dropdowns fed from the same file |
| fem-2d | results | `◻️2d/…/📌️panels/📊️results/🦀️.rs:128,185,216` | local `section()` wrapper, no `TreeWindows` | N/A | **`.take(RESULT_SOURCE_OPTIONS = 24)`** on `result_sources()` (dropdown, not tree rows) — see finding #3 above |
| fem-3d | artifact (outliner) | `🧊️3d/…/📌️panels/🗿️artifact/🦀️.rs:228-321` | same shape as fem-2d | Yes | none |
| fem-3d | inspection | `🧊️3d/…/📌️panels/🔍️inspection/🦀️.rs:415,450,457` + raw `.section(` at 375,376,420,424,428,432,436,440,444,449,456,461 | `window_section` for ids/case-loads/combination-terms; raw `.section(` for fixed field groups | Partial | `select_row()` **`.take(SELECT_ITEMS_MAX = 24)`** (line 44/143) on node/element/material/section/solid reference dropdowns — see finding #3 above |
| fem-3d | results | `🧊️3d/…/📌️panels/📊️results/🦀️.rs:129,186,217` | same as fem-2d | N/A | `.take(RESULT_SOURCE_OPTIONS = 24)` |

### 📐️cad / 🏭️process (process3d) / 🌀️procedural (generation2d/3d)

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| cad | editor dispatch | `📐️cad/…/✏️editor/🦀️.rs:1309-1314` | `TreeWindows::for_body` per body | Yes | none |
| cad | artifact (document tree) | `…/📌️panels/🗿️artifact/🦀️.rs:127,132,163` + `tree_window_item` (:59) | `window_section`, `window_section_or_placeholder` | Yes | none |
| cad | catalogue | `…/📌️panels/🛍️catalogue/🦀️.rs:28` | `window_section` | Yes | none |
| cad | inspection (selection ids) | `…/📌️panels/🔍️inspection/🦀️.rs:94` | `window_section` | Yes | none |
| cad | inspection (reference/node fields) | `…/📌️panels/🔍️inspection/🦀️.rs:140,153,168` | raw `PanelTreeBuilder::section`/`ui_node_list` | No | none (bounded fixed field lists, not scaling data) |
| process3d | editor dispatch | `…/✏️editor/🦀️.rs:1340-1342` | `TreeWindows::for_body` | Yes | none |
| process3d | artifact / catalogue / workshop / inspection | `…/📌️panels/{🗿️artifact:102-103, 🛍️catalogue:102,110,119, 🛠️workshop:76,88, 🔍️inspection:90,102,133,145,149}` | `window_section` throughout, incl. a per-machine loop (line 149) | Yes | none |
| generation3d | editor dispatch | `…/✏️editor/🦀️.rs:313,317-319` | `TreeWindows::for_body` for artifact/catalogue; inspection deliberately gets no `TreeWindows` param | Yes / N/A | none |
| generation3d | inspection | `…/📌️panels/🔍️inspection/🦀️.rs:30,43,113` | raw `.section` + local `UiFixedList` | No | none (bounded per-widget-kind fields) |
| generation3d | catalogue / artifact | `…/📌️panels/{🛍️catalogue/🦀️.rs:72,79, 🗿️artifact}` | `window_section` + `tree_window_item` | Yes | none |
| generation3d | generate-mode "generations" window | `…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:40` → `🫀️core/🖼️semantic-ui/🦀️.rs:179-181` | `window_section_or_placeholder` + one bounded raw `.section` ("add" row) | Yes | none |
| generation3d | edit-mode flow graph outline | `…/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:160,183,186` | `window_section_or_placeholder` (nodes, edges) + `tree_window_item` (ports) | Yes | **none now** — a comment at lines 177-179 names *this ticket* as the fix for a prior fault refusing at the 33rd row |
| generation2d | editor dispatch / artifact / catalogue | `…/✏️editor/🦀️.rs:1976,1983-1984`; `📌️panels/🗿️artifact/🦀️.rs:37`; `📌️panels/🛍️catalogue/🦀️.rs:29,34,38,42` | `TreeWindows::for_body`, `window_section_or_placeholder`, `window_section` ×4 | Yes | none |
| generation2d | inspection | `…/📌️panels/🔍️inspection/🦀️.rs:34` | raw `.section` (empty-state placeholder) | N/A | none |

### 🏛️architect / 📸️remodel / 🔋️energy

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| architect | viewer register window | `👁️viewer/…/🪟️windows/📋️register/🦀️.rs:59-64` | `window_section` + `tree_window_item` | Yes | none |
| architect | document panel | `✏️editor/📌️panels/🗿️artifact/🦀️.rs:56-62` | raw `.section` (fixed 3-row meta) + `window_section`/`window_section_or_placeholder` | Yes | none. Header notes the old `.chunks(UI_FIXED_LIST_ITEMS)` "Registers 1–32" idiom is gone |
| architect | inspection | `✏️editor/📌️panels/🔍️inspection/🦀️.rs:39` | raw `PanelTreeBuilder::section` only | No | none (bounded summary) |
| architect | catalogue | `📚️catalogue/🦀️.rs:77-79` | `window_section` ×2 | Yes | none |
| architect | adjacency / report / trace windows | `🎭️modes/✏️edit/🪟️windows/{↔️adjacency:101-113, 📓️report:67-69, 🧭️trace:51-52}` | `window_section`/`window_section_or_placeholder` + `tree_window_item` | Yes | none. Trace window comment: "the `.take(12)` truncation is gone" |
| remodel | play editor dispatch | `✏️editor/🦀️.rs:960-966` | dispatches with `TreeWindows::for_body` per panel | Yes | none |
| remodel | media / parameters / artifact / tracks / quality / results / calibration panels | `📌️panels/{🗂️media:30-32, ⚙️parameters:62-63, 🗿️artifact:78-79, 🏃️tracks:28-30, ✅️quality:49-51, 🧵️results:40-41, 🎯️calibration:34-39}` | `window_section`/`window_section_or_placeholder`, raw `.section` only for fixed status/summary rows | Yes | none |
| energy | artifact (outliner) | `📌️panels/🗿️artifact/🦀️.rs:399-468` | `window_section`/`window_section_or_placeholder` (11 sections) + `tree_window_item` (zone→space/surface, surface→openings) | Yes | none. Doc comment: "virtualised, never paged… no `+N` row, no row quota" |
| energy | inspection (entity forms) | `📌️panels/🔍️inspection/🦀️.rs:642-721` | raw `.section` for every entity-detail form; one `window_section_or_placeholder` (construction layer stack, line 416) | Partial | none in-tree; see finding #3 (`.take(MARKED_IDS_LIMIT)` line 385, `.take(SELECT_ITEMS_MAX)` lines 159/518 on marking/dropdown lists) |

### 🔱️trinity / 🧱️block / 💠️lowpoly / 📏️layout

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| trinity | jack / Document | `…jack/…/📌️panels/🗿️artifact/🦀️.rs:35-37` | `window_section` ×2 | Yes | none |
| trinity | jack / Catalogue | `…jack/…/📚️catalogue/🦀️.rs:54-57` | `window_section` ×2 + raw `.section` (3-item static "kinds" enum) | Partial | none (bounded) |
| trinity | jack / Inspection | `…jack/…/🔍️inspection/🦀️.rs:18` | raw `.section` | No | none (single fixed placeholder row) |
| trinity | rewriting / Document, Catalogue, Inspection | `…rewriting/…/{🗿️artifact:34-35, 📚️catalogue:35-38, 🔍️inspection:18}` | same shape as jack | Yes/Partial/No | none |
| block | 2d/3d/5d Document | `{2d,3d,5d}/…/📌️panels/🗿️artifact/🦀️.rs` | `window_section_or_placeholder` ×2 each | Yes | none |
| block | 2d/3d/5d Inspection | `{2d,3d,5d}/…/🔍️inspection/🦀️.rs:{62,94,80}` | raw `.section` (3-5 fixed rows) | No | none (bounded); **note:** block-3d inspection (line 75-77) also populates a `<select>` from `definition.representations` in an unbounded `for` loop — not a tree row, flagged as an out-of-scope growth risk |
| lowpoly | Document (meshes→groups→elements) | `…/📌️panels/🗿️artifact/🦀️.rs:108,115,127` | `window_section` + `tree_window_item` ×2 nested levels | Yes | none |
| lowpoly | Layers / Catalogue | `…/{🗂️layers/🦀️.rs:38, 🛍️catalogue/🦀️.rs:43}` | `window_section` | Yes | none |
| lowpoly | Inspection | `…/🔍️inspection/🦀️.rs:65,114,115,116` | raw `.section` ×3 (empty/object/transform) + `window_section` (utility-params) | Partial | none (bounded) |
| layout | Document (9 sections) | `…/📌️panels/🗿️artifact/🦀️.rs:180-242` | `window_section` ×7 + `window_section_or_placeholder` ×1 | Yes | none |
| layout | Catalogue / Preflight | `…/{🛍️catalogue/🦀️.rs:66-69, 🚦️preflight/🦀️.rs:244-245}` | `window_section`/`window_section_or_placeholder` | Yes | none |
| layout | Inspection | `…/🔍️inspection/🦀️.rs:42-44` | raw `.section` (4 fixed rows) | No | none (bounded) |

### 🌊️flow / 🕸️dag / ✒️writer / 🪐️space / 🎥️shooting / 🌍️gis

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| flow | document / catalogue / inspection | `…flow/…/📌️panels/{🗿️artifact:42-60, 🛍️catalogue:66-101, 🔍️inspection:37-47}` | `window_section_or_placeholder`/`window_section` throughout (catalogue loops sections + calls `append_extension_sections`) | Yes | none |
| flow | generate-mode "generations" window | `…/🗂️generations/🦀️.rs:102-113` | `window_section_or_placeholder` + 1 bounded raw `.section` ("add" row) | Yes | none |
| flow | 🧩️extensions/{brep,dictionary,bim,logic,primitive,math,fixtures,list,draw,text} | — | no panel-tree code at all | N/A | none |
| dag | document / catalogue | `…/📌️panels/{🗿️artifact:36-54, 🛍️catalogue:26-38}` | `window_section_or_placeholder` ×2, `window_section` | Yes | none |
| dag | inspection | `…/🔍️inspection/🦀️.rs:79,108,121` | raw `.section` only (1-3 items) | N/A | none |
| writer | document | `…/📌️panels/🗿️artifact/🦀️.rs:68-82` | raw `.section` (fixed, non-jack branch) + `window_section_or_placeholder`/`tree_window_item` (jack AST) | Yes (mixed) | none |
| writer | catalogue / inspection | `…/{🛍️catalogue/🦀️.rs:25, 🔍️inspection/🦀️.rs:36-44}` | raw `.section` (static/fixed) + `window_section` (diagnostics) | Partial/N/A | none |
| space | engine catalogue (S-Play) | `⚙️engine/🪐️space/📌️panels/🛍️catalogue/🦀️.rs:111-120` | `window_section` + recursive `tree_window_item` | Yes | none |
| space | engine inspection | `…/🔍️inspection/🦀️.rs:159-264` | raw `.section` ×3 (bounded header/media-nodes/app-instances) + `window_section` (app-parameters) | Yes (mixed) | none |
| space | engine parameters | `…/🔢️parameters/🦀️.rs:192-282` | `TreeWindows::slice` on a plain `column` (not a `Tree`) | **Partial (documented gap)** | **`ContainerProps` has no window carrier — `total` never stamped, panel pinned at first-paint slice** (finding #2 above) |
| space | artifact members | `🪐️space/…/📌️panels/👥️members/🦀️.rs:75-78` | raw `.section` (3 fixed action rows) + `window_section_or_placeholder` (members) | Yes (mixed) | none |
| shooting | document / catalogue | `…/📌️panels/{🗿️artifact:42-51, 🛍️catalogue:44-50}` | `window_section` ×2 each | Yes | none |
| shooting | inspection | `…/🔍️inspection/🦀️.rs:69,83` | raw `.section` only | N/A | none |
| gis | gismap document / catalogue | `gismap/…/📌️panels/{🗿️artifact:42-43, 🛍️catalogue:34-35}` | `window_section` | Yes | none |
| gis | gismap inspection | `…/🔍️inspection/🦀️.rs:39-54` | raw `.section` only (2-3 items) | N/A | none |
| gis | gisterrain | — | no panel-tree code | N/A | none |

### 📋️forms / 🌿️vcs / 🖨️raster / 💡️reasoning (wires) / 🎬️sequence / 🗒️note / 🖍️draw / 📜️imperative / 🎞️animate / 📕️norm

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| forms | document | `…/📌️panels/🗿️artifact/🦀️.rs:87-88` (+ `tree_window_item` :70) | `window_section_or_placeholder` | Yes | none |
| forms | catalogue | `…/🛍️catalogue/🦀️.rs:69-71` | `window_section` + raw `.section` (2-row fixed actions) | Yes (mixed) | none |
| forms | inspection | `…/🔍️inspection/🦀️.rs:36` | raw `.section` | No | none (bounded) |
| vcs | document/history | `…/📌️panels/🗿️artifact/🦀️.rs:40-72` | `window_section_or_placeholder` (:51) + `window_section` (alternatives, :72) | Yes | none |
| vcs | inspection | `…/🔍️inspection/🦀️.rs:54` | raw `.section` | No | none (bounded) |
| raster | document (layers) | `…/📌️panels/🗿️artifact/🦀️.rs:99-102` (+ `tree_window_item` :88) | `window_section` | Yes | none. Header: "There is no `+N` row" |
| raster | masks / catalogue | `…/{🎭️masks/🦀️.rs:58-59, 🛍️catalogue/🦀️.rs:31}` | `window_section_or_placeholder`/`window_section` | Yes | none |
| raster | inspection | `…/🔍️inspection/🦀️.rs:36` | raw `.section` | No | none (bounded) |
| reasoning (wires) | document / catalogue | `…/📌️panels/{🗿️artifact:102-104, 🛍️catalogue:56-66}` | `window_section_or_placeholder` ×2 each | Yes | none |
| reasoning (wires) | inspection | `…/🔍️inspection/🦀️.rs:37-51` | raw `.section` | No | none (bounded; selection feature removed, documented gap) |
| sequence | step tree (document) | `…/📌️panels/🗿️artifact/🦀️.rs:96-106` (+ `tree_window_item` :84, nested slots) | `window_section_or_placeholder` ×2 | Yes | none |
| sequence | catalogue | `…/🛍️catalogue/🦀️.rs:39-41` | raw `.section` (5 fixed step-kind actions) + `window_section` (per-slot shortcuts) | Yes (mixed, by design — comment explains the split) | none |
| sequence | inspection | `…/🔍️inspection/🦀️.rs:56` | raw `.section` | No | none (bounded) |
| note | blocks document | `…/📌️panels/🗿️artifact/🦀️.rs:75-78` (+ `tree_window_item` :50, nested `Group`) | raw `.section` (5 fixed quick-add rows) + `window_section_or_placeholder` (blocks) | Yes (mixed, by design — comment cites **this ticket**, §8.1) | none |
| note | catalogue / inspection | `…/{🛍️catalogue/🦀️.rs:35, 🔍️inspection/🦀️.rs:44-45}` | raw `.section` (enum-driven/fixed) | No | none (bounded) |
| draw | layers | `…/📌️panels/🗂️layers/🦀️.rs:152-153` (+ `tree_window_item` :110-111, nested Group/Boolean) | `window_section` | Yes | none |
| draw | catalogue / properties | `…/{🛍️catalogue/🦀️.rs:89-90, 🔍️properties/🦀️.rs:26-28}` | `window_section` / no tree builder (single text node) | Yes / N/A | none |
| imperative | document / catalogue | `…/📌️panels/{🗿️artifact:48-49, 🛍️catalogue:28-29}` | `window_section_or_placeholder` / `window_section` | Yes | none |
| imperative | inspection | `…/🔍️inspection/🦀️.rs:36` | raw `.section` | No | none (bounded) |
| animate | presentation deck | `…/📌️panels/🗿️artifact/🦀️.rs:34-35` | `window_section_or_placeholder` | Yes | none |
| animate | inspection | `…/🔍️inspection/🦀️.rs:38-39` | raw `.section` | No | none (bounded) |
| **norm** | shared `render_inspection` (all ~15 codes) | `📕️norm/🖥️app-surface/🦀️.rs:199-215` | raw `PanelTreeBuilder::section` | No | none (bounded, 4 fixed fields) |
| **norm** | shared `render_report` ("Results" window, all ~15 codes) | `📕️norm/🖥️app-surface/🦀️.rs:159-177` | **No tree builder at all** — `ui::column().try_children(report.checks.iter().enumerate().map(...))`, zero window call | **No** | **YES — HIGH, see finding #1.** Fanned into production via 15 call sites, e.g. `📕️norm/🗿️artifacts/⚖️en1990/…/📊️results/🦀️.rs:23` |
| norm | catalogue (all codes) | `📕️norm/🖥️app-surface/🦀️.rs:193-194` | placeholder text only | N/A | none |

### 🎪️demonstrator / 🪵️sourcing / 📖️playbook / ➗️mathematical / 🗄️stdio

| Plugin | Crate/Panel | File:Line | Builder used | Windowed? | Residue found |
|---|---|---|---|---|---|
| demonstrator | playground viewer/editor `main` window | `🎪️playground/…/🪟️windows/🪟️main/🦀️.rs:26` (viewer & editor) | `TextWindowKit::render` | N/A — not a tree | none |
| sourcing | curation editor `preview`/`grid`, viewer `pool` | `🗂️curation/…/{👁️preview:49, 🔢️grid:97, 🏊️pool:32}` | `MeshWindowKit::render` / `TableWindowKit::render` | N/A — not a tree | none |
| playbook | viewer `steps` window | `👁️viewer/…/🪟️windows/🌳️steps/🦀️.rs:48`, wired at `👁️viewer/🦀️.rs:76` | `TreeWindowKit::render_windowed(&TreeView{roots}, windows)` with a real `TreeWindows::for_body` | **Yes** | none |
| playbook | builder-mode `builder` window | `✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️.rs:75` | `scene_surface(..., BlockList, build_playbook_list_scene(...))` | Out of scope — not a `PanelKit`/`TreeWindowKit` tree | none observed |
| mathematical | equation viewer `geometry` window | `➗️equation/…/📐️geometry/🦀️.rs:36` | `TableWindowKit::render` | N/A — flat coordinate table, not a tree | none |
| stdio | json (rfc8259 base + i-json), xml (1.0 base + valid), zip (2.0 base + iso21320) — 6 editor+viewer pairs | e.g. `🧾json/…/🧱️base/{✏️editor,👁️viewer}/…/🪟️main/🦀️.rs`; dispatch wiring at each `✏️editor/🦀️.rs`/`👁️viewer/🦀️.rs` (`TreeWindows::for_body(view_state, BODY_KEY)`) | `TreeWindowKit::render_windowed`, all 12 sites wired to a **real** `TreeWindows::for_body` (none fall back to `::unhosted()`) | **Yes**, all 6 pairs | none. Zero raw `PanelTreeBuilder`/`TreeSectionBuilder`/`TreeItemBuilder` hits anywhere in stdio's ~5670 files |
| stdio | bcf (2.1 markup) viewer `main` | `💬️bcf/…/🖊️markup/👁️viewer/…/🪟️main/🦀️.rs:33` | `TableWindowKit::render` (comment mentions `TreeWindowKit` only as a rejected alternative) | N/A — flat table | none |
| stdio | remaining ~350 format viewer/editor pairs | — | `TableWindowKit`/`TextWindowKit`/`MeshWindowKit`/`ImageWindowKit` (none build a `Tree`) | N/A | none found in a full residue-pattern grep across all 5670 files |

---

## Residue list (all confirmed hits, file:line + snippet)

**Genuine residue (production, matches the ticket's failure shape):**

1. `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:159-177` (`render_report`) —
   ```rust
   let children = report.checks.iter().enumerate().map(|(index, check)| { ... }).collect::<UiAssemblyResult<Vec<_>>>()?;
   ui::column().try_children(children)...
   ```
   Unbounded iterator over `report.checks`, zero `TreeWindows`/window call, feeds `ui::column().try_children(...)`
   (a fixed-capacity admission). Fanned into all ~15 norm-code artifacts (en1990/91/92/93/94/95/96/97/98/99,
   din4108/16798/18599, iso16757, vdi3805), e.g. call site
   `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:23`.
   A report with enough checks hard-faults the whole render with `ui.fixed-capacity` instead of degrading — exactly
   the pre-migration failure mode. Only unit test (`📕️norm/🖥️app-surface/🧪️tests/🔬️unit/🦀️.rs:57-75`) exercises 2
   checks; no oversized-report test exists.

**Documented gaps (not `+N`, but incomplete per §4/§5):**

2. `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔢️parameters/🦀️.rs:181-191` — production comment: *"Virtualised
   through `TreeWindows::slice` rather than `window_section`: this body is NOT a tree... only the slice
   `[offset, offset+len)` of `projection.parameters` is BUILT... `TreeWindow` has no carrier on `ContainerProps`...
   so the full `slice.total` extent cannot be stamped for the host, and this body therefore files no window requests
   and stays at its first-paint slice."* Self-documented SDK-capability gap: no `total`/paging round-trip exists for
   non-tree `Container` bodies.

**Out-of-scope but same silent-cap shape (dropdowns / marking lists, not tree rows — flagged per audit brief):**

3. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📊️results/🦀️.rs:152` and
   the mirrored `◻️2d/…/📊️results/🦀️.rs` — `.take(RESULT_SOURCE_OPTIONS)` (`const RESULT_SOURCE_OPTIONS: usize = 24`)
   in `result_sources()`: silently drops load cases/combinations past 24 from a results-source `<select>`.
4. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs:143`
   (`select_row`, `const SELECT_ITEMS_MAX: usize = 24` at line 44) — same pattern for node/element/material/section/
   solid reference dropdowns. Comment explicitly justifies it ("a document with more nodes than this is re-pointed
   in the viewport, not from a dropdown"), i.e. a deliberate, documented control-arena limit, not an oversight.
5. `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs:385`
   — `.take(MARKED_IDS_LIMIT)` (= `UI_FIXED_LIST_ITEMS`) caps the marking (`selected`/`highlighted`) stamp list, not
   row materialization — doesn't drop tree rows, but is a `.take()` on a list derived from potentially-large
   `interaction.selected_ids`.
6. `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs:159,518`
   — `.take(SELECT_ITEMS_MAX)` truncating schedule/layer catalogue `<select>` option lists.
7. `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs:75-77`
   — `for representation in &definition.representations { select = select.try_item(...) }`: unbounded loop into a
   `<select>` dropdown (not a tree section/item), no cap and no window primitive applies to `<select>` today.

**Everything else that matched a residue keyword is a false positive**, confirmed by reading context:

- All `.more`/`continuation`/`setPanelPage`/`IDS_ROWS`/`paged_` hits outside the above are inside **test files**,
  as negative regression assertions the old bug is gone (e.g. `assert!(!json.contains(".more"), "a windowed
  container never mints a continuation key")`, `puzzle3d…/🧪️tests/🔬️selection-scale/🦀️.rs:139`) or in **doc
  comments** citing the historical bug/fix (e.g. `generation3d…/🕸️flow/🦀️.rs:177-179`, `puzzle3d…/🔍️inspection/
  🧪️tests/🔬️unit/🦀️.rs:198`, `puzzle2d…/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:5`).
- `section_pages`/`section_id` hits in `stdio/🖊️dwg` are DWG page-table decoding internals, unrelated to UI panels.
- `"more"` as a literal JSON test key in `procedural…/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs:134,168` is an unrelated
  tick-outcome flag, not a UI row label.
- `paged_text_carrier` (cad, generation3d test/doc-comment hits) is an unrelated text-chunking primitive for large
  string payloads, not a row-continuation mechanism.
- No hit anywhere for `artifact_tree_cache` / `Puzzle3dArtifactTreeKey` (already deleted, per §8.3) or a locally
  redefined `fn ui_node_list` shadowing the SDK's.

---

## Window-law test coverage gaps (informational, not residue)

Tests asserting `TreeWindowRequest`/`window.total`/`tree_windows`/an explicit no-continuation law exist for the
document/artifact panel of essentially every plugin. Coverage is thinner in these spots:

- **norm**: **no window-law tests at all** (consistent with never adopting `TreeWindows`).
- **space**: engine `🔢️parameters` panel has no `TreeWindowRequest` test (consistent with its documented gap, #2).
- **fem-2d/3d**: inspection panels use `TreeWindows::unhosted()` in tests but don't assert `window.total`/`.more`
  explicitly; results panels have no window-law tests at all (they take no `TreeWindows` param).
- **remodel**: only the calibration panel has a window-law test; media/parameters/pipeline/tracks/quality/results
  panels are windowed in production but untested for the window law specifically.
- **energy**: only the artifact/outliner panel has a window-law test; the (`.section`-heavy) inspection panel does
  not.
- **cad**: inspection panel test file exists but never asserts `.more`/`TreeWindowRequest`/`window.total`, despite
  `selected_object_section` windowing a potentially large multi-selection ids list.
- **lowpoly / layout**: catalogue and inspection panel test files exist but carry no window-law-specific assertions
  (consistent with their small/fixed data).
- **trinity (jack/rewriting)**: only the document panel has a `.more`-absence test; catalogue/inspection do not.
- **block (2d/3d/5d)**: only the document panel has a window-law test; inspection panels (non-windowed, bounded) do
  not.
- **stdio**: only the 3 "base"-subset editor windows sampled (`zip/2.0/base`, `xml/1.0/base`, `json/rfc8259/base`)
  have their own `TreeWindowRequest` test; sibling variants (`iso21320`, `valid`, `i-json`) and viewer-side windows
  reuse the same render logic without independent coverage.
- **puzzle-5d**: inspection has no window-law test (consistent with taking no `TreeWindows` param at all).

Everything else audited (puzzle-2d/3d artifact+catalogue, fem-2d/3d artifact, architect ×3 panels, dag, writer,
forms, vcs, raster, reasoning/wires, sequence, note, draw, imperative, animate, gis, shooting, space
artifact-members + engine-catalogue, flow, playbook) has a dedicated `TreeWindowRequest`/`window.total`/
no-continuation test for its windowed panel(s).

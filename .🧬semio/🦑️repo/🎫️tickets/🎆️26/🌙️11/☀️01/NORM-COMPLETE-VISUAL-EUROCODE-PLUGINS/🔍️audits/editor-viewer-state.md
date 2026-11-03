# 🪟 Editor and Viewer State

Read-only audit of the norm app-surface (`✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`) and the editor/viewer trees of the ten Eurocode artifacts (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/{⚖️en1990 … 🪶️en1999}/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}`). No source file was modified. Findings come from reading the code; nothing was built or run. Points marked "unverified" were not checked against the framework runtime.

## 1. Artifact Matrix

Counts are from `grep`/`wc` over the editor and viewer trees. "Field-meta" counts are table rows in the field-meta module, not a coverage percentage. "Tests" counts `#[test]` and `#[async_test]` attributes under `✏️editor` plus `👁️viewer`.

| Artifact | Editor windows | Viewer windows | Panels (editor) | Commands | Field-meta (rows, location) | What is rendered | TS twins | Tests (ed+vw) | Examples |
|---|---|---|---|---|---|---|---|---|---|
| ⚖️ en1990 | inputs, results | report | document, catalogue, inspection | 6 | 55, `✏️editor/🏷️field-meta` (+ unit test) | generic input tree, check tree, `TableWindowKit` table | 11 (ed 7, vw 4) | 31+5 = 36 | 7 |
| 🏋️ en1991 | inputs (regrouped ≤32-child projection), results | report | document, catalogue (no `TreeWindows`), inspection | 6 | 113 match arms, `✏️editor/🏷️field-meta` | as above, plus second "grouped" document shape | 11 | 30+5 = 35 | 2 |
| 🏛️ en1992 | inputs, results | report | document, catalogue (+ dead `catalogue_markdown`), inspection | 6 | 113, `✏️editor/🏷️field-meta` | as above | 11 | 31+6 = 37 | 5 |
| 🔩️ en1993 | inputs, results | report | document, catalogue, inspection | 6 | 171, `✏️editor/🏷️field-meta` (+ unit test) | as above | 11 | 35+6 = 41 | 2 |
| 🧩️ en1994 | inputs, results | report | document, catalogue, inspection | 6 | 98, `✏️editor/🏷️field-meta` | as above | 11 | 32+6 = 38 | 3 |
| 🪵️ en1995 | inputs, results | report | document, catalogue (`TreeWindows::unhosted()`, localized `reference_tables(locale)`), inspection | 6 | 66, `✏️editor/🏷️field-meta` (+ unit test) | as above; examples via `examples()` module fn | 11 | 35+6 = 41 | via `examples()` |
| 🪨️ en1996 | inputs, results | report | document, catalogue, inspection | 6 | 58, `✏️editor/🏷️field-meta` | as above | 11 | 34+6 = 40 | 2 |
| 🌍️ en1997 | inputs, results | report | document, catalogue, inspection | 6 | 83, in `🧬️schema/💡️inferences/⚖️checks` (no editor field-meta module) | as above | 11 | 32+6 = 38 | 3 |
| 🫨️ en1998 | inputs, results | report | document, catalogue, inspection | 6 | 127, subset-root `🏷️field-meta` (not under `✏️editor`) | as above | 11 | 31+6 = 37 | 4 |
| 🪶️ en1999 | inputs, results | report | document, catalogue, inspection | 6 | 111, `✏️editor/🏷️field-meta` | as above | 11 | 16+6 = 22 | 2 |

Common facts across all ten:

- Each has the same six commands (`evaluate`, `setSelectedCheckIndex`, `setField`, `insertItem`, `removeItem`, `applyRemedy`), the same `edit` mode, and the same inputs/results layout at 42/58.
- Each has three editor panels (document headline, catalogue, inspection) and one viewer window.
- Each has 11 TS files: editor 7, viewer 4. All TS files are typed twins of the Rust view-models, and all of them are stale (see section 2.4).
- No artifact renders a diagram, chart, 2D plot, or 3D scene. The only visuals are text, trees, and one table.

## 2. What `render_document_editor` and the Report Tree Produce

### 2.1 Inputs window (`📝️document-editors/🦀️.rs`, `render_value_editor`)

The document is turned into its value tree (`ToValue`) and walked recursively (depth cap 24, then a `…` text node). Each node becomes one of these widgets:

- Object: `tree_window_section_or_placeholder`, a collapsible section titled `label (count)`, open only at the root.
- Array: a section with one `tree_group` per row, each row holding the child editor plus a `Remove` tree item (`removeItem`). An `Add item` tree item (`insertItem`) appears only when the section is open. Row labels come from `list_element_selector`.
- Bool: `ui::select` with `true` and `false` options, bound to `setField` on change.
- Number: `ui::input(InputKind::Number)` whose value is the text `"<number> <unit>"`, bound to `setField` on change. The unit suffix is part of the text.
- String with `choices`: `ui::select` capped at 32 options (`choices.take(32)`, silently truncated). Bound to `setField` on change.
- String without choices: `ui::input(InputKind::Text)` with `commit("blur")`, bound to `setField` on commit.
- Bytes: a description item showing the byte count. Null: a description item showing "empty".

Labels come from field-meta (`en`/`de`) when present, otherwise from the last path segment. Layout is the raw document key order, not a domain grouping.

Why this is ad hoc:

- It is a generic key/value tree. Structural engineering inputs (members, sections, loads, load combinations) are not grouped, drawn, or related to each other.
- Units are inconsistent. Inputs show raw SI with a suffix (`"12.5 m"`, `"1200 Pa"`) in a number input. `format_quantity` in `🏷️ids-chrome-and-labels` shows results in kN, MPa, mm, and m, and it switches Length between mm and m at 1 m. The framework input builder already has `display_factor`, `precision`, `limits`, and `try_snap` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/⌨️input-toggle-and-select-builders`, `🧩️component/🎨️basic-widget-props`, `🧩️component/🎚️slider-geometry-and-key-handling`). The norm surface uses none of them.
- A number input whose text value carries a unit suffix is not a valid number value for the input element. This is a likely defect; it was not verified in the browser.
- The 32-cap is a framework limit (`UiFixedList`). en1991 works around it by copying the whole document into a second `GroupedEditor` struct and mapping paths back (`🏋️en1991/…/📥️inputs/🦀️.rs` and `🏷️field-meta/🦀️.rs` line 136). That is a second, hand-maintained document shape.
- No inline utilization or status is shown next to an input, so a failing check cannot be traced to the field that causes it.
- `value_json` travels as a JSON string inside the DSL payload (`set-field` payload), which is stringly typed.
- Each input change is a full document mutation (`bind_change`, `Trigger::Change`). The framework's draft lanes (`InputBuilder::draft_target`, `publication_revision`) are not used, so a slider or spinner drag would commit on every step.

### 2.2 Results window (`🌳️report-tree/🦀️.rs`, `render_report`)

One flat `PanelTreeBuilder` section, windowed via `window_section_or_placeholder`. It holds a `part` row per part (`part — complies (✓n ⚠n ✗n · u=x)`) and a `check` row per check (`n. [Status] title — subject @ path — computed vs limit u=x — clause`).

- `Fail` and `Warning` rows expand into an `Explanation` item and one item per remedy. A remedy is an `applyRemedy` tree item (`checkId`, `remedyIndex`), or for `OneOf` one `setField` item per option.
- Passing checks are single flat rows.
- Status is text only. There is no colour, bar, heat map, or sort.
- Numbers are shown with `format_quantity` (units inconsistent with the inputs, see 2.1).

### 2.3 Other editor panels

- Document panel (`render_summary`): one text line, `family — N checks: … ✓PART ✗PART … · u=max`.
- Inspection panel (`render_inspection`): one key/value tree item per field of the selected check, plus its remedies. The selected check comes from the results config.
- Catalogue panel (`render_catalogue`): examples as `setActiveExample` tree items, then reference tables (psi factors, gamma_I, partial factors, and so on) as tree rows whose text is `col=value · col=value`. Tables are not rendered as tables.

### 2.4 Viewer (`👁️viewer`)

The viewer has one window, `report`, built with `TableWindowKit` (framework `app::TableWindowKit`). It shows seven string columns (Part, Clause, Subject, Status, Utilization, Title, Remedy) from `report_table_columns` and `report_table_rows` (`🏷️ids-chrome-and-labels`). This is a read-only flat table, and it is the only tabular view in the norm stack. It does not share the editor's results tree, so the same report is presented two different ways.

Stale TS twins (all ten artifacts):

- Inputs twin declares `documentJson: string` ("pretty-printed JSON"). The Rust window renders a tree.
- Results twin (seen in en1997) declares `checks` with `computed`, `limit`, `utilization` as numbers and a status union `Pass | Fail | NotApplicable`. The Rust `CheckStatus` (`⚖️compliance/✅️check-results`) also has `Warning`, and the tree is text, not those fields.
- Viewer report twin declares `clause`, `status`, `utilization`, `message`. The Rust table has seven columns.

### 2.5 Other ad hoc or dead items

- `render_document_json` in `📝️document-editors` is labelled "Legacy pretty JSON — kept for tests". It is used only by tests. The label conflicts with the no-legacy rule.
- `NORM_LIST_VIRTUALIZE_THRESHOLD = 64` in `📝️document-editors` is declared and never used.
- `catalogue_markdown` in en1992 builds a Markdown string that nothing renders.
- The doc comment on `evaluate` (`🎮️commands/🧮️evaluate` in en1990) says "NormHost::from_document re-evaluates unconditionally". `from_document` does not exist, and the code uses a cache (see 3).
- Mojibake (`â`, `ð`) appears in header comments of two files in each artifact (for example the en1990 editor root `🦀️.rs`).

## 3. How Evaluation Is Triggered

Mechanics (from `⚖️compliance/🏛️norm-error-and-family`, `📇️registry/.../🧵️retained-commands`, `🪟️manifest-and-media`):

- `NormHost::from_artifact(doc)` reads `cached_report_for::<F>(doc)` and returns an empty `CheckReport` on a miss. It never evaluates.
- The cache key is `(family, document_revision_key(doc))`. `document_revision_key` serializes the whole document to JSON and hashes it. That runs on every `from_artifact`, and every paint calls it several times (results, inspection, document panel).
- The only command that populates the cache is `evaluate`. It is an interactive job with two progress stages ("Preparing evaluation", "Evaluating checks") and cooperative cancel. It emits zero mutations (`HostOnly` lane). The `export_media("report:out")` path also fills the cache on a miss.
- `set-field`, `insert-item`, `remove-item`, and `apply-remedy` commit document mutations. Nothing in the norm surface dispatches `evaluate` after them. After any edit the document revision changes, the cache misses, and the Results window (and inspection panel) renders "No checks computed." until `evaluate` is dispatched.
- No in-window control dispatches `evaluate`. It appears only in the manifest action list (`action_with(ActionDefinition::new("evaluate", …, ActionKind::View, …))`). Whether the framework re-dispatches View actions automatically after mutations is unverified; I found no such mechanism in the framework source.
- Tests hide this. `renders_the_computed_checks` (in every artifact's results window test) calls `host.evaluate()` on a local host before rendering. That call writes the process-global cache, so the test passes without exercising the real edit path.
- The viewer's `report` window calls `evaluate(document)` directly on every render, with no cache. It is therefore always current on re-render, while the editor reads a cache that can be stale or empty. Editor and viewer use different evaluation paths.
- `NormHost::apply` clears the report to default after a mutation, which matches the empty-results behaviour.

Result: results do not update immediately after an edit. They update only after an explicit evaluate, and the editor's default state after edits is an empty results panel. This is the opposite of the required "live re-evaluation on every edit".

## 4. Copy-Pasted Code Across the Ten Artifacts

Measured by normalizing the artifact number (`en1990` to `enX`, `1990` to `X`) and comparing files byte for byte with en1990:

| Taxonomy group | Pairs compared | Identical after normalization |
|---|---|---|
| `🎮️commands` (6 files each) | 72 | 49 |
| `🎭️modes` | 90 | 62 |
| `👁️viewer` (all files) | 72 | 63 |
| `👁️viewer/🦀️.rs` (viewer root) | 9 | 9 |
| `🧪️tests` | 119 | 66 |
| `🎚️config` and `👥️presence` | 18 + 18 | 18 + 18 |
| `✏️editor/🦀️.rs` (editor root) | 9 | 0 (differs by 12 to 54 lines: examples, command macro arity, catalogue windows, description text) |
| `🏷️field-meta` | 9 | 0 (data differs, structure repeated) |
| Windows and panels (`🪟️windows`, `📌️panels`) | 54 + 54 | 26 + 26 |

Totals: of 16,772 Rust LOC across the ten artifacts, about 5,100 LOC (the nine non-en1990 copies, roughly 510 to 605 LOC each) are byte-identical to en1990 after normalization. The duplication is larger than that, since some windows and panels differ only in one or two lines.

Specific repeated code (candidates for generation or the shared surface):

- Editor root manifest (`create_*_app`): the same six `action_with`/`action_interactive_job`/`action_describe` blocks with the same argument shapes. Only the strings differ. Also the same `app_commands!` enum, the same `render` match, and the same `export_media`/`import_media` forwarders.
- Viewer root (`Viewer::builder`, `render` match): identical in all ten.
- Six command files per artifact (`set_field`, `insert_item`, `remove_item`, `apply_remedy`, `selected_check`, `evaluate`). Each is a payload struct plus a one-line call into the shared helper.
- Inputs and results windows: `WINDOW_*`/`BODY_*` constants, `definition()`, and a one-line `render` wrapper. Only the window id and icon differ.
- Inspection panel (32 lines), document panel (`render_summary` wrapper), and catalogue `render` wrapper: identical across the ten.
- Field-meta helpers: `const fn choice` is repeated in 10 files and `fn m` in 4. The `choice` helper and the `*_CHOICES` constants repeat across artifacts (ANNEX, EXPOSURE, STRUCTURE, and so on).
- Catalogue helpers: `psi_columns`, `psi_row`, `psi_table_en`, `psi_table_de`, and `importance_gamma_i` are repeated in en1990 and similar tables recur in other artifacts. Only the values differ, and they belong in the checks crate.
- Test scaffolding: `context` module, `history_edit_acceptance_law!`, `retained_command_dispositions_match…`, `every_command`, and the `renders_the_computed_checks` test in every results window.

Inconsistencies that show the copies have drifted:

- en1991 and en1995 pass the catalogue without `TreeWindows`. The other eight pass it.
- en1997 and en1998 put field-meta somewhere other than `✏️editor/🏷️field-meta`.
- en1995 uses a localized `reference_tables(locale)` signature. The others use static tables.

## 5. What a Rich, Immediate, Real-Time Editor Needs

### 5.1 Already provided by the framework (not used by the norm surface)

- Widgets: slider, number stepper, progress, table, tree with windowing (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component`), number scale with `display_factor`, `precision`, `limits`, and snapping.
- Draft lanes for live drag: `InputBuilder::draft_target` and `publication_revision`. The norm surface commits every change instead.
- Interactive jobs with progress, cancel, and checkpoints (`ArtifactCommandWork`). The norm `evaluate` uses them, but only for explicit evaluate.
- Undo and redo keybindings (`mod+z`, `mod+shift+z`) are already declared.
- Presence and transient slots (`NoPresence`, `NoTransient` today) for shared, multi-user state.
- Media port `report:out` (structured JSON of the report), already implemented.
- 3D: world3d scene and snapshot (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌐️world3d`, `🌍️world3d-snapshot`), `Viewport3dOrbit` and the viewport module (`🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d`). The FEM plugin already uses them (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/🎬️scene/🪟️viewport`).
- 2D: canvas2d snapshot (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot`), `◻️2d` compute (`🧰️framework/🔨️modules/◻️2d/🧮️compute`). The norm handlers receive `EngineHandles` and ignore it.
- Node graph, gestures, framing, and scene modules (`🕸️node-graph`, `🖐️gestures`, `📷️framing`) are available in the UI module.
- Chart snapshot exists only in the print product (`🧰️framework/🛍️products/📓️print`), so it is not yet a UI chart node.

### 5.2 Missing and needed

Real-time evaluation:

- Re-evaluate after every mutation through a cancellable, debounced job, with a draft lane for drags.
- Make the cache revision cheap. Replace the full-JSON hash on every paint with a revision counter from the event log.
- Keep the editor's results derived from the same path as the viewer, so there is no stale cache.
- Fix the stale `from_document` comment and remove the test that writes the cache before rendering.

Units and entry:

- One display-unit policy shared by inputs and results (`display_factor` plus `precision`), with a unit selector where the norm allows it.
- Number inputs that never carry the unit suffix in the value text.
- Replace `value_json` strings with typed payloads.

Domain layout instead of generic trees:

- Group inputs by domain (geometry, sections, materials, loads, combinations, annex) using a schema, not a copy of the document.
- Remove the en1991 `GroupedEditor` projection and the 32-cap workaround. Use a windowed or paged container from the framework.
- Show utilization and status next to each input that feeds a check.

Visualization:

- Results as a table or heat map using the framework table and progress widgets.
- Utilization bars per check and per part.
- Diagrams per standard: M-N interaction curves, stress blocks, S-N curves, spectra, and similar.
- 3D: members, sections, joints, foundations, load paths, and utilization colouring, using the world3d scene and the FEM viewport precedent.
- Viewer and editor should share the same results renderer.

Multi-user and state:

- Presence for collaborators (currently `NoPresence`).
- Clear separation of persisted local, persisted shared, ephemeral local, and ephemeral shared state, as the brief asks.

Cleanup needed before building on this:

- Refresh the TS twins for inputs, results, and report (section 2.4).
- Remove the dead helpers: `render_document_json` (legacy), `catalogue_markdown`, `NORM_LIST_VIRTUALIZE_THRESHOLD`.
- Fix the mojibake header comments.
- Make the ten artifacts share one generated editor and viewer root, with per-standard data only (section 4).
- Align field-meta location (move en1997 and en1998 into `✏️editor/🏷️field-meta`) and the catalogue signature (`TreeWindows` in all ten).

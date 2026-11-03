# Shared v5 (wave 2)

Nothing compiled (no cargo). Python run: harness dry run, bless of the new section cases, one pytest law. "ready for rebuild: shared v5" appended to integrate/latest.md and build-en1990..99/latest.md. Paths relative to `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface` unless marked.

## 1. Shortcuts execute
- Chain: key -> `dispatch_shortcut` -> `norm.shortcut.<action>` -> `command_from_action` (macro `live`, new arm `shortcut_view_act`) -> `ViewportAct{viewportStepCheck|Governing|Combination, number ±1}` -> `view_event` -> `LaneEvent::ViewStep{step,forward}` -> resolved in `live_ephemeral` against the live report and scene (`🎹️view-steps`: `resolve_view_steps`) -> `ViewCheck(i)` / `ViewTrack+ViewSeek+ViewItem` -> `NormTransient.view` (new field `ViewLane.check`; rs, proto, ts, graphql twins by hand).
- `setSelectedCheckIndex` mirrors into the lane (`ViewCheck`), `LiveView::with_lanes` lets the lane check win, so selected check, 3D selection (entity of the check) and checks window follow. Camera: `toolbar::lowered_viewport` frames the selected entities when the lane holds a check.
- Help overlay: `toolbar::render_shortcut_help` (closed section in the viewport panel, generated from `shortcuts()`, EN/DE labels, chords, mac glyphs).
- Fixed a v4 layering bug: `step_combination_event` lived in live-core but used `accessibility` (visual-chrome); removed, logic now in panels.
- Tests: `🎹️view-steps/🧪️tests` (six: key, event, view state for `]` `[` Alt+G Alt+Shift+G `.` `,`), live-lanes step intent test, viewport-panels camera and help tests, feature file.
- Not done: viewer apps (no editor commands) do not execute the steps; combination stepping needs a scene `Combination` track (en1990 offers one, others do nothing).

## 2. Screen reader
- `♿️accessibility/📡️published-nodes`: store keyed by window role and artifact domain, builders `viewport_nodes` (list, option per entity, slider per handle via `handle_node` with `rest_parameter`), `diagram_window_nodes` (`diagram_nodes`), `window_scope`. `check_window_nodes` (table + `check_row_node` per listed row, selected state) in the checks window. Renderers publish; `NormShellHooks::accessible_nodes(scope)` serves them by window id.
- `📡️node-contract::assert_default_document_nodes` + one test per artifact editor (`...default_document_publishes_an_accessible_node_for_every_check_handle_and_diagram`, appended to the ten `✏️editor/🧪️tests/🔬️unit`), EN and DE.
- Limits: slider value of handles whose kind is not along-axis/angular is the lower limit or 0; node names follow the render locale.

## 3. NDP check parameters
- `🏳️check-parameters`: inspector section "Check parameters (n)" from `check.ndps`: symbol, clause, `[verified|unverified]`, id, annex and value (EN/DE). Factor-override link by convention (`...psi.<cat>.<n>` -> psi<n> on category, symbol γ -> gamma, xi, k_fi; last matching row); the row activates `setSelectedCheckIndex` of the governing check of that override row. `InspectorOptions.check` set from `NormSelection::inspector_options`. Existing "Annex parameters" section kept. Tests in the module.

## 4. Oracle adapters (Python, run)
- `🔮️oracles/🏃️harness/🔌️adapters/sectionproperties`: checks `mechanics.analyse-section` (rings, nu, cells; Rust `SectionConstants` field names) and `mechanics.analyse-thin-walled` (centreline walls as united solid strips; comparison needs a case tolerance of order t/l, set 0.1 for J). Pure-Python `🧭️dry-run` validates geometry (rings, holes inside, open connected wall tree) and returns closed forms (area, J); harness `--dry-run FILE...`, `dry_run()`; pytest law added and green.
- Cases `🧪️tests/🧫️fixtures/🔮️oracle-cases/section-mechanics.json` blessed with the repo `.venv` (rectangle nu 0.3, sharp RHS, channel walls). Rust driver adapters feeding these to `analyse_section` / `analyse_thin_walled` are not written.

## 5. Coordinator item
viewport-config was already listed in the contract umbrella packages and build-owners; `verify build-owners sync` passed (0 changes). No "Bounded physical Source revision" error.

## Risks (unverified by rustc)
`UiFixedList` capacity for the 41-row help; `Vec` into `ui::column().try_children`; slice pattern in `override_key`; `crate::selection_mirror::governing` visibility; `NdpEntryAnnex` import path; `selected` on `AriaNode` rows; new `ViewLane` field in any hand-built literal elsewhere (none found by grep).

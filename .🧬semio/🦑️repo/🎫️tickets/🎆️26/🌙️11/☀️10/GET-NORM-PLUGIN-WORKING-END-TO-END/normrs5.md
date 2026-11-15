# normrs5 - norm Rust workspace check/test (non-artifact crates; artifacts owned by wave agents)

## Status (11:40)
Check: `repo cargo ... check --all-targets --keep-going` on the whole norm workspace: 0 errors (logs `normrs5-check7/9.log`, packages re-checked after each later change: `checkp3`).
Tests, non-artifact crates (`-p 'semio-s-plugin-norm-*'`, log `normrs5-testn4.log`, tally `python3 T/normrs5-tally.py <log>`): 788 passed, 62 failed.

| crate | pass/fail |
| --- | --- |
| compliance lib / `unit` | 44/0, 29/0 |
| design-case (new) | 25/1 (python oracle) |
| compliance-core | 7/4 (all python oracle) |
| i18n-census (new), themes (new) | 6/0, 8/0 |
| structural-scene lib / unit | 74/0, 19/0 |
| structural-scene-case-kit | 39/3 |
| structural-scene-foundation | 27/1 |
| structural-scene-meshing | 25/6 (4 python, 2 tessellation volume) |
| mechanics: fastening, fibre, geotechnics, moment-line, section, unit, latency | 4/1, 5/3, 5/5, 3/1, 11/2, 22/1, 3/0 (almost all python oracle; 2 fibre numerics) |
| results-config, viewport-config | 4+2 / 0, 4+1 / 0 |
| app-surface root | 47/1 |
| edit-rules | 5/1 |
| entity-editing, foundation, interaction-topology, live-editor, shell-hooks, viewport3d | all pass (37, 8, 3, 23, 24, 44) |
| live-core | 65/3 |
| outliner | 8/1 |
| panels | 37/6 |
| panels-inspection | 54/10 |
| visual-chrome | 55/4 |
| inspector-forms | 11/8 (the 8 `paid::static_source` tests) |

Failure classes of the 62:
- 23: no Python interpreter (`SEMIO_PYTHON` unset, no `.venv`). `repo dependencies ensure` refuses (stale receipt: `node_modules/@semio-tech/course` changed); needs `repo dependencies sync --all` (not run: installs the whole workspace).
- 14: cascade, "UI arena already has an actual lifetime owner": a UI test that panics while holding `UiArenaFixture` leaves the process-wide registry owned (the fixture forgets its lease on panic). They pass once the real UI failures below are fixed.
- 8: inspector-forms `paid::static_source` (see below).
- 17 real test or behaviour drifts, not fixed (diagnosed):
  - outliner `unsupported_list_controls...`: `ui.tree-window.duplicate-key "norm-inputs-unsupported"` (two windowed containers share a key; source fix in the outliner renderer).
  - panels `a_viewport_action_changes_the_rendered_state` (no "Fire duration: 30 min"), `the_keyboard_help_lists_every_shortcut...` (`Focus(Outliner)` shortcut missing from the expected list).
  - app-surface `an_absent_row_is_addressed_by_its_id...`: `address_pending` returns `None` for a missing case id; edit-rules `declared_companion_arguments...` (10 vs 2 payloads).
  - live-core `a_pointer_drag_previews...` (state is not `Hover` at 4.27) and `evaluation_is_debounced_per_tier...` (no T1 evaluation).
  - panels-inspection `a_multi_selection_shows_the_common_value...`: "UI arena descendants remain borrowed at lifetime closure".
  - visual-chrome accessibility slider `value_text` ("3.. m"); diagrams2d `format_number` now prints 4 significant digits ("1500" not "1500,5") so the table-twin test and the golden layer fixture disagree with the code (decide the rule, regenerate golden).
  - mechanics fibre: rectangle fibre integration of a linear law is 0.04 % low (midpoint rule without the fibre's own inertia) and the balanced-strain moment differs by 3e-6 from the oracle.
  - case-kit: `no primitive dimension follows the width input b by the factor 1.25` (forms) and the profile-forms oracle (Prism).
  - meshing: shell/weld volumes 3e-6 off the closed form (segment count at `rmax` vs the test's radius).

## Generated inputs created graph-free
- `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/*` (normcomp, via `renderCatalogArtifacts("rust")`).
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` (+ `.py`): `T/normrs5-generate-styling.ts` calls `generateStylingArtifacts(writeGeneratedFileIfChanged)` via `repo tools exec --light bun -- run`. Producing target: `@semio-tech/ui-styling-tokens:generate`.

## Fixes
Framework:
- `plugin/📜️declaration/🧩️app/🛬️lifted/🎬️media-export`: `has_failure()` -> `failure_untaken()` (migration).
- `plugin/🧵️instance/👷️jobs/📥️staging/🏷️reserved`: removed the duplicate `handoff` (merge duplicate; kept the Option-slot sized version).
- `plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder`: `observed` converted to `u64` for `receive_observed_work`.
Norm:
- `visual-windows/🎨️themes`: `ramp_for` covers `ResultField::Airflow`/`HeatFlow` through the Cool/Warm ramps (same mapping as `ResultField::ramp`).
- `panels` tests: dropped the `Gesture`/`commit_writes` assertion that needs live-editor (cycle) and moved it to `live-lanes/🎛️editor-wiring` unit tests; `playback` import.
- `app-surface` unit tests: `collect()` removal for `DslValue::object`, `diagrams2d::marker_cell_id`, `WindowLayoutChild` import.
- Source-boundary gate (cap 4000 active lines / 256 KiB per crate in test cfg, found by `cargo test` only):
  - visual-chrome 4426 lines: `i18n-census` moved to own crate `semio-s-plugin-norm-i18n-census`, `themes` moved to `semio-s-plugin-norm-app-surface-themes`.
  - compliance 4698 lines + 2.7 MB `include_str!` of the NDP JSON: new crate `semio-s-plugin-norm-design-case` (design-case + spec-trace, re-exported from compliance under the same paths), the `unit` tests became an integration test target (`[[test]] unit`), NDP JSON read at runtime, `serde` added as dev-dependency.
- compliance tests: NDP oracle uses `entry.verification(AnnexChoice::En)` (the builder's annex) instead of the base status; mock units declare `sections.*.id` reads.
- compliance `incremental-evaluation::EvaluationSession::update`: planned units whose read hash drifted are now marked stale, so a cancelled run followed by a retry plan still reruns them (root cause of `incremental_equals_full_on_random_mutation_sequences`).

## More fixes in this pass
- `design-case`: `declared()` dropped vector inputs (the lookup refused a sequence base key), now returns vectors by base key; surface test count 4.
- `scene` schema: `field-or-layer` keyframe rule moved to the track and waived for combination tracks; scene-kit test sets `detail` and a German description; diagram2d basis-note test; scene delta test; wire-shape test; mechanics twins (`MechanicsText` in proto/graphql/ts) and the feature-table tests (explicit tables); results/viewport config no-op inverse; device-profile layout counts (5 windows) and mobile handle drag (FocusHandle); themes schema path; i18n census (blank pair is not a finding, findings in source order).
- 50 UI tests got `UiArenaFixture::new()` (script `T/normrs5-arena-insert.py`), dev-dependency `semio-ui-contract-values` (testkit) in 6 app-surface crates.
- Source-boundary gate predictor: `T/normrs5-closure.ts` (run with `repo tools exec --light bun -- run`) prints files/bytes/lines per crate; mechanics and structural-scene tests were moved to integration targets to fit the 32 files / 256 KiB / 4000 lines cap.

## static_source (8 tests)
Cause pinned down: the test harness drives `NormInspectorStaticSourcePreparation::advance` (Phase::Read, which calls `JsonNativeInputCursor::advance` and `NativeReceivingOriginal::advance_once`) under `session_nested`. The cursor's local actions open original Work actions (`OriginalWork::prepare` needs `state.scope` and an issued quote, `🎟️work-action/⚙️operation`), which only exist inside an effect (`Authorities::step_effect` / `with_original_work_loan`), but `session_nested` only binds the loans. Fix to build: a `session_step` in `🌱️value/🧪️testing/📥️admission/driven.rs` exposing `Authorities::step_effect` (it hands out the wired control plus the effect), and use it for the Read/Receive turns in `paid/static-source-tests.rs::Owner::receive_under`. Not built (needs several cold test cycles).

## Open
- python oracle tests need `SEMIO_PYTHON` (`repo dependencies ensure`).
- Whole-workspace `cargo test` source-boundary gate may hit more crates; each needs a domain split, not a raised cap.

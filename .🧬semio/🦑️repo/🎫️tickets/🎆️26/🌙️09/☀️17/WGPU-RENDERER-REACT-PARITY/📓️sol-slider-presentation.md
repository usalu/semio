# Slider Presentation and Pointer Geometry

## Scope

This packet aligns the retained Slider's default React presentation: a fixed numeric readout cell, token-sized muted rail, semantic filled range, 16 px thumb, and pointer mapping restricted to the track cell. The row may inherit RTL placement while the Slider's value axis remains explicitly LTR. A unit-bearing declarative Slider additionally reserves one measured, shrink-to-content sibling for React's external `value unit` label.

The language-neutral contract is:

- schema: `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🎚️slider-presentation/🔣️.json`
- fixture: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎚️slider-presentation/🔣️.json`

It covers minimum, fractional middle, maximum, and outer-RTL cases. It fixes the shared compact tokens at a 22.4 px row, 28.8 px readout, 3.2 px rail, and 16 px thumb, plus track/value-cell pointer cases. The same fixture owns standalone-LTR and inline-Tree-RTL unit placements, the 3.2 px outer gap, and the three distinct labels: internal `2.5`, external `2.5 mm`, and spoken `2.5 mm`.

## Implementation

`layout::slider_presentation` is the inner geometry authority for paint and input. `layout::slider_control_presentation` first splits the accepted bounds into the growing Slider and measured unit sibling, following the outer LTR/RTL flow. The inner helper then places the readout and track cells and resolves range and thumb percentages from the physical left of the track.

Both retained and immediate test painters use the helper. The rail uses `theme.muted`; range and thumb use `theme.text_element`. The internal numeric readout comes from `format_ui_number` and never includes the unit. Retained paint prefers an in-progress edit string, then a local slider draft, then the declared value. An edit uses the unscoped 12.8 px input font and ordinary text color; a compact Tree rest uses its scoped 9.6 px display font and element color. The measured external sibling uses the declared value and muted body typography, matching React's wrapper rather than the Slider's live local draft.

Mounted layout shapes the external label once and publishes its accepted suffix width with the accepted frame. Paint, ordinary pointer commit, and double-click hit testing consume that same accepted width. Pointer commits first exclude the suffix and readout, require `track_cell.contains(x, y)`, then call `slider_value_at` with that track cell. The readout remains available to the editing packet's double-click router; the external unit sibling stays inert.

The React Slider now forwards an explicit `aria-valuetext` to its semantic thumb. The declarative Interpreter derives that value from the shared accessibility projection, so a unit-bearing value announces `2.5 mm` while the Slider's own visible readout remains `2.5`. WGPU uses the same formatter for its live accessibility value and external label; a draft `4.5` therefore announces `4.5 mm` while the authored external sibling remains `2.5 mm` until the declarative state catches up.

## Independent oracles

The production React Slider test validates the schema with Ajv 2020, mounts the real component under an RTL parent, asserts its explicit LTR slider root and token classes, verifies the formatted readout and spoken value, proves a readout pointer press is inert, and maps the fixture's track coordinate.

```text
Test Files  1 passed (1)
Tests       1 passed | 12 skipped (13)
Duration    46.83s
```

The actual declarative Interpreter oracle mounts `renderUiControl` with a unit-bearing Slider and proves the internal numeric readout, external value-unit sibling, and `aria-valuetext` are distinct.

```text
Test Files  1 passed (1)
Tests       1 passed | 702 skipped (703)
Duration    101.95s
```

## Native laws

The native laws awaiting the root-owned native queue are:

- `slider_cells_range_and_thumb_match_the_shared_react_geometry`
- `slider_unit_is_a_shrink_to_content_sibling_outside_the_inner_track_and_readout`
- `slider_pointer_mapping_is_confined_to_the_shared_track_cell`
- `slider_external_unit_sibling_is_inert_and_never_enters_track_value_mapping`
- `slider_paints_the_shared_rail_range_thumb_and_numeric_readout`
- `painting_a_slider_keeps_its_numeric_readout_independent_from_the_external_unit_sibling`
- `inline_tree_slider_paints_the_same_external_unit_sibling_and_preserves_its_accepted_width`
- `compact_slider_edit_uses_the_unscoped_input_font_while_its_resting_readout_stays_compact`
- `an_editing_unit_slider_projects_its_live_spoken_value_and_numeric_spinbutton`
- `the_engagement_body_paints_reacts_control_row_with_reacts_ids_and_intents`

Rust sources were formatted successfully through the Nx-routed formatter. No native pass is claimed until the root-owned grouped run completes.

## Lowering closure

All existing WGPU Slider producer paths now preserve `unit`: direct retained components, action arguments, staged Tree controls, and the Shell engagement primary control. The retained Slider remains one semantic control node; the external sibling is a measured and painted presentation segment of that node, so standalone and inline Tree controls share one accepted geometry without weakening Tree's typed control slot.

# Section and Field Geometry Parity

## Scope

This packet ports the actual React `Section` and `Field` vertical presentation to the retained WGPU layout and paint pipeline. It owns only the generic Section/Field chrome geometry. NumberStepper presentation and Shell-specific settings composition remain outside this packet.

The language-neutral contract is:

- schema: `🧰️framework/🔨️modules/🖱️ui/🧬️schema/📐️section-field-presentation/🔣️.json`
- fixture: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️section-field-presentation/🔣️.json`

The fixture fixes the shared compact-density metrics and one wrapped example:

- Section title: 21.6 px semibold, 30.4 px line height, 16 px title/body gap, 32 px trailing margin.
- Field: 19.2 px clipped label line, 3.2 px gaps, 16 px wrapped description/error lines, and the control's own 22.4 px height.
- At 140 px the Section title wraps to two lines and the content begins at 76.8 px.
- At 112 px the Field control begins at 57.6 px, error at 83.2 px, and the field hugs 115.2 px.

## Implementation

The styling source now owns `text2xlPx` and `text2xlLineHeightPx`; the existing generator publishes the values to Rust, TypeScript, Python, and CSS token artifacts.

The WGPU flex model now carries measured top and bottom chrome on `Field` and `Section`. Shared `section_chrome_metrics` and `field_chrome_metrics` functions calculate every band so intrinsic measurement, accepted layout, immediate test paint, and bounded retained paint use the same arithmetic.

The mounted layout job admits Section title and Field label/description/error text into bounded glyph ranges. It measures wrapping from retained glyph advances, updates composite chrome before intrinsic measurement, and recomputes it at the accepted owner width before arrange. A Section without an accepted first child recovers its wrapped title height from the accepted owner.

The retained painter uses the accepted bands:

- Section paints only its wrapped 21.6 px semibold title in the title band.
- Field paints a clipped semibold label, optional required marker, wrapped muted description, then the retained child control, then wrapped error text.
- No text band overlaps the child control.
- A Field error-only change now dirties layout as well as paint.

The existing retained Section disclosure state remains intact; the chevron no longer participates in Section title paint geometry.

## Independent oracle

The actual React oracle is in:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx`

It compiles the neutral schema with Ajv 2020, mounts the production React `Section` and `Field`, and asserts the production token classes and DOM order. It also evaluates the neutral geometry equations independently of Rust.

Receipt:

```text
Test Files  1 passed (1)
Tests       1 passed | 7 skipped (8)
Duration    26.52s
```

Log: `🗑️generated/sol-section-field26/react-oracle-long.log`

Generated styling-token verification:

```text
framework/ui/styling: generated artifacts are fresh
NX Successfully ran target check-generated for project @semio-tech/ui-styling-tokens
```

Log: `🗑️generated/sol-section-field26/styling-check-generated.log`

## Native laws

The focused WGPU laws are:

- `section_and_field_measure_the_shared_wrapped_chrome_fixture`
- `section_and_field_chrome_paint_only_inside_their_measured_bands`
- `changing_only_a_field_error_dirties_layout_for_the_new_error_band`
- `a_field_reserves_its_label_band_and_its_control_keeps_its_own_height`
- `a_section_stacks_children_below_its_header_at_their_own_height`

Root launched these in native session 99626. Its receipt belongs at `🗑️generated/astra-runtime/settings27/border-section-green.log`; no result is claimed here until that session completes.

## Regression correction

The first focused native rerun proved that flex-growing a Field's child was the wrong interpretation: an accepted Field that is taller than its intrinsic content must keep the child control at its own 22.4 px line. The flex law now constructs a real Control leaf and asserts that height.

The paint overflow was caused by two implementations of the same text measurement contract disagreeing. The bounded worker prices ASCII at `fontSize × 0.625`; the dependency-free bitmap atlas previously used a fixed 10 px advance at every requested size. At the fixture width this measured the description as two lines but painted it as three. The bitmap atlas now uses the worker's size-relative advance, preserving every glyph and the common wrap points. A temporary broad vertical glyph clip was removed because it would have hidden valid text rather than repairing the measurement authority.

## Validation boundary

No browser journey was run in this packet. Root owns the shared WGPU build and browser acceptance session. Rust sources parse through rustfmt; the shared files contain pre-existing/concurrent formatting drift, so the check-mode formatting log is retained only as a diagnostic and is not recorded as a passing formatting gate.

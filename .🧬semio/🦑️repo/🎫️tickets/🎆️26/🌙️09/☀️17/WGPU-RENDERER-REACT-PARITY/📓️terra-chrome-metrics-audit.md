# Terra Chrome Metrics Audit

Read-only audit of the checkpoint-16 geometry receipt and the current React and WGPU sources. This supersedes the earlier inference that the absence of `s-presence-peers` from the WGPU hit registry means that its text was not painted.

## Confirmed repairs

### The example picker loses React’s explicit 192 CSS-pixel minimum

At the 1600 × 1000, DPR-1 boot receipt, React paints `playground.navbar.fixture` at `[706, 3.1875, 192, 22.390625]`; WGPU reports `[750.135, 3.2, 110.179, 22.4]`. The 81.821-pixel maximum error shifts the whole centered navbar cluster. React's computed receipt identifies the selected value as `Concrete Forest`, with `12.8px / 19.2px Anta`.

This is a direct implementation difference, rather than a font fallback hypothesis:

- [`NavbarExampleSelect`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx#L65) gives the trigger `w-full min-w-[12rem] max-w-md`; at the default root scale `12rem` is the observed 192 pixels.
- [`shell_example_control`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L17243) creates the same id as a plain `ShellNavbarControl`, whose fields have no width policy ([`ShellNavbarControl`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L17161)).
- WGPU then computes the center reservation through `navbar_control_band_width` ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L23074)) and the painting rect through `retained_chrome_group_item_width` ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L16233)); both use only label advance plus icon and padding.

The narrow repair is a width constraint on the shared navbar-control presentation, applied to the fixture-select variant by `shell_example_control`, and used by both the reservation and paint paths. Do not add a post-layout x-offset: `navbar_center_width` is intentionally the single centered-band reservation ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L23085)).

The neutral regression law should use the existing `navbar-centered-band` fixture: with the same selected short label and a 1600-pixel viewport, the React computed trigger rect is 192 pixels wide and the WGPU rect is the same within one CSS pixel; the center-band origin must be recomputed from that width. A second, long label should prove the existing `max-w-md` cap rather than encoding one label’s glyph advances.

**Confidence: high.**

### The task-manager tab uses different localized product text

The same boot receipt records React `os.task-manager` as `Tasks`, `[1528.5, 974.40625, 68.3125, 22.390625]`, versus WGPU `[1405.188, 974.4, 113.625, 22.4]`. This 123.813-pixel maximum error moves the entire bottom-right dock band.

- React creates that exact id using `shellLabel("ui.panelToggle.taskManager")` ([`ChromePanels`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx#L1443)). The English bundle value is `Tasks` ([`ui React target`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx#L3541)); the German value is `Aufgaben` ([`🟦️.tsx`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx#L2655)).
- WGPU creates the same id with `shell_chrome_string("taskManager.title", …)` ([`Shell`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L8774)), where the strings are `Task Manager` and `Task-Manager` ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L27446)).

Align the WGPU chrome label with the React `ui.panelToggle.taskManager` value in both locales. Keep the longer `Open Task Manager` command label separate; it is a different affordance. The neutral oracle should assert the id, text, and rect for English and German from the same locale fixture before deriving the band rect. It should not assert only the geometry, because an accidental truncation could produce a matching width with wrong accessible text.

**Confidence: high.**

## Measurement correction: presence is painted but is deliberately noninteractive

The observed WGPU image contains `No one else is here`. Its earlier “absent” geometry row was a hit-registry limitation:

- the footer lays out a presence rect and calls `render_footer_status_step` with `interactive = false` ([`Shell`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L24629)); the generic painter still paints the label before its optional hit phase ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L16265)).
- Chrome accessibility derives nodes exclusively from `input.hits()` ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L29297)), and the generic painter records the text name and hit only when `register_hit` is true ([`🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L16328)). Therefore neither the current hit receipt nor AX projection exposes this painted noncontrol’s rect.
- The current physical probe already explicitly excludes this one key when one side lacks a hit rect and says that hit absence does not establish missing paint ([`parity-interact-probe.mjs`](../🐍️parity-interact-probe.mjs#L56)).

Do not make presence interactive merely to satisfy geometry collection. Until a separate painted-text rect channel exists, exclude it from hit-based geometry acceptance and retain a pixel or text-layout receipt for it. A future diagnostic should publish noninteractive chrome text as `key`, `text`, and `rect` from the completed painter, separate from controls and accessibility actionability; it must not reuse `HitTarget` or an actionable AX role.

**Confidence: high.**

## What the receipt does not establish

React’s reported regular chrome font is already `500 11.2px / 11.2px Anta`; the WGPU theme uses the same design size. The remaining 0.2–0.4-pixel coordinate and height deltas can be ordinary fractional layout rounding. This audit found no source evidence that a global font or baseline change would resolve the two material 82- and 124-pixel failures, so it recommends neither.

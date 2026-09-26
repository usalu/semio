# Fullscreen and Utility Pressed Accessibility

## Contract

The shared `wgpu-accessibility-interaction` fixture and schema now define the exact fullscreen, utility Button, utility Toggle, and utility Collection hit ids and labels. Fullscreen and utility Toggle are ordinary buttons with `aria-pressed`; Button and Collection retain button roles and accessible labels without a pressed state. The fixture covers both `true` and `false` values.

## Implementation

The retained utility painter now authors presentation records on the concrete ids it registers as hits: `framework.utility.button.{id}`, `framework.utility.toggle.{id}`, and `framework.utility.collection.{id}`. It stages the presentation only after the retained paint completes, beside the accepted-frame hit. Utility Toggle and `ui.fullscreen.toggle` publish role `button` and their painted active value as `pressed`. The rule is intentionally scoped and does not alter panel, mobile, pane, or Collection expanded semantics.

The test-only utility painter uses the same ids and Toggle hit kind, preventing its accessibility projection from masking production drift.

## Oracles

The React accessibility suite mounts the production React `Toggle` for the fullscreen and utility fixture rows. It compares native button role, computed accessible name, `aria-pressed`, and absence of `aria-checked`.

The native law `fullscreen_and_utility_painters_publish_reacts_pressed_buttons_on_their_actual_hit_ids` walks the retained chrome painter for fullscreen and the production retained utility painter for Button, Toggle, and Collection. It asserts exact hit identity, visible label, button role, and pressed-state scope.

## Validation

- Fixture and schema parse successfully with independent JSON parsers.
- Focused React command: `SEMIO_TEST_LEVEL=long NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- --run <absolute accessibility interaction test path>`.
- Focused native law is queued in the root-owned Shell run.

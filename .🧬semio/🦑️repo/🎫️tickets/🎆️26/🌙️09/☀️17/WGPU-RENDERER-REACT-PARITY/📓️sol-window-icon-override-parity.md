# WGPU Per-Instance Window Icon Override Parity

## Result

The WGPU shell now owns a bounded renderer-local icon override for each exact mounted app/window instance. An accepted host command can update any live instance, including two windows of the same kind, without changing its sibling. Dock presentation resolves `override -> projection-derived icon -> declared kind icon`.

The map stores the app plugin and app-instance token beside the window id. Commands for missing windows and empty or over-capacity identifiers are refused. Dock reconciliation retains only the current app instance's live window ids; the direct close lane and World3d retirement lane erase the same owner. Reusing a retired window id therefore starts from projection or kind fallback.

World projection selection, split creation, and accepted dock drop now publish through the generic command when their template has a canonical icon. The existing template map remains the first-mount fallback.

## Neutral contract

- Fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🪟️window-icon-overrides/🔣️.json`
- Schema: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🪟️window-icon-overrides/🧬️schema/🔣️.json`
- React oracle: `applies the neutral per-instance icon sequence through Reacts SET_WINDOW_ICON reducer`
- Native law: `accepted_window_icon_commands_are_instance_scoped_precede_projection_and_retire_before_id_reuse`

The corpus covers two same-kind instance overrides, replacement of one instance, arbitrary override precedence over a valid projection icon, projection and kind fallback, rejected commands, retirement, and id reuse.

## Validation

Focused React/Ajv oracle:

```text
NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --run --silent=false --reporter=verbose -t 'applies the neutral per-instance icon sequence through Reacts SET_WINDOW_ICON reducer'
Test Files  1 passed (1)
Tests  1 passed | 700 skipped (701)
Duration  14.38s
```

The root runner owns the focused native and WASM verification because Cargo builds are coordinated centrally in this shared checkout.

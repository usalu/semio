# Table Row Button Accepted Accessibility

## Neutral Contract and React Oracle

The dedicated `Table/🧬️schema/🔘️button-accessibility` contract describes the app-backed `Werkstatt Ada` row. Its actions cell contains one visible `Open: Werkstatt Ada` row button and one `Archive: Werkstatt Ada` menu-only action. The expected retained control fixes the stable host/row/column/ordinal key, button role, accessible name, focusability, tabbability, and unmodified app descriptor.

The actual `TableHost` validates that fixture with strict Ajv, mounts only the row-placement action, admits it into keyboard focus, and sends the exact descriptor on activation. The actual browser accessibility mirror consumes the expected projection and sends one fully addressed `accessibility-activate` event. The existing app conformance law remains the independent end-to-end React authority for moving from the `Werkstatt Ada` row into the same named button.

## Accepted WGPU Presentation

The WGPU Table now stages a `TableButtonAccessibilityCell` for each visible row-placement button. It uses the same row, column, drag reservation, scroll, and segmented-cell derivation as paint and pointer hit testing, clips partially visible row geometry to the body, and excludes menu-placement actions. The stable key is `{host}.row.{rowId}.{columnId}.{rowButtonOrdinal}`. The label is exactly the authored button label; an icon is never substituted as a name.

The shell seals, acknowledges, and discards these candidates with the same presentation epoch as pixels and hits. The interpreter publishes accepted entries as focusable, tabbable virtual buttons. Activation requires the current window generation, node hash and key, visible nonretiring retained Table host, live row/column/row-placement ordinal, unchanged label, and unchanged descriptor before it writes the same action used by pointer input. Removing the button or accepting an empty successor makes the old address inert.

## Validation Receipt

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/sol-table-ax/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧱️elements/📊️Table/🧪️tests/🔘️button-accessibility/🟦️.tsx' --silent=false --reporter=verbose
```

Result: 1 file passed, 2 tests passed; Vitest 84.75 seconds, Nx 1 minute 33 seconds. The only diagnostic was the existing multiple-Three.js-instance warning.

The source-complete WGPU Scenes law checks exact segmented geometry, row/menu filtering, accepted action fidelity, and live Table revalidation. The Interpreter law checks published button semantics, exact accepted activation, wrong generation/key refusal, and successor-presentation stale refusal. Their focused native execution remains pending behind the shared compiler lane; no passing Rust result is claimed here.

## Current Limits

The browser receipt covers the real DOM mirror and addressed wire event. Native assistive-technology publication is covered by the retained projection unit path rather than operating-system screen-reader automation. Root owns the full renderer/native/WASM and paired-shell runtime gates.

# EventFeed Accepted Accessible Entry Parity

## Contract

The dedicated `EventFeedHost/🧬️schema/♿️accessible-entry` schema defines actionable and passive entries over one neutral fixture. Each row fixes its stable retained key, accepted temporal label, visible title/detail text, role, focusability, tabbability, and exact `{ surfaceId, id }` action. Actionable rows are buttons and one Tab stop; passive rows remain labelled paragraphs outside the Tab order.

The actual React host is the independent DOM oracle. Its decorative icon is hidden from the accessibility name, actionable rows admit click, Enter, and Space with the same descriptor, and passive rows have no handler. The actual browser accessibility mirror consumes the same expected projection and sends one fully addressed `accessibility-activate` event.

## Accepted WGPU Presentation

`render_event_feed` derives virtual controls from the same clipped visible-row geometry used for paint and from the host temporal reply visible to that frame. Controls are staged with the candidate frame and only exposed after the shell seals and acknowledges the presentation epoch. An actionable control retains the exact descriptor used by the pointer path; passive controls retain no action.

The interpreter appends accepted controls as virtual buttons or paragraphs. Activation requires the current window generation, stable node hash and key, retained visible EventFeed scene, current entry id, current activation action, and matching accepted descriptor. A successor accepted presentation without the row makes the old address inert.

The visual row was aligned to the React oracle while this path was added: title begins after the optional icon, accepted time is right-aligned, detail shares the title origin, and the WGPU-only tone dot was removed.

## Validation Receipts

The first focused React run failed before collection because the new test used one too many parent segments for the local host/schema/fixture imports. After that correction, the next run executed both tests: the browser mirror law passed, while the actual React oracle exposed the fallback icon text in the button's accessible name (`circle-check Build finished …`). Wrapping that decorative icon in `aria-hidden` repaired the cross-renderer name divergence. A subsequent harness-only failure found that this repository's `screen` helper omits `queryByRole`; the passive absence check now uses the mounted container.

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/sol-event-feed/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧱️elements/📡️EventFeedHost/🧪️tests/♿️accessible-entry/🟦️.tsx' --silent=false --reporter=verbose
```

Final result: 1 file passed, 2 tests passed; Vitest 96.34 seconds, Nx 1 minute 48 seconds. The only diagnostic was the existing multiple-Three.js-instance warning.

The two focused native laws are source-complete: the Scenes law validates schema-derived clipping, accepted labels, exact descriptor staging, and passive refusal; the Interpreter law validates published button/paragraph semantics, exact addressed activation, and successor-presentation stale rejection. Their focused execution remains pending behind the shared Rust compiler lane.

## Current Limits

The WGPU receipt is an accepted-state unit path rather than a native screen-reader automation. The browser test proves the real DOM mirror element and wire address. Root owns the full React/native/WASM and paired-shell runtime gates.

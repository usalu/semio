# WGPU Accessibility Activation

## Result

The browser accessibility mirror now exposes focusable native elements with the retained projection's role, label, description, live state, shortcut, disabled state, checked/selected/expanded state, hierarchy, range/value state, and renderer focus. Focus, activation, and value input enter the existing lossless browser transport as semantic node events. No accessibility event carries pointer coordinates.

Each event addresses `{ windowId, windowGeneration, nodeId, nodeKey }`. Retained surfaces accept the event only while the document is live, the surface lifetime generation matches, and the current document record still binds that node id to that key. Closing and reopening the same window id advances its lifetime generation. Shell chrome applies the same id/key check against the current published projection and hit registry. Rejected stale events do not focus, activate, or mutate a value.

Accepted retained events use the real `EventRouter` focus and action paths. Accepted chrome events use the existing Shell hit/action and input commit paths. Mirror click/input handlers stop propagation, canvas listeners remain separate, and renderer-driven focus restoration is suppressed from echoing another focus event. Focus preservation includes the window generation, so a retired same-id surface cannot transfer DOM focus into its replacement.

## Contract

The language-neutral fixture is `🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json`. It fixes:

- 512 UTF-8 bytes for a window id and node key;
- 1,024 UTF-16 code units for a value;
- focus, activate, and value wire shapes;
- native role/state hierarchy;
- current focus feedback;
- exact-once mirror activation with independent canvas keyboard delivery;
- rejection of retired windows, prior generations, and reused node ids with another key.

The shared accessibility projection now carries `checked`, `selected`, `expanded`, and `level`, and publishes live retained focus/value state. Its Rust and TypeScript projections remain schema twins.

## Executable laws

- `wgpu accessibility interaction contract` mounts an independent React/jsdom mirror and checks native hierarchy, roles/states, focus, activation, value, exact-once canvas separation, and every fixture rejection.
- `transfers node-addressed accessibility focus activation and value events exactly once` exercises the real `BrowserFrameTransport` batch.
- `rejects accessibility addresses and values outside their fixed transport credits` exercises UTF-8 id, control-character, and UTF-16 value rejection.
- `accessibility_fixture_decodes_into_node_addressed_dispatch_without_coordinates` exercises the production Rust wire decoder and `DispatchEvent` projection.
- `accessibility_dispatch_uses_current_window_generation_and_node_identity` publishes and reconciles a real retained document, dispatches focus/value through the production table/router, retires it, reopens the same id, and checks the prior generation remains rejected.
- `chrome_accessibility_dispatch_validates_current_identity_and_activates_once` publishes the real chrome hit registry, focuses and activates the search control through Shell, and proves stale generation and id/key mismatch cannot toggle it.

## Verification

Executed:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx' '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts' --config '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts'
PASS: 2 files, 43 tests.

rustfmt --edition 2021 --config skip_children=true --emit stdout <each touched Rust source> >/dev/null
PASS: every touched Rust source parsed; this was a read-only parser pass.

git diff --check -- <accessibility packet paths>
PASS: no whitespace errors.
```

Root-owned focused native commands:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/ui-rs:test-wgpu-engine -- accessibility_dispatch_uses_current_window_generation_and_node_identity --exact

NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit -- accessibility_fixture_decodes_into_node_addressed_dispatch_without_coordinates --exact

NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit -- chrome_accessibility_dispatch_validates_current_identity_and_activates_once --exact
```

Cargo, native, WASM, generated browser artifacts, and browser runtime acceptance remain with the root-owned activation jobs.

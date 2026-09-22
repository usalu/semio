# Settings Tabstrip and Presented Chrome Accessibility

## Scope

This packet joins two runtime failures that share one accepted-frame boundary:

- a browser accessibility mirror address is currently validated against a later, recomputed chrome projection with a fixed generation;
- a constrained panel tab row stops iterating at the right edge, so its tail is absent from paint, pointer authority, and the accessibility projection.

The repair keeps pointer geometry clipped while retaining the complete semantic row. An accessibility address belongs to one accepted input epoch and resolves only within that epoch. Selecting a semantic tail tab updates the active path; the next accepted frame reveals its pointer chip.

## Neutral fixture and independent oracle

`semio.renderer.wgpu.accessibility-interaction.v1` now includes:

- two accepted chrome epochs, 41 and 42, whose Display/Settings ordering changes;
- one constrained 300 px Settings row with exactly six declared tabs;
- Default Apps and Conflicts as semantic tail tabs.

The schema validates the epochs and exact six-tab/two-tail shape. The browser law mounts the production React `PanelTabBar`, proves all six native buttons remain mounted at the constrained width, activates a tail button, and observes the selected state. A second law drives a real `BrowserFrameTransport`: the current mirror activation carries epoch 42 while a detached prior mirror element continues to carry epoch 41, giving native code an exact stale-address witness.

Focused Bun/Nx receipt:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx' --config '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts' --reporter=verbose
PASS: 13/13 tests, Vitest 11.22s, Nx 27.6s.
```

Receipt: `🗑️generated/astra-runtime/settings-ax-browser-red/run-2.log`.

## Native fail-first laws

`a_delayed_chrome_mirror_address_cannot_activate_after_its_presented_epoch_retires` publishes two real Shell input frames and requires the first address to be rejected while the second activates Settings once.

`a_constrained_settings_strip_retains_all_semantic_tabs_and_reveals_an_accessibility_selected_tail` paints and accepts the constrained Settings row, requires clipped pointer geometry but all six semantic entries, activates Conflicts through the semantic address, then requires its chip in the next accepted pointer registry.

Native execution is root-owned. Native141 stopped in the shared Plugin crate before either law ran; no RED or GREEN claim is made from that attempt.

## Production boundary

The intended boundary is one bounded presented chrome accessibility registry published with `InputState::publish_hits`. It carries the accepted input epoch, projection nodes, and exact resolved chrome targets. The Interpreter publishes that epoch instead of a fixed generation. Dispatch rejects a mismatched epoch before any state change and never re-enumerates a later projection to validate an older mirror address.

For panel rows, declared semantics and pointer geometry are distinct:

- every node in every currently displayed row remains in the bounded semantic catalogue;
- paint and pointer hit rectangles are translated by retained horizontal row offset and intersected with the row clip;
- zero-area intersections never become pointer hits;
- selecting an offscreen semantic tab updates the active path and reveals it in the next candidate;
- drag insertion and mobile rows use the same translated row geometry.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/♿️wgpu-accessibility-interaction/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`


## Native144 RED and production repair — 2026-09-21

Native144 established both intended failures: chrome activation still accepted a stale mirror generation and a constrained Settings strip omitted semantic tail tabs outside its pointer clip. Receipt: `🗑️generated/astra-runtime/renderer-native144-compact-ax-locale-red/run.log`.

The production repair publishes one bounded chrome accessibility catalogue under the same accepted epoch as the hit/owner/widget/geometry registries. Interpreter diagnostics carry that exact epoch instead of a constant generation. Dispatch validates `(epoch,node_id,key)` only against the accepted catalogue and never re-enumerates a candidate tree to validate a delayed address.

Panel tab rows now retain every declaration, paint through a row scissor, register only positive-area clipped pointer rectangles, and keep a per-anchor/per-row horizontal offset. Each open row contributes all declared tabs to the accepted semantic catalogue, including offscreen tail tabs. Accessibility Focus/Activate reveals the target row before selecting the tab; the next accepted frame clamps the sentinel to the authored maximum and publishes its physical hit. Pointer wheel deltas address the row's bounded offset, while ordinary vertical scroll regions retain their existing axis.

Rustfmt parse validation succeeded for the Shell, Interpreter, UI component/reconcile, native laws, and UI wire test files. Native145 is the first integration build containing this production packet.

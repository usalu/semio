# Puzzle3D Settings Retained Paint Repair

## Fault

The retained NumberStepper phase-0 paint admitted the generic fixed-node grant of eight output items. Its two `push_control_border` calls each emit one fill and four border quads, so the deterministic output is ten items. The second border therefore exhausted the retained grant and surfaced `Settings -> paint-node` at checkpoint 8.

## Repair

- Kept the generic fixed-node grant at eight and introduced a NumberStepper-specific ten-item chrome grant.
- Routed only retained NumberStepper phase 0 through the dedicated grant. The paint remains transactional and faults remain visible.
- Added a language-neutral fixture representing Puzzle3D's actual default Settings `UiDocumentTree`: one open section, four fields, four uniform NumberSteppers, and the authored Change bindings.
- Added a native acceptance law that publishes and reconciles that document, accepts layout, drives the production frame ladder to `Ready`, verifies the cleared terminal paint census, and checks each control's layout, hit target, two five-quad borders, glyph runs, and authored action.
- Added an independent React acceptance law over the same fixture. Testing Library verifies four spinbuttons and real increment interaction verifies the authored semantic intents.
- Registered the React law in the renderer suite manifest.
- Repaired `shell_with_nested_app_settings` by synchronizing real dock tabs before inserting the app Settings leaf, so the helper operates on the Settings branch produced by the shell. The synthetic helper then clears its fixture-only guest session: branch activation stays a local chrome test and cannot fall into a nonexistent plugin action program.

## Evidence

The focused React law passed through the repository's Bun/Nx entry point:

```text
nx run @semio-tech/framework-renderer-react:test-long ../../../../🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx --silent=false --reporter=verbose
Test Files  1 passed (1)
Tests       1 passed (1)
NX Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

The root-owned UI13 native gate then passed all 610 tests with no skips in 21 seconds. That run included the actual Puzzle3D Settings document law and its terminal census, retained glyph, two-border, layout, binding, and hit-target assertions.

The first focused run exposed a test-only identity expectation: the interpreter intentionally prefixes explicit node keys with the document surface in DOM ids. The law now asserts the full production identity `puzzle3d.panel.settings/<key>` and passed.

Cargo and native build commands were intentionally not run by this subtask; the root integration agent ran the sequential native gate and owns the browser gates. The native law's terminal census follows `frame_into_step`'s `Ready` contract: the completed paint frame has been consumed, so it requires `frame=false`, `phase=None`, no fault site, no paint key, and an empty cursor.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧱️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/⚙️puzzle3d-settings-document/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`

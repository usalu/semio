# Sol Settings Nested Selection

## Result

Panel activation now resolves the requested id against the live dock and reconciles its full root-to-leaf path. For an app Settings leaf placed before General, selecting General stores:

```text
[framework.settings, framework.settings.general]
```

The shared route serves pointer, assistive-technology, programmatic reveal, and every other nested dock leaf. It contains no General, Puzzle3D, or app-specific branch.

The neutral accessibility fixture records an app-first Settings branch, the expected General path, the Appearance control, and both observed pre-measurement and measured pointer rectangles. Native laws send a real pointer down through each published hit and send node-addressed activation through `handle_accessibility_event`. They require the exact path, active General leaf, `checked` and `selected` projection state, rejection of the app-first fallback, and a General retained document containing Appearance. A mounted React oracle consumes the same fixture and reaches the same active path and control.

## Renderer11 Laws

The owned renderer census failures were corrected without changing their intended behavior:

- Theme tests inspect the canonical Tree projection and custom-theme Delete gate.
- Built-in theme paint equality allows one `f32` rounding step.
- Appearance paint and alpha rows are reached through the virtualized scroll viewport.
- Every temporary theme viewport record set is dropped and the real `UiValue` retirement pump restores arena headroom before the next publication.
- Engagement assertions recognize the semantic Field that now owns a labelled primary control; an absent engagement-options payload no longer expects an options group.
- An empty Tool body expects its stable panel root and empty options section.

The retained paint census now includes the active node key plus node phase, section, item, depth, glyph byte and line, visited count, chrome route, and popup route. The next root-owned run can attribute the separate `puzzle3d.panel.settings` paint-node fault. This packet does not claim that app-owned body is repaired.

## Verification

Passed:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx' '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts' --config '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts'
2 files passed; 45 tests passed.
```

Read-only `rustfmt --edition 2021 --emit stdout` parsing passed for every edited Rust production and law file. Targeted diff whitespace checks passed. No Cargo, native, wasm, shader, or browser artifact build was launched. The new and corrected native laws await the root-owned run.

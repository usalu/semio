# Blend Modes and SVG Round Trips

## Changed Behavior

The PDF painter's blend vocabulary now matches authored Draw documents: `colorDodge`, `colorBurn`, `hardLight` and `softLight` map to their PDF blend names. The prior implementation expected kebab-case names, so valid authored modes either silently became Normal or were rejected by the newly added strict compositing check. Native tests cover the mapping and rejection of noncanonical authored names.

The shared compositing corpus now has 37 scenes: the original five, plus each of sixteen modes on an isolated overlapping group and on a layer with overlapping fill and stroke. The reference SVG is explicit fixture data. Canvas and SVG output are independently rasterized and compared over all pixels.

SVG import now converts CSS kebab-case blend names into Draw's camelCase vocabulary in both Rust and TypeScript. It accepts `isolation:auto` and the `isolation:isolate` scopes written by Draw's exporter when opacity or blend already requires isolation. Unsupported blend names, noncanonical CSS spellings and invalid isolation values fail atomically. An explicitly isolated, otherwise unmodified nested group still requires a real authored isolation property and is refused; this remaining schema/renderer feature has not been replaced by an approximation.

The new round-trip test exports each shared scene, incrementally imports the result into editable layers, re-exports those layers, and compares the rendered pixels with the independent fixture SVG. Native tests separately check import/export acceptance and preservation of all nonnormal authored blend names after scene projection.

## Executed Evidence

- `56902`, `tests-all-blends.txt`: exit 0; 229 passed, one skipped, 145,371 assertions across 27 files. All new canvas/SVG blend pixel cases passed.
- `41570`, `tests-svg-blend-red.txt`: exit 1; 244 passed, one skipped, 26 failed before the import fix. Failures reproduced the unsupported exported isolation and blend-vocabulary behavior.
- `77011`, `tests-svg-blend-current.txt`: exit 0; 270 passed, one skipped, 145,453 assertions across 27 files. All 37 rendered SVG round trips and four new rejection fixtures passed. Both existing publication-authority and Ajv field-patch audits passed.

The skipped test is the native PDF.js pixel oracle, still waiting for native fixture exports. This is not native PDF parity evidence. Native run 5483 and component build 93519 remain active; their final coverage must be inspected because these sources changed after launch. No browser actions occurred.

## Remaining Work

Add an explicit authored isolation property across the schema, mutations, scene projection, controls and exporters to support otherwise unmodified isolated groups. Complete the native PDF fixture/oracle run, PNG/vector raster fidelity, typography, SVG features not yet represented, progress/cancellation and browser end-user journeys. See the acceptance plan; the full goal and ticket remain open.

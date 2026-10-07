# Original Primary Point and Wire Object Style

## Existing Visual and Palette Authorities

Read-only original WorldInstanceNode uses `useWorldInstanceChrome` and `resolveMeshSelectionPreviewStyle` to resolve disabled/provisional/celebrated/selected/highlighted/hovered/neutral priority. The same original `MeshStylePalette` owns authored semantic CSS paint: neutral border-normal-color, hover border-emphasized-color, selected primary. `WorldInstancesLayer` updates its original chrome store from delivered selection.ids/hoveredId and local hover, so object state already reaches every original instance visual.

Primary wire `lineSegments` already binds `style.lineColor`; curve-only wires also widen from neutral2 to active4. Primary pure-point `pointsMaterial` instead binds `semanticColorsFromPalette(palette).edge`, which always reads neutral.lineColor. Component selected/hovered overlay paints remain separate. Source alone suggests object-mode points cannot display selection/hover through their base material, while wires already use the original style owner; runtime baseline is required before changing production.

## Schema-first Neutral Cases

New original World fixture/schema: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️primary-geometry-style/🔣️.json` and `🌍️world/🧬️schema/🎨️primary-geometry-style/🔣️.json`. Point/wire/edgeOnlyWire reuse the unchanged actual zero-index position/vertex-ID/edge buffers in lower Scene `🧫️fixtures/🎯️component-source/🔣️.json`. Four states require normal-border neutral paint, emphasized-border hover paint, primary selected paint, and primary selected+hovered paint. No new palette API, hard-coded hex table, fabricated topology, or source evaluator is introduced.

Mounted original component law `paints primary point and wire objects with the existing selection and hover palette` feeds each delivered selection state through the actual World3dHost/WorldInstancesLayer/WorldInstanceNode owner. Strict Ajv validates the shared style case. Unmocked Three PointsMaterial/LineBasicMaterial resolves the expected semantic line reference; actual mounted material paint must match, while no triangle mesh may appear. Existing WebGL/hardware seams remain unchanged.

Baseline61017 live, original registered Nx React target, log `🗑️generated/enduser-react-primary-object-style-baseline.log`. No production style edit has been made. Both launch JSONC documents parse and include exactly one new `⚖️test-primary-object-style🌐️World3dHost🟦️` row in the existing presentation order. Root owns matching Native baseline and same palette-owner reuse.

Selected-shell replay65701 remains live independently in original native preparation, log `🗑️generated/enduser-live-selected-shell-runtime-2.log`; no second native gate was launched here.

## Actual Mounted RED and Original Visual Owner Correction

Baseline61017 terminated actual RED1/1, nine unrelated tests skipped, Vitest12.54s/test170ms/Nx15s. Neutral point paint matched #7b827d, but delivered point object hover still painted #7b827d versus independent Three expected emphasized-border #001117. The original geometry/material mounted correctly; this is a behavioral failure, not a schema/harness/compilation floor.

Only after actual RED, original WorldInstanceNode's existing base pointsMaterial binding changed from `colors.edge` to `isPointOnly ? style.lineColor : colors.edge`. Pure-point primary visuals now use the existing resolved instance style/palette; ordinary surface vertex grips retain their original neutral component paint and selected/hovered component overlays. No new style owner/palette API/material/evaluator/geometry/dependency was added. Wire production is unchanged.

Combined original component+interaction regression dispatched against this source and all31 laws, log `🗑️generated/enduser-react-primary-object-style-green-regression.log`; terminal pending. The new law executes all point/wire/edgeOnlyWire × neutral/hovered/selected/selected+hovered states using existing semantic refs and independent Three material colors. Native primary paint remains Root's separate pending scope.

### Wire Selector Readback; No Wire Production Defect Attributed

Combined75643 terminal30 passed/1 failed31; every existing law passed, all four point style assertions passed, and the new law stopped on wire/neutral because its mixed-case compound CSS selector returned null. Bounded mounted readback94804 confirms actual DOM contains LINESEGMENTS with a direct LINEBASICMATERIAL child painted neutral #7b827d. This is a selector defect in the new test, not absent wire paint. Same readback directly logged all four point colors: neutral #7b827d, hover #001117, selected/selected+hovered #ff344f, all equal independent Three.

Corrected only the new law's compound material selectors to actual lowercase HTML tag names. The expected mounted visual, semantic paint/color, shape roster, states and assertions remain intact; no wire production was changed. Original registered targeted visible-console replay dispatched, log `🗑️generated/enduser-react-primary-object-style-green.log`, followed by the full original31-law regression after its terminal.

## Actual Primary Paint GREEN

Targeted registered35863 terminated exit0, actual GREEN1/1 with all12 geometry/state assertions executed, nine unrelated tests skipped, Vitest12.29s/test329ms/Nx15.2s. Direct DEBUG logs for point, wire and edgeOnlyWire each show neutral #7b827d, hover #001117, selected and selected+hovered #ff344f, exactly matching unmocked Three material colors from the original semantic refs. Wire and edgeOnlyWire required no production change. The sole production edit remains the point-only material binding to original style.lineColor. These headless values are outputs of the current original palette, not a new color table or runtime expectation override.

Final original31-law component+interaction regression launched after the exact selector repair and targeted GREEN; log `🗑️generated/enduser-react-primary-object-style-green-final-regression.log`. No additional native lane was launched. Root owns matching Native palette and GPU qualification; selected-shell65701 remains live.

## Final Current Mounted Regression Actual GREEN31

Original combined component+interaction gate79682 terminated exit0: actual GREEN31/31, two files, zero skip, Vitest13.85s/tests2.44s/Nx16.4s. Log `🗑️generated/enduser-react-primary-object-style-green-final-regression.log`. This includes all previous29 laws, the real guest mesh/source-refresh gesture law, and all12 primary point/wire/edgeOnlyWire style states. Direct target35863 separately logged every semantic color versus actual Three. No further portable rerun is needed without a new source/fixture change or unresolved concern.

Point-only primary paint now uses the original instance style line color after actual point-hover RED; wire production remains unchanged. Original selected-shell65701 stays live in native preparation (~14min at this readback); no shell inference/history or full metadata assertion is claimed. Root owns Native gesture/style runtime and production qualification.

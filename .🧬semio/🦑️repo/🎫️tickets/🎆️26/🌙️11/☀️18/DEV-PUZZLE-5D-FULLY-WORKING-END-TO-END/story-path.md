# 5D timeline story path

Static chain verified: package exports map (3 subpaths -> `🟦️.ts` re-exporting `?raw` `.dsl.semio`, named exports puzzle5dConcreteForestDsl / NakaginCapsuleTowerDsl / CapsuleDreamDsl exist); `puzzle5dParseDslJson` exists (`🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌉️bridge/🦀️.rs`, re-exported by hub puzzle crate); `GraphTimelineHost`, `World3dHost`, `ActionDescriptor`, `UiComponentSceneNode`, `SharedBorderRow`, `HistoryColumn`, both selection helpers all exported.

Broken: story read `fixture.camera3d`, which the parsed Puzzle5dSnapshot JSON never has (camera is absent), so `cameraJson` was undefined; parse failure left the story on "Loading" forever; docs cited deleted paths.

Changed: story (camera fit from part bounds, error state `puzzle5d-timeline-error`, doc paths), Playwright smoke `🧪️tests/🧊️storybook-spatial/🟦️.ts` (Concrete Forest 1/0 and Capsule Dream 2880/2864), Rust test `every_example_parses_into_its_fixture_counts` + fixture `🧪️tests/🧫️fixtures/📚️example-counts/🔣️.json`.

Not verified: wasm build (`repo run @semio-tech/puzzle-plugin:wasm`) stayed queued in the graph service (>15 min); log `🗑️generated/story-wasm.log`. Rust test, typecheck, Playwright not run. Nakagin 180 parts is from the existing smoke; fasteners 0 for forest/nakagin and 2864 for dream are unverified expectations in the fixture.

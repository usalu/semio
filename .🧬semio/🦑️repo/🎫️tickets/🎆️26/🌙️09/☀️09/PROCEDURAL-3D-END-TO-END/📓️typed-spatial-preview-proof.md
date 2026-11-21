# Typed Spatial Preview Proof

## World preview and retained mesh paths

Generation3d admits explicitly tagged `geometry`, `vertex`, `edge`, `face`, `shell`, `point`, `vector`, `plane`, and `mesh` values. Native leaves also require a recognized BRep handle. Coordinate leaves require finite numeric axes. A typed plane accepts numeric origin and normal coordinates with or without child schema tags. The same leaf rule controls current enumeration and traversal into the previous evaluation. Arrays, numeric-keyed lists, and metadata dictionaries can still carry typed children.

[Spatial Preview Schema Audit](📓️spatial-preview-schema-audit.md) identifies the originating Flow or authored schema constructors. The retained-path and direction fixtures now use these producer shapes. The lifecycle fixture uses typed values for its simulated Flow results.

## Native verification

- The changed fixture cases reproduced extra world items, shifted directional instance IDs, and a retained handle taken from metadata on the original implementation. Three of four directional and payload cases failed, and the retained-history test failed.
- With typed admission, the same native tests passed four of four directional and payload cases plus one retained-history test covering thirteen path scenarios. The native witnesses compile the production traversal and Edit and View payload projection against the cached Pack JSON crate at `.cache/b/r/m/u/l/p29a0c8`, hash `5262b28961bbf9e7`. The direction witness supplies resident packs in memory and verifies tessellation demand, retention, instance order, and shared hover targets. Generate imports the Edit payload projection and uses its own evaluation in the same call. The compact lane does not boot the app or load the kernel.
- Reproduce through `repo tools exec --light bun --` with [the directional runner](preview-direction-native.ts) and [the retained-history runner](preview-channel-history-native.ts). Pass the absolute cache directory and the crate hash after each runner path.

## Independent oracles

- SQLite JSON traversal and joining confirmed native admission, previous addresses, both plane-member encodings, and the inline instance count across [the directional oracle](preview-channel-directions.test.ts), [the retained path oracle](preview-value-paths.test.ts), and [the inline oracle](preview-inline-overlap.test.ts).
- The seven portable suites passed 43 tests, including evaluation validation, float32 markers, painted bounds, and world hover ordering.
- The full Cargo suite and a fresh app mount were outside this compact verification lane.

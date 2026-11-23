# Catalog example ids: plain ids are canonical

- Cause: `discoverExamplesForPlayground` emitted emoji path slugs (`🎬️demo`); manifests, `examplesForApp`, boot-query and `PLAYGROUND_LOCKED_EXAMPLE_ID` use plain ids.
- Fix: `🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts` now emits deduplicated plain ids via new `exampleIdOfSlug` (tail after U+FE0F). Catalog regenerated (`s-dev-composition:generate`): 0 emoji ids of 2888.
- Removed the dashboard slug-or-id compat layer (`canonical_value` in `🎛️dashboard/🎮️registry/🪶️entry-draft/🦀️.rs`, caller in `🚀️resolve-and-launch`); updated Rust unit tests/TS registry test; native-entrypoint fixture + test use `demo`. Rust edits not compiled (no cargo builds allowed).
- Tests: discovery test asserts plain ids and `examplesForApp` resolution; integration test asserts every catalog id is a plain id. Both pass under bun.
- Remaining gap: many catalog ids (puzzle3d/demonstrator `capsule-dream`, `demo-session`, bitmap, wfc...) are absent from the checked-in descriptor `manifest.examples`, so `examplesForApp` still drops them; descriptors are cargo-generated and need a rebuild.

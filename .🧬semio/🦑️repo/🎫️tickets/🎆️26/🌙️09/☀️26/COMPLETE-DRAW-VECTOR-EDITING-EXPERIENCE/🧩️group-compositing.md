# Isolated Group Compositing

Drawing scene projection now carries each leaf's isolated ancestor scopes, including IDs, opacity and blend mode. Leaf transforms still include the complete ancestor transform; leaf opacity is not multiplied by group opacity. Ordinary unit-opacity normal-blend groups remain pass-through, preserving child blending against the surrounding scene.

The React canvas host paints isolated scopes into reusable, viewport-aligned surfaces and composites each completed scope once. A layer with opacity/blending also composites its fill/stroke together before applying those settings. This fixes the overlap error from applying opacity separately to each child or paint operation. Scratch contexts use the same camera/device transform, clear between uses, and restore the destination context after composition. Surfaces are cached per nesting depth in the mounted canvas session.

Rust and TypeScript SVG exporters retain the same nested `<g>` scopes with opacity and isolated blend styles. They reject repeated, conflicting or noncontiguous group scopes. Canvas hierarchy validation runs before painting. A neutral schema defines the group metadata; five neutral pixel fixtures cover overlapping children, nested opacity, sibling scopes, multiply blending and overlapping fill/stroke paint.

## Verification

- `tests-compositing-red.txt`: missing canvas compositing entry point reproduced, exit 1.
- `tests-compositing-current.txt`: **194 tests / 31,573 assertions** passed, exit 0, including the first four pixel fixtures.
- `tests-compositing-final.txt`: **197 tests / 46,952 assertions / 26 files** passed, exit 0. Each pixel of our SVG export and the actual canvas painter (using the already-installed test-only native canvas implementation) agrees with an independent Sharp/libvips SVG reference within two channel levels. Further checks cover zoom, cleared reused surfaces and invalid hierarchy rejection. The 48-case field-patch and 36-command audits also passed.
- `tests-compositing-renderer.txt`: **52 tests / 3 suites passed**, exit 0, using the registered renderer target for paint, path and input-contract suites.
- Final malformed-blend-name validation follow-up 24022 (`tests-compositing-validation.txt`) passed **197 tests / 46,955 assertions / 26 files** and both audits, exit 0.
- Native projection/export tests were added; fresh native run **5483** is live in `tests-native-compositing.txt`. The existing component build **93519** remains live. No browser actions occurred in this turn.

## Native Fixture Corrections

Prior native run **88159** is terminal: **418 tests run, 414 passed, 4 failed, zero skipped**, exit 1 (`tests-native-svg-gradients.txt`). It compiled the arrangement/stack/ungroup additions and executed their real command tests. Three history tests failed before command dispatch because their document IDs were empty; the store's envelope validator explicitly refuses that input. These fixtures now use stable nonempty IDs.

The remaining inspector assertion read `disabled` from `project_and_retire_fixture_tree`, whose fixture projection intentionally carries keys/components/bindings/accessibility/children but omits disabled state entirely. The assertion now observes the real `BuiltNode.disabled` field before retirement and retains the projected assertions for the other properties. No production lock behavior was changed. The earlier workaround treating a missing projected value as false could never verify the true case and has been replaced.

## Remaining Work

- PDF, native rendering and other raster/export paths still need equivalent isolated compositing; this change does not establish parity for them.
- Ungroup still refuses groups with compositing effects, because splitting an isolated group into individual layers cannot in general preserve its overlap appearance. A complete workflow needs an explicit appearance-preserving strategy or an explicit user-visible release of the group effect.
- Offscreen storage currently scales with viewport area and isolated nesting depth. Deep/large documents need a budgeted rendering strategy, progress/cancellation and tighter surface allocation. Projection also duplicates ancestor metadata per leaf.
- Source-level integration and independent pixels do not prove rebuilt application behavior. Component activation and browser journeys remain pending.

The full editing goal and ticket remain active and incomplete.

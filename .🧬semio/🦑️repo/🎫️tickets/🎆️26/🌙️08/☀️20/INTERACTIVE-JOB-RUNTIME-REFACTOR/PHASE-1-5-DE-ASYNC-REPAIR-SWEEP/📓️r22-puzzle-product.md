# Puzzle Product Compiler Repair

## Scope

Compiler-exact repair of `semio-s-plugin-puzzle` after the framework de-async and semantic-UI contract changes. The Phase 4 controller, fill, collision, precompute, and action state-machine ranges were excluded from broad rewrites. The sole Phase 4 command edit removes a decorative `async` from `fill_build_tick`; the function has no suspension point and retains the same enqueue/poll behavior.

## Diagnostic Reduction

- `r22-puzzle-native-1`: 2,699 Puzzle diagnostics.
- `r22-puzzle-native-2`: 327 Puzzle diagnostics.
- `r22-puzzle-native-3`: 176 Puzzle diagnostics.
- `r22-puzzle-native-4`: zero Puzzle diagnostics; two upstream stdio diagnostics remained.
- `r22-puzzle-native-5`: 139 Puzzle diagnostics after concurrent framework API/semantic-node churn.
- `r22-puzzle-native-6`: 122 Puzzle diagnostics after the first compiler-exact repair pass.
- `r22-puzzle-native-7`: 15 Puzzle diagnostics, all narrowed contract/future seams.
- `r22-puzzle-native-8`: stopped before Puzzle by a missing host interpreter include.
- `r22-puzzle-native-9`: stopped before Puzzle by host interpreter `fingerprint` E0425.
- `r22-puzzle-wasm-1`: 40 diagnostics, including 19 semantic-builder namespace/import diagnostics and 12 wasm Store/render awaits; all Puzzle spans were repaired.
- `r22-puzzle-wasm-2`: reached stdio and stopped on its single upstream diagnostic. The structured log could not finish because the workspace volume had 179 MiB free.

Exact structured evidence is retained as `📝️r22-puzzle-native-{1..9}*` and `📝️r22-puzzle-wasm-{1..2}*` in the Phase 4 ticket.

## Repairs

- Removed decorative async signatures and stale awaits only where compiler/API contracts proved the operation pure; retained true Store, renderer, menu, and media suspension.
- Restored typed `Emit`, `AppIo`, action-definition, selection, utility/tool, scene, and projection boundaries with explicit `resolve_ready` only at synchronous trait seams.
- Migrated Puzzle 2D/3D/5D editor and viewer presentation roots from legacy `UiNode` to `BuiltNode` without a compatibility converter.
- Encoded Board2d and World3d scenes through `semio-framework-ui-scene` and `semio-framework-ui-contract`, with direct package dependencies and semantic surface builders.
- Migrated document/catalogue/inspection/settings panels to semantic tree/field builders while retaining nested rows, interaction selection bindings, row actions, drag payloads, labels, descriptions, dimming, and number-stepper actions.
- Updated the Puzzle 3D document memo to cache the completed immutable `BuiltNode` tree by the existing geometry fingerprint.
- Restored wasm Store constructor/dispatch/snapshot/envelope/generation and GPU render awaits.

## Phase 4 Protection

- Puzzle 3D geometry has no owned diff from this compiler packet.
- The broad compiler sweep excluded precompute, fill-build-tick, wasm, and root action/controller ranges. Subsequent edits in those areas were individually compiler-proven; no timing, determinism, freshness, collision, or replay assertions were weakened.

## Current Gate Truth

`cargo fmt --package semio-s-plugin-puzzle -- --check` parses the full crate and reports formatting differences only. An authoritative native, release, wasm32-unknown-unknown, wasm32-wasip2, behavioral, determinism, and isolated warm timing matrix remains required after the upstream host/stdio walls and workspace disk exhaustion are cleared. No unexecuted gate is represented as passing.

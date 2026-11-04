# Puzzle Editor Surface Test Port — 2026-11-04

The attempted native disposer-birth filter compiled the parent editor-surface integration target and failed before selecting any test. The current log reports twelve errors: a missing OriginalFixture binding, generic ArtifactCommandWork/step inference after the current editor abstraction, a removed action_declarations helper and excessive nested agent-probe future layout query depth. It contains no current ToolCommandWire error.

Ported exactly three existing test files, preserving all 93 assertions and all 38 named functions (including helpers), without removing or moving tests:

- unit/🧮️extent-and-step-laws: select EditorApp<Puzzle3dPlayApp> explicitly for trait extent and retained testing step/close calls; construct the previously missing original fixture with the existing neighboring fixture authority, and close/finish its actually constructed suggestion work.
- unit/🎛️dispatch-and-operation-laws: assert the same forbidden-verb absence using the existing current dispatchable_actions helper, which enumerates canonical window_kind_actions.
- 🤖️agent-lane: box and await the actual agent probe future at the test call boundary, avoiding embedding the deeply nested future layout in another async test future. The entire real probe and every divergence/document assertion remain intact; recursion limits are unchanged.

No production API, compatibility helper, manifest, dependency, grant, operation ID, assertion threshold or recursion limit changed. Current compile failure is the actual red evidence; diff --check passes. Native compilation and nonzero test execution are pending Runtime's coordinated Cargo window; no Cargo was run in this lane.

The actual newly authored disposer-birth law belongs to semio-s-artifact-puzzle-3d-editor-window, whose manifest is editor/🪟️window/📦️packages/🦀️rust/Cargo.toml and whose lib mounts window/🧪️tests/🔬️unit/🦀️.rs under cfg(test). The parent editor-surface target does not mount dependency library tests, so its filter cannot select that law even after this port. Runtime and root were notified to execute the rightful window package's --lib filter config_store_disposer_birth_quotes_the_actual_bounded_owner and explicitly require a nonzero count. This keeps the law in its original rightful module while separately repairing the existing editor-surface target.

All Source edits in this lane are stable; Runtime owns the native retry.

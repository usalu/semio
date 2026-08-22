# P8 Bounded Reducer Classification Audit

## Verdict

The production catalog no longer obtains `Migrated` from `ActionSemantics::for_kind`. Every ordinary catalog declaration opts into the bounded-reducer constructor, and every source-visible open-ended reducer loop is explicitly overridden to `BatchOnlyPendingRewrite`. Non-migrated declarations receive no production factory and the UI dispatch backstop rejects them with `interactive-job.not-ui-safe`.

## Inventory

`bun ./📜️script.ts verify interactivity tool-jobs --format json` completed successfully with:

- 50 production macro host files and 50 production macro invocations.
- 771 production command rows and 769 unique file/id rows. The two duplicate occurrences are the intentional three Writer `setEditorSetting` variants.
- 1 separate `#[cfg(test)]` parser-fixture host, 2 fixture macro invocations, and 4 fixture rows; fixtures cannot inflate production coverage.
- 765 explicitly bounded production rows.
- 6 `BatchOnlyPendingRewrite` rows.
- 0 unclassified, forbidden, or deleted rows.
- 1 production factory implementation, registration path, and dispatch path.
- 0 verifier failures.

## Open-Ended Reducers

The command-taxonomy scan covered every `🎮️commands/**/🦀️component.rs` under `✏️s/🔌️plugins` and searched runtime code for `loop` and `while`. Parser-macro implementation loops and test-only prose/fixtures were excluded. Exactly six production command IDs contain or enter open-ended reducer loops:

| Plugin | Command ID | Unbounded work | Disposition |
| --- | --- | --- | --- |
| Remodel | `runReconstruction` | Repeated reconstruction-engine advancement, with a 200,000-tick safety cap | `BatchOnlyPendingRewrite` |
| Remodel | `retryStage` | Repeated reconstruction-engine advancement, with a 200,000-tick safety cap | `BatchOnlyPendingRewrite` |
| Remodel | `runStage` | Repeated reconstruction-engine advancement, with a 200,000-tick safety cap | `BatchOnlyPendingRewrite` |
| Draw | `canvasPointerDown` | Gesture FSM macrostep/ancestor/frontier loops | `BatchOnlyPendingRewrite` |
| Flow | `duplicateWidget` | Collision-driven widget and synapse ID searches | `BatchOnlyPendingRewrite` |
| Forms | `setTryValue` | User-index-driven array expansion | `BatchOnlyPendingRewrite` |

The same taxonomy scan found no direct filesystem or network I/O in production command reducers. Shell-facing import/export declarations construct typed effects; the host owns the external I/O. Serialization, decoding, collection traversal, and geometry helpers remain on the worker lane and are subject to the executable 8 ms reducer watchdog. A step that exceeds the ceiling is rejected as `interactive-job.step-overrun`; it is not accepted as bounded merely because it returned from its first poll.

## Enforcement Composition

The six production dispositions are enforced in three layers:

1. The permanent verifier requires the exact six file/ID overrides and rejects any return to the blanket `for_kind => Migrated` behavior.
2. Factory registration intersects the macro-generated `OpBinary::TOOL_JOB_IDS` catalog with manifest declarations whose disposition is exactly `Migrated`; these six therefore have no registered production factory keys.
3. The executable UI backstop test iterates every non-migrated classification and proves action and command dispatch return `interactive-job.not-ui-safe`. The activated-registry integration test additionally proves Platform-visible factory keys equal the migrated declaration set exactly.

## Completed Evidence

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | PASS: 771 production rows; 765 bounded; 6 batch-only; 4 separately counted fixture rows; 0 failures |
| Isolated `cargo check -p semio-framework-plugin` | PASS |
| `cargo test -p semio-framework action_bus::tests --lib` | PASS: 6 passed, 0 failed |
| `cargo test -p semio-framework-plugin generated_tool_job_catalog_is_an_exact_bijection_with_rows --lib` | PASS: 1 passed, 0 failed |
| `cargo test -p semio-framework-plugin activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations --lib` | PASS: 1 passed, 0 failed; dispatch count proves traversal of Platform's shared bus |
| `cargo test -p semio-framework-plugin ui_dispatch_backstop_rejects_every_non_migrated_action_and_command --lib` | PASS: 1 passed, 0 failed |
| `cargo test -p semio-framework-plugin bounded_reducer_registration_is_rejected_when_its_actual_step_exceeds_eight_milliseconds --lib` | PASS: 1 passed, 0 failed |
| `bun nx run @semio-tech/energy-plugin:describe --skip-nx-cache` | PASS: descriptor regenerated from the wasip2 module; SHA-256 `f5682b07f9aba3dce74baae1b880aaf70dc6681a307d6e0348a526bac8f88b8e` |
| Default-stack `cargo test -p semio-s-plugin-energy --lib` | PASS: 292 passed, 0 failed; includes `energy_model_viewer_never_mutates` |
| Focused native release check | PASS |
| `cargo check -p semio-framework-job --target wasm32-wasip2` | PASS |
| Warning-denied `wasm32-unknown-unknown` actor/job path | PASS |
| Native warning-denied framework-job | PASS |
| Native warning-denied framework-plugin | Initial 24 dependency lints plus seven newly exposed leaf lints repaired; focused warning-denied cohort and 436 tests pass; a separate 225-lint OS-kernel cohort remains |
| Phase 8-owned source `[DEBUG]` census | PASS: zero matches |

## Pending Evidence

All requested focused gates completed. The only strict-gate blocker is the separately owned native OS-kernel Clippy cohort in framework-plugin dependencies; Cargo still stops before linting the Phase 8 plugin crate itself.

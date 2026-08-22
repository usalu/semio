# P8 Production Tool-Job Coverage Audit

## Verdict

The production command seam now uses a heterogeneous erased `ToolJobFactory` registry shared by `Platform` and activated applications. The schema-generated command catalog supplies typed factory keys, production dispatch submits boxed payloads to the worker scheduler, and completion returns through a send-capable channel. UI dispatch contains no direct typed-handler or synchronous `run_to_completion` bypass.

Classification is explicit. Ordinary declarations opt into a bounded reducer job whose actual worker step is subject to the 8 ms watchdog. Six source-audited reducers with open-ended loops are `BatchOnlyPendingRewrite`, receive no production factory, and are rejected by the UI backstop.

## Exact Production Inventory

`bun ./📜️script.ts verify interactivity tool-jobs --format json` passed with:

- 50 production macro hosts, 50 production macro invocations, and 771 production rows.
- 769 unique production file/ID rows. The two duplicate occurrences are the intentional three Writer `setEditorSetting` action variants.
- 765 bounded rows and 6 `BatchOnlyPendingRewrite` rows.
- 1 separately counted `#[cfg(test)]` parser-fixture host, 2 fixture invocations, and 4 fixture rows. Fixture rows do not contribute to production totals.
- One production factory implementation, activation registration path, and worker dispatch path.
- Zero verifier failures.

## Non-Interactive Reducers

| Plugin | Command ID | Reason |
| --- | --- | --- |
| Remodel | `runReconstruction` | Reconstruction advancement loop |
| Remodel | `retryStage` | Reconstruction retry advancement loop |
| Remodel | `runStage` | Stage advancement loop |
| Draw | `canvasPointerDown` | Gesture FSM frontier/ancestor loops |
| Flow | `duplicateWidget` | Collision-driven ID searches |
| Forms | `setTryValue` | User-index-driven array expansion |

Registration intersects generated command IDs with manifest declarations classified exactly `Migrated`; none of these six IDs is registered. The executable UI backstop iterates every non-migrated declaration and proves both action and command dispatch return `interactive-job.not-ui-safe`.

## Runtime Evidence

| Gate | Result |
| --- | --- |
| ActionBus unit tests | PASS: 6/6 |
| Generated schema-row bijection test | PASS: 1/1 |
| Platform-visible migrated-key bijection and real dispatch test | PASS: 1/1; the shared bus dispatch counter increments |
| Non-migrated UI action/command rejection test | PASS: 1/1 |
| Actual reducer step >8 ms rejection test | PASS: 1/1 with `interactive-job.step-overrun` |
| Focused framework-plugin debug check | PASS |
| Energy descriptor regeneration | PASS from `wasm32-wasip2`; module SHA-256 `f5682b07f9aba3dce74baae1b880aaf70dc6681a307d6e0348a526bac8f88b8e` |
| Full Energy library suite on the ordinary test-thread stack | PASS: 292/292, including viewer mutation guard |
| Focused native release check for framework job + plugin | PASS |
| `cargo check -p semio-framework-job --target wasm32-wasip2` | PASS |
| Warning-denied `wasm32-unknown-unknown` actor/job path | PASS |
| Native warning-denied framework-job | PASS |
| Native warning-denied framework-plugin | The initial 24 dependency lints and seven newly exposed leaf lints are repaired and their 436 focused tests pass; the gate now reaches a separate 225-lint OS-kernel cohort |
| Phase 8-owned source `[DEBUG]` census | PASS: zero matches |

The only remaining strict-gate blocker is the repository-wide OS-kernel Clippy debt reached through framework-plugin dependencies; no Phase 8 plugin lint was reached before Cargo stopped. Detailed classification methodology is in `📓️p8-bounded-reducer-classification-audit.md`, and the completed upstream leaf repair is in `📓️p8d-strict-upstream-warning-repair.md`.

## Scope

`compose/`, generated targets, and test-only macro fixtures are excluded from production inventory. The fixture parser path remains exercised and is reported separately.

# Flow Native Fixture Recovery Audit

Static, read-only review of the recovery report and current Flow sources. The live `flow-recovery-5` native session was still running during this review; this report makes no runtime-pass claim.

## Recovery Contract Confirmed

- The reported first panic has a concrete source fix: `flow_app_with_registry` builds `AppActionRegistry` from `create_flow_app`, binds the instance, and registers the real `content` child (`🧪️tests/🔬️testkit/🦀️.rs:54-62`). This exercises the exact factory/manifest join rather than a registry-less fixture.
- `flowEvalResolve` is now a hidden runtime `View` command with English/German labels (`🦀️.rs:2083`). The exact id occurs in `FLOW_HOST_ONLY_TOOL_IDS` (`1415`), `FlowHostEffectJobFactory::PUBLICATION_CONTRACTS` (`1623`), the bounded factory proof (`1673`), registration (`1827`), the runtime handler (`1500`), and migrated action catalog (`2176`). `VcsArtifactApp::with_registry` runs `tool_job_registration` (`framework plugin 🦀️.rs:16929-16932`), which validates against the real migrated catalog. No manifest/factory mismatch remains in current source.
- Generalizing fixture helpers to `PluginApp` did not weaken their behavior: `settle_registered_typed_operation` retains the one-item maintenance bound, exact result ACK, Fault failure, and drains result/effect/event/UI/query channels (`framework plugin 🦀️.rs:6417-6459`). `close_registered_fixture_app` retains bounded close steps and its terminal-empty assertion (`6463-6482`).
- Both recovery tests retain empty immediate-admission checks, use the shared settlement helper, preserve the exact parent content handle, and keep terminal-empty close. The one-command retained route also confirms no pending typed operation after settlement (`➕️add-widget/🧪️tests/🔬️unit/🦀️.rs:45-71`).

## Actionable Assertion Gaps

1. **Exact receipt contract is not asserted.** The two-command child-edit test checks only counts of `Child` and `Terminal` lanes (`➕️add-widget/🧪️tests/🔬️unit/🦀️.rs:24-27`). It accepts an unexpected extra lane or a wrong lane order. This weakens the stated one-child-group-plus-terminal semantic contract, while the factory declares `addWidget` as Child-only (`🦀️.rs:1405-1410`). For both receipts, assert the complete ordered lane vector is exactly `[Child, Terminal]`. The retained one-command test has the same count-only pattern at lines 46-49 and should use the same assertion.
2. **The first undo state is only cardinality-checked.** The two-command child-edit law checks the first inverse leaves `before.len() + 1` nodes (`➕️add-widget/🧪️tests/🔬️unit/🦀️.rs:34-40`). A defect that removes `note_2`, changes its coordinates, or rewrites prior child content while retaining one node would pass. Build the expected first-undo child snapshot from `content_before` plus the asserted `note_2` at `(40, 40)`, and compare the full snapshot after the first undo. Keep the existing exact `content_before` comparison after the second undo.

These two changes make the recovery report's repeated-publication and two-actual-undo-state claims exact. No other semantic assertion was found to have been removed or relaxed by the fixture conversion.

## Native Evidence

The active root-owned command is `flow-recovery-5`, sequentially running `@semio-tech/flow-plugin:child-identity-check`, `child-edit-check`, and `add-widget-retained-check`; its designated raw outputs are `🗑️generated/flow-recovery-5-{child-identity-check,child-edit-check,add-widget-retained-check}.txt` and `flow-recovery-5.tsv`. It was not read as a completed result and was not started, interrupted, or otherwise affected by this audit.

## Flow Command-Table and Source-Contract Follow-up — 2026-09-09T13:53:17+02:00

This was a static, read-only source audit. No TypeScript test, Rust test, Cargo invocation, Nx task, or native build was run.

The current `FlowCommand` macro in `✏️editor/🦀️.rs` has 34 rows. The six host-wire fixture ordinals now agree with that table exactly: `evaluate=13`, `contextMenuAt=21`, `openSpotlight=23`, `replaceImage=24`, `flowEvalTick=32`, and `flowEvalResolve=33`. The Rust wire witness reads every ordinal from that fixture, so it has no independent stale literal. The `SetGridVisible` unit witness still encodes ordinal 18, which is also its current table position.

One separate command-surface inconsistency remains. `✏️editor/🧪️tests/🔬️unit/🦀️.rs` constructs 36 `every_command()` values while asserting 37, and the two additional values are `FlowCommand::SetContributions` and `FlowCommand::DuplicateWidgetStep`. The current macro is the only `FlowCommand` definition; it has neither row, and no separate constructor implementation exists. `🧫️fixtures/🎬️action-cohort/🔣️.json` repeats the same two non-table routes, declares `routeCount: 37`, and physically contains 36 routes. Its `exactCensus` law is therefore contradictory. The coherent repair is to retire both obsolete entries from the unit/census and set the count to 34, unless the intended contract is instead to reintroduce full macro rows and their app/factory semantics. Changing only 37 to 36 retains the two undefined command constructions. The parent Flow catalog `✏️s/🔌️plugins/🌊️flow/🔣️.json` also contains both IDs and needs alignment with whichever authority is selected.

The checked TypeScript source contract has no obsolete window-owner-trio assertion. Its editor/viewer hook assertions still name current store, presence, and transient disposer hooks. Current Flow transient source uses `ArtifactEphemeralTransferPreparationFactory::new(...)` and `WindowTransientOwnerBundle::new(...)`; the old trio scan returned zero Flow Rust references. This is source consistency only, not runtime acceptance.

The bounded receipt is `🗑️generated/flow-source-contract-command-ordinal-audit.json`.

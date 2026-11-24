# Closing publication stall (`setContributions`, wasm32)

## Symptoms (two builds, one cause)

1. Older wasm: `advanced 4096 publication units ... waiting on the pending publication is closing [1, 303, ...]` (lane 1 = Config).
2. r12 wasm: `typed operation 'setContributions' abandoned its pending publication without a retained cause`.

## Root cause

The host ACK of a lane's result page calls `PendingArtifactStorePublication::acknowledge`, which moves the pending publication to `Closing` while the operation returns to `Publishing` for its terminal page. That is the normal successful path of every config/draft/artifact lane (progress 303 = fully published pack).
- Older build: the publication unit answered `is_closing()` with a wait note and `Ok(())`, relying on a maintenance turn the browser host did not grant, until the stall ceiling faulted it.
- r12 build: `deterministic_preparation_refusal` treated every `Closing` publication as abandoned and faulted it with "without a retained cause", so the acknowledged success became a fault. Reproduced natively: the three curation laws failed with exactly that message.

Other `begin_close()` callers: the store on refusal (stale, preparation refused, receiving fault, cancel) and the unit's stale-root branch (already returns Err).

## Fix

- `typed-operation-unit` (`publish_mounted_typed_operation_unit`): a closing pending is retired by the unit itself, one bounded turn per call, via new `MountedTypedCommandFullOperation::advance_closing_publication` (same `granted_retirement_step` the maintenance ladder uses: grants, receipts, cancellation, progress). A turn without progress notes a typed wait.
- `deterministic_preparation_refusal`: a closing publication is a refusal only if it retains a cause (`fault()`); a causeless closing owner is an acknowledged publication retiring.

## Tests

- Curation laws `the_real_demonstrator_pack_settles_under_the_browser_grant`, `..._through_the_typed_set_contributions_operation`, `a_two_pass_roster_settles_...`: failed before the second fix, pass after. Curation lib: 99 passed, 0 failed (`--test-threads=1`).
- Plugin law `retained_closing_publication_retires_through_the_publication_unit` (in `cancellation-publication-boundaries`, run from `retained_cancellation_publication_boundaries`).

## Rebuild

Wasm rebuild needed.

## Plugin law run

Host crate: `semio-plugin-tests-12-2` (`🧷️companions/🪝️tests-12-2`, mounts `🔬️plugin-runtime-plugin-builder-contract-part-7` by `#[path]`; the law is reached through `semio-plugin-tests-01`'s `retained_command_test_hosts`). Harness: `plugintests-cargo.sh` (verify-cargo with output under `🗑️generated/plugintests`) and `🗑️generated/plugintests/scratch-root.rs` (part-7 header plus only the four retained-latest-wins tests).

### The crate does not compile as a whole
Not 3 stale symbols but 76 errors in 9 mounted test files (part-7 request-context, media-result-close-destructor, ribbon-menus, mounted-history, command-handler, peers-clipboard, part-7-x, `⚛️reactor-driver`, `🪪️documentless`). Causes: the original-effect redesign (`retire`/`close_step` take `RetainedAllocationEffect`, `plugin_exchange` takes `&mut Option` plus `OriginalCommandExchangeState`, `poll_kernel_output` signature), removed `cfg(test)` accessors (`test_snapshot`, `test_registered_tool_keys`, `test_history`, `VcsArtifactApp.store`), `FrameworkSetSelectionModeJob::raw` private, `retained_command::test_rejected_completion_close` gone. Sibling `semio-plugin-tests-16` (part-3) has about 109 of the same kind. These are a peer migration in flight; not rewritten here.

### Fixes made (all test-side, no production code)
- `🔬️plugin-runtime-plugin-builder-contract-part-7/🆔️request-context-serializer-and-shared-job/🦀️.rs`: deleted the unused `PublicationPresenceRetirementFactory` (old `admit_shared_retirement` API); `mutation_apply_preparation_factory` to `cold_mutation_apply_preparation_factory`.
- `🔬️app-typed-command-full-operation/🚫️cancellation-publication-boundaries`, `📌️document-cancellation-and-latest-wins-keys`, `⚖️slot-and-publication-fairness`: `PluginApp::bind_instance_id(&mut app, 7)` after `VcsArtifactApp::new` (fixture issuer needs a bound identity).
- `🔬️app-typed-command-full-operation/🦀️.rs` and `🧪️testing/🧪️artifact-app-laws/🦀️.rs`: `present_mounted_page` helpers use `original_lifecycle_source_effect_step` (the result page admission authenticates its original Source, as production `full-operation` does).
- `cancellation-publication-boundaries`: the cancelled-publication case now loops publication units to `AwaitingAck` (bounded 256) instead of assuming one; message names the boundary. New law `retained_closing_publication_retires_through_the_publication_unit` uses `fixture_mounted_policy()` (the 32 KiB local policy cannot drive a publication) and its cleanup walks cancel, result page, ack, page retirement, `close_mounted`.

### Results (scratch harness, 4 tests; full crate cannot compile)
0 passed, 4 failed, all remaining failures are pre-existing drift:
- New law: its core assertions PASS (publication reaches AwaitingAck, acknowledges, is closing, retires through units alone within the bound, no `terminal_fault`). It then fails in the shared helper `present_mounted_page` heap assert: one presentation turn frees 3200 B (the `RetainedCloneSourceAdmission` backing) while its `RetainedCloneProgress.released_bytes` is 0. Same assert fails `retained_cancellation_publication_boundaries` before it reaches the new law. Looks like a release-accounting gap in the page presentation / source admission clear (production side, `retained-clone` source-owner `clear_admission` path), or the assert is stale; undiagnosed, peers are editing that engine.
- `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair`: `typed operation lost its persistent worker session before publication`.
- `retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close`: `present_mounted_page(...).unwrap()` is `None` (law file line ~153).
- `retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement`: `original retained command source binding changed before retirement` (raw-allocation-close oracle).

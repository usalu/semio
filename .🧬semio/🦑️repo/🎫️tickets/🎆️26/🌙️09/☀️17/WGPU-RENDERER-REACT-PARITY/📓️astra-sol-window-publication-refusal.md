# Window Publication Refusal and Journal Gating

Source packet completed on 2026-09-20. This is the bounded follow-up to `📓️astra-terra-canvas-window-publication-audit.md`, `📓️astra-sol-window-body-publication.md`, and the two production findings in `📓️astra-terra-publication-settings-audit.md`.

## Accepted positive path

Checkpoint 11 proved the ordinary browser path: close all windows, use the retained Display transfer handle, publish a distinct `puzzle3d-main-2` World3d body immediately, then orbit, pan, and zoom it. All five immutable artifact hashes remained stable. Native19 passed 1,172 of 1,173 tests; its only red was the existing Display-law World3d terminal-retirement assertion. This packet leaves that terminal `Drop` assertion intact.

The remaining defects were negative-path ownership failures:

1. `refresh_ui` could refuse `main-2`, then `complete_window_topology_refresh` pushed it back to the front and stopped. A successfully rendered `main-3` behind it could not release its journal and repeated Full refreshes retired its lease.
2. `finish_dock_drag` mutated and persisted the dock before the fixed-credit action queue could refuse item or byte admission. The caller saw an error after an observable topology commit.

## Production contract

A topology publication now owns an optional `u64` journal token. Tokens run parallel to the existing fixed-credit `BoundedActionQueue`; `BoundedActionQueue::remove_at` removes a logical queue position while preserving peer order and exact byte accounting. A later body can therefore release its own action without taking the refusing peer's FIFO head.

`finish_dock_drag` now performs the physical transfer in this order:

1. validate the retained body and publication-entry credit;
2. construct, reserve, and publish the exact `shell.windowSplit` or `shell.windowMove` action into the bounded queue;
3. apply the dock drop;
4. roll back the reserved action if the dock rejects the drop;
5. persist the accepted topology and commit an infallible publication entry carrying the token.

An item- or byte-credit refusal happens before `DockState::apply_drop`, so the dock, active window, persisted layout, body roster, and publication queue remain unchanged. The release branch still routes its semantic up and retires retained capture before returning the exact refusal. `finish_dock_drag` contains no awaited guest or render work.

Direct Display opens and the `Open in New Window` host action continue to have no React transfer journal. They preflight publication capacity, commit the dock, then register a journal-free required-body entry.

## Fixed refresh cohort

`complete_window_topology_refresh` snapshots the publication count at entry and visits each member exactly once:

- a ready body removes and decodes the action selected by its token, then defers it exactly once;
- a live body without a lease increments its bounded retry count and returns to the cohort tail;
- a removed window discards only its own token/action;
- the third refused attempt discards only its own token/action and records one terminal surface fault.

This removes head-of-line blocking without letting newly appended work extend the current refresh. If one peer releases while another remains owed, the settle scheduler drains the released peer before performing the next retry refresh. The released window's own body therefore remains a prerequisite, while an unrelated refusing body cannot delay its acknowledgement.

## React command direction

React's template-drop producer emits `shell.windowSplit` with `{ windowKindId, instanceId }`. WGPU NewWindow transfers use the same command and detail; existing-window moves remain `shell.windowMove`. React's Display Windows tree is drag-only: rows publish `dragData` and no click handler. WGPU's direct `openDisplayWindow` host action therefore publishes a body without adding a transfer journal.

## Shared schema and independent oracle

The neutral window-lifecycle fixture now defines:

- retry ceiling 3 and refusal/recovery/permanent-fault outcomes;
- a two-window cohort where `main-2` refuses once and ready `main-3` releases during the first refresh;
- exactly one eventual dispatch for each token;
- item- and byte-credit refusals with `dockMutation: none` and `journal: refused-before-mutation`;
- `shell.windowSplit` for physical transfers and no journal for direct Display opens.

The TypeScript law validates the fixture with Ajv, independently reduces the cohort instead of mirroring WGPU source, and reads the actual React `handleTemplateDrop` and Display producer for command/MIME direction.

Focused command:

~~~text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace --skip-nx-cache -- bun test ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪟️window-lifecycle-template-drag/🟦️.ts
~~~

Result: **9 passed, 0 failed, 40 assertions**.

## Native laws awaiting the root gate

The packet adds or strengthens these laws:

- `required_window_body_refusal_holds_the_transfer_journal_then_recovers_once` proves a refused ProgramBridge body holds its token until recovery and releases one journal;
- `permanent_window_body_refusal_retires_the_journal_at_the_fixture_ceiling` proves three refusals terminate without a success-shaped action;
- `refused_window_publication_does_not_block_a_ready_display_peer` drives two windows through the actual Display producer and physical Shell ingress, scripts `main-2` refusal and `main-3` success through the real ProgramBridge fixture, and requires the `main-3` journal before the `main-2` retry plus one eventual dispatch per token;
- `actual_display_release_refuses_item_and_byte_credit_before_dock_mutation` fills each fixed-credit dimension, performs an actual captured Display handle down/move/up, requires an unchanged dock and empty publication roster, and requires retired capture;
- `token_addressed_removal_preserves_peer_order_and_exact_byte_ownership` validates shared queue removal and byte ownership;
- the existing `display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag` continues to prove successful body-gated release and the terminal World3d owner lifecycle.

The missing `crate::dock::DockNode` import reported by Native20 was repaired. Rustfmt parsed the changed Rust sources with exit 0 and `git diff --check` is clean. No Cargo, native, wasm, generator, or browser job was launched. Root should run Native21 against this coherent source boundary.

# Terra Post-remediation P5b Source Acceptance Reaudit — 2026-08-24

## Verdict

**RED — reject.** The Sol remediation closes the prior reactor/tracker/retained-registry/sequence/WindowMeasure findings, but two production-reachable document close paths still discard a nonterminal `UiDocumentLease` after exactly one close opportunity. This violates the integrated B3 terminal-first retained close and B5 lossless bounded saturation/handback gates.

This was a read-only source audit after `📓️sol-integrated-p5b-red-remediation-2026-08-24.md`. I read the P5b contract and the prior integrated Terra RED, then traced the current reactor, tracker, reconciler, document arena, renderer, Shell, WindowMeasure path, and verifier. No implementation file was edited.

## Rechecked Gates

| Gate | Result | Live-source result |
|---|---|---|
| B1 — one retained opportunity; no production recursive/dynamic measure conversion | PASS | The mounted reconciler has the retained fixed semantic cursor. Poll dirty work is `DirtyPollOwners` with fixed `SurfaceId`/`UiIntent` lists (`reactor component.rs:994-1022`). `partition_window_measures` and Shell use bounded borrowed fixed lists/stacks; the rail receives `kind.options.measures.as_slice()` rather than cloning it (`Shell component.rs:10667-10669`). |
| B2 — exact fixed owner backing | PASS | `PatchTrackerState` now uses fixed `SurfaceId` slots for deferred/unadmitted state (`patches component.rs:227-241`). Renderer retained surfaces and rejections are fixed, generation-qualified registries. The prior dynamic dirty map/list and retained/rejection maps are absent from their live P5 ownership paths. |
| B3 — terminal-first incremental close | **FAIL** | Shell and command-batch aggregation remove a document owner after one `close_step()` without testing its terminal witness. |
| B4 — checked, transactional, permanent sequence exhaustion | PASS | `RendererSequenceAuthority` reserves with `checked_add`, commits only its reservation, rolls back on reservation Drop, and permanently exhausts at `u64::MAX` (`renderer glue.rs:3162-3219`). |
| B5 — lossless saturation/refusal/ordinary Drop | **FAIL** | Both saturation paths below let a released-but-nonterminal lease fall into `Drop`; it then has no retained closer or exact refusal owner. |

## Blocking Counterexamples

### P5b-R1 — Shell close lane erases nonterminal owners

`ShellState::refresh_ui` calls `close_document_one()` once at the start of every refresh, then transfers all old window/panel/spawned leases to `closing_documents` (`Shell component.rs:2050-2119`). The close helper is not terminal-gated:

```rust
let _ = self.closing_documents.get_mut(0).map(UiDocumentLease::close_step);
drop(self.closing_documents.swap_remove(0));
```

at `Shell component.rs:2129-2135`. A normal nonempty document needs multiple close opportunities: `UiDocumentLease::close_step` releases it and retires only one arena item; it reports terminal only once its handle is inactive (`ui contract document.rs:729-745`). Its `Drop` merely calls `release` when that release did not already happen (`document.rs:749-757`), so after the one explicit step it provides no additional retirement work.

The saturation branch is independently unsound. `retain_document_for_close` rejects a full `UiFixedList`, performs one close step, and then lets `rejected` go out of scope (`Shell component.rs:2123-2127`). A ninth simultaneously retired window/panel document is therefore not kept in `closing_documents`, a durable overflow owner, or an exact caller-visible refusal. `UI_DOCUMENT_LEASE_SLOTS` is finite, while `refresh_ui` may replace multiple windows and panels, so this is a normal production saturation branch—not test-only or a frozen wire-schema field.

The global `close_ui_document_page_one()` fallback does not repair the proof: it retires one arena item but no longer has the particular owner/witness (`document.rs:759-763`). It cannot establish that the Shell's discarded lease reached terminal-empty.

### P5b-R2 — Command-batch output saturation has the same ordinary-Drop escape

`KernelPoolState::exchange_commands` retains a command driver and runs pages until completion (`renderer glue.rs:5136-5191`). It aggregates returned surface documents into the bounded `combined.surfaces`; once full it does:

```rust
if let Err(mut rejected) = combined.surfaces.try_push(surface) {
    let _ = rejected.document.close_step();
}
```

at `renderer glue.rs:5171-5175`. The `rejected` document is then dropped unconditionally, whether `close_step()` returned `false` or not. A command batch can yield more than `UI_DOCUMENT_LEASE_SLOTS` surface documents across its retained pages, so the ninth result reaches this branch. No `RetainedSurface::exchange_closing`, Shell close slot, or caller-visible exact refusal receives the rejected owner.

The corrected `RetainedSurface` itself is terminal-gated (`renderer glue.rs:4384-4400`), which is good but does not cover this independent aggregate-output path. The owner must enter an explicit fixed close/refusal lane before the aggregation loop continues.

### Why These Are B3/B5 Failures Rather Than Acceptable Admission

The fixed document arena is a backing/retirement implementation, not the source-level close owner required by the ticket. Admission may use fixed backing and a close cursor, but it must retain the specific `UiDocumentLease` until `close_step() && terminal_is_empty()`; the source above drops it on a known nonterminal result. This is neither frozen wire-schema data nor an exact handback protocol.

I also checked `advance_retained_one`'s exchange rejection branch. Its current two callers create an empty fixed output list, so that branch cannot be reached with a full list in the current call graph; it is not charged here. The Shell and command-batch branches above are independently reachable and sufficient for RED.

## Corrected Prior Findings That Hold

- The direct run-to-completion reconciliation compatibility method is test-only; the reactor performs the fixed mounted reservation/commit path.
- ACK now consumes the exact published owner before tracker mutation and restores that owner on a rejected tracker advance (`patches component.rs:1758-1805`).
- The semantic map census retains `UiMapCursor` page state and advances one entry without BTree/range restart traversal.
- Renderer sequence reservation/commit/rollback has no wrapping increment.
- WindowMeasure partition/render actions are borrowed/fixed: `WindowMeasureActionRegistry` has fixed slots (`Shell component.rs:6092-6120`); measurement uses a borrowed `UiFixedList` stack (`6144-6163`); Select/Slider/Toggle bindings copy bounded `UiText` only at the action-dispatch boundary. The remaining `WindowMeasure` `Vec`/`String` fields are frozen application schema inputs, not a new live cloned staging owner on the accepted rail.
- The first-render ProgramBridge `AdvanceRetained` opportunity, fixed turn-patch transport, and one-page interpreter/document application remain connected.

## Laws, Mutations, and Test Census

Existing live P5b laws cover the repaired core one-for-one: semantic zero/low-fuel and retained-map cursor progress; cap-plus-one producer identity; persistent-credit handoff; terminal-full unadmitted/rejected/surface progress; max/first-post-max/repeated generation refusal; public Drop handback; owner-first ACK; renderer sequence rollback/exhaustion; and stale/cancel/fault document-builder close. The checked symbols include `semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged`, `terminal_full_plus_matching_*`, `generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation`, `public_drop_handback_is_lossless_at_terminal_cap_and_plus_one`, `published_owner_first_ack_rejects_*`, and `stale_cancel_and_fault_document_builds_retain_closer_until_terminal`.

The coverage is incomplete for the two blocking owners. There is no hostile law or verifier mutation that:

1. fills `ShellState::closing_documents`, verifies an additional lease remains exact and is retained until terminal, and verifies `close_document_one` does not remove a false-returning lease; or
2. fills `combined.surfaces` across a command batch and verifies the rejected ninth lease is retained/refused without ordinary Drop.

The narrow predicate therefore passes while missing these integrated callers. A mutation replacing either terminal guard with the current unconditional `swap_remove`/scope exit would be falsely accepted today. No scoped test file was deleted: working diff had no scoped paths and cached/HEAD status showed only `M`, never `D`.

## Static Checks

| Check | Result |
|---|---|
| `bun -e '…interactivityLiveReconcileSelfTests…'` | PASS: `p5b-live-reconcile-selftest=green`. It validates the narrow P5b reconcile/tracker predicate but does not inspect the two blocking close callers. |
| `bun ./📜️script.ts verify interactivity` | FAIL (8 global findings): four unrelated P1 database findings and four non-P5b layout/intent/TurnResult policy findings. It performed no Cargo/Nx/build work. It does not negate the narrow P5b self-test result, and is not evidence that these two close paths are safe. |
| scoped working + cached `git diff --check` | PASS (no output). |
| scoped `git diff --name-status` | working tree clean in the audited paths; cached/HEAD contain only `M` paths, no deletion. |
| `rustfmt --edition 2021 --check` | PASS for tracker, reactor, document, renderer glue, Shell, ui-wgpu component, and widgets; FAIL for `ui runtime reconcile.rs` (formatting-only diff). No formatter ran in write mode. |

## Bounded Repair Packets

1. **P5b-R1 Shell retained close.** Keep index 0 until `close_step()` returns true *and* `terminal_is_empty()`. On full `closing_documents`, either return the exact lease to the caller before mutation or put it into a fixed overflow/close authority that is itself terminal-gated. Add the two hostile laws above.
2. **P5b-R2 command aggregate handback.** Before accepting a command turn result, reserve an exact fixed output or close/refusal slot. If `combined.surfaces` is full, preserve the rejected `ExchangeSurfaceDocument` in a fixed terminal-gated owner (or fail the request while returning its exact owner) rather than calling one close step and falling through Drop. Add capacity, cancellation, stale, and interrupted-close mutations.
3. Format the changed reconciliation file with the repository's selected rustfmt form after the functional repair, then rerun the same scoped check.

## Acceptance Conclusion

Do not mark integrated P5b GREEN. The remediated core is substantially correct, but B3/B5 require every live owner path—not only `RetainedSurface`—to preserve a nonterminal document lease and its exact close witness.

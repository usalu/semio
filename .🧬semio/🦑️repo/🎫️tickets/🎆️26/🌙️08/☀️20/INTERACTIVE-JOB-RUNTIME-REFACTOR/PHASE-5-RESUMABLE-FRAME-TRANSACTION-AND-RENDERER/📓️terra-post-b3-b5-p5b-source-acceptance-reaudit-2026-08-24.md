# Terra Post-B3/B5 P5b Source Acceptance Reaudit — 2026-08-24

## Verdict

**RED — source-handoff gate not complete.** The latest B3/B5 remediation closes the two previously blocking live owner paths: I found no remaining functional B1–B5 counterexample in the scoped P5b source. However, the required scoped Rust formatting gate is still non-green because `ui runtime reconcile.rs` fails `rustfmt --edition 2021 --check`. Per the P5b source-handoff rule, that prevents GREEN even though the functional/liveness audit is now green.

This is a fresh, read-only source audit. I read the updated Sol remediation and the prior post-remediation Terra RED in full, then inspected the live Shell, renderer command registry, retained exchange, reconciler/tracker/reactor, document arena, WindowMeasure rail, and permanent verifier. No implementation source was edited.

## Functional B1–B5 Reaudit

| Gate | Result | Source evidence |
|---|---|---|
| B1 — bounded work and no working-set materialization | PASS | Mounted reconciliation remains cursorized/fixed. Dirty poll owners retain bounded `SurfaceId`/`UiIntent` lists. WindowMeasure keeps borrowed fixed partition/traversal stacks; `Vec`/`String` in the externally declared measure schema is not re-materialized by the live rail. |
| B2 — fixed, qualified owner backing | PASS | Tracker and retained/rejection authorities remain fixed and generation-qualified. Shell retirement has `[Option<ShellDocumentRetirementSlot>; 64]` plus checked epochs; command retirement has 64 fixed qualified slots. |
| B3 — incremental terminal close | PASS | Shell and command close cursors each advance one slot and remove only after `close_step() && terminal_is_empty()`. |
| B4 — permanent checked sequence | PASS | Renderer sequence reservation uses checked nonzero allocation, transactional commit/rollback, and permanent exhaustion at maximum. |
| B5 — exact saturation/refusal/drop handback | PASS | Shell returns the exact refusal owner/tail; command pages pre-reserve all destinations before guest work and convert saturated output to `Closing`, rather than one-step Drop. |

## B3/B5 Repair Verification

### Shell terminal registry

`ShellDocumentRetirementRegistry` is a fixed 64-slot owner registry (`Shell component.rs:1250-1303`). `try_admit` returns the unchanged `UiDocumentLease` before slot/epoch transfer failure (`1272-1281`). A slot stores its epoch, generation, optional source surface, and lease. `close_one` rotates one cursor opportunity and removes a slot only after both the close result and terminal witness are true (`1283-1293`).

The live refresh path now transfers window/panel maps through `retain_document_map_for_close`; a capacity or epoch refusal reconstructs the exact unaccepted map tail and restores it to the corresponding state field before returning (`2186-2210`, `2267-2277`). Spawned-document refusal restores the same option owner (`2244-2258`). This is fixed admission/handback, not a bulk drain or a transformed replacement.

The two added laws exercise the former failure boundary:

- `shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement` places nine actual lease owners in the registry, proves the first false close preserves all nine, then drains terminally (`1332-1354`).
- `shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation` exhausts the one remaining slot epoch, gets the unchanged owner back, retries it, and drains all retained owners (`1356-1382`).

Stale/faulted leases are also preserved as the exact by-value input: a failed header merely records `surface: None`; it does not replace, drop, or otherwise consume the lease. `UiDocumentLease::close_step` is terminal for an already-stale handle, so the normal terminal guard remains correct.

### Command-batch document registry

`CommandDocumentRetirementRegistry` has a fixed `UI_DOCUMENT_LEASE_SLOTS * UI_DOCUMENT_LEASE_ALIASES` (64) slot arena and a complete `Reserved → Pending → Closing` state (`renderer glue.rs:4599-4740`). Before `run_turn`, `exchange_commands` reserves exactly one eight-destination page (`5313-5323`); a full registry returns before another guest turn can create a document. A turn result moves every fixed surface document into one pre-reserved pending slot, and unused reservation tokens are returned (`5353-5369`).

Every fault/stale/suspend/observe/remove-terminal branch releases unused destinations and converts matching pending documents to `Closing` before the retained command driver is closed (`5324-5408`). `publish_batch` moves at most eight output owners; a ninth output refusal becomes the identical `Closing { surface, document }` owner (`4694-4708`). `command_maintenance_step` advances one close cursor at a time (`4818-4860`), and `close_one` removes only a terminal witness (`4710-4719`). The worker runs this maintenance before dequeuing subsequent requests (`5924-5931`). Instance close also converts these owners and waits for `instance_is_empty` (`5174-5244`).

`command_batch_ninth_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired` proves the ninth owner remains after a first nonterminal close, rejects a stale batch-generation close without mutating it, returns eight owners, and converges the remaining precise owners (`6085-6127`). `command_document_page_saturation_refuses_before_turn_owner_production` fills all 64 reservation credits and proves maximum + 1 fails before a guest turn (`6129-6141`).

### Equivalent exchange overwrite

The formerly latent exchange overwrite is closed: `advance_retained_one` only aliases/pushes while `exchange_closing.is_none()` (`renderer glue.rs:5681-5690`). A failed output push places the exact alias in that closer; subsequent calls first advance its terminal close and cannot overwrite it. Current callers still begin with empty fixed output lists, but the generic branch is now safe too.

## Core Regression and Verifier Review

The prior repaired gates remain present in production source: fixed dirty poll owner lists; fixed `SurfaceId` tracker state; pre-reserved mounted reconcile with the retained `UiMapCursor` semantic census; owner-before-tracker ACK restoration; fixed renderer retained/rejection registries; transactional permanent sequence exhaustion; and borrowed nonrecursive WindowMeasure action bindings.

The permanent predicate now reads the Shell and command production regions (`📜️script.ts:7360-7399`). Its hostile mutations erase the Shell terminal guard, drop the refused Shell tail, make the Shell registry dynamic, remove command preflight, restore the ninth one-step Drop, erase the command terminal gate, remove all four new laws, reintroduce dynamic registries/queues, and restore recursive/cloned WindowMeasure conversion (`7480-7499`). The mutations operate on the real production-shaped regions, rather than token-only fixtures.

## Checks and Test Census

| Check | Result |
|---|---|
| isolated `interactivityLiveReconcileSelfTests` | PASS — printed `p5b-live-reconcile-selftest=green`; baseline and all hostile mutations were accepted/rejected as required by the self-test. |
| scoped working and cached `git diff --check` | PASS — no output. |
| scoped working/cached/HEAD name-status | No deleted test/source path. Current scoped integration paths are modifications only (`📜️script.ts`, renderer glue, Shell). |
| `rustfmt --edition 2021 --check` on tracker, reactor, document, renderer glue, Shell, ui-wgpu component, widgets | PASS. |
| same rustfmt check on `ui runtime reconcile.rs` | **FAIL** — formatting-only diff remains (first at lines 1196, 1305, 1319; further retained-cursor/test formatting differences). No formatter ran in write mode. |

No Cargo, Nx, build, runtime, browser, or network command was run.

## Bounded Remaining Repair

Run the repository-approved edition-2021 rustfmt formatting on the existing `ui runtime reconcile.rs` source, without changing its logic, then rerun the same scoped check. The B1–B5 owner/close/rejection/liveness source gates require no further functional repair from this audit.

## Acceptance Conclusion

Do not mark P5b GREEN yet: all re-audited functional gates pass, but the explicit source-handoff rustfmt condition remains unmet.

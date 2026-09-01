# Resident Release Phase Source Candidate

## Status

Implemented the source slice authorized by `📓️coordinator-resident-r10-repair-authorization-2026-08-28.md`. Before edits, Retained explicitly confirmed no resident overlap, source hold, or active compiler. **No Cargo, rustc, native test, Wasm check, or source-oracle command was run by this lane.** This candidate awaits root review and sole-executor GO; compilation and behavior are unverified.

The actual baseline remains R10's 18 executed/17 PASS/one intended FAIL/.089s, with Data charge `(152,1,1)` already gone after actual System free and exact cleanup. Control remains unexecuted in that historical run. R10 evidence is unchanged.

Exactly two canonical files changed in this implementation turn: resident `🦀️.rs` and its existing `🧪️tests/🦀️.rs`. The reviewed future-seven file, baseline file, fixtures/schema/controller, package metadata, Opening7, Plugin, Store, Kernel, UI and WGPU were not edited. This report is the only new ticket file.

## Mounted Production Behavior

- Replaced the original `retiring` slot with one inline `Option<ResidentRelease>` in the same original root. No new allocation, queue, pool, Arc, Box, public receipt, or refund API.
- Added private origin/allocation/stage descriptors with original partition/charge and exact Layout. Allocated descriptors are neither Clone nor Copy. Refund/Clear contain no pointer or destructor function.
- Original pending admission, admission list, pending consumer and consumer list remain structural. Their source/next/prepared fields are detached only after checked source+destination work; aliases, admission references and exact source emptiness remain barriers under the original gate.
- Replaced combined typed release functions with concrete empty-node destruction functions. Domain C/S payloads and node-owned links must already be empty before entering Destroy.
- Added separately granted Destroy, Free, Refund and Clear. Free retains the actual pointer in the original in-place stage through allocator return; checked arithmetic precedes deallocation. The borrowed original stage is then assigned directly to pointerless Refund—no `unwrap`, error propagation, allocation, or fallible call follows the actual free before that assignment. Full usage stays charged until Refund.
- Kept both genuinely unallocated rollback call sites distinct: failed consumer-reference acquisition and null record allocation. Neither is routed through a fabricated free.
- Added a LedgerState-specific close gate, not generic poisoned access. Ordinary access stays poisoned. Only pointerless Refund/Clear or an exact empty-root check may proceed under poison. The same single CAS/Release guard is used; no poison clear or live-page recovery was added.
- Terminal checking now includes all original pending/list/prepared/release fields, actual allocated bytes, and all Data/Control axes. In cfg(test), it also requires the existing interlock owner absent. Closed-state repeat completion rechecks structural emptiness.
- Added the reviewed dynamic `release_slot_bytes` and `pending_consumer_bytes` diagnostics. Descriptor/root sizes are derived from actual types; no native byte size or fit result is claimed before compilation.

Read-only source census finds one production `std::alloc::dealloc` site, in Free. The old `retiring`, combined consumer/record/page release functions, and their allocated-source refund-before-free branches are gone. The existing Send/Sync implementations and concrete constructor bounds were not broadened; no new unsafe Send/Sync implementation was added.

## Test Mount and Hooks

The baseline include and its original after-System-return observer remain intact. The unchanged seven-law file is now included as the sibling `release_phases` child of the existing test module. The same existing allocator calls its fixed observer after System return, alongside the baseline observer. A small cfg(test) forwarding function in the parent test module lets the production empty-shell step notify the unchanged child observer only after actual destruction returns; no visibility or content edit to the seven-law file was needed.

There is one global allocator. The observation hooks remain inactive outside their test scopes and are absent from non-test builds. No test expectation, capacity, source oracle, native test body, or loop bound was changed. Read-only source enumeration yields **25 prospective tests =17 original +1 baseline +7 new**, not an executed or compiler-enumerated roster.

## Explicit Implementation Detail and Bound Review

One close-control detail is now charged explicitly: if a caller invokes `close_step` without first calling existing `begin_close`, the first admitted step writes only the root's closing bool and returns Pending with one item/`size_of::<bool>()`. Zero/short grant cannot perform that previously uncharged write. Callers that already invoked `begin_close`, including the R10 baseline and exact final-root law, do not incur this extra turn. This is an existing close latch, not a new public API or capacity permit. Root should include this detail in its pre-executor review.

Existing consumer-reference detachment work now sums the actual optional reference slot and the atomic admission-count field written; list-source transfers include their next/prepared writes. No work limit increased.

**No test-bound expression or numeric bound was edited.** The baseline and seven laws still use their reviewed Layout-derived expressions, which will naturally see the changed root/release types. Existing17 fixed drain limits remain unchanged. For the declared three-page fixture, the source phases comprise at most five record steps, six admission/reference steps, six consumer/revocation steps and one final-root step, plus at most the explicit one-step root close latch when not preclosed. This is a source-phase inventory, not runtime liveness or timing proof. Actual Layout values, compiler errors, and any test-bound failure must be preserved by the first native run rather than guessed or repaired preemptively.

Potential ordinary compiler warnings from intentionally retained diagnostic `origin` / Clear Layout fields have not been suppressed. Existing `fetch_update` sites remain. No compiler diagnostics have yet been produced for this candidate.

## Exact Source Release

| Input | SHA-256 |
| --- | --- |
| Canonical resident authority | `e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3` |
| Canonical test/allocator module | `e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175` |
| Unchanged included future-seven Rust | `8949cb8507c798758108a5b77d01221d8c87ff9d5feb2cd4f8522cca67d55019` |
| Unchanged included baseline Rust | `2e73f918e3100c7a232edf22032edfe87ab225ba2d40fa232c3814d5c4420c6f` |
| Both children's unchanged ticket JSON | `2c82d7ad51115a6c5d2dc85bec5d0b2c31818275dcd4f68d7995d6556dcf828c` |
| Unchanged Opening7 Rust | `01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1` |

The actual compiled-input capture must include both path-included Rust children and their JSON, plus the existing resident/package/toolchain/loader inputs. Both include paths were checked read-only; only the baseline had actual prior compilation under R10. No future-seven compile claim is inferred from a valid filesystem path.

The existing `@semio-tech/value-resident-rs:test` target remains no-argument/exhaustive through the shared budget, with no alternate selector or new runner. Root must review this exact snapshot before Retained's sole compiler receives GO. The lane is holding these inputs for that review/capture and owns no compiler lease.

## Explicit Nonclaims

Opening/root backing preadmission, RuntimeAppCell original-parent field capabilities, Store FIFO/displaced-reservation binding, Store detach, unknown/live poison cleanup, arbitrary original-root caller loss, final parent-shell timing, and whole-callback deadline compliance remain separate and unimplemented by this change. Existing structural Option handoffs were not promoted to funded receivers. No Store or Plugin consumer has been switched to this candidate.

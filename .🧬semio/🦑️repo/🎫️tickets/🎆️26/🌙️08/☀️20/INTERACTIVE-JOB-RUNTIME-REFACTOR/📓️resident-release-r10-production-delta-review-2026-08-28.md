# Resident R10: Exact Production Delta for Review

## Actual RED Readback

Read completely: Retained's `📓️resident-release-r10-semantic-red-2026-08-28.md` and `🧪️member-resident-release-r10-2026-08-28.md`, including actual output, assertion, footer, command, and capture description. Actual R10: **18 executed, 17 PASS, one intended FAIL, zero skipped, .089s, Nx1**. The existing17 completed successfully despite the runner's cancellation announcement.

The Data baseline observed exactly one returned System deallocation, size152/alignment8; actual allocated bytes changed152→0. Its original charge `(152 bytes,1 slot,1 owner)` also changed to zero in the same call. Exact root cleanup completed before the intended line85 assertion. There was no secondary abort. **Control did not execute** after that Data assertion. All72 captured tuples and domain membership remained stable. This is the actual semantic RED for the proposed separation; no additional allocator claim is needed or inferred.

This turn changes only this review report. No production repair, future-seven include, test edit, native command, source-oracle replay, or compiler lease has been undertaken.

## Proposed Source Delta — One Authority Only

Production file: `🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs`. Keep its existing public capacity, grants, prepare/read/claim/install/handoff signatures. No second package, pool, registry, queue, Box, allocation source, or runtime dependency.

| Exact existing region | Proposed change |
| --- | --- |
| `ResidentNativeLayout` / `native_layout` | Add the reviewed `release_slot_bytes` and `pending_consumer_bytes`; measure actual changed root and source types with `size_of`/`Layout`, never copied TS prices |
| `LedgerState.retiring` | Replace with the single inline `release: Option<ResidentRelease>`; constructor initializes it to None; all other original pending/list/prepared fields remain |
| New private release declarations | The approved `ResidentReleaseOrigin`, non-Clone/non-Copy `ResidentReleaseAllocation`, `ResidentReleaseStage::{Destroy,Free,Refund,Clear}`, and `ResidentRelease` exactly as declared in the initial packet |
| `ConsumerPage.release` / `release_consumer` | Replace combined destroy/free function with a private concrete empty-node destroy function; constructors capture the same type-qualified function |
| `ErasedRecord.release` / `record_release` | Same split for `RecordNode<S>`; keep original source Layout/charge/allocated metadata until admitted detach |
| `AdmissionPage::release` | Replace combined release with the concrete empty `AdmissionNode` destruction function; deallocation belongs solely to root Free |
| `close_step` | Advance an occupied release slot first; otherwise preflight then detach one eligible original source into that slot; one admitted transition per call |
| `LedgerState::release` call sites | No counter subtraction in an allocated-source detach/destroy/free branch. Full original partition charge is subtracted only in Refund. Existing genuinely unallocated allocation-failure rollback remains distinct |
| Access / terminal check | Add a private **LedgerState-specific** close acquisition path for the approved pointerless sticky-poison residue; ordinary access unchanged; terminal observation checks all original fields and both complete resource triples |

The allocation descriptor holds only original pointer+Layout while allocated; stage Destroy additionally holds the type-qualified empty-node function. After actual free returns, Refund stores only the original diagnostic `Option<Layout>`; original origin, partition, and full charge stay in the enclosing inline release slot. Clear retains those numeric diagnostics but is already refunded. No freed pointer survives Free, and no externally constructible receipt or public refund method is added.

## Preservation of Each Original Source

All source mutation happens only after complete checked work and empty destination preflight, under the original one-attempt root gate. No fallible allocation, user callback, telemetry, or task handoff occurs during descriptor commit.

| Source | Required preflight / exact ownership transfer |
| --- | --- |
| Admission's `node.record` | Original record source empty or never initialized; aliases zero; actual allocated bytes agree with its Layout; transfer that original descriptor directly to root release. Admission and consumer owners remain in their original list |
| `pending` admission | First detach its exact empty consumer reference in an admitted reference step. Keep reservation-only vs raw page distinction. Transfer pending charge and optional actual allocation; never synthesize a free for None |
| `head` admission | Record absent; aliases zero; exact consumer reference absent. Include original next-link and matching prepared-pointer writes in work preflight; move next back into `head`, clear matching prepared pointer, retain current page in release |
| `pending_consumer` | Preserve reservation/raw/initialized state; initialized source must be empty; transfer exact original page or reservation. No public alias is minted by this transfer |
| `consumers` | First revoke writes with existing close flag. Source empty, actual aliases/admissions zero. Preserve existing cfg interlock law. Include next-link and prepared-pointer writes; retain exact page in release before any destruction |

An occupied release slot blocks replacement. Alias acquisition and source eligibility are checked while holding the same gate as detachment; after detachment, the original prepared/list lookup no longer exposes that source for a new alias. Existing raw/pending cancellation and allocation-failure owners are not moved into a temporary cleanup sink.

## Exact Phase Mutation and Grants

Let `R = size_of::<Option<ResidentRelease>>()`. All sums are checked before source mutation; bytes above the unchanged4096 grant remain blocked, not silently clamped or granted more work.

| Phase | Work | Mutation after preflight |
| --- | --- | --- |
| Detach | Actual source slot + R + any original next/prepared fields written | Transfer original source only; neither allocated count nor capacity usage changes |
| Destroy | Actual typed node Layout size + R | Invoke only empty-node `drop_in_place`, then replace Destroy with Free; neither count nor usage changes |
| Free | Actual allocation Layout size + R + `size_of::<u64>()` | Precompute checked allocated-byte remainder; deallocate exact pointer/Layout; immediately install pointerless Refund and remainder; **usage unchanged** |
| Refund | R + `size_of::<ResidentResources>()` | Precompute subtraction of exact original charge from exact original Data/Control counter; assign counter and Clear; allocated count unchanged |
| Clear | R | Clear only the root's original descriptor; no second subtraction |
| Final root | Actual root size | Require release/list/pending/prepared fields empty, allocated0, full Data/Control triples0; mark closed |

Free reads its original pointer/Layout from the in-place private stage. It does not take the charge owner out of the root before calling the allocator. There is no fallible operation between allocator return and pointerless stage assignment. `GlobalAlloc::dealloc` may not unwind; no test will inject such an unwind. Destroy is restricted to a node whose `Option<C/S>` and all owned links are already empty under the gate; no domain payload destructor is accepted as a cheap terminal proof. Actual type field emptiness, not `Send` alone, permits the shell drop.

No new `unsafe impl Send` or relaxed generic bounds are proposed. Existing source creation still requires `C: Send + 'static` / `S: Send + 'static`, and the same private original-root serialization owns those captured allocations. The new private slot inherits only those exact admitted sources; it does not admit arbitrary erased pointers. Existing unsafe erasure must retain its original justification rather than gain blanket thread-safety through the new enum.

## Sticky Poison and Final Observation

Ordinary `try_lock` and all forward APIs retain their existing sticky-poison rejection. The specialized close path uses the same one-attempt CAS and never clears poison. Under poison it permits only an already-pointerless Refund/Clear slot, or the exact empty-root final/terminal observation. It cannot choose another source, invoke Destroy/Free, walk live pages, or return a generic mutable guard through a public API.

If other live/pending roots remain after clearing a pointerless residue, the next close remains Poisoned with those original roots retained. Unknown/live poison cleanup is **not** solved here. The existing poisoned-scalar law remains unchanged. The new proposed poison law uses one actually freed consumer allocation and a concrete unit panic after allocator return; only its original pointerless charge is finished. Full terminal checking must include prepared pointers, release, all other original sources, allocated bytes, and every Data/Control axis.

## Test Mount Boundary for Later Authorization

The actual baseline remains mounted and unchanged. The future seven remain unchanged ticket-only source `🧪️resident-release/🦀️.rs`, SHA `8949cb8507c798758108a5b77d01221d8c87ff9d5feb2cd4f8522cca67d55019`:

```text
resident_release_record_keeps_charge_after_actual_free
resident_release_cancellation_covers_allocated_and_reserved_frontiers
resident_release_short_grants_preserve_every_original_phase
resident_release_aliases_block_destruction_and_live_payload_drop
resident_release_concurrent_close_frees_and_refunds_once
resident_release_poison_after_free_keeps_pointerless_charge
resident_release_metadata_is_inline_and_measured_before_detach
```

Their existing exact tuple/schema and all-axis partition laws are not weakened. Once separately authorized, the future-seven test child would be included in the same existing test module, with its fixed observer called after the existing System return and after actual empty-shell destruction. The existing baseline hook stays intact; observers are independently inactive outside their specific scopes, and there remains one global allocator. No production callback/observer is added: hooks are cfg(test)-only.

Expected combined source roster after that later mount: **17 original + 1 actual baseline + 7 future laws =25**. This roster is not compiled or executed yet. The canonical no-argument resident target remains unchanged; sole-executor coordination is required for any next run. R10's Data-only failure must not be relabeled as a passing Control/phase test.

## Parent/Store Boundary Still Absent

Read current compiled authority and confirmed directly to Retained: it still has no privately issued RuntimeAppCell→Store FIFO field receiver, funded original-parent receipt, or exact displaced-reservation binding. Existing consumer/admission/record handoffs still take structural `&mut Option` slots. This production proposal does not convert those into funded targets.

The selected Store shell remains one `ResidentRecord<ArtifactStoreBackboneRetirement>` with an eventual exact private binding in the existing FIFO. Its original parent must retain that binding through Clear, without retaining a public record-access alias through Free. Opening root backing and the changed inline Layout still require real preadmission. Store detach must remain blocked until that distinct original-parent capability is mounted and proved; OS Send/helper compilation cannot substitute for it.

## Current Read-Only Receipt

After reading R10 and preparing this review, observed unchanged:

- Canonical authority: `508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f`.
- Canonical existing17 + baseline hook/include: `987e2ba2933b15a79a3334b799e35830a3af99cf0b565babb338d4912f67ec1a`.
- Included baseline: `2e73f918e3100c7a232edf22032edfe87ab225ba2d40fa232c3814d5c4420c6f`.
- Future-seven Rust: `8949cb8507c798758108a5b77d01221d8c87ff9d5feb2cd4f8522cca67d55019`.

Only this Markdown report was authored during this review turn. Production, all18 mounted test inputs, future seven, Opening7, and shared schema/controller were not edited. Source repair awaits root review; no compiler lease is held.

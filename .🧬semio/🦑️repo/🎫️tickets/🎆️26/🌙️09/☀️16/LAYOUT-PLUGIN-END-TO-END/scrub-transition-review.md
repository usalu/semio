# Move-only Scrub Transition Review

2026-10-29. Read-only actual Scrub/runner/ledger/fixture/oracle inspection. No implementation, build or tests. Recommendations sent directly to Rust pure transition owner and Medium mounted adapter owner.

## No Payload Duplication Is Semantically Required

ScrubMachine actions receive borrowed event and scrub_replace clones each leaf into Upsert commands; this is an effect interface artifact. Scrub::persist clones transaction entries into persisted state. ScrubLedger::send clones previous state to resume while preserving rollback copy. These three paths explain M:Clone. A Tick semantically retains latest absolute leaves only; Commit publishes latest absolute leaves and retains no open payload; Abort publishes no payload. No state/event transition requires identical M in both new state and output.

Existing runner has fallible generic effect settlement and discards transaction on refusal, so removing Clone bound alone cannot preserve rollback. A typed owned transition should first decide from borrowed control metadata and chart state, validate read fence/lateclosed/tool/base/gesture/restore and admit all metadata/output backing before consuming owners. Then one infallible commit moves actual input leaves to next state or committed output, and moves displaced previous state into an explicit retirement recipient. On refusal ledger and Option<Input> stay exact originals.

## Ownership Matrix

Tick same press: replace old absolute entries with input leaves; keep transaction identity, retire displaced old payload. Shrink needs no cloned retract payload; prior tail lives only in displaced owner. Tick another gesture/tool/base: retire entire prior row, mint new transaction using actual actor/clock/tool and move latest leaves. Commit same press uses existing transaction identity but newest release leaves; idle Commit is one-shot; empty Commit retains current no-history semantics. Abort and lateclosed input must not silently destructor-drop move-only input or old payload; return residues to original controlled close. Stale named abort preserves unrelated open press. All matching closed/live counters/source_revision updates must occur only at successful commit boundary.

No fake Clone, erased bound or compatibility facade is justified. Metadata-only transition decisions may follow existing chart tables and retain manifest/fingerprint proof; actual payload effects are ownership moves. Transaction map/key/String allocations also require prior admission in mounted adapter; a move-only API is not itself proof of zero allocation. Retained read fence returns Unclosed before consuming owners; checked source revision exhaustion must likewise be preflighted.

## Existing Neutral and Third-party Oracles

framework/tool-machine/tests/fixtures/scrub-law JSON has13scenarios: same-press net value/late tick, host abort, two presses, idle release, empty leaves, shrink/grow, another gesture, moved base, another tool, independent windows/freeze, roster retirement, stale abort and abort input. Native unit/scrub-laws pins chart tables/fingerprint/manifest/phases and replays exact fixture projected step/open state. Preserve all original negatives and transaction minting/order.

Independent conformance TypeScript lines488–679 builds ScrubOracle with xstate setup/transition from fixture chart, separate Map transaction semantics and fast-check random resume/late sequences. This is a viable existing independent oracle; no new runtime dependency is needed. Extend actual native move-only law with payload pointer/nonClone owner, displaced/residual retirement counts, source-read-held refusal and occupied output/funding refusal preserving oldstate+input. Compare projected semantic output against same neutral corpus/independent oracle while separately asserting ownership evidence.

## Limits

This report proposes a valid transactional seam, not a completed design implementation or runtime proof. Rust owns proper pure transition API; Medium owns mounted original StepContext, allocation admission and controlled retirement integration. Exact native semantic/ownership execution remains pending coherent source and owned gate.

## Existing Funded Storage Interfaces

Inspected value resident/dictionary/ordered/lookup/native decode. Immutable OrderedMap<V> has real begin_set_shared/begin_remove_shared UpdateCursor with exact next_copy_byte_demand/next_capacity_byte_demand/next_depth_demand and advance grant checks. Capacity quotes actual Entry/Node Arc layouts and AVL rebuild/rotation nodes; removed value and obsolete roots remain explicit shared/close custody. SharedOwner::admit prices allocation_bytes before move and returns unchanged V on refusal. Native ordered insert shows actual NativeDecodeControl cumulative charge plus same cursor; it is synchronous cold retirement, not mounted close shortcut. NativeDecodeControl::allocate_vec<T> checks count*sizeofT, charges then try_reserve_exact. ResidentRecord install/handoff_into preserves source/target until granted but is a typed cell, not keyed map. FixedKeySlots is noalloc Copy-key finite registry, not arbitrary String map. None of these makes opaque stdBTreeMap entry insertion fundable by sizeofentry.

OrderedMap is not drop-in mutable RetainedOriginalSlot storage (no get_mut); source slots need their own existing indexed ownership or immutable row design. Rust subsequently identified RetainedOrderedMap/InsertCursor bounded paged map as a better direct fit and owns exact implementation; no unnecessary storage rewrite was proposed here.

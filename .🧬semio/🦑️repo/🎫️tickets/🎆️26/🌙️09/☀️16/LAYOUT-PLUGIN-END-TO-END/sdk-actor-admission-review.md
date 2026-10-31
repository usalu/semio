# SDK Actor Admission Review

Read-only source inspection on 2026-10-29; concurrent source-owner fixes were reread. No builds, tests, native jobs, cache writes, or source edits were performed.

## Production Demand Paths

Root: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

`🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs:237` forwards the original StepContext directly to press admission with zero tag-copy cost. Lines 270–272 use the same admission path with the operation registry transfer copy cost. Lines 277–279 forward that context to typing admission with the equivalent registry transfer copy cost. These copy costs describe Vec tag transfer, not actor allocation; neither caller duplicates actor capacity demand or manufactures a new grant.

Current press `🛠️tool-machine/🧵️settlement/🦀️.rs:22–33` has one authoritative admission_demands path. Capacity includes owner box, leaves backing, actor UTF8 bytes, 64-byte encoded base, and SharedUtf8::admission_demand().capacity_bytes. The birth calls SharedUtf8::admit and adds its returned frame capacity once to the actual aggregate receipt. OriginalConstructionDemand remains one item, and no nested native capacity charge was added.

Current typing `🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs:28–42` separates owner box birth, parent-source loan admission, and actor capture across calls. capture_demands includes actor UTF8 bytes plus the shared frame; capture uses that same demand and returns actual text capacity plus the frame receipt once. Upstream callers retry this owner until captured. There is no additional upstream actor quote mirror requiring a frame addition in the inspected paths.

SharedUtf8 admission contract is `🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/🦀️.rs:19–26`: zero copy bytes, one item, depth one, independent shared String frame capacity. Thus the hardcoded depth one in these aggregate current demands matches the actual primitive contract.

## Earlier Coverage Gap — Resolved by Final Source Review

The current Layout quote-minus-one and exact-capacity laws do not independently measure physical allocation at the two actual actor births.

Layout test root: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️transform-tool-transactions`.

- Main `🦀️.rs:519–524` denies press capacity quote-minus-one, checks unchanged tag/output/backing/receipt, then accepts exact capacity with one item and compares declared cumulative receipt delta against quote.
- `⌨️typing-edit/🦀️.rs:32–35` does the equivalent at typing capture after the owner and parent loan already exist; it asserts actor-byte-plus-frame quote and tests two captures in each cancel/commit branch.
- Both invoke OriginalFixtureScope.cx().with_retained_loan directly. Their actualCapacity values come from retained_progress, not HeapWitness. They can validate demand/receipt agreement while still missing a shared omission in both.
- `🧪️testing/🎟️caller/🦀️.rs:49–54` exposes the fixture context without a physical observation wrapper. Its physical decoder-close observation at line 40 does not cover body actor births.

Existing SDK physical pattern is `🛠️tool-machine/🫧️host/🧪️tests/🦀️.rs:7–11`: global HeapWitness, observe_heap_allocations_on_this_thread around caller.turn_with_decoder, then exact requested/released versus retained/released receipt equality. However, its tests exercise OriginalScrubDispatch, not OriginalPressSettlement. Likewise `🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🧪️tests/🦀️.rs:6–10` has that physical wrapper but exercises OriginalTypingCommit, not OriginalTypingSettlement.

Required precise coverage is physical observation around each denied and accepted actual birth/capture loan in the Layout laws: denied heap requested/released zero and unchanged receipt; exact one-item accepted physical requested/released equals that loan receipt delta including the actor frame. This gap was sent to the source owner. No claim of native passing follows from this inspection; framework run 84839 excludes SDK, and SDK/Layout native gates are still required.


## Final Recheck

The physical-coverage gap above is resolved in source. Transform test root `🦀️.rs:10–13` now owns the single ACTOR_HEAP global allocator and actor_admission_turn helper. That helper observes actual per-thread requested/released heap bytes around the supplied call and compares them with actual StepContext retained-capacity/release deltas before unwrapping the result. It does not create or replace a receipt.

Current press minus-one and exact calls at lines 525 and 527 both pass through this helper. Current typing capture minus-one and exact calls at `⌨️typing-edit/🦀️.rs:33–34` both pass through the same helper. The denied branches retain their unchanged full receipt assertion; combined with heap equality this requires both requested and released physical bytes to be zero. Exact calls still require one admitted item, complete canonical capacity, and reported capacity equal to that capacity. Existing tag, leaf backing, source binding, partial move, cancellation, and terminal closure assertions remain.

The transform source tree contains one global_allocator declaration. The typing module is mounted from the transform root and uses its helper through super imports. The plugin crate allocator is cfg(test), so its declaration is not enabled merely by consuming the plugin as this separate test binary dependency. The transform manifest places semio-framework-trace solely under dev-dependencies, at line 41.

The reread production methods still use the same authoritative admission_demands/capture_demands from actual births, add the canonical shared frame once, call SharedUtf8::admit under the actual retained grant, and return one aggregate native construction receipt without nested control.charge. No additional upstream actor demand mirror was found.

No concrete current-hunk defect was found in this focused recheck. This is source verification only: no tests or native jobs were run by this reviewer, and no native pass or observed six-record output is claimed. SDK/Layout execution and comparison of the fresh native records against the reference remain required.

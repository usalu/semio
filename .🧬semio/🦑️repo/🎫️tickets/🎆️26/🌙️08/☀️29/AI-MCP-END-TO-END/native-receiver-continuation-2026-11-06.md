# Native Receiver Continuation — 2026-11-06

No native or MCP end-to-end pass is claimed. Root's sole AllocationReturn --lib qualification is live as session23348/PID39484. Do not submit NativeOwned/Default until Root explicitly declares the coherent checkpoint after that result. AllocationReturn and ControlContract source/data/tests must remain stable.

## Mounted Work

NativeOwned's canonical factory separation is mounted. NativeReceiverFactory<I,T> lives in native-receiving-contract; the local NativeReceiveFactory witness selects the same concrete NativeReceive receiver. Standard-type implementations stay local, avoiding orphan implementations. Slot, Option, Scalar and String implementations and the generic backing retirement helpers live in native-leaf, with exact NativeOwned aliases/reexports. No DTO or fallback implementation was introduced.

NativeSequenceCollection and NativeSequenceInput live in native-reservation. Reservations retain exact selection/actual allocator effect and accept the immutable actual extent DATA, not const N. PagedBytes and CausalPayload external adapters belong to Launch. Existing capacity covering the requested extent needs no additional reservation.

NativeRecordMatcher now has funded key access, cached real child projection, retained active/pending/frame owners, bounded comparison, actual reservation/push/pop/release and full marker-aware retirement. NativeArrayReceiver and dynamic NativeTreeReceiver use preborn value/retirement/backing effects, actual work view, bounded processing, preserved partial ownership and full close. NativeOwned's handwritten private Row test receiver was migrated without deleting its private identity, phase, field, callback-refusal or output-denial assertions.

NativeArrayDefaultCursor production is rewritten against Root's exact current NativeDefaultCursor interface. It has own-only is_closing/begin-close, separate paid child closing intent, real value/retirement effects, mandatory maxWork quotes, marker-last cleanup, initialized-prefix retention, typed internal placements and public take copy0. Zero-N defaults now produce a real empty array under paid metadata publication; they are not permanently refused.

The Default facade explicitly reexports native_decoding and full OriginalRetirementOutcome/Failure; cfg(test) source_fixture is canonical. Its Cargo dev dependency includes testing-admission. Tests/original-default-placement.rs exercises String zero-copy denial/full actual Option<String> quote/no allocation, plus zero-work refusal and real [] success for zero-N arrays. This law is authored and unrun.

## Canonical Contracts

RetireOwned uses type Retirement and fn retirement(self)->Self::Retirement. RetainedClone keeps its separate Cursor.

Marker LAST on clone advance/close, Source close, binding.close_one, projection.close_step, return_inputs and take_authority. Never use an ephemeral &mutNone where an allocator can run.

NativeReceiver.close_step(g,actual) and NativeDefaultCursor.close_step(g,actual) return full OriginalRetirementOutcome. NativeDefault demands(body,maxWork), advance_work_demand(body,maxWork), take_work_demand, close_work_demand(body), advance(control,body,maxWork,g), pending recovery and own-only begin_close_admitted are mandatory.

New canonical Source prices from Server: RetainedCloneRef.binding_work_demand(binding) =0 existing/6 new; binding_copy_bytes is full typed metadata. RetainedOwnedProjection.constructor_work_demand() =7 and constructor_demand() includes full alias/Self placement. Our production leaf/sequence/tuple/array/schema/tree binding selectors delegate the real port. Projected child selection adds canonical constructor work plus actual accessor/capture operations. Source binding fixture bindingWork changed6; successful scalar read+capture2 and failed read1 are reported distinctly.

NativeOwnedStorage provides reserve/move/take_box/release demand/work ports. Canonical backing helpers use take_box_demand pointer-only and delegate actual Storage work ports. Retired empty Box release pays Option<Box<C>> clear and exact physical deallocation. Public receiver take remains pure ownership handoff/copy0; internal typed captures/storage placements are separately priced.

Source normal/recovery callbacks now receive (native,actual_body_grant), with a preborn Option<OriginalNativeReceivingOutcome> supplied by the caller. No second manual Source posting. NativeOwned itself has no direct cx.with_native_receiving call sites; its standalone test helper issues the actual finite NativeDecodeControl work/construction loans.

## Test Helpers

Canonical testing-admission now has close_step(actual,g,available_work,demand_result,quote_result,apply)->OriginalRetirementOutcome. Existing actual marker is cleared in a separate one-item/sizeofOptionusize/one-work step before another child action. Otherwise real demand/work are checked before effects, actual marker is checked against physical capacity receipt, and full error is retained. External heap observation remains independent. Pure close_demands/close_work_demand helpers select the marker stage. Root's close_owned helper independently retained/validated/paid-cleared actual markers.

New default-discovered testing-admission tests/original-close-marker.rs drives genuine Vec<String> retirement, zero-work denial, marker retention, exact paid clear, independent heap receipts and complete release. Unrun.

Most NativeOwned test close and ControlledRetirement drivers now use persistent actual markers and finite existing caller work. Cell-budget take tests and retained Vec output close were migrated. Current scanner finds no obsolete default_cursor() calls, no one-argument allocator close calls except allocation-free NativeSequenceReservation, and no receive_retained_work in NativeOwned tests. The special shared recovery Construction recipient was made finite and cumulative instead of deriving grant from body demand. Remaining tests may still need compiler diagnostics, especially changed lower Source pricing.

## Evidence

Latest 27-root rustfmt parser scan: zero syntax errors; exit1 is formatting differences, not type qualification.

Neutral AJV/SQLite batch:7 tests /516 assertions passed across scalar, tree work, typed-default/zero-N, and persistent-list laws. Persistent-list fixture import erroneously pointed at a schema; it now imports the actual data. Stale strict schema fields were synchronized without deleting cases. Child custody now models reserve, initialize, pointer-only take-box, release-box and release-raw at both4-byte and8-byte pointer layouts. The independent SQLite model agrees on all20 custody rows. Scalar test observed static RED then GREEN for actual read outcome work.

Fresh NativeOwned cfg(test) census before latest small changes:27files/249342bytes/1138activeLines, within unchanged32/262144/4000. Leaf1file~35KiB, tree1file~27KiB, matcher1file~19KiB, reservation1file~19KiB. Normal owner graph30owners/142edges/0cycles with explicit normal path dependencies, workspace membership, exclusions and path-length checks. Re-run current cap census before native submission.

## Remaining Work

1. Await Root's sole AllocationReturn result/coherent checkpoint; no duplicate native.
2. Finish static/API census for SnapshotProjection tests. They still have old begin_projection(g), advance(source,body,max,g), close_step(g) and old direct RetirementCursor calls. Production projection belongs to Root; these native tests are ours.
3. Recheck lower Source binding/projection price fallout in neutral fixtures and helper assumptions. Actual native/source admission expectations may change after Sealed birth pricing.
4. Compile current NativeOwned and NativeDefault library/tests once authorized by coherent checkpoint; include native-tree original-tree-work, Default original-default-placement and helper original-close-marker law packages/targets deliberately. Use fresh OS+all-store lease census and4h ticket wrapper; do not retry a missing handle without actual census.
5. Preserve all failed/partial owners and complete causes when compiler/native diagnostics expose gaps. No cap increases, skipped assertions, copied source backups or legacy overloads.
6. Keep actual Note WASM publication, command/action/inference/history/restart and attached UI gates pending. Source/neutral/parser evidence is not native readiness.

Key scripts are ticket inputs: native-factory-owner.ts, native-default-placement-law.ts, native-default-test-construction.ts, native-default-test-work.ts, native-row-test-effect.ts, native-test-close-markers.ts, native-controlled-test-markers.ts, native-child-custody-model.ts, native-all-owner-rustfmt.ts and native-owned-boundary-census.ts. Do not rerun source-creation scripts against changed files; use fresh narrow CAS.

## Continuation: Derived Receiving And Direct Close Frontiers

The canonical NativeReceiver contract and NativeReceiveSlot now expose own-only is_closing/begin_close_demand/begin_close_work_demand/begin_close_step(grant,maximumWork). The selected one-bool transition is paid before any direct child intent; closing a parent no longer recursively marks the child. Canonical backing cleanup selects the actual underlying receiver intent and then its genuine close work. String selects the RetainedFieldCursor close-start work before its close. Option, Sequence, Array, Tuple and Unit wrappers select their direct child intent independently. Array place quotes the actual child-closing bool with its initialized-prefix writes.

Both actual NativeReceive emitters are mounted with a new retained State separate from their preborn Value and OriginalRetirementFailure effect owners. Record and enum plans cache genuine RetainedOwnedProjection children only inside funded callbacks; quote paths borrow these cached proofs and call only accessor-work quote ports. Generated records use the actual RecordStorage reserve/initialize/place/remove/take/release prices, retain defaults, pending typed fields and all initialized fields, and retain every original alias until funded close. Generated enum carriers preserve schema/tag/content/rename/recursive semantics and retain the selector through physical cleanup. Generated public take remains the canonical pure ownership handoff with zero payload-copy receipt. Enum defaults now use the real mandatory typed NativeDefaultValue/NativeDefault admission and cursor signatures.

Normal generated callbacks refuse semantic advancement while any parent or child recovery is pending. Full Value and Retirement effects persist across callback/receipt failure. Marker-last close paths forward the actual allocator marker; zero-capacity branches use a local absent marker only where the selected quote proves no physical allocation.

The correct owning derive expansion root is value/✨️derive/⚙️expansion/🦀️.rs, not the proc-macro facade. Latest actual cfg(test) closure measured 10 files / 230725 bytes / 2765 active lines, before the last small recovery gates. Its Rust parser reports zero syntax errors; rustfmt exit 1 is formatting differences only. The separate facade closure was 8/138027/1125 and was not an emitter qualification.

The neutral derive frontier law was observed RED against the old emitter and is now GREEN: strict AJV plus SQLite arithmetic, 1 test / 14 assertions. The retained selection requires accessor work 1 + real alias constructor work 7 + two real parent placement writes 2 = 10. Server subsequently added the shared close-intent corpus; its portable run reports 2 tests / 30 assertions. Actual expansion AST laws preserve all prior tests and add records, tuple structs and tagged enums, getter-free quote checks, mandatory work/recovery/marker interfaces and retained independent effects. They have not run natively.

Normal Tuple and Sequence parent placements were audited and corrected. Tuple child capture, alias capture, source clear/index/stage changes and paid close intent now have their actual CPU and typed metadata quotes. Sequence length/count, alias capture, child capture, push parent index/phase, direct close intent, source clear, child-slot reset and final output publication are similarly selected before effects. Server is migrating the canonical collection push result from Option<Progress> to RetainedCloneStep so genuine partial UI insertion retains its real progress. Sequence phase 3 is already mounted against that exact protocol: Progress returns the actual child receipt/work without changing index or phase; Complete applies the two funded parent writes. Server owns trait/Vec/Paged/UI implementation migration and the narrow Array vector test branch.

The actual Vec push primitive is priced for its donor Option, typed row and length metadata (3 CPU) and refuses insufficient work/grant before taking the donor; Server is preserving this in the new result protocol. A stale external UI map adapter was discovered at ui/contract/values/fixed-map-and-bytes/receive: the old whole try_insert cannot count as bounded receiving. Server owns its real phased implementation; there is no qualified fallback.

The public sequence-storage admission law was migrated to actual retained NativeReceivingEffect, explicit finite WorkLoan and construction recipient scopes. Its original null-allocation, denial, physical pointer/capacity, duplicate destination and retirement assertions remain; typed Vec metadata copy replaces the old false zero-copy admission assertion. The actual Array inline law now forwards maximum work, retains the allocator marker through separately paid clear, pays the receiver's own close intent, preserves initialized prefix/denial/content assertions and uses the real 6-work placement receipt. These are authored, not native-qualified.

Latest NativeOwned census after Root's genuine selector owner factor was 27 files / 244638 bytes / 1132 active lines. Schema owner 2/35192/126, Leaf 1/36577/128, reservation 3/29871/358, receiving contract 1/4578/31. All 27 selected Rust entries parse without syntax errors (format differences only). A fresh cap census is still required after the latest placement changes.

Native execution remains frozen pending Root's sole allocator baseline retry session 94772. AllocationReturn and ControlContract production were untouched. Prior 23348 reached actual compiler/tests (10 pass, 1 stale expected-work assertion) and Server corrected the test only. No NativeOwned/Default/derive replacement native run was submitted from this lane. No actual Note WASM, action, command, inference, history/restart or attached UI success is claimed.

## Qualification Request And Subsequent Independent Receiver Laws

Root's sole AllocationReturn baseline retry 94772 completed with actual exit 0 and all 11 native tests passing (generated/mcp-42968.json). This qualifies that lower baseline only. Root authorized exactly one next derive-expansion --lib run.

Before submission, the actual doctor process census found no matching target; the actual all-seven-store lease result generated/mcp-23396.json also had no matching target (rechecked with the full derive.*expansion path pattern because displayed commands truncate before the package argument). The submitted request is native-derived-expansion-test.json, via native-derived-run.ts/repo-mcp.ts, retaining a four-hour finite client. It is exec session 68020, MCP PID 48612, actual CLI PID 21656, parent 39996, repository session 5cea31605a9b.a1um4ils79z, agent codex-guest-derived-expansion. Full progress is retained by the wrapper. It remains live: genuine planning-capture/dependency source admission/CargoLock refresh/workspace selection have occurred; no target compiler/test result yet. Generated lease read mcp-22644.json showed actual lease progression through task 185 configuration view, 198 registry and 219 compiler writer. There is no duplicate run.

The emitter source remained stable after submission. Its current correct owning expansion census is 10 files / 231185 bytes / 2765 lines, within unchanged caps.

Independent NativeOwned follow-up now prices scalar construction as its actual Option<T> placement (public take still copies zero payload); parent work uses checked addition and preserves retained output/physical receipt on overflow refusal. Strict AJV, SQLite and nullable-integer validation pass, 1 test / 6 assertions. The corresponding native scalar test asserts sizeof<Option<u64>> and pays its own close flag before cleanup.

PagedList empty storage construction now prices the actual optional storage header before installation, preserving zero-allocation behavior. The public storage native law tests zero-copy denial/no owner/no work/no allocation and full typed quote success, using the actual construction/work issuer. It remains uncompiled.

The collection push protocol is now RetainedCloneStep: Progress preserves exact child progress and work; only Complete permits parent cursor advancement. A new private initialized-prefix law in Array's existing test module exercises the real generic SequenceReceiver with a genuine RetainedCloneSource binding and actual Native construction/work issuers from a finite test Wallet. It starts from an explicitly funded retained test prefix, verifies Progress changes only the child bool with index 0/phase 3/donor retained, then verifies Complete performs exactly two additional parent writes with index 1/phase 4/donor returned. It drains all real receiver/source owners with persistent allocation marker and separately paid marker clearing.

This is a generic receiver prefix unit law, not production OriginalCallerSource/Actor/UI proof. It uses a concrete no-heap first-party test collection to expose the partial/complete frontier; Server's real UI law separately tests the production phased UI map. The same closed portable storage corpus passes strict AJV plus SQLite: 1 test / 17 assertions, including finite work 0/1/2 denial and 3 completion, optional empty storage shape and exact partial-parent cursor/donor results.

After Root repaired a temporary delimiter in its NativeReceivingOriginal region, the fresh actual NativeOwned closure measured 28 files / 257078 bytes / 1248 lines. No cap was raised and no assertion or case removed. Roughly 5 KB byte headroom remains. The parser run that caught the original delimiter captured the old version; a fresh parser pass remains needed after all concurrent receiver changes settle. No NativeOwned or Default native target was submitted.

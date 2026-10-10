# Local Receipt Fix Readback

2026-10-09 read-only current source audit, no compilation/tests run. Root reports red31991 had one pass/one fail/three not run; updated native18182 ended at infrastructure invocation before compiler, zero assertions. None of the updated laws is credited as passing here.

## Fixed Custody

Both native advance methods now compute checked next and reject overflow/declared workload excess before assigning completed. This resolves the public same-stage WorkLimit corruption reported in local-receipt-audit.md. A checkpoint refusal after valid assignment leaves valid actual performed work in the receipt, which is appropriate.

Both local bound hop methods borrow and validate the stored receipt before take. Their validation exactly covers current resume's owned/maximum and completed/total conditions; resume adds no fallible callback or allocation. Therefore current resume cannot fail after take for a receipt accepted by that precheck. Invalid internally constructed receipt remains accessible for owned_bytes/detach. Scope still restores the actual scalar receipt on operation refusal after resume. Keep precheck and resume validation synchronized when adding future invariants.

Detached release_recipient_box keeps the original consuming receipt and caller Option<Box<Recipient>> borrowed. Identity mismatch, occupied recipient, invalid scalar receipt, missing depth/release grant and cancellation all refuse before take; zero item grant returns zero performed work before touching the box. Genuine release drops exactly the original empty recipient box, returns one item and its concrete layout size, leaves native cumulative owned_bytes unchanged and leaves caller slot empty. Second release refuses missing box. Borrow lifetimes terminate through pause/detach before rebinding or final Option transfer; no apparent borrow conflict was found by source inspection.

The API relies on caller admission for recipient birth, as other borrowed recipient installation APIs do. It does not independently prove that any arbitrary caller-created box was previously charged; the new physical laws explicitly charge and observe the real birth. The detached scalar remains after final box release; its unique identity cannot bind to a newly created recipient, so no new recipient authority is produced.

## Test Paths and Physical Evidence Design

Decode tests module is mounted under decode continuation and uses `../🧫️fixtures/🔣️.json`, which resolves to its continuation fixture. Encode invalid-receipt module is mounted under encode continuation and its `../../../../🛬️decode/...` path climbs from tests to Value before selecting Decode fixture; this is correct. New rejectedWork/invalidReceipt fields exist in that fixture. Both test modules can access ancestor-private receipt fields through super. The package root exposes pub mod value and reexports value types; crate::value::observe_retirement_allocations is genuinely exported pub(crate) under cfg(test), and its existing global allocator provides the birth/free event collector.

Two physical laws now measure actual Box<Recipient> birth, real String plus ControlledRetirement wrapper birth, each close's heap births/releases against its reported receipt, and actual final recipient-box deallocation against final release receipt. The aggregate equality includes recipient+wrapper+text+close-born capacity. Oracle serialization happens outside the observed spans, avoiding contamination. The separate initialBytes charge is a deliberately scalar prior admission; no actual heap birth is asserted for those nine bytes. Thus the exact physical equality concerns the real allocations observed by this law, while cumulative native accounting additionally retains that prior scalar charge.

The two refused-work laws exercise the actual public stage-overrun route then complete that same stage. Decode and Encode invalid tests deliberately construct private invalid receipt state to verify defensive non-consumption; they do not claim external callers can forge receipt fields.

## Remaining Qualification / Coverage

No obvious current compile error, fixture path error or final-box custody loss was found. This is not compiler confirmation. Native infrastructure must reach and execute the updated six laws before success claims.

Current physical law directly tests zero-item final release and canceled final release, then productive exact final release. The implementation also checks occupied/mismatched/short-depth/short-release/invalid cases, but those final-box branches are not directly exercised by the displayed two physical laws. Add targeted observations if claiming all rejection branches preserve physical custody. The earlier whole Probe plain/codec/session adoption frontiers remain separate from these local receipt primitives.

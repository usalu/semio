# Current Native UI Resident Authority Surface

Read-only handoff for the native composition permit extraction. Current canonical implementation: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️component.rs`. It is one process-static `RESIDENT_LEDGER`, not a per-reconciler budget. It owns 64 epoch-keyed slots, exact owner bits1(root)/2(output), aggregate item/byte snapshots, and fixed atomic deferred returns. Limits are 8MiB per surface, 32MiB aggregate, 4097 surface items, 131076 aggregate items. Existing contract/static backing is included from initialization; one exact runtime static domain is added once by `try_register_runtime_backing` and a different later byte total is rejected.

## Public Boundaries

- `UiResidentPermit::try_reserve(limits, &mut Option<Permit>, admitted_bytes)`: target/metadata preflight before ledger mutation; exact slot+checked epoch; typed capacity/contention/poison faults.
- `try_shrink`: only the sole root owner may shrink before splitting; no growth or post-split shrink.
- `split_output_into`: one same-slot owner2, not a second quota reservation.
- `close_step(maximum_items)`: disarms the exact affine key under the ledger guard; only the final paired owner returns the retained quota.
- Drop uses fixed atomic owner bits, with no lock/allocation; `drain_one` advances one slot/owner and prevents reuse until deferred return is consumed. `snapshot` and `try_observe().owns` are read-only, not constructor/publication authority.
- `required_reservation_bytes`, `contract_backing_bytes`, `fixed_backing_bytes`, `has_pending_returns` expose accounting/diagnostic boundaries.

## Actual Root Binding

`UiDocumentAssembly::open_with_permit` at contract/📄️document/🎟️assembly/🦀️component.rs moves the already-admitted owner1 into the document slot; it does not reserve a second quota. The slot in contract/📦️packages/🦀️rust/🦀️document.rs physically holds `resident: Option<UiResidentPermit>`. Existing aliases share this canonical slot/root and keep its credit until final reader and typed descendant retirement. `split_resident_output` derives owner2 from this same root obligation. Runtime reconcile.rs transfers owner2 through Ready/Published/Ack cleanup; old root epochs remain independently occupied/charged.

A neutral extraction must preserve this exact root-associated ownership, same-slot paired final release, deferred-return/epoch safety, and current static/dynamic census. A new native composition ledger must not charge UI payload a second time or silently run beside an independent UI32MiB ledger. Current owner bits and hard-coded domain limits are UI policy, not a general arbitrary-owner composition API. The needed neutral core/policy join should be designed before changing names/storage; there is no general composition permit API already exposed here.

No source or limit was changed for this handoff. Existing R81 full runtime120 and prior UI159 evidence remain their exact historical scopes; this read-only report grants no new native proof.


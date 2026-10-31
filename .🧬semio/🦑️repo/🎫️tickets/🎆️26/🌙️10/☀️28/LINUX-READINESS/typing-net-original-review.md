# Typing Net Original Ownership Review

Static current Source review, 2026-10-29. No Source edit, test, Native/compiler, lease or process operation ran. Current Source is not reported green by this review.

## Physical Net Handoff

The plugin typing commit owner is framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs. Its phase1 now validates the original open transaction shape and uses funded mem::take(&mut owners.row.as_mut().unwrap().net) directly into owners.values. This moves the original Vec header, preserving data pointer/length/capacity/order. It performs no element clone, separate vector allocation, reverse or per-entry key retirement. The remaining row holds a fresh empty Vec with no allocation.

The phase1 demand is fixed::<(Vec<M>,Option<Vec<M>>,u8)> and the mutation occurs inside admit_original_construction, with a receipt for that same header-move demand. Thus the handoff should record zero requested/released heap allocation and a bounded copy charge independent of net length. This is a static ownership argument, not a measured receipt. Empty net with retained capacity must preserve its original allocation in values too; mem::take does so.

take_into requires phase5, empty external transaction/values recipients and an emptied row.net. It transfers the original transaction and values, retains row metadata in CommitOwners, and advances phase6 under a funded fixed demand. Destructuring net:_ drops only the new empty Vec, not the original backing now owned by values. If take_into has not completed, the original values remain in the owner for controlled closure; after completion the caller owns them exactly once.

## Identity And Exclusivity

Admission still requires exact ledger.resident_key/window/original_live and refuses while original_write_pending or the target has a retained reader. Every advance validates OriginalSourceBinding, native decoder recipient, exact ledger object address and captured source_revision. Phase0 rechecks target reader state and uses withdraw_original under the exact key; its actual transfer macro also checks the supplied busy predicate. Typing registration in framework/🔨️modules/🛠️tool-machine/🔗️read/📤️transfer/🦀️.rs:43 supplies original_write_pending; this fence was not bypassed by the net move.

Current TypingLedger parent now includes resident module and Vec<TypingWindow<M>>, writing state and Vec-backed reader lookup. The earlier BTreeMap/orphan-resident shape mismatch is corrected in Source. Slot keys remain issuance+generation; other-window immutable reader owners remain boxed/sealed, so native row movement does not itself move their actual payload backings. Same-window transformation/withdrawal must still wait for all readers.

## Cancellation And Residue

advance validates identity before cancellation refusal and does not perform phase work after a cancelled context. close_step validates the captured binding/decoder, enters the existing RetainedCloneClose controlled owner retirement, and reaches terminal only when all CommitOwners fields and queued retirement are empty. Drop continues to assert paid terminal closure. Cancellation before phase1 leaves net in row; cancellation after phase1 leaves it in values; cancellation after take_into leaves it in the recipient. Each location has one owner. Do not claim rollback/republication from this cancellation law: it proves physical ownership/residue closure, not domain commit semantics beyond the existing pipeline.

The owner does not mint a new Original or relax the ledger/revision/slot/factory guards. A refusal before funded mutation must leave the source and output slots unchanged; a refusal after an earlier funded phase must retain all already-transferred owners for paid cleanup.

## Current Shared Laws And Remaining Test

framework/🔨️modules/🛠️tool-machine/🔗️read/🧪️tests/🦀️.rs now parameterizes shared reader and transfer laws by field. Typing passes net; scrub/gesture pass entries. Typing source has8197native leaves. Existing laws compare actual field pointers, enforce denied grants, reader mutation refusal, issuer return, source revision change and controlled cleanup. This is a legitimate field correction, not justification to replace pointer checks with value equality.

The targeted missing resident case should retain a large net reader in window z, then perform genuine Original edits creating enough other windows to force both sorted insertion before z and Vec capacity growth. Capture original key, net pointer, length/capacity and retained read projection; after every paid step verify the same key/pointer/value/issuer, no target mutation and exact heap requested/released bytes equal receipts. Return z's reader under the original factory/key, then commit z through OriginalTypingCommit and require the recipient values pointer equal the original net pointer. Verify order remains unchanged and no intermediate net allocation occurred.

Exercise denial of each items/copy/capacity/release/depth currency at reserve/shift/withdraw/header-move/close boundaries and cancellation before/after phase0, phase1 and take_into. Verify the same-window writer stays refused until reader return; other windows remain usable without reconstructing z's slot. A stale index/key or wrong Original/decoder/ledger/revision must refuse, never select another moved row. Close every reader/edit/commit/recipient and the Vec ledger allocation through actual paid retirement, with zero unaccounted heap events.

No concrete additional ownership defect was found in the reviewed phase1 change. Actual compile, existing neutral suites, new large-window reader/physical heap case and canonical plugin/tool-machine gates remain required. No queued or prior gate result is relabelled as validation of this latest Source.

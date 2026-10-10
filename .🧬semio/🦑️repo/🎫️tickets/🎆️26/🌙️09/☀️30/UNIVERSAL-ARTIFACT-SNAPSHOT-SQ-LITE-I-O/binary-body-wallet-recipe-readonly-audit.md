# Binary Body Wallet Recipe Read-Only Audit

2026-10-10. Source review only; no builds/tests/source edits. Concurrent execution work is not qualified here.

## Genuine Original Primitives

`🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs::NativeSnapshotBodyWallet` already has `copy_bytes_into` as well as `copy_text_into`. Each requires empty zero-capacity actual destination, pre-admits work1/copy literal length/capacity literal length/depth, calls the same original native control, and records actual copied prefix length and actual retained capacity before propagating refusal. Raw Binary need not invent another copying primitive.

Native underlying `🌱️value/🛬️decode/🦀️.rs::copy_bytes_into` charges full length, checkpoints0 before reserving, reserves exact and copies64KiB chunks, checkpointing after each chunk. UTF8 version uses64KiB boundary-safe spans. Native charge can succeed before first-copy cancellation while physical birth remains zero; cumulative native accepted bytes and physical retained capacity are different currencies. Small byte fixtures have no interior copy checkpoint: use >65536octets for an actual nonterminal prefix cancellation law.

`NativeSnapshotDecodeOwner::receive` calls `⏱️control/🛫️snapshot/🦀️.rs::receive`: original frame needs one item, depth>=2 and exact frame capacity before construction. It charges native wrapper capacity/checkpoints, then allocates Box<NativeSnapshotReceiving<T,O>>. Body starts after consuming frame item/capacity and lowering depth by1. Successful return additionally pays publication copied item1, `size_of::<Option<O>>()` copy and release of frame bytes. Exact full-owner success limits therefore include frame and publication, not only intrinsic child lengths.

Without parent pending slot, explicit installed retirement recipient is mandatory. On operation error it retains the same frame/intermediate Option, while performed body progress is folded into original owner receipt. Returning `slot.take()` on success moves typed ownership into frame.output before publication checkpoint so late cancellation still retains it. Avoid taking typed slot before an operation error; leave it installed.

## Binder Recipe

Before any typed Option assignment admit header work/copy/depth; checkpoint native; install empty BinarySnapshot with String::new/Vec::new; record actual header receipt. Copy canonical `STDIO_BINARY_DOCUMENT_SCHEMA` through body.copy_text_into into the installed schema field; raw octets through body.copy_bytes_into into the installed bytes field. If a law requires a native ceiling before Option birth, precheck native allowance for this typed frontier before installing Option. A zero ceiling may already refuse owning frame before construct.

Hex must be fully borrowed validated first (digit acceptance, whitespace counting, even extent, row/schema semantic limits). For literal zero total heap birth on malformed input, validate outside owner.receive; validating inside receive can keep typed Option absent but already has paid frame birth. Distinguish these assertions.

For hex direct output, allocate original byte vector using body.allocate_vec_into, pre-admit octet copy work/bytes, push into installed vector, and settle every performed prefix before propagating cancellation. `allocate_vec_into` records work1 plus Vec header copy and actual capacity, so do not charge capacity a second time when recording octets. Native `advance` may fail after bytes pushed; retain that original field and record prefix before returning Err. Do not decode into a temporary Vec and then assign it.

Test binder-only zero/one-short limits separately from complete owner.receive limits, using descriptive demanded receipts then exact equality success for each authority. Original body release0 can be sufficient for canceled typed binding but cannot fund successful wrapper publication/frame release. Keep close grant separate and do not replenish body after cancellation.

## Fixtures And Native Semantics

BinarySnapshot has schema and bytes only, no source_form; source_form belongs to glTF. Raw/text Binary both normalize native schema because carrier does not represent it. SQLite preserves arbitrary literal schema. Existing fixture `🧫️fixtures/🔣️.json` has `[0,1,127,128,255,0]`; semantic fixture includes empty and octet cases. Existing TypeScript tests use Bun Buffer UTF8 and Bun SQLite edited schema/octet entities, but that does not prove new original raw/hex custody. Add actual mixed-case/whitespace hex, odd digits, invalid digit, empty native and long intrinsic prefix fixtures with independent Buffer hexadecimal oracle.

## Current CSV Continuation Review

Current CSV owning test continuation law arms the captured original observer only after `admission::admit` and before actual bind, retains typed schema prefix in slot, validates body error receipt, calls funded close while canceled, asserts unchanged cumulative owned bytes and pending recipient, rearms same Cells, then helper closes. It uses original16MiB policy and makes no aggregate heap claim. The helper continuation currently runs after zero-item close; that close is guaranteed observer-free and preserves pending state, so this placement does not defeat canceled funded-close witness.

The recorded typed pointer is only checked nonzero; it does not independently establish identity after erasure. This is a evidence limitation, not a demonstrated correctness defect. Same frame's automatic receiving transfer supplies source custody evidence. Post-drain complete physical conservation is not measured by this particular integration law and must remain separately qualified. No runtime pass is inferred.

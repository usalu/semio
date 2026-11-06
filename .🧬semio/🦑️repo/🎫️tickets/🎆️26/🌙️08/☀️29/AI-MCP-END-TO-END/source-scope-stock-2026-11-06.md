# Source Scope Stock

This is mounted source, not a qualified native result. No Cargo invocation was started by this lane. Parent AllocationReturn and AST passes do not compile this boundary.

## Exact owners and APIs

- Budget stock: 🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/📦️stock/🦀️.rs.
- Budget stable scope: its 🎟️scope/🦀️.rs child.
- Actual Context authority: 🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs.
- Decoder/issuer composition: 🧰️framework/🔨️modules/🧵️job/🛬️receiving/🎟️state/🦀️.rs.

OriginalReturnStockClaim<I> and OriginalReturnStockWitness<I> retain one actual same-Source row, all five currencies, cumulative actual/unused receipts and pending observation. top_up_stock grows only the same live row from genuine remaining Source stock. It refuses foreign/different destination, pending observer, frozen/staged table, exhausted stock and tombstones.

OriginalReturnScopeLoan<I> is non-Clone and movable. The actual claim/witness remain in the heap-stable budget scope slot. Budget methods admit_scope, scope_remaining, enter_scope, finish_scope and observe_scope check actual Source/budget/serial/destination.

Source reservation setup is staged: claim costs2 owner items/work; top-up costs1. The canonical callback returns false after these real setup effects, without entering body or producing an outcome. Callers yield on false. Table birth remains a contiguous physical allocation requiring the actual owner quote, not a virtual4KiB allocation.

StepContext.native_receiving_setup_demand() quotes origin/table setup. native_receiving_requires_recovery() exposes actual pending issuer state. Normal entry rejects pending work before body. Recovery returns the exact original action against its frozen grant and pooled held stock, and blocks new construction during that callback.

with_native_receiving, with_native_retirement_receiving and with_native_retirement_receiving_value<T> share the same generic operation/scope implementation. The formerly separate typed fresh-post path was removed. Complete native output/context/receipt and independent Source/placement causes remain owned.

Controller metadata from a prior real hold is reserved observation; new native body work remains fresh. Old pending action recovery uses held currency. A post-refused observation stays in the stable budget and blocks another settlement.

## Retirement transfer hook

scope_requires_retirement(&loan) is conservative after new capacity or typed output. Balanced allocation/release totals never prove output retirement.

transfer_scope_credit(&mut loan, &mut actual_preborn_Option<OriginalReturnStockClaim<I>>) moves the SAME opaque claim, preserves Source held counters and frees the stable scope slot. It requires no running callback/pending witness and retirement-required custody. Context MUST authenticate the actual native/actor-local recipient and fund the move before calling; this pure fiscal hook alone cannot publish constructor Ready.

original_return_scope_transfer_copy_bytes<I>() quotes credit placements, scope reset, actual loan-option clear and returned flag. Attachment also prices its own sealed recipient binding. OriginalJobDecodeOwner.scope is crate-private; after transfer, the attachment clears its actual returned option under that quote.

No genuine output root qualifier exists in current admit_original_construction: it still accepts bare Option<O>. Constructor/AppFrame remain unqualified until actual native OR actor-local managed custody owns the value and cleanup claim.

## Existing native ports

Implementation: 🧰️framework/🔨️modules/🧵️job/⏱️context/🎟️source/🏠️native/🦀️.rs.

StepContext.admit_original_native(input:&mut Option<T>, output:&mut Option<OriginalNativeLease<T>>).
lease.recipient_identity(); with_native_borrow and paired borrowed variants.
OriginalNativeLease::return_slot_step(&mut actual_option,cx).
close_original_native_recipient_step(identity).
original_native_recipient_closed(identity) and stronger original_native_retired_witness(identity).

OriginalNativeRetirement requires Send+'static. Generic actor-local non-Send outputs cannot acquire a new Send bound or escape untracked. Parent owns actual native/actor-local cleanup attachment.

## Evidence

Portable stock/top-up/setup:3tests49 assertions, AJV+SQLite.
Output-lifetime event oracle:1test4 assertions. Retiring oldB100 does not refund liveA100 despite equal aggregate counters.
Guard widened-field oracle:1test14 assertions. x64 observer265, probe51; metadata256 remains unchanged input.
Rust parser: no parse errors; exit1 formatting-only is not compilation.
Latest cfgtest census: Context24files262012bytes1713lines; Budget9files72768bytes363lines; JobReceiving6files37001bytes229lines. Limits remain32/262144/4000.

Native inline financial laws are authored but uncompiled: foreign source/target, pending observation, cumulative no-op, exhausted stock, frozen snapshot, tombstone, oldreserve200/commit0/new100, >255 items, and retention despite balanced capacity/release. These are financial laws, not native heap journey claims.

## Remaining qualification

1. Actual budget stock native compile/tests.
2. Genuine Source pending allocation → zero-fresh recovery/cancellation with original allocator marker and heap witness.
3. Native/actor-local managed output attachment before Ready.
4. Authentic selected capture plan and persistent exact-target Source metadata backend BEFORE the body P/CPU loan. No owned effect mints permissions from positive DATA grants.
5. Audit stale512 sized-claim ceiling against widened row/counters; quote actual owner demand, keeping Source limits unchanged.
6. Constructor callers, AppFrame and external marker chains remain incomplete until these boundaries compile.

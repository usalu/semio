# Exact Tail Held Capacity Review

Read-only bounded review of [held PagedList provider](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/child-paged-complete-family/exact-tail-source-provider-held.rs), 2026-10-06. No Native compiler or law run.

No new definite static blocker found in the inspected capacity/admission paths. Exact payload admission uses min(page_items, limit-capacity); byte8194 can therefore retain genuine4096/4096/2 payload backing without granting a third4096 allocation. The next request beyond a short funded tail refuses before allocating or mutating because capacity is no longer aligned with full page_items. Legacy reserve_page_using delegates to N, retaining the existing allocation-counter interception seam.

Metadata root height remains adaptive. Growing the root moves the existing root Vec into child0, preserving actual old backing pointers. Both metadata and payload branches record actual Vec capacity and retain the allocation in the owner before returning an over-admission refusal. Short tails record separately reserved logical slots, which retirement/parent-transfer subtract rather than assuming a full page. Full-parent refusal tests remain necessary for both payload and metadata layouts.

This API funds actual capacity, not a permanent independently stored final logical cap. Calling exact admission with a lower limit after already funding a larger capacity returns no new allocation, and does not revoke existing slots. Callers requiring a fixed original source extent must retain and enforce that authored extent themselves. A non-page-aligned funded final tail is physically immutable under subsequent extension; it does not retrofit a final cap onto an owner already allocated with ordinary reserve. Native laws must exercise fresh exact owners and preserve these distinctions.

Static review does not qualify allocator layout, parent return, physical retirement, original8194 source identity or any authored operation caller. The genuine independent allocation and original grant laws remain required.

# 1103 Deflate Accounting Review

STATUS: READ-ONLY REVIEW COMPLETE. Closure copy-byte correction required; initialization depth underquote identified. No source edits, Cargo or budget changes.

## Normative Decision

A bounded collection discard of plain Copy elements must quote and report count * size_of::<T>() copied_bytes. Zero-copy scalar retirement is not the rule for draining a collection.

Evidence:
- value/🎟️control/🧬️contract/🦀️.rs:7-10 separates payload-copy authority from physical-release authority; copied_items and copied_bytes are independent receipt currencies.
- value/♻️retirement/🦀️.rs:35 states retirement_element_copy_bytes declares complete inline payload work before a collection discards the element.
- The same file:208-247 drains plain Vec elements with truncate and ProcessedBytes(count*width), even though no physical copy need survive compiler optimization.
- Portable copy-demand fixture and third-party TypeScript Uint32Array oracle under retirement/🧪️tests/🧫️fixtures require a four-u32 list to account sixteen bytes; lone u32 and String owner transfer report zero payload work.

Therefore the current Deflate cursor retirement helper (🧰️framework/🔨️modules/🗜️deflate/🛫️encode/⏳️cursor/🦀️.rs:56-61 at inspection) must not quote zero and pop up to512 slots with copied_bytes zero. Use one shared bounded count for demand and mutation; quote/report exactly count*width. HeadSlot/u32/u8 batches are at most4096/2048/512 bytes respectively. Keep copied_items=1 as one explicitly bounded batch event; physical allocation release remains separate. This matches the planned runtime owner correction, not a new policy.

## Concrete Depth Quote Issue

initialize_demands currently uses next_pop_depth_demand().max(1)+1 for all reserved initialization. On the first reserved leaf, len=index=0 while has_reserved_slot=true. PagedList::next_pop_depth_demand returns0 for an empty list, yielding parent quote2. The operation actually appends through push_reserved, whose next_push_depth_demand requires height+3 (three for first leaf), plus one Deflate parent level: four.

Use the push path quote when index==len with a reserved slot, and the appropriate get path if overwriting initialized slots. This issue predates batching, but an exact-quote test must not certify too-small authority. Reported to runtime_laws.

## Boundedness And Custody Assessment

Initialization count is bounded by paid capacity remaining, declared list maximum remaining and4096/width. Actual private instantiations are nonzero widths1,4,8 and fixed list limits; no actionable division-by-zero, multiplication overflow or index-overflow was found for these call sites. Generic hypothetical types are outside this private helper use.

PagedList::pop changes logical length and returns one Copy value; it does not free pages. Thus batching up to512 pops does not hide physical releases. Empty backing remains owned until release_empty_page under exact release demand; terminal Drop witness remains intact. No new whole-owner drain, unbounded loop or custody escape found.

Existing advance preflight checks item/copy/capacity/depth before initialization mutation. close_step similarly checks item/copy/release/depth before begin_close and pop; once nonzero batch copy demand is quoted, demand-minus-one can remain completely inert. The helpers must use the same count at quote and execution.

## Focused Verification Recommendation

Extend the existing successful codec/miniz/zlib/allocator laws with closure grant deficits: item zero, copy demand-minus-one, release demand-minus-one and depth-minus-one. Compare zero receipt, no heap effects, unchanged phase AND actual backing/list lengths (public phase/input/output only misses a hidden discard). Then exact demand should account the exact number of discarded element bytes, release zero on discard turns, copy zero on physical backing-release turns, and total release equal total observed allocation.

Test the first reserved-but-empty initialization frontier explicitly against next_push_depth_demand+parent, including depth-minus-one. Existing producer deficit checks do not independently establish that the advertised depth reflects the actual path.

All findings sent to runtime_laws and root. The runtime owner retains source and active editor run ownership.

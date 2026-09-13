# Retained Pack Symbol Table Root Review

Root reviewed the stable RetainedPackSymbolTable source during SliceOne cutover. API coherence is now released and the existing composed suite passes; new physical-owner laws remain pending. No Pack source was edited by root.

Root also inspected the value cursor. Actual allocator failure must set its first sticky fault, and logical pushes must enforce maximum_frames independently of any allocator overcapacity. The execution agent has corrected both and routes root-initialization errors through the same ledger. This is source review evidence pending the focused native laws.

## Required Correction: Full-Count Allocation Query

next_symbol_allocation_bytes(target) calls PagedList::next_allocation_bytes once target exceeds capacity. That PagedList query returns0 while any already-reserved slot remains. RetainedRecordBodyCursor::allocation_need requests the entire announced symbol count before consuming the count byte. For a count larger than one span leaf, the first leaf remains logically empty, capacity is still below target and the query returnsSome(0). reserve_capacity_one(target,0) then cannot allocate the next page. Repeating the protocol cannot advance.

The fix belongs in either a target-aware shared PagedList demand query paired with reserve_capacity_one, or an explicitly incremental record-body span-admission protocol before each new/empty symbol. It must preserve valid multi-leaf symbol tables. Add actual >one-leaf count evidence and maximum+1 refusal, including cancellation at that pending byte. Root sent this finding directly to the agent.

## Required Correction: Checked Symbol Coordinates

symbol_char, symbol_chars and symbol_span narrow u64 symbol identifiers using `as usize`. On32-bit targets,2^32 aliases index0. Span start/length use the same unchecked narrowing and start+character addition. Reject unrepresentable/out-of-range identifiers and spans using checked conversion and checked addition. Native64-bit tests can still exercise u64::MAX and malformed span relationships, and a language-neutral32-bit model must cover2^32 rather than claim native32-bit execution.

## Owner Authority Review

The table stores maximum_utf8_bytes but push_scalar_reserved/push_symbol_reserved do not enforce cumulative UTF-8 or span validity. The record-body parser separately counts bytes and the catalog accesses the underlying symbol/scalar lists directly. The extraction centralizes storage but has not yet centralized symbol admission. Before calling it a shared logical owner, enforce cumulative scalar/UTF-8/span relationships at this owner, or define and validate an explicit admitted token produced by the parser; do not expose unchecked span publication as if the table validated it.

Several table error exits bypass remember: PagedList demand failures, push_reserved failures and zero-byte allocator errors. They can return an error without establishing the first sticky fault. A subexact caller grant is a non-error refusal; an actual allocator/admission failure must have an explicit consistent sticky policy. Verify repeated errors return the first fault and retain every successful backing.

The close order is otherwise aligned: logical scalar/span retirement precedes physical scalar backing and then span backing; demand uses that same order and includes actual allocation bytes. Table close_step(0,0) still sets closing=true when logical lengths are empty; add an explicit zero-grant witness if complete owner-state preservation is promised. The catalog's outer close already guards its own zero grant.

Root observed the agent's coherence-pack-2 native executable remain in all three catalog laws past60 seconds after its10.91s compile. Root attempted a one-second read-only macOS sample, but the process had already exited; no stack sample was obtained and root stopped no process. The log contains no completed native result. This observation does not attribute the stall to the full-count demand finding without the agent's subsequent diagnosis.

## Follow-Up Source and Native Evidence

The agent added PagedList::next_capacity_allocation_bytes(capacity) and uses it for table and catalog target-capacity queries. The query checks the requested target and calls the real next-page demand even while the first leaf has unused slots. Symbol lookups now use checked u64→usize conversions and checked index arithmetic. Publication checks consecutive span start/end and exact pending UTF-8 length, and scalar admission owns cumulative UTF-8 limits. Actual reserve and push failures preserve the first table fault; zero-grant close now preserves an unclosed empty owner.

Catalog and record-body close forward the actual item grant to the symbol table, allowing its final empty-owner close transition. The fresh catalog coherence-pack-3 run passes3/3,93 filtered,in0.01s through registered Nx in15.0s. Root read the durable log at [retained-pack-value-coherence-pack-3.log](🗑️generated/retained-pack-value-coherence-pack-3.log). Dedicated multi-leaf symbol-table/record-body and32-bit-coordinate laws remain required before accepting SliceOne as a whole. The all-callsite compiler check is still running; root has not inferred the API END marker from this catalog result.

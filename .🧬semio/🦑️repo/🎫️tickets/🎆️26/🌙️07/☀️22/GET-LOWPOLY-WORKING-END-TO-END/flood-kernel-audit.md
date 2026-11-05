# Flood Kernel Audit

Read-only source audit, 2026-11-05. No Native/server/test run or source mutation.

## Sound Algorithm Properties

Rust/TS both perform4-connected exactRGBA region coverage, enqueue seed once, mark seen when enqueuing, and inspect each queue entry once. At most pixelcount entries can enqueue; queue preallocation count prevents algorithmic growth. Nonmatching frontier entries do not expand. Extent validation caps16M pixels and side16384; count*4/count*6, u32 queue index and neighbor arithmetic remain representable on supported32/64-bit Rust/JS number ranges. maxWork1..65536 bounds initialization pixel events and flood queue events; each event has≤4neighbor checks. Mask result is unavailable before terminal success and after cancellation.

Rust borrowed source identity is pointer+length; Uniform is inline packedRGBA value. TS borrowed source checks object identity/length. These are source lifetime contracts, not content hashing. External caller must preserve borrowed bytes.

## Actionable Issues Sent Runtime

1. Rust cancellation unit currently cancels after just one allocation advance, not after actual pixel progress. TS unit correctly advances until completed>pixelcount. Rust must test actual flood progress, unpublished result, retained workspace and paidclose.
2. Rust allocation records actual Vec.capacity (good) but phase0 demand quotes requested count/count*4. try_reserve_exact guarantees at least requested capacity, not necessarily equal. If actual capacity is larger, progress can exceed quoted admission and no grant is passed to advance to refuse excess. Need explicit ownership-safe handling/guarantee; never drop oversized owned allocation as a side effect of refusal.
3. TS done ignores cancelled while Rust done includes !cancelled. After cancel signal delivered to completed cursor, result refuses in both but done differs.
4. TS creates target Uint8Array on EACH allocation turn (three times) and overwrites prior owned4byte targets. workspaceBytes and releasedBytes omit those buffers; physical paidclose drops final target uncounted. Rust uses inline4byte target. Inline scalar/tuple snapshot avoids this hidden ownership.
5. TS Uniform source retained as caller tuple reference; same tuple mutated between advances compares against itself, missing changed originalRGBA unlike Rust packed Uniform identity. Snapshot initial uniform value inline rather than comparing aliased tuple.

## Measurements And Retirement

Rust demands initialization count*2+seed5; flood conservative32bytes/event covers usize head8+mask1+four enqueue writes20, plus2Self header. Reported copied_items1 and work≤maxWork fit basic event model. Source reads are not reported as copied allocations. Actual allocation capacity is measured, not merely requested, but quote risk above remains.

Source mismatch/refusal/cancellation returns before freeing existing ManuallyDrop workspace. begin_close then RetainedCloneClose transfers original queue/mask/seen owners; Drop asserts fully retired except unwinding. Test close uses generous grant and checks progress fits but does not probe denied close retaining exact owners/capacity. TS close(0) test verifies no release; TS backing release is logical JS reference release, not proof of runtime GC timing.

Large queue capacity can be64MiB at maxpixel extent; caller close grant must admit actual full backing release or close intentionally cannot progress. Existing256x256 fixture fits1MiB release grant. No hidden whole-image flood loop exists; Rust resize does at most maxWork zeros. TS Uint8Array allocation inherently zero-initializes full buffers during allocation phase; it must be treated as an allocation/capacity operation rather than claiming work1 measures all physical bytes.

## Evidence Limits

Rust/TS corpus equivalence and Graphlib oracle tests exist, but this audit did not execute them. Findings are source issues, not native failures. All actionable items sent runtime; root owns verification scope.


## Paged Rust Follow-On Source Review (2026-11-05)

At Runtime's request, read-only reviewed current4096pixel paging. No Native/source edits. New Vec<Vec<u8>> seen/mask and Vec<Vec<u32>> queue allocate three directory backings, then one leaf per paid allocation turn. At maximum16,777,216pixels,4096pages require98,304bytes per directory on64bit (4096×24); leaf backings are≤4096bytes seen/mask and≤16384bytes queue. Thus neither directory nor leaf has the old4MiB indivisible release need under actual1MiB mounted release authority. No generic extent truncation was introduced.

Allocation quotes include cursor metadata plus Vec header copy; actual capacity is recorded before overquote refusal, and owned directory/leaf remains retained for paid close. Allocation count/page_length logic covers final partial page. Initialization writes only maximum_work pixels; BFS uses head/queue_len, marks seen before enqueue and therefore enqueues≤pixelcount. Index/page mapping and seed insertion match FIFO semantics. FloodMask exposes borrowed at/iter access without full flatten allocation.

Exact retained close API checked in value/retained-clone/close/🦀️.rs: next_owner_demands<T:RetireOwned>(present,binding,body), begin_granted<T:RetireOwned>(&mut Option<T>,grant), select(body)->RetainedCloseSelection and selected.step(grant) match new calls. Existing generic Vec RetireOwned and sealed empty-owner implementations support nested Vec element retirement; no false flattened-owner type is used. The cursor quotes child retirement through the same body request before delegation, preserving explicit owned retirement instead of dropping nested vectors at scope exit.

This is a source/API compatibility review, not Rust compilation, allocator trace, actual close execution, or timing proof. Page-directory actual capacity remains allocator-dependent; overshoot refusal/retained close must be verified by the authored allocation observer tests. Generic retirement birth may consume funded capacity before later release phases; do not infer identical TS/native metadata phase receipts from page counts alone.

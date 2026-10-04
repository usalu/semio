# CommonMark Intrusive Retirement Light Audit

Fresh read-only mounted-source inspection after Root's actual MD24 receipt: 22 passed, two failed, including 25,192,449 retirement visits against a 114,696 ceiling and the final SQL candidate cancellation boundary. This auditor ran no Cargo/build/test. Root owns the corrected rerun. The prior report's inferred linear complexity for enum continuations was wrong and is withdrawn; the executed failure takes precedence over that textual inference.

## Corrections To Earlier Findings

The actual ordinary block reconstruction has a post-match leftover-field predicate (`raw.is_some()` included). A Paragraph carrying raw text therefore refuses. My earlier silent-acceptance finding missed that predicate and is withdrawn. Ordinary malformed deep reconstruction still uses ordinary recursive drop; no broader safety conclusion follows from this field refusal.

The SQLite terminal now wraps `reconstruct(...)` in `OwnedSnapshot` before its final fallible `(ReconstructSnapshot,1,1)` checkpoint and takes the candidate only afterward. This repairs the inspected unguarded cancellation window. Runtime confirmation remains pending.

## Intrusive Frame Ownership Readback

Actual `📦️pack/🦀️.rs` now defines `InlineContinuation { work, previous }` and `BlockContinuation { work, items, previous }`, replacing synthetic enum continuations. Each frame occupies exactly the slot just freed by a successful `work.pop()`. Thus the slot address is within the allocation, below capacity, and outside the vector's new initialized length. The frame never becomes a typed MdBlock/MdInline element and is excluded from Vec's element destruction.

The pointer originates from that live vector's `as_mut_ptr().add(work.len())`. Moving its Vec header through `mem::take` into the frame does not move or resize the backing allocation. The frame's retained Vec owns that same backing buffer while the raw pointer chain addresses it. Child work uses distinct owned Vec allocations. No reserve, push, shrink, reconstruction from raw parts, or allocation-moving operation occurs while a frame pointer remains outstanding.

Unwinding a frame reads it once with `ptr::read`, restores its saved `previous`, and moves its work/items back into active locals. The stale frame bytes remain outside initialized Vec length. Subsequent pops may write new frames into different vacated positions in that restored allocation; the consumed pointer no longer occurs in the active chain. A vector is deallocated only after its frame has been read and its original elements consumed. No inspected double-drop, dangling previous pointer, or forgotten pending list vector was identified on normal execution.

Block descent saves the currently pending item groups in the frame. A List then becomes the active items frontier; a Quote starts with empty active items. Empty work selects one pending item vector at a time, including empty ones, before restoring the enclosing frame. Both abandoned empty work buffers and empty item vectors deallocate normally on assignment/drop. Heading/Paragraph inline vectors are consumed by a separate iterative inline drain; this adds one fixed function frame, rather than stack depth proportional to nesting.

## Layout And Platform Boundary

Compile-time assertions require each frame's size and alignment to fit the owning node slot. They are target-specific and reject a target whose actual enum layout cannot hold a frame. Slot alignment follows MdBlock/MdInline alignment; Rust layout alignments are powers of two, so the asserted smaller/equal frame alignment is sufficient. No exact 32-bit or 64-bit enum size is assumed in this audit. Successful compilation and runtime on one target do not certify another target; a 32-bit build is still independent evidence.

## Work And Allocation Boundary

Each original node is popped once; each List item vector is popped once; each child-bearing node writes one frame and that frame is read once. Frames are no longer synthetic nodes reintroduced into the owned tree, so there is no enum-wrapper revisit mechanism from the failed implementation. This provides a specific structural linear-work argument for the replacement. The existing visit counter counts original nodes and item pops, not frame reads or empty-loop branches; a bounded number of those uncounted operations accompanies each counted descent/item.

The loops allocate no storage: frame writes/reads, Vec header moves, pop and normal payload drops reuse existing buffers. No String payload is intentionally forgotten; Link URL/title, Image fields, code/info/raw fields and snapshot schema all reach ordinary nonrecursive payload drop. Retirement is nonfallible and has no controller. Unexpected panic while intrusive frames are outstanding is not a demonstrated recovery path; these frames deliberately live outside Vec length. No ordinary production fallible/callback operation appears inside the drain loops.

No new actionable normal-path memory ownership defect was found by this readback. This is neither a Rust alias-model proof nor a runtime green receipt. The allocator-refusal/deep/wide law and final-candidate cancellation law must execute on the corrected mounted state; their earlier MD24 failures must not be described as resolved before that receipt.

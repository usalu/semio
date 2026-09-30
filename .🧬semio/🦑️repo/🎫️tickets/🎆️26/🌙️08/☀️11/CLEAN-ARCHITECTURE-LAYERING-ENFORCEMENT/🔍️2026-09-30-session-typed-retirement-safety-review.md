# Session Typed Retirement Safety Review

Read-only source inspection of the current execution-owner handoff. No native tests/builds run here. New tenth native law remains execution-owner validation.

## Genuine Retained Ownership

Session authority census is an ordered map containing every open authority, including empty authorities, admitted under claims lock. Port admission and authority seal share that transaction. Root clones share closed state; siblings own distinct state. Family extraction holds claims and requires the census empty, then try_write kernel and try_lock cache, returning zero-progress Pending while a live reader retains its lock. Once these exclusive locks succeed, older sealed Arc aliases reference empty shells and reject new geometry work. Existing callback signatures do not return borrowed kernel references. This is a viable sealed-authority handoff without unsafe Arc strong-count snapshots.

Native BREP `🧬️schema/⚙️engine/🧹️retirement/🦀️.rs` implements actual typed frontiers. Brep.detach_retirement moves Body and live without full GC. Each arena tail pop charges a slot including holes; its empty backing and free-list payload move to buffer retirement. Live ordered entries transfer handle text, Wire members, Compound ids, and Curve/Surface owned geometry. NURBS surfaces transfer rows separately. TessellationJob.detach_retirement transfers edge order/faces, ordered edge samples, transfer numeric arrays, and metadata strings. TessellationReport is scalar Copy and owns no omitted collection. Cached MeshData retirement transfers every current buffer and optional paint texture text.

PayloadRetirement preserves buffers while consuming bounded byte credit against their capacity, and advances one frontier per structural item. This is real ownership/byte accounting; allocator release remains indivisible, matching the established local convention. No geometric terminal rebuild, reachability traversal, or compact_unreachable call was found inside these retained close paths.

## Zero-Grant Admission Mutation

Session `close_step` around lines 794–800 invokes begin_close before checking item/byte grant or pause. Therefore a fresh close_step(0,positive) reports Blocked while sealing authority, removing claims, and transferring jobs. The new native law invokes explicit begin_close before its zero-budget assertions, so it cannot detect implicit zero-grant sealing. Sent to execution owner/coordinator: move grant/pause guard before implicit seal if zero budget promises no work, or make admission explicitly separate and assert the fresh-state invariant. Mesh/instance-owner close currently avoid fresh zero-budget work.

## Unfinished Drop Policy

Kernel, cache, and job collections now use ManuallyDrop shells, preventing hidden native payload destruction during ordinary Arc/state teardown. However the inspected SessionState and SessionRetirement lack Drop guards for open/unextracted ownership. Dropping the final never-closed Session while its retirement queue is still empty silently leaks all those native payloads: PayloadRetirement's empty-queue assertion passes, and the shells suppress destruction. The required lifecycle is not enforced in this path.

Sent to execution owner/coordinator: enforce authority sealed/jobs-transferred before SessionState release and family extracted/payload-empty before final retirement authority release (except unwinding), or provide an actual persistent recovery owner. The peer state can legitimately release before full family extraction after transferring its jobs/claims, so an authority-state check must differ from the final shared-family check. A family-wide terminal assertion belongs at the final shared retirement authority, not every sibling Drop. No test failure is claimed; this is the exact unchecked final-drop source path.

# Retained IFC Payload and Close Boundary Review

Read-only, current source. The new retained IFC geometry foundation was not yet present in the inspected IFC Rust graph; these are concrete integration gates against existing APIs, not defects claimed in unwritten code.

## Fresh pending bridge

Fresh C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🖼️render-surface/🦀️.rs has correct five-argument preparation call. Grant intersects mounted preparation and original caller. record(grant,cx)? runs before issuing snapshot_read. False readiness still reaches pending poll with None. No grant returns before callback. No visible type/borrow mismatch in this block; native execution unclaimed.

## Payload preparation observations

C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs JsonBorrowedWriteCursor first measures, then allocates output, then emits. normal_step_demands quotes frame/path allocation, final exact output allocation, next scalar copy and depth. step returns normal_step_progress; errors attach that progress. The caller must debit those actual receipts even on error, not only successful Some(output).

NativeEncodeControl must resume its continuation across turns (pause/resume), retaining cumulative byte/owned-allocation admission. A newly constructed control each turn resets cumulative authority. Existing exact precedent: JSON 🧪️tests/🔬️unit/🫳️borrowed-source-laws/🦀️.rs borrowed_json_writer_retains_cumulative_admission_across_real_observer_grants. It checks zero-unit stability, monotonic owned bytes, immutable original source pointer, final serde_json output equality, cancellation leaves writer frontier unchanged, and retirement after interruption.

JsonWriteSource is borrowed and can be queried repeatedly in both passes. Therefore its geometry scene must remain immutable through measurement/emission; changing normals/selection payload while writer is pending breaks measured-size authority. Cache core geometry separately from interaction decoration, or freeze a complete payload source until publication. Do not emit mesh string early while instance/status string is still pending. One atomic completion owner should retain all outputs until aggregate byte cap and final original cancel/stale checks pass.

The returned output is moved out at terminal phase3; writer progress() reports phase==2 rather than a general terminal enum. Do not use progress().1 alone as completed-output authority: use actual Some(String) transfer and retain it exactly once. After transfer writer still owns frame/path backing, so it needs retirement even when output is already cached.

## Close requirements

JsonBorrowedWriteCursor implements RetireOwned with frames, path projection iterator and optional output. Use defining controlled retirement; no large bare drop on cancellation. The scene, pending strings, source index buffers, Boolean frames and source lease are separate custody even when writer itself is empty. Close must pay release receipts and reach terminal emptiness for each before dropping mounted owner. Source lease return should be witnessed when operation completion depends on Store registry retirement; SnapshotRead::Drop returns an untaken poll lease automatically, but that is not proof a retained worker closed all owned buffers.

Allocation refusal can leave frame/path or output backing owned. Store the exact failed cursor until controlled close; do not replace it with new empty cursor to retry. Stale revision must close the old cursor before admitting a replacement that could consume the same capacity. Closing should retain original failure and cancellation identity rather than publishing a partial cached result.

## Required added IFC proofs

- Tiny grants through source capture/index, primitive projection, ring/Boolean work and both JSON passes; monotonic stage counters with no replay.
- Observe heap and original receipts separately at source admission, final output allocation, string transfer and retirement; reported copied_items alone cannot prove bytes/capacity/release coverage.
- Cancel immediately before and after final output transfer; retain both strings on failure until close, and publish neither.
- Exact aggregate cap across meshes, instances and status, including JSON escapes, negative zero and finite float formatting; independent serde_json/JSON.parse oracle.
- Close at every phase and after error, then assert source owner/reference counts and Store returned-lease retirement witness, not only empty mesh count.
- Stale canonical revision, generation and provisional source while waiting; locale/selection updates should reuse prepared geometry only where its immutable source contract remains true.

No numerical normal-render fuel/deadline limit or normal-host inability is established here. Short-grant convergence is the required retained-work property, independently of host defaults.

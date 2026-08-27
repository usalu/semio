# Owned UI Wire Source Review

The coordinator read the complete ordinary value decoder and retirement implementation after the executor's coherent five-test checkpoint. No live renderer consumer is switched by this review.

## Verified Source Properties

- Admission requires an entire, non-shared ArrayBuffer-backed view and a successful explicit transfer. Shared buffers, partial views, detached buffers and non-transferable Wasm memory are not silently copied or adapted.
- Varints, symbol-table edits/lookups, values, fields and container members retain their own state between advances. Production does not call the old recursive decoder.
- Native text is bounded at 512 UTF-8 bytes and ordinary UiValue containers at 256 members. Each scalar/metadata operation fits the unchanged 4,096-byte minimum grant; smaller grants block instead of accumulating fictional credit.
- Canonical ordering, finite numbers, exact integers, framing, field identity and trailing input have explicit checks. Nested members and array length are made non-writable/non-configurable before completed roots become visible. Own `__proto__` fields use property definition rather than prototype assignment.
- Cancellation releases decoder frames before roots, then construction owners in parent-before-child order, symbol-index owners and finally input bytes in bounded zeroing pages. Published immutable values are not modified by input retirement.

These are logical ownership and work-accounting properties, not a measured 8 ms or JavaScript GC/allocator guarantee. Buffer transfer also requires the caller to relinquish all aliases; JavaScript cannot discover an unrelated alias on its behalf.

## Remaining Domain And Runtime Work

The ordinary 256-member rule is not the full native UI schema. In particular, SurfaceDoc bytes have a 32-KiB byte-sequence domain, and node/patch collections have their distinct declared bounds. The executor is implementing an explicit immutable paged-byte view and its actual retained consumers, without pretending it is an Array or increasing one eager allocation to 32,768 entries.

The typed semantic projection, exact immutable record authority, tree construction, canonical hash, notification scheduling, actual ACK, React/wgpu integration and transport-level byte admission remain open. The existing larger typed semantic fixture domain is separate from these native wire bounds.

Targeted executor evidence is in `📓️renderer-owned-wire-checkpoint-2026-08-27.md`. The independent full renderer R8 run is pending unexpectedly long at this checkpoint; no full pass is inferred from the five-law decoder run.

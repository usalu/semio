# Neutral Primary: Exact Test-Only Mount Delta

## Review Boundary

This is an unapplied proposal. Canonical resident authority e23ec406 and tests e81bcca1 remain unchanged. The new child is ticket-only at `🧪️resident-primary/🦀️.rs`. Its seven bodies use the existing root, list, gate, allocator and Release; the six proposed methods and private metadata do not exist yet.

If separately authorized, the first mount changes ONLY the existing resident test module as shown below. It mounts seven missing-API bodies alongside the existing25. It adds no root metadata, production method, alternate allocator, new crate, feature, capacity, helper shim or compiler route. Missing-API compile RED would execute zero tests.

## Exact Existing Test Module Delta

Target: `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs`.

```diff
@@ unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
     unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
+        primary_recovery::observe_allocator_enter(layout);
         COUNT_ALLOCATIONS.with(|enabled| { if enabled.get() {
@@
-        if FAIL_NEXT_ALLOCATION.with(|value| value.replace(false)) { std::ptr::null_mut() } else { unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) } }
+        let failed = FAIL_NEXT_ALLOCATION.with(|value| value.replace(false));
+        let pointer = if failed { std::ptr::null_mut() } else { unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) } };
+        primary_recovery::observe_allocator_return(layout, !failed, pointer);
+        pointer
@@
         release_baseline::observe_system_dealloc_returned(layout);
         release_phases::observe_system_dealloc_returned(layout);
+        primary_recovery::observe_system_dealloc_returned(layout);
@@
-pub(super) fn observe_release_destroy_returned() { release_phases::observe_destroy_returned(); }
+pub(super) fn observe_release_destroy_returned() {
+    release_phases::observe_destroy_returned();
+    primary_recovery::observe_destroy_returned();
+}
 //#endregion 🧪️ReleasePhases
+
+//#region 🧪️PrimaryRecovery
+#[path = "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs"]
+mod primary_recovery;
+
+pub(super) fn observe_primary_recovery_pointer_load(registration: u64) {
+    primary_recovery::observe_recovery_pointer_load(registration);
+}
+//#endregion 🧪️PrimaryRecovery
```

The allocator failure branch is exactly the existing one-shot FAIL_NEXT_ALLOCATION. Entry records the actual global-allocator request; Return records whether System was delegated and whether the actual returned pointer was null. Injected null suppresses System delegation; it does not claim System returned null. No allocate/free-and-report-null, root access inside allocator, allocation in observation, assertion, unwind injection or user callback is introduced.

Each hook uses const-initialized TLS Cells and a32-entry fixed array per selected call, with explicit overflow. Diagnostics are copied values only. Context is read from the actual charged Pending anchor under the original gate before the measured prepare call; it is not a public allocation permit or source pointer.

## Later Implementation-Owned Pointer-Read Hook Contract

The wrapper above has no production call site at initial missing-API mount. When the six methods are implemented after actual RED, each recovery-path header/payload pointer load must be preceded by:

```rust
#[cfg(test)]
tests::observe_primary_recovery_pointer_load(pin.registration.get());
```

For initial list-head pin acquisition, use that actual owned ConsumerPage registration before loading its header. For successor acquisition, use the actual linked successor descriptor's registration. Never dereference a possibly stale pointer to obtain the diagnostic registration. A resumed call checks original root phase, exact cursor/mode and revoked status BEFORE any saved-pin pointer load or hook.

The hook must sit at the actual source load boundary, not at public method entry or behind the result branch. Reads inside a granted method count; short/zero/closed/revoked calls must count zero. Copying the root's pointerless cursor phase is not a node load. Capture's returned alias is a separate counted facade, not a pointerless continuation.

The existing production destroy-return hook already runs after the actual typed destructor and stage change. The delta forwards it to the new fixed observer; no extra destructor is executed by tests. Actual Free is recorded only after System.dealloc has returned.

## Private Test Access And Counter Injection

The child directly reads actual private ConsumerNode/ConsumerHeader/ConsumerPage, LedgerState and Release under the existing root gate. Its snapshot has four scalar node entries, enough for the largest declared four-consumer fixture; an unexpected fifth node is an explicit error, not silently ignored.

The only proposed fault injections are already-existing allocator null, test-only upward registration counter seeding near u64::MAX, temporary alias/pin counter exhaustion, and a unit panic under the actual root gate ONLY after all pointers are physically freed and Release is Refund. Counter injections are restored to the actual outstanding leases before returned aliases are dropped and before cleanup. No live poison is cleared, no unknown payload is erased, and no freed-pointer dereference is allowed.

The exclusive poisoned snapshot uses `&mut ResidentLedgerRoot` after no thread or facade remains and only after checking the actual anchor is pointerless Releasing, cursor absent, list empty, allocated bytes0 and exactly one free. It does not use that exclusive access to mutate or reset poison, charges or owners.

## Layout / Work Additions To The Original Proposal

The original proposal remains historical. The child now adds the ordinary reservation/publication changed-field sums:

- Ordinary reserve: Option<ConsumerPage> + u64 counter + ResidentResources + Option<NonNull<ConsumerHeader>> prepared reset.
- Ordinary allocate/init: same actual node/layout write sums as primary.
- Ordinary publish: 4×Option<ConsumerPage> + Option<NonNull<ConsumerHeader>>.
- Close latch: only actually changed root.closing and cursor.revoked bool fields. Both absent-cursor and active-cursor states get zero/one-short/exact tests.
- Primary Clear under pointerless poison: Option<Release> + Option<PrimaryAnchor>; Refund: Option<Release> + ResidentResources.

These are desired checked source write sums, not executed native measurements. All sums use actual private types, checked arithmetic and an unchanged4096 maximum. An independently measured ConsumerNode<[u8;4096]> deliberately exceeds4096: its retained pending reservation must refuse allocation without losing charge, then cancel through pointerless Refund/Clear.

Cleanup bounds are fixture-local: at most four consumers × seven declared close stages, one admission × six stages (consumer-reference retirement plus detach/Destroy/Free/Refund/Clear), eight root/cursor control stages, plus checked ceiling(Layout/4096) units for actual root/node/admission/anchor/cursor/Release extents. No100000 retry, quota increase or arbitrary-schedule liveness claim. A nonterminal/error cleanup is retained as a failure; assertions occur after the attempt.

## Prospective Native Boundary

Unchanged package `@semio-tech/value-resident-rs:test` takes NO arguments and invokes the existing budgeted Cargo test route with `--lib`. On separate root GO, the prospective whole roster is resident25 + primary7 =32. Do not pass a selector, change the router, request Wasm, compile Plugin, or claim that a lexical source roster is native execution.

The minimal include/allocator delta is the only requested mount. Production/API implementation remains a later separately reviewed action after actual missing-API RED. Original Opening7, full CUT1, RuntimeAppCell/Store/private field funding and SyncSession detach are outside this packet.


# Neutral Primary Test-Only Mount And Exact Inverse

## Outcome

Mounted only the reviewed resident test-module include and hooks into its existing sole allocator. The canonical production authority remains e23ec406. The canonical test module changed e81bcca1→30af821b; the ticket leaf changed57728a88→4f6dc45f only for explicit zero-byte probes.

Before patch, both e23/e81 were rechecked and the full current canonical module was compared with its retained in-memory preimage. After patch, actual full module bytes exactly matched the predicted four-hunk delta. Applying the inverse replacements IN MEMORY to the actual mounted text reconstructed the complete original e81bcca1 preimage byte-for-byte. The inverse was recorded, not executed on disk.

No production methods/fields, native command, new allocator, router, Cargo metadata, Runtime, Opening/CUT1 or Store source changed. Retained confirmed no overlapping resident writer or native process before this mount. No compiler GO is inferred.

## Probe Scope

All five helper/inline short loops now use the fixed allocation-free helper:

```rust
fn refused_grants(bytes: u64) -> Result<impl Iterator<Item = ResidentGrant>, ResidentFault> {
    if bytes == 0 { return Err(ResidentFault::Count); }
    Ok([Some(ResidentGrant::new(0, bytes)?), Some(ResidentGrant::new(1, 0)?), if bytes > 1 { Some(ResidentGrant::new(1, bytes - 1)?) } else { None }].into_iter().flatten())
}
```

The exact cases are zero-items/full-required-bytes, one-item/zero-bytes, and one-item/one-short-bytes. At required bytes1, zero-bytes and one-short are the same and run once. This is NOT all byte lengths.

The shared phase loop covers primary/ordinary constructor phases, begin/advance/match, and the conditional close latch. Dedicated inline loops cover pending allocation, pointerless poisoned Refund/Clear, capture and revoked pin/cursor clear. Seven names, actual layouts, resource capacities,4096 grant bound, original cleanup assertions and ownership requirements remain unchanged.

The pre-mount source-only boundary was recorded BEFORE the canonical patch in [zero-byte-pre-mount](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-zero-byte-pre-mount-2026-08-28.md). No new Nx reference or native command was run for the changed leaf. Historical R3 remains its exact earlier captured source result; it is not relabelled as this revision.

## Mounted Test Delta

Exactly four canonical hunks:

1. Add fixed allocator-entry observer.
2. Split the existing failure/System delegation expression into equivalent local variables and add fixed return observation.
3. Add fixed observer after actual System.dealloc and the two existing release observers.
4. Forward the existing destroy-return hook and add the ticket child plus cfgtest node-load wrapper.

The node-load wrapper has no production callsite yet: the proposed APIs remain absent. The code is intentionally awaiting the separately authorized missing-API native inventory, not claimed compiled. Whole prospective roster is existing25 +7 =32 through the unchanged no-argument resident target. A compiler RED executes0 tests; no selector or Wasm requested.

## Exact Inverse And Recovery Record

[Full machine-readable inverse, forward delta and complete preimage](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-test-mount-inverse-2026-08-28.json):

```text
30d8aaab8b393e00f074f11a3bb571f584b1cc54300f2900807be77efe918a6e  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-test-mount-inverse-2026-08-28.json
```

The inverse below is retained evidence, NOT an instruction to execute it automatically. Any future inverse action must first check the current module exactly matches the recorded30af821b source and must preserve peer edits.

```text
*** Begin Patch
*** Update File: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
@@
-    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
-        primary_recovery::observe_allocator_enter(layout);
+    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
@@
-        let failed = FAIL_NEXT_ALLOCATION.with(|value| value.replace(false));
-        let pointer = if failed { std::ptr::null_mut() } else { unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) } };
-        primary_recovery::observe_allocator_return(layout, !failed, pointer);
-        pointer
+        if FAIL_NEXT_ALLOCATION.with(|value| value.replace(false)) { std::ptr::null_mut() } else { unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) } }
@@
-        release_phases::observe_system_dealloc_returned(layout);
-        primary_recovery::observe_system_dealloc_returned(layout);
+        release_phases::observe_system_dealloc_returned(layout);
@@
-pub(super) fn observe_release_destroy_returned() {
-    release_phases::observe_destroy_returned();
-    primary_recovery::observe_destroy_returned();
-}
-//#endregion 🧪️ReleasePhases
-
-//#region 🧪️PrimaryRecovery
-#[path = "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs"]
-mod primary_recovery;
-
-pub(super) fn observe_primary_recovery_pointer_load(registration: u64) {
-    primary_recovery::observe_recovery_pointer_load(registration);
-}
-//#endregion 🧪️PrimaryRecovery
+pub(super) fn observe_release_destroy_returned() { release_phases::observe_destroy_returned(); }
+//#endregion 🧪️ReleasePhases
*** End Patch
```

## Current Selected Hashes

```text
e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3  🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
30af821b98f7323f487a4ce8c027b6c3b8c7987f0fe64b0d7471ed521945426e  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
4f6dc45ffc159ee419529114f0eeb2a95f8ab6e6982436a6aa0a5f9bc098cc7f  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs
61abe9bf4f41f6dea97ed67f68f044df5055263f88882f8cc4a2e842ba91f964  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🔣️.json
6ac52fcda67a17989d8f5b632f985046fbd8fb3ef7a4fe9e9e5fb07f019f1e04  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🧬️schema/🔣️.json
c4a9fe89e5b06ae1ba6f9a0a344ebe558f59825250f4f3930d6f1dae97a8bcbe  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/📜️script.ts
01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🦀️.rs
4c6b34304b5d9c0746b922d65d2c570bf31679f21ff0747c4b8ff6363f0b3217  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🧪️cut1/🦀️.rs
03fa92102a54e32aba33927d87b41b211a94fc19562c3788a82cc0a4d830ea32  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-zero-byte-pre-mount-2026-08-28.md
```

Only canonical resident tests, ticket native leaf, the pre-mount/mount reports and inverse artifact were changed/created in this turn. Historical reports/raw R1–R3 and canonical fixture/schema/controller were not rewritten. Original Opening7/CUT1 and production authority remain byte-identical. No edit or process remains in flight; awaiting root review before sole-executor compiler GO.


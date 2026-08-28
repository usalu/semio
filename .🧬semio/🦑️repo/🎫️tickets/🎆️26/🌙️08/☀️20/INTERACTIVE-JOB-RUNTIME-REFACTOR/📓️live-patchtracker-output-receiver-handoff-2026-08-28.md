# Live PatchTracker Output Receiver Handoff

The live source now reserves a shared output queue and exact entry before returning `MountedReconcileGrant`. Original `NativeCloseKey` and admission generation survive the surface/grant/producer/ready metadata. Shared job-to-output transfer writes directly into the preadmitted registry entry; the old production ownership-returning `take_ready_patch`/`return_ready_patch` route is absent. A explicitly cfg(test) oracle helper calls the granted in-place route.

## Exact Consumer Surface

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`.

```rust
reserve_mounted(surface: SurfaceId, key: NativeCloseKey)
    -> Result<MountedReconcileGrant<'_>, SurfaceId>
ready_patch_key(&self)
    -> Result<Option<(NativeCloseKey, u64)>, &'static str>
take_ready_patch_into(&self, key: NativeCloseKey, generation: u64,
    target: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize)
    -> Result<bool, &'static str>
```

The receiver must reserve and retain its structural empty target **before** calling take. Pass the peeked original key and admission generation unchanged; neither is reconstructed from surface, revision, current runtime generation, or guest lifetime. Busy/stale/occupied/insufficient-byte outcomes retain the exact original owner. Busy RefCell access currently returns a typed error, not a fake empty/terminal result. The API debits `size_of::<ReadySlot>()` plus the output pool's two-Ready transfer requirement; current native laws pass the existing 32 KiB physical admission, not the 4 KiB wire page/work grant. Do not assume the wire grant admits native owner transfer.

Exact close reservation checks original keys of Surface, Ready, rejected, terminal, producer-terminal, and unadmitted owners. A foreign reused numeric instance fails before close admission. An uncommitted matching grant remains a close obligation; generation survives cancellation.

## Executed Scope

Superseding R8 full-module gate: actual 30 passed, 0 failed, 492 skipped, 0.933s, exhaustive/no-fail-fast/no exclusions. The only R6 failure was corrected in the existing test to account for actual static backing; R7 independently passed with fixed=534368, per=8388608, capacity=3, full=25700192, restored=534368. No production limits changed. Full tool output: [R8 report](./📓️plugin-patchtracker-full-r8-native-2026-08-28.md).

Historical R6 canonical exhaustive full PatchTracker: 29 pass / 1 fail / 492 skipped, all seven live-output laws pass. At that checkpoint the sole old capacity fixture failure was preserved pending its fixed-backing-aware correction. The actual two-thread test fills 63 shared entries, admits exactly one of two concurrent producers, retains the loser, and returns all 64 entries after close. Original same-process R5 2/4 failure and six isolated passing diagnostics remain separate.

Runtime R83 direct receiver: one passing law, 8,936 declared transfer bytes; runtime R84 full121 passed and R85 both Wasm checks compiled. The callback panic in R83 happens after method/guard return, not inside the registry critical section.

## Still Open

Dag owns actual WIT `reserve_mounted`/Pending receiver adoption and exact issued receipt binding. Native fixture keys are not guest lifetime capture proof. Eager PatchTracker fixed banks and producer-constructor allocation/root movement still need canonical composition admission. The consuming producer-to-job construction remains outside the current callback-unwind law. No guest/native aggregate, full Plugin, WGPU, or callback timing completion is claimed.

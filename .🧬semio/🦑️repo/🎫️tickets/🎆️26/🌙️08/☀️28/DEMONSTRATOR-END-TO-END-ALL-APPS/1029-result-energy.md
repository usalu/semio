# 1029 result: energy

STATUS: IN PROGRESS (07:50 10/29). First hub check (energy-chk1) listed 14 red energy crates + stdio-xlsx-strict; all fixed in source below, NOT yet re-verified: the compiler image (48 GiB) was saturated by the fleet (ENOSPC in `energy-m2/m3/m4`), so no check has completed since the fixes. Scripts `C/1029-tools/energy/{chk.sh,mchk.sh,w.sh,errs.py,census.sh}`; logs `C/🗑️generated/1029/energy-*.{txt,rc}`.

## Findings (chk1 root causes)
- The multi-implementation split matters: `snapshot::reference::EnergyModelSnapshot` (explicit owned reference, `Default`, `FromValue`, `Model`, children) is what all `🧊️reference` mutation ops and the `ReferenceEnergyModelDiff` run on; `snapshot::EnergyModelSnapshot` (= `original`) is the native persistent owner (`EnergyModelState` groups, `SealedShared`/`PersistentList`, no Clone/Default/FromValue/ArtifactDsl/ArtifactPack). Several "stale" crates mixed both.
- The 10/28 mut-* split dropped the crate-root `pub use` rows of 7 ops (window-layer-optics, -visible) and left `crate::X` derive references to moved types (`EnergyNativeMutation` derive emits `crate::ReplaceFenestrationVertices`, `crate::AddAnnualScheduleHoliday`).
- The hub requires `ArtifactDsl + ArtifactPack` on the native snapshot (`document_codec_bare::<EnergyModelSnapshot,_>`), so the native snapshot cannot stay DSL-less.

## Fixes (energy-owned)
(see Files changed)

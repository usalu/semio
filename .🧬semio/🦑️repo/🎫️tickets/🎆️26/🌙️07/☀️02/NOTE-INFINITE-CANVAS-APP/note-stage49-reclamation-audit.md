# Attempt 49 Reclamation Audit

No repository-owned command was identified that selects only completed old Note staging generations from this ticket while preserving attempt 48 and all peer data. No reclamation, resize, process change, or deletion was performed.

The compiler unit-cache collector in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts::collectGarbageSingleFlightV1` operates over the compiler image: it collects superseded units, inactive incremental sessions and units, then plans idle-lane eviction. Its `GcPlanV1` identifies lanes and incremental sets, with no ticket or component-generation filter. It rechecks live lane status, but live-lane protection does not establish retention of a completed attempt 48 artifact and its provenance. The previously reviewed general `repo cache gc` does not own Cargo image contents.

Read-only inspection confirmed attempt 48’s run root `.cache/b/r/m/components/runs/wJjV81j` still contains report/provenance, staged descriptors, and both `w/a0026a5091a9a7920bcb1345/component.wasm` and `w/a0026a5091a9a7920bcb1345.linked/component.wasm` (127556752 bytes each at inspection). Their presence does not establish that they are safe to copy or publish. Preserve the entire run and associated cache/provenance until a repository-owned retention-aware operation can prove its custody.

Attempt 49’s log locates its ENOSPC in `tryClaimWithBallastV1` called from `admitImageHeadroomV1`; retry alone cannot be assumed to succeed while the image has insufficient space to create the headroom claim. The scoped recovery requested remains unverified.

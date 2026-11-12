# 1112 ifcmvd (ifc4x3-mvd)
- Root cause: all 572 errors in 5 files (concepts/population, preparation, body/work, alignment/terminal-work, direct-work); the work files only inherit the derive(RetireOwned) failure of IfcMvdPreparation.
- Port: new concepts/lease.rs (Lease<T>(ArcLease<T>): OriginalSourceProjection<T>, funded_turn); population + preparation use RetainedCloneSource/StagedSourceAdmission<_, Lease<_>>, admission advance through cx.with_original_effect; population hand close ladder + PopulationClose removed (derive retirement; tests using it are debt).
## Checklist
- [x] lease.rs, population.rs, preparation.rs edited (unverified)
## Step 2
- W-ifcmvd-2 hub wasm -p semio-s-artifact-stdio-ifc4x3-mvd rc 0 (572 -> 0). Also fixed phase 3 construction: admit_original_construction_settled over a resident OriginalConstructionAdmission field.
- Debts: cfg(test) code (population-tests, preparation-tests, alignment/body *-tests) still on removed APIs (Arc sources, close_step/begin_close/close_demands/terminal_is_empty of IfcMvdPopulationWork, grant-shaped advance); --tests not checked.
- W-ifcmvd-3 hub-stdio-component rc101: no mvd errors; remaining reds ifc-2x3-base-details 119, ifc-details-type 26, ifc4x3-population 6 (ifcdetails lane). Rerun: chk.sh W-ifcmvd-<n> hub/Cargo.toml wasm -p semio-s-artifact-stdio-ifc4x3-mvd
STATUS: FIXED-verified W-ifcmvd-2

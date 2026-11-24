
## E0277 `dyn ErasedSnapshotRetirement` not Sync (derive)

- Root cause: `BudgetedOwnedEntry` -> `RetainedCloneClose` holds `Box<dyn ErasedSnapshotRetirement>` (`Send + Any`, intentionally not `Sync`; 182 implementors incl. `&mut`-holding cursors). `#[derive(RetireOwned)]` emits a shared-original cursor with the bound `#name: Sync` on a non-generic type, which is a trivial bound and hard-errors when the struct is not Sync.
- Fix: the existing opt-out `#[value(send_only)]` (same as `OriginalSourceCloseFailure`, `AffineImageJob`) on `TopologyOutput`, `DeconstructOutput`, `Metadata`, `GeometryOutputData` in `analytic-curves`. Adding `Sync` to the trait was rejected (would constrain 182 implementors; `unsafe impl Sync` forbidden).
- Test: `derive/tests/generic-owner` `send_only_owners_holding_erased_retirement_derive_without_sync`.
- Verification pending: disk gate (9 GiB free < 15) blocked `repo cargo check`.

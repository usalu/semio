# Current Native Compiler Gate

The fresh Draw native check exited 1. Production compilation advanced through Kernel and now fails in Plugin after concurrent API migration. The frozen compiler log is `🗑️generated/2026-10-09-native-plugin-current.log`. No full native Draw or mounted editor pass is claimed.

```json
{
  "error[E0432]: unresolved import `crate::store::SnapshotRetirementStep`": 1,
  "error[E0432]: unresolved import `store::SnapshotRetirementStep`": 6,
  "error[E0432]: unresolved import `store::os_store::SnapshotRetirementStep`": 1,
  "error[E0433]: cannot find `SnapshotRetirementStep` in `semio_framework_value`": 2,
  "error[E0433]: cannot find `SnapshotRetirementStep` in `store`": 154,
  "error[E0407]: method `next_close_byte_demand` is not a member of trait `semio_framework_job::InteractiveJob`": 2,
  "error[E0425]: cannot find value `owned_retirement` in module `semio_framework_value::retirement`": 2,
  "error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`": 14,
  "error[E0404]: expected trait, found derive macro `RetireOwned`": 3,
  "error[E0425]: cannot find type `SnapshotRetirementStep` in crate `store`": 15,
  "error[E0433]: cannot find `os_vcs` in `crate`": 1,
  "error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in crate `store`": 2,
  "error[E0425]: cannot find type `HistoryInputDrafts` in crate `protocol`": 6,
  "error[E0433]: cannot find `completion_retirement` in `crate`": 4,
  "error[E0433]: cannot find module or crate `kernel` in this scope": 4,
  "error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in crate `store`": 1,
  "error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2": 28,
  "error[E0050]: method `begin` has 2 parameters but the declaration in trait `dsl::ArtifactStoreOneItemPreparationFactory::begin` has 3": 2,
  "error[E0046]: not all trait items implemented, missing: `begin_demand`": 2,
  "error[E0046]: not all trait items implemented, missing: `next_close_copy_byte_demand`, `next_close_capacity_byte_demand`, `next_close_release_byte_demand`, `next_close_depth_demand`": 3,
  "error[E0050]: method `retire` has 2 parameters but the declaration in trait `retire` has 3": 7,
  "error[E0050]: method `close_step` has 3 parameters but the declaration in trait `dsl::ErasedSnapshotRetirement::close_step` has 2": 8,
  "error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`": 8,
  "error[E0050]: method `retire_owned` has 2 parameters but the declaration in trait `retire_owned` has 3": 4,
  "error[E0046]: not all trait items implemented, missing: `retirement_birth_bytes`": 4,
  "error: could not compile `semio-framework-plugin` (lib) due to 284 previous errors; 684 warnings emitted": 1
}
```

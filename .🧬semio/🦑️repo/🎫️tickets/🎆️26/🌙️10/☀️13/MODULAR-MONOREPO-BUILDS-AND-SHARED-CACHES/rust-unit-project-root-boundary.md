# Rust Unit Project Root Boundary

Read-only inspection in response to the ordinary-pair root-collision finding. `rustUnitNxProjectsV1` currently sets a unit project root to the owning Cargo manifest parent. The ordinary package aggregate has that same root, and the graph reader correctly refuses duplicate roots. The aggregate must remain; graph root uniqueness must not be weakened.

The selected descriptor producer already writes a distinct immutable closure descriptor for each selected compiler unit. Its current layout is `captureDirectory/units/<descriptor-digest>.json`, so all these files share one parent. A correct unique physical project root can be the containing directory of `captureDirectory/units/<descriptor-digest>/descriptor.json`. This binds scheduling identity to an actual admitted preparation descriptor directory without inventing an unrelated virtual source root.

The final selected unit project retains `metadata.nativeRoot` for the actual Cargo manifest parent, explicit compiler Source inputs and immutable descriptor observations, and the command cwd remains the workspace. The helper currently emits projects for every unit in a closure before the producer filters the selected root. Assigning the same descriptor parent to all of those intermediate emitted projects would create another collision. Either explicitly select the one emitted unit or assign the physical root only to the final selected project after filtering.

Ignored preparation descriptor roots remain outside broad native workspace file inventory. Genuine selected task planning/hashing and pinned-graph observation must prove those exact files explicitly. Native whole-workspace inventory is a separate proof boundary; no static root change can claim to close it by itself.

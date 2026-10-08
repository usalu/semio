# Process Archive Door Live Stack

Bounded read-only macOS sample: PID57626, exact `every_example_loads_through_the_member_less_archive_door`, elapsed7m22s and15.1% CPU at observation. One-second sample retained under generated output; no process or lease changes.

The sampled test thread is inside `settle_member_less_archive` → `poll_document_archive_load` → `maintenance_step`. One stack follows `maintenance_stage_step` → `drive_store_replacement_jobs` → `ActiveArtifactStoreReplacement::drive_member_open` → SemioMembersOpen → `InitialMemberStoreOpen<SemioBrepSnapshot,SemioBrepMutation>::step_store` → `check_authority` → StepContext clock. Another hits envelope decode worker clock. Main test harness and pool workers wait normally. This is an active maintenance/member-open path, not evidence of a dead graph/preparation lease. The small sample cannot establish whether the work is finite or blocked by a grant; root/publication owners need the existing bounded phase protocol to distinguish it.

The run is preserved under its original assertion budget. No timeout increase, query override or kill was applied.

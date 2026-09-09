# Inference Maintenance Close Review

## 2026-09-09 Source Review

Root inspected the current production completion paths for both abandoned
requests and prepared approvals in `🌎️hub/💡️inference/🏃️runtime/🦀️.rs`.
Each now copies only maintenance/cleanup/watch ownership into a completion
record, clears its request owner, drops the full task/committer, and only then
removes the maintenance barrier and releases the cleanup reservation.

The completion record has no database, Store, verifier, document map or publisher
reference. Clearing the task owner prevents its Drop implementation from parking
already-completed work. The close loop tests both maintenance and cleanup
emptiness after its document map is drained, so the completion notification no
longer precedes that task's Store-owning committer release.

This review supports the intended ownership ordering; it does not qualify the
native law. The authoritative native receipt is
`gis-map-proposal-native/exact-cargo-laws-eou3J6/00`, owned by Home. The preceding
receipt passed laws0–2 but law3 failed on one unreturned Store snapshot lease.

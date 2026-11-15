# Sealed partition group failure

## Cause
`No coherent authenticated partition group is available` came from `acquirePartitionGroupV1`: the door snapshot (5d20d8b498d1) and the task kernel snapshot (363abba48753) were captured from different states of the moving live tree. Five files overlap with different hashes (storage retirement-file, storage whole, observation `.mjs`, project-assembly `.mjs`). `sealedTreesCoherentV1` correctly refuses such a pair. Under `explicit` promotion the door is never re-promoted when it is merely drifted, so a freshly promoted kernel could never pair with an older door.

## Fix
`🚦️trampoline`: when the candidate group (door plus kernels) is not usable, the start re-promotes the door and all kernels from the live tree and re-reads the pointers, up to 4 attempts (`partitionAttemptsV1`). `partitionUsableV1` is exported from `🔀️partition/📌️retention`. Law: `🧪️tests/🔀️partition/📌️retention` (door and kernel from different tree states are not usable until both match).

## Verification
`SEMIO_REPO_SEAL=sealed repo show project @semio-tech/norm-plugin` promoted a new coherent door (ab18bac04a7c) and runtime kernel, passed the partition admission and reached `door.capacity` (waiting for free memory on the loaded host). Log: `🗑️generated/sealed-show.log`.

## Separate blocker (not the seal)
`describe18live.log` died with "The graph service was denied access to the checkout ... macOS does not let `repo` read your Documents folder": grant Documents Folder (or Full Disk Access) to `/Users/ueli/.local/share/semio/launcher/bin/repo`. This also hits sealed runs.

## Not done
The sealed describe (describe19sealed) was not launched; the coordinator asked for a light check first. `repo test` on the partition tests returned exit 0 with empty output, so the new test was not confirmed executed.

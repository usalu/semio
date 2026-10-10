# Current Disk Audit

2026-10-09T16:06:09.105217+00:00

Read-only examination; no builds, deletes, cache mutations, or process changes.

Filesystem available: /dev/disk3s5   971350180 922162144   7757684   100% 22495681 77576840   22%   /System/Volumes/Data

## Ticket Candidates

All candidate paths below are relative to this ticket’s `🗑️generated`. Sizes are allocated bytes (du -sk × 1024).

- `nn`: 352,559,104 allocated bytes; process argv references 0; open-file references 0; registered matching lease resources 0. Generated Nx data/log output candidate; Root must confirm no scheduled reuse before removal.
- `native-job`: 55,566,336 allocated bytes; process argv references 0; open-file references 6; registered matching lease resources 0. EXCLUDED: six open descriptors actively write router-identity-source3.log, held by bun 19076/19078 and node 19487; preserve the entire directory.
- `ns`: 36,737,024 allocated bytes; process argv references 0; open-file references 0; registered matching lease resources 0. Generated Nx data/log output candidate; Root must confirm no scheduled reuse before removal.
- `nx-root-canonical-join-data`: 27,709,440 allocated bytes; process argv references 0; open-file references 0; registered matching lease resources 0. Generated Nx data/log output candidate; Root must confirm no scheduled reuse before removal.
- `nx-physical-preparation-birth-data`: 27,705,344 allocated bytes; process argv references 0; open-file references 0; registered matching lease resources 0. Generated Nx data/log output candidate; Root must confirm no scheduled reuse before removal.

No existing `cargo-root-norm-owned` or `cargo-native-job` build directories remain. This ticket contains approximately 639,692,800 allocated bytes total, so it cannot explain 922 GB of used disk by itself.

## Original Resource Lease Evidence

4 lease resource registrations reference this ticket. Registration is persistent metadata, not proof of a live lease. Live evidence checked through fresh lsof open-file snapshot:
- `9032b94cfb8640cb5e08e222409adfdae3fd566201924e11a295dd1ba6892e95.sqlite` open=False: `["semio.cargo.build-lease/v1","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated/cargo-native-job","debug"]`
- `4ec5d69deea16fa9dd54fd02813f3020850f1209fa6caee5ed49e6ceb83334ef.sqlite` open=False: `["semio.cargo.build-lease/v1","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated/cargo-root-norm-owned","debug"]`
- `39513e0366861fcdd163860ffbb4add78efdd4e4e31adf502c7d164710106fd1.sqlite` open=False: `["semio.cargo.build-lease/v1","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated/cargo-root-norm-owned","wasm-dev"]`
- `23038755a5a2f286275b71049023027948ba187e6e6d3897c6ddcceb70d4aacf.sqlite` open=False: `["semio.cargo.build-lease/v1","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated/cargo-io-owned","wasm-dev"]`

## Nx Outside Deletion Scope

`.nx` allocated bytes: 1,091,944,448; workspace-data 1,051,881,472; workspace-data-terminal 40,054,784. Main graph 175,742,773 bytes, source maps 123,140,415 bytes, file map 28,502,437 bytes, nx_files 23,207,980 bytes. Watchers and dashboard each account for approximately 323 MB. These are generated infrastructure outputs, but preserved because active Nx processes hold workspace database/graph locks.

Fresh open-file proof includes node PIDs 15355 and 16295 holding workspace DB/WAL/SHM; 17894 and 18128 holding project-graph.lock. Fresh Cargo PID 17217 builds ARTIFACTIO ticket output; preserve that output and shared cache.

No source/input/report deletion candidates. No shared Cargo cache deletion candidates. Process/open-file snapshots are momentary; recheck immediately before deletion.

Root reports no deletion needed with approximately 7.4 GiB available. Do not infer that the original terminal ENOSPC native run remains active: other tasks are currently active. Fresh 16:06 UTC lsof finds router identity output held in this ticket as documented above. No disk recovery is attributed to this read-only audit.

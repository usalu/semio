# g1-m10 fleet state recovery

The server’s `attest.json` stopped advancing while the replica was following the journal. The local event-loop watchdog and the worker’s readiness and pool reports continued independently. The old server record carried 44,102 incoming entries, and the replica recorded repeated NFS store stalls. A slow follow turn could hold up the heartbeat even when the mount could still serve the small status write.

The replica now schedules attestations independently of the replication loop. It publishes with a separate deadline guard and keeps at most one attestation tick in flight. It copies the cursors with the manifest snapshot so an asynchronously published record describes one observation. The normal shutdown cancels the heartbeat, and an approved code restart still exits at a safe replication boundary.

The regression test holds the replication turn open and checks that the server receives successive, schema-valid attestations through the real store writer. It failed on the previous implementation and passed after the fix. Ajv independently validated the on-disk record. The attestation and replica resilience suites passed all 157 tests.

## Local rollout

The previous replica shut down gracefully and saved state. Its launchd job returned, but the installed native launcher blocked before starting the replica. A process sample showed the launcher waiting in `open`. I started the installed fleet boot entry through the repository-managed Bun runtime, and its worktree replica acquired the existing daemon lock and entered the follow phase.

The server’s NFS store showed two new attestations from the updated daemon at 23:51:17 and 23:51:37 CET. The reported incoming queue dropped from 41,170 to 41,110 between those receipts. The recovery process is detached. The launchd-managed native launcher was still blocked during verification, so automatic launch after logout or reboot remains unverified.

The first rollout also exposed a startup delay in the readiness reporter. Its supervisor was serviced only after a replication turn completed. The heartbeat now also ticks the sidecar supervisor once the replica has read its retirement state, so the readiness reporter can start and renew its lease while journal catch-up continues. A second regression test holds a later replication turn open and observes the sidecar start.

After reloading with the sidecar change, the server received a fresh attestation at 23:55:13 CET and a fresh `ready` record at 23:55:14 CET. The build agent registered with the scheduler at 23:55:14 CET. These updates arrived with 523 entries still reported in the replication queue.

The final server check showed `ready` and zero incoming replication entries. Both regression tests and all 158 attestation and resilience tests passed. Two existing sidecar process tests exceeded Bun’s default five-second timeout in the combined run. All 12 sidecar tests passed when rerun with a 30-second per-test timeout.

# Peer Publication Gap — 2026-11-21

At 03:19 UTC the g1 coordinator’s local shared-store origin records showed:

- g1-m04: last attestation 2026-11-20 23:43:11 UTC; last Ready and pool publications around 23:53 UTC. The old Ready record says tools running/adopted; it is historical, not current liveness.
- g1-m06: attestation, Ready and pool last published about 2026-11-20 18:25 UTC. The old Ready record says workspace running/adopted; it is historical.
- g1-m09: origin directory contained only the registration file `fleet-auth.json`; there was no attestation, Ready, pool or head publication. Its contents were not read.

No per-peer current boot heartbeat was present in these origin records. Coordinator health reported its local store `ok`; this verifies neither peer mount nor transport. The topology declares the g1 NFS store and locally supervised member sidecars. The earlier `audit-m09-startup-surface-2026-11-20.md` inspected the registered replica CLI: `run` is a local operation and provides no remote origin selector. Together these observations identify no supported coordinator-side recovery for the three peers. Old or absent publication does not establish the host is offline, a failed boot, or a transport fault. The next distinguishing evidence is each machine’s own boot/replica and store-mount status and logs. No remote operation, restart, process control or peer access change was performed.

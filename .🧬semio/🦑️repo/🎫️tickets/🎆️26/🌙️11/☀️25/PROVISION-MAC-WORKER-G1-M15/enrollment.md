# Mac Worker Enrollment

The Apple silicon Mac has 16 GiB of memory and an existing checkout at `/Users/ueli/Documents/semio`. Initial inspection found no local replica identity, mounted store, or replica service. The checked-in fleet hostnames did not resolve, so the enrollment plan uses the documented `g1` LAN address `192.168.178.70` and subnet `192.168.178.0/24`. The server answered on NFS and cache ports, and `showmount` listed `/Users/Shared/semio-store`.

The pinned launcher, Bun runtime, and native repo command were installed. The native MCP server handled the goals resource and opened this ticket. The proposed origin is `g1-m14`, following the existing numbering. The authoritative origin check and adoption plan are pending.

Noninteractive sudo requires a password. The background macOS administrator command was denied access to the provisioner under Documents. macOS also denied Apple Events access to Terminal. Opening a Terminal command through Launch Services worked. The authenticated plan refreshed sealed replica code, preserved the replaced copies under `.cache/f/lost`, and mounted the store. The helper records the adoption plan in the ticket's generated folder.

The live registry showed that `g1-m14` belongs to another active Mac. Its latest readiness record and replica attestation were fresh. The store had no `g1-m15` entry, so enrollment uses `g1-m15`. This ticket supersedes this session's initial local `g1-m14` ticket entry.

The plan exited 0 and reported shared Git history, a fleet-ahead tip, zero staged paths, 293,012 identical files, and 7,326 stale versions. The two new paths were this session's enrollment notes and plan helper. The sole conflict was the initial `g1-m14` ticket metadata, which already existed in the fleet. The plan will be recomputed by the provisioner during adoption. Its conflict handling preserves the local copy under `.cache/f/lost`.

The live adoption finished with 293,011 identical files, 190,359 taken from the fleet, five new paths published, 7,327 stale copies preserved locally, one conflict preserved locally, and zero entries queued. The adoption report is `.cache/f/lost/adopt-2026-11-25T20-55-49-343Z.md`. The provisioner loaded the replica and compaction LaunchAgents. The store received a live attestation for `g1-m15` with synchronization enabled. The remote-cache command confirmed the endpoint was reachable and authorized.

The first fleet preflight verified identity, mount options, store writes, LaunchAgents, and replica access to Documents. Replica status, file watcher, and event visibility timed out during initial catch-up. Native readiness was still running.

After catch-up, replica status reported `g1-m15` as `ok` against a shared checkpoint with the group's current replica code. The launcher requested fresh Documents access after the code change; a later preflight confirmed access. A verification run during live replication checked 492,572 shared paths and reported six differences, plus ten existing overlong paths recorded as refused. The watcher and event visibility passed in a separate run. Fleet code changed its address option during setup, so verification used the current topology's explicit `--addresses g1-server=192.168.178.70`.

The foreground provisioner exited 3 when the Rust toolchain installer hit a component conflict. The automatic readiness supervisor retried the native stage successfully, installed the native repo command, restored dependency scopes from the fleet cache, and completed tool setup. Workspace setup is the final stage.

A later fleet preflight completed all 23 checks with zero failures. It verified the correct NFS mount options, writable store, daemon, compaction, launcher access, watcher, and cache endpoint. The event-visibility p95 was 1,166 ms from 100 samples, below the 5,000 ms limit. The two warnings were pending native readiness and the privilege required to read the system clock synchronization policy.

The remote-execution scheduler reported a fresh, non-draining worker for `g1-m15` with the Darwin sandbox and macOS arm64 Rust toolchain. The worker advertised no spare build capacity at that sample, and no offloaded job had been recorded for the current worker process.

A second live replica verification checked 492,723 paths and found ten differences on a different set of paths, plus the same ten overlong refusals. The subsequent status showed synchronization enabled, zero events behind for `g1-m15`, no queued events in its checkpoint row, and a difference in the Git worktree view. It reported no manifest difference for this origin.

Fleet enrollment is complete. The installed readiness supervisor is still running the final workspace build check. Shared-tree changes caused several graph passes to restart, and the graph worker handed off once at its configured heap threshold. The replacement reused verified fleet records and continued reading owners. At the last check, `ready.json` reported `ready: false` at stage `workspace` and `.cache/f/native-bootstrap.done` was absent. The remote-execution scheduler still reported a fresh heartbeat and no spare capacity. No offloaded workload was submitted as part of this enrollment.

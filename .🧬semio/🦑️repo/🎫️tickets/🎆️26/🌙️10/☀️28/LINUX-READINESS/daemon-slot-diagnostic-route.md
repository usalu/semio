# Nonpruning Slot Holder Diagnostic

Read-only Source/API inventory; no lock files, live registry, process table, Native or diagnostic query inspected/executed by this reviewer.

Exact slot resource for resolved root /home/deck/semio and finite index0 is nx:daemon-slot:5f78bd319d83b5ec:finite:0. Derive through nxDaemonSlotResourceV1(root,finite,0), not an independently guessed schema. Canonical directory is repositoryLeaseDirectoryV1(root), which returns the materialized leases partition root/data; the ordinary CLI coordinate is root/.cache/b/l/m/data.

Strictly nonpruning API: otherLeaseHoldersV1(directory,resource), exported through process leases/order and the leases barrel. Registry implementation calls readLeaseRegistryEntriesV1 with prune=false, selects exact resource holds and omits this process. For fuller evidence use readLeaseRegistryEntriesV1(leaseRegistryRootV1(environment),directory,Date.now(),false) and filter the exact holds. It returns pid,command,startedAtMs,updatedAtMs,standing,holds mode/since/task and ancestry. Keep the explicit false argument.

The supported repo leases status --json route is not strictly read-only under this task: readLeaseReportV1 calls the registry reader with default prune=true, which can unlink dead/stale registry records. It is appropriate ordinary diagnostics but differs from the requested nonmutating query. Do not substitute raw lock/registry file reads.

Registry liveness is advisory: row shape and directory, process.kill(pid,0) with EPERM considered alive, and a finite heartbeat age. It does not independently verify kernel start epoch here, even though startedAtMs is available. No row does not prove that the physical slot lease is free. Retain the existing genuine public test Original/provider context for the one coordinator query; these read APIs do not justify issuing a broader Original or new Native authority.

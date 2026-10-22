### 🔒️ Leases, lock order and queues

Every shared resource is claimed through a lease: a SQLite file `<leaseDir>/<sha256 of the resource name>.sqlite`, plus a `.queue/` directory of tickets for the queued kind. Queued leases (`acquireQueuedResourceLease`) are served in arrival order and a writer is not starved by readers that arrive after it; the raw kind (`acquireResourceLease`) is polled and prefers readers. A waiter at position 2 or later prints a `lease-queue` line every 15 s that names the holder's process id (see Waiting for a store in the reference). The rest of this section is the contract for lock order, deadlocks and the status command ({{@leases}}).

**Lock order.** {{?leases::::The rules below are those of the staged set. }}A resource name falls into one class; a class has a rank, a scope and an owner, all declared in the lock-order table {{?leases::`🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json`::of the lock-order policy}} (a closed schema next to it, English and German messages). A task that holds a lease and asks for a lower rank is out of order. Unranked resources are rank 0 and never constrained.

| Rank | Class | Scope | Resources (examples) |
|---|---|---|---|
| 1 | compiler capacity | machine | `semio.compiler.cpu-picker/v1`, `semio.compiler.cpu/v1/<n>` |
| 2 | tool singletons | checkout | `nx-tooling-*`, `bazel-*` (metrics, snapshot restore, slots), `bun-*`, `rustup:*`, `cache-prune` |
| 2 | Cargo coordination | crate | `cargo:registry:*`, `cargo-preparation:<root>`, `cargo:source-admission` |
| 3 | graph | checkout | `nx:graph-pool:*`, `repository-graph-facts:<workspace>`, `build-graph-*` |
| 4 | namespaces | project | `rust-*`, `javascript-dependency-*`, `typescript-package-binding:*`, `native-snapshot*` |
| 5 | artifacts | path | `artifact:<path>` (shared reads, exclusive publish) |
| 6 | images | whole image | `native-fixed-image:<directory>` (exclusive for sync, install and walks, shared for readers) |

- **Rules.** *Out of order* is reported (`observe`, the shipped mode) and flips to a hard failure with one line of policy once `repo leases status` shows no violation during the end-to-end battery. *Hold-and-wait* fails a blocking request that has waited longer than 15 s while its task holds another lease exclusively; the message names the held lease, the blocking process, its pid and how long it holds. Shared holds, declared edges and try-locks are exempt. *Self-deadlock* (shared then exclusive on the same lease, or exclusive then anything unbounded) fails at once. *Cycle*: every waiter checks the wait-for graph every 500 ms and exactly one member of a cycle, the latest waiter, fails while the others proceed. `SEMIO_LEASE_ORDER=off|observe|enforce` overrides all four.
- **Waiting no longer blocks admission.** A raw acquire used to keep the shared directory-admission gate while it waited for a busy resource, which blocked every other acquire; it now takes the gate per attempt and waits outside it. `withResourceLeases` acquires in rank-then-name order through the fair queue, so three overlapping readers cannot starve a writer.
- **For authors.** Take the narrowest resource that makes your step correct (per project, crate, generation or path, not per checkout); never hold an exclusive lease while you wait for another; when you introduce or split a key, add a class with its scope and owner to the table. Optional and separate: `SEMIO_LEASE_INHERITED` makes child processes count their parent's holds, which can make previously accepted parent-child nestings fail the self-deadlock and hold-and-wait rules, so it ships after one clean cycle.

```bash
repo leases status
repo leases status --json --violations=50
```

- **`leases status`** {{?leases::::(staged) }}shows the holders (mode, class, rank, pid, hold seconds, command), the waiters (queue position, wait seconds, blockers), the wait-for edges, deadlock cycles with their victim, the recent order violations and the machine admission queue of the host capacity governor with each waiter's reason. Options: `--json`, `--all`, `--lease-directory=<path>`, `--violations=<n>`. It reads the per-user registry (`~/Library/Caches/semio/lease-registry`, `$XDG_CACHE_HOME` or `~/.cache`, `%LOCALAPPDATA%`; `SEMIO_LEASE_REGISTRY_DIRECTORY` overrides it; dead pids are pruned, heartbeat 60 s) and the queue tickets, so processes on older code still appear as holders and waiters, without names.
- **Limits.** Task attribution is per invocation, so parallel sub-tasks of one invocation count as one task; deadlock detection sees only processes on the new code; the queued `withResourceLeases` adds claim and queue file churn per acquisition; the compiler capacity picker still holds its lease while it polls cores, so a small request can wait behind an unsatisfiable `exact:N` request.
- **If a command waits today.** Read the `lease-queue` line for the holder's pid and `[capacity]` lines for the governor; both are queues, not hangs. {{^leases::Do not start a second copy of the same long command, because it joins the queue behind the first::`repo leases status` lists who holds what and who waits for whom.}}

# Resource Leases

## Design and Verification Plan — 2026-09-08

Existing artifact publication uses an exclusive-create PID file. A crash can leave it behind, and readers/Nx cache restoration do not participate. Session records reject stale UUIDs but their read/check/write sequence has no cross-process transaction. These are remaining concurrency gaps, not completed safeguards.

The proposed primitive holds a rollback-mode SQLite transaction for a resource’s lifetime: shared reads coexist; an exclusive transaction excludes other readers and writers. The operating system releases file locks on process exit. Each resource uses a stable hash-named database, outside cached deliverables, so invocation IDs do not create an unbounded namespace. No heartbeat or age-based lock stealing is involved. The [SQLite locking specification](https://www.sqlite.org/lockingv3.html) describes the kernel-backed shared/exclusive behavior and Unix/Windows backends.

Bun’s [built-in SQLite interface](https://bun.sh/docs/runtime/sqlite) and Node’s [built-in SQLite interface](https://nodejs.org/api/sqlite.html) will sit behind a repository-owned interface. No new package dependency is needed. Native Python SQLite will act as an independent process oracle; language-neutral cases will cover sharing, exclusivity, unrelated resources, waiting cancellation, process death and repeat-use storage.

Required boundaries: the database must stay in rollback mode; an acquired lease is released after the protected operation stops, never merely when cancellation is requested. Waiting must be cancellable and report progress. Multiple resources must be acquired in stable order. The implementation must reject symlinked database/store paths and foreign database identities. Native Nx cache restoration and every relevant producer/consumer still need explicit integration before this primitive can claim to protect the whole monorepo. Network filesystems and platform-specific qualification remain separate gates.

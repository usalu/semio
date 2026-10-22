### 👥️ Many agents in one folder

Several developers and agents work in one checkout at the same time, on different crates and packages. The build system is arranged so that unrelated work does not wait: results are content addressed and shared, a writer holds a shared resource only while it publishes, per-session state is private, and every wait is a first-in-first-out queue that names who holds the resource. This section is the map; the sections after it hold the details. Every feature carries its state. **Live** means applied in the checkout. **Staged** means built, tested in an overlay copy of the tree and waiting to be applied: its commands do not exist yet, and its description is the contract it ships with (the numbers quoted for it are overlay measurements, not live ones).

| Feature | Command or switch | State |
|---|---|---|
| Host capacity governor | `repo capacity status`, `repo capacity run`, `SEMIO_HOST_CAPACITY=off` | {{@capacity}} |
| Capacity ramp and bounded overtaking | automatic | {{@capacity-ramp}} |
| Fair lease queues that name the holder | the `lease-queue` progress line | live |
| Lock order, deadlock detection, lease status | `repo leases status`, `SEMIO_LEASE_ORDER` | {{@leases}} |
| Incremental conformance judge | `repo incremental` | {{@incremental}} |
| Concurrent-session acceptance harness | `repo concurrency` | {{@concurrency}} |
| Cache ledger, budgets, lock-aware collection | `repo cache status`, `repo cache gc` | {{@cache-ledger}} |
| Graph policy and graph generations | `--graph-policy=fresh\|current` | {{@graph-policy}} |
| Per-session Bazel server slots | `SEMIO_BAZEL_SERVERS`, `SEMIO_BAZEL_SERVICES` | {{@bazel-sessions}} |
| Cargo lanes and compile-unit cache | `repo cargo --store=shared`, `SEMIO_CARGO_LANE`, `SEMIO_UNIT_CACHE_SLOTS` | {{@cargo-lanes}} |
| Shared Cargo build-script runs and component passes | `SEMIO_UNIT_CACHE_BUILD_SCRIPTS` | {{@cargo-buildscript}} |
| Nx graph reuse and pool without hold-and-wait | automatic | {{@nx-generation}} |
| Nx daemon cap and idle shutdown | `SEMIO_NX_DAEMONS`, `SEMIO_NX_DAEMON_IDLE_MS` | {{@nx-daemons}} |
| Nx cache put slots | `SEMIO_NX_PUT_SLOTS` | {{@nx-put-slots}} |
| Concurrent dependency generations | `repo dependencies ensure`, `repo dependencies sync` | {{@deps-concurrent}} |
| Lease-free generation maintenance and size admission | automatic | {{@deps-concurrent-admission}} |
| Incremental dependency sync | `repo dependencies sync` | {{@sync-incr}} |
| Per-crate cache keys | automatic | {{@crate-keys}} |
| Narrow, derived task inputs | automatic | {{@input-precision}} |
| Per-package TypeScript check units | library engine, no command yet | {{@ts-units}} |
| Test sandbox | automatic in `repo run <project>:test` | {{@sandbox}} |
| Output parity of Nx and Bazel | `repo verify build-parity` | {{@parity}} |
| Recovery after a reboot or a crash | automatic at the next `repo` command | {{@boot-sweep}} |
| CI on the neutral interface with a shared remote cache | `BUILD_TOOL`, `SEMIO_REMOTE_CACHE_ENDPOINT` | {{@ci}} |

Rules for working together:

1. **Everything through `repo`.** Never run `nx`, `bazel`, `cargo` or `rustc` yourself and never set a private `CARGO_TARGET_DIR`: the shared stores are fixed-capacity partitions, and private copies exhaust the disk (a fleet of private Cargo targets reached 926 GB on 2026-10-20).
2. **Work on your own project.** Edit the crate or package you own and run its targets; `repo run <project>:<target>` builds the closure of that task only. Files that many projects read are shared inputs and re-key wide areas: the root `Cargo.toml` and `Cargo.lock`, `nx.json`, the `tsconfig` roots, every `package.json` and `bun.lock`, the taxonomy files. Batch such edits and tell the others (Incremental development shows what they cost).
3. **Ask the machine before a heavy command.** Wrap an ad-hoc heavy command in `repo capacity run --memory=<megabytes> -- <command>` and read `repo capacity status` before you start several things at once (Host capacity).
4. **Waiting is a queue, not a hang.** Read the line it prints; it names the holder. A holder that makes progress works for everybody: do not kill a process you did not start. Ctrl+C cancels yours and gives back what it held.
5. **One dependency change at a time.** Lock and sync are repository-wide. Run them once, after the last manifest edit; everybody else runs `repo dependencies ensure`, which installs only when something is missing or changed.
6. **Tests use their sandbox.** A test never writes into the checkout, into a shared cache route or to a fixed port (Test sandbox).
7. **Leave build state alone.** Never delete or detach anything below `.cache/b`, never move a receipt; use the controls named in Recovery after a reboot or a crash.

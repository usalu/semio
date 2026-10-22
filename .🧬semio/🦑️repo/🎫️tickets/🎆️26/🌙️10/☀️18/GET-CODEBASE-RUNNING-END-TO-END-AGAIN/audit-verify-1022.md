# 🔎️ Verify Audit 1022 (auditor verify-audit)

Date: 2026-10-22. Scope: every law, verification and diagnostic command the repo CLI offers (`bun repo --help` and the subcommand help texts). Brief: `brief-wave-1022a.md`.

## Method

- Every run: `cd /Users/ueli/Documents/semio`, `bun repo <args>`, foreground under `nice -n 10`, one at a time, bounded by a shell watchdog (max 9 min). Logs are in the scratchpad `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad/verify-audit/`.
- `bun repo verify` does not exist. `repo --help` lists no `verify` command, so the brief's `verify` wording maps to `tools verify`, `preflight`, `cache verify`, `leases status`, `incremental`, `concurrency`. The `verify dependencies literal-external` gate named in memory is not in this CLI and was not run.
- Host state during the run: 8 to 10 heavy repo processes from other agents, memory near the floor (712 MB spare above 6553 MB at the last check), 4 cargo requests queued for capacity.
- Side effects: `cache status`, `cache verify --deep` and `cache gc --dry-run` write their ledgers. `incremental prepare` and `incremental discard` used the disposable roots `/private/tmp/sinc1022` and the default temp root (never created). Nothing in the checkout was modified by the auditor.
- Process hygiene: my stalled `show targets` (pids 80529/80530) was killed by me. No process started by me remains. Other agents' processes (pids 10113, 17643, 49807, 54172, 54769) were left alone.

## Command table

Durations are wall-clock. The first attempts of `remote-execution status|env`, `incremental list` and `cache gc --dry-run` were killed by my 60 to 284 s watchdogs (see notes) and rerun with longer bounds; the rerun durations are shown.

| Law / check | Command | Verdict | Duration | First failing findings / note |
|---|---|---|---|---|
| Unknown `verify` (brief name) | `bun repo verify --help` | FAIL | untimed | `error: Unknown command: verify` (the CLI prints its command list) |
| Repo help | `bun repo --help` | PASS | untimed | Lists `tools`, `preflight`, `incremental`, `concurrency`, `dependencies`, `cargo`, `cache`, `capacity`, `leases`, `remote-execution` |
| Host preflight | `bun repo preflight` | PASS | 20 s | exit 0, no findings |
| Pinned tools | `bun repo tools verify` | PASS | 50 s | All 20 pinned tools match their pins |
| Lock order | `bun repo leases status` | FAIL | 5 s | Lock order violated: `semio.compiler.host-capacity/v1` (rank 1) requested while `rustup:home` (rank 2) held shared; 10 recent violations (see F1) |
| Cache ledger | `bun repo cache status` | PASS | 25 s | About 30 undeclared entries (`repository:undeclared/*`, `native:impl-exp`, `native:qe2e`, ...), collected after 14 days (F4) |
| Remote cache contract | `bun repo cache remote status` | PASS | 36 s | `configured: false` (optional, not in use) |
| Cache verify (deep) | `bun repo cache verify --deep --json` | PASS (weak) | 15 s | exit 0, `mismatches: 0`, `stale: 0`, but `groups: 2, verified: 0, bytes: 0`: nothing was actually re-hashed (F5) |
| Cache collection dry run | `bun repo cache gc --dry-run` | PASS | 417 s (rerun; first attempt killed at 284 s) | Would remove 2741 entries, 12.63 GiB, 48 kept (F4) |
| Remote execution status | `bun repo remote-execution status` | PASS (optional service) | 25 s (rerun; first attempt killed at 120 s) | Docker daemon not running; `endpoint.answers: false` (F6) |
| Remote execution contract | `bun repo remote-execution env` | PASS | 25 s (rerun; first attempt killed at 60 s) | Prints `SEMIO_REMOTE_EXECUTION_ENDPOINT=grpc://127.0.0.1:8980`, platform `linux-arm64`, instance `main` |
| Machine capacity | `bun repo capacity status` | PASS | 5 s | 712 MB spare above the 6553 MB floor, 8 to 9 running, 4 cargo waiting (informational) |
| Incremental scenarios | `bun repo incremental list` | PASS | 20 s (rerun; first attempt killed at 60 s) | 15 scenarios across framework-ts, rust, wasm, vite, test, activation areas |
| Incremental plan | `bun repo incremental plan --scenario=all` | FAIL | 15 s | `Prepare the disposable root first` (blocked by F2) |
| Incremental prepare | `bun repo incremental prepare` | FAIL | 46 s | Default root: `Path exceeds 255 characters: /var/folders/.../T/sinc/✏️s/🔌️plugins/✒️writer/.../🪆️subsets/✳...` at `🏃️runner/🟦️.ts:104` (F2) |
| Incremental prepare (short root) | `bun repo incremental prepare --root=/private/tmp/sinc1022` | FAIL | 50 s | `Path exceeds 255 characters` inside the checkout copy: `./🧰️framework/…/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/dist/prototype/next/standalone/🧰️framework/…` (F3) |
| Incremental discard | `bun repo incremental discard --root=/private/tmp/sinc1022` | PASS | 15 s | `ready: /private/tmp/sinc1022` (cleanup of the auditor's own root) |
| Concurrency workloads | `bun repo concurrency list` | PASS | 40 s | 4 workloads: disjoint-projects, same-target-cold, warm-fanout, readers-while-edit |
| Concurrency plan | `bun repo concurrency plan --workloads=all --sessions=3` | FAIL | 15 s | `Prepare the disposable root first` (blocked by F2) |
| Build graph and targets | `bun repo show targets --graph-policy=current` | SKIPPED | stopped at about 9 min | Waited in the host capacity queue (needs 5120 MB, free memory below floor); killed by me under the 9 min rule. Not a law |
| Incremental proof | `bun repo incremental run` | SKIPPED | n/a | Needs a prepared root (blocked by F2). 15 scenarios times 3 phases times 2 tools would exceed 9 min |
| Concurrency proof | `bun repo concurrency run` | SKIPPED | n/a | Needs a prepared root (blocked by F2). Runs 3 to 4 simulated agents building at once, over 9 min |
| Build graph print | `bun repo graph` | SKIPPED | n/a | Same capacity queue as `show`. Diagnostic, not a law |
| Dependency ensure | `bun repo dependencies ensure` | SKIPPED | n/a | Mutates the shared dependency partition. Exclusive lease `javascript-dependency-build` held for 700+ s by another run (pid 60336) |
| Dependency sync / lock | `bun repo dependencies sync|lock --all` | SKIPPED | n/a | Mutating (install or `bun.lock` rewrite) |
| Cargo gate | `bun repo cargo --store=shared --configuration=prototype -- check ...` | SKIPPED | n/a | Compiles workspace crates, 4 cargo requests queued for capacity; whole-workspace build |
| Run targets | `bun repo run|run-many|affected|exec|watch ...` | SKIPPED | n/a | Whole-workspace builds and tests, well over 9 min |
| Capacity wrapper | `bun repo capacity run -- <cmd>` | SKIPPED | n/a | Wrapper with no law of its own |
| Cache walk | `bun repo cache status --deep|--tasks` | SKIPPED | n/a | Walks every cache home, same trees as `cache gc --dry-run` (417 s) |
| Cache collection | `bun repo cache gc` (without `--dry-run`) | SKIPPED | n/a | Mutating: would remove 2741 entries |
| Cache / tool reset | `bun repo cache clear`, `reset`, `tools setup` | SKIPPED | n/a | Mutating |
| Cache server | `bun repo cache serve` | SKIPPED | n/a | Long-running server |
| Cluster | `bun repo remote-execution serve` | SKIPPED | n/a | Long-running containers; Docker not running (F6) |
| Input hashes | `bun repo inputs <project:target>` | SKIPPED | n/a | Per-target diagnostic, no law of its own |

## Findings

- F1 (lock order, tool-iface / Rust). `leases status` reports `semio.compiler.host-capacity/v1` (rank 1) requested while `rustup:home` (rank 2) is held shared, repeated for `cpu-picker/v1` too. The lock class is declared at `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json:49`. The `rustup:home` holder is the rustup installation path at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts:52-57`, which takes the lease and then nests the capacity request.
- F2 (graph / incremental). The default disposable root `/var/folders/pm/…/T/sinc` pushes clone paths over 255 characters. Check at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏃️runner/🟦️.ts:104` (inside) and `:118` (clone). `incremental plan` and `concurrency plan` cannot run without a prepared root.
- F3 (graph / path length, AGENTS 256-char rule). The checkout contains a recursive Next standalone output: `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/dist/prototype/next/standalone/🧰️framework/…` nests copies of the framework inside itself (gitignored via `.gitignore:638`). A scan (excluding node_modules, .cache, .git, target) counted about 349,500 entries whose clone path under `/private/tmp/sinc1022/` exceeds 255 characters. This is the reason `prepare` fails even with a short root.
- F4 (cache, informational). `cache gc --dry-run` would remove 2741 entries (12.63 GiB), mostly retired `native:held-javascript` roots and about 30 undeclared repository areas. `cache status` lists the undeclared areas. Ledger declarations are missing.
- F5 (cache, weak pass). `cache verify --deep` exits 0 but verified 0 bytes across 2 groups. Treat as no evidence until a group is populated.
- F6 (Bazel / remote execution, optional). Docker daemon at `unix:///Users/ueli/.docker/run/docker.sock` is not running, so `remote-execution status` reports the runtime unavailable and the endpoint 127.0.0.1:8980 does not answer. Optional, so not a failure of the build.
- F7 (docs, naming). The brief, the AGENTS note and memory refer to `verify` as a command, but `repo verify` does not exist (`Unknown command: verify`). The memory gate `verify dependencies literal-external` is not in this CLI.
- F8 (docs / ticket hygiene, AGENTS path rule). The ticket folder contains staged copies of the framework: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/round6-tools/staged/pm-seam/staged2/new/🧰️framework/…` exceeds 255 characters (seen in the `prepare` error). AGENTS forbids copying version-controlled sources into tickets when paths get that long.

## FAIL list grouped by owner area

- tool-iface: F1 (`leases status`: host-capacity requested under `rustup:home`). Owner: rustup bootstrap and lease ordering.
- graph: F2 (`incremental prepare` root path), F3 (coordinator Next standalone recursion, path length). Blocked: `incremental plan`, `concurrency plan`.
- docs: F7 (`verify` does not exist in the CLI). F8 (ticket staging copy over 255 characters).
- cache: no FAIL. Findings F4 and F5.
- Bazel: no FAIL. F6 (Docker absent, optional).
- Rust: no FAIL of its own. F1 is a Rust cargo path lock-order violation. `cargo` was SKIPPED.
- i18n, taxonomy, plugins: no FAIL. Not exercised (help texts printed in English only; no DE check).

Report: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/audit-verify-1022.md`

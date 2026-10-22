### 🧱️ Incremental development

The aim is that an edit costs the work that depends on it and nothing else, for every agent at the same time. The table says per layer what the unit of reuse is and what an edit re-does.

| Layer | Unit of reuse | An edit re-does | State |
|---|---|---|---|
| Task key | the declared inputs of one task | the tasks whose declared inputs contain the file, then the dependents whose input snapshot changed (early cutoff under Bazel) | live |
| Derived inputs | an input set derived by rule instead of a whole-repository glob | tasks that read broadly declare what they read: the tasks above 100,000 declared files fell from 243 to 11, and the 947 tasks that changed declare 12.0 M instead of 34.3 M input files (static reconstruction of 8,028 tasks) | {{@input-precision}} |
| Rust key | {{^crate-keys::a named input `cargoToolchain` that holds the root `Cargo.toml`, the whole root `Cargo.lock`, the config and the toolchain (139 projects, 824 cached targets)::the resolution identity of one crate: its locked dependency slice, the workspace fields it inherits, toolchain and config}} | {{^crate-keys::any edit of the root lock or of those files re-keys every one of them::the crates whose slice or inherited field changed; comments, formatting and workspace member lists move nothing}} | {{@crate-keys}} |
| Project graph | {{^graph-generations::one sealed graph of the whole workspace::one immutable fragment per owner inside a generation}} | {{^graph-generations::any source edit stales the seal and the next fresh reader rebuilds under one exclusive lease::the dirty owners and the owners that consumed the same fact; readers take no lease}} | {{@graph-generations}} |
| Dependencies | one generation per scope (one `bun.lock` and its `node_modules`) | the scopes whose generation key moved; the rest is verified by receipt | {{@sync-incr}} |
| TypeScript | {{^ts-units::one program and one incremental state per owner, compiler and configuration::one check unit per package, cut by a declaration boundary}} | {{^ts-units::any edit re-checks the whole program::a body edit re-checks one unit (2.9 s measured against 27 s), a signature edit re-checks its dependents until their declarations are equal}} | {{@ts-units}} |
| Rust compile | {{^cargo-lanes::one `repo cargo` invocation::one compile unit, content addressed, in a cache shared by all agents}} | {{^cargo-lanes::the crates Cargo finds stale in the one shared build directory::the units whose source or dependency artifacts changed; the focused `-p` crate keeps rustc's incremental state in the agent's own lane}} | {{@cargo-lanes}} |
| Bazel | action key in the shared disk cache | the actions whose key moved; dependents are spared when an output is byte-identical | live |
| Nx | task hash in the task cache | the tasks whose hash moved | live |

**Prove it with `repo incremental`** ({{@incremental}}). The command judges, in a disposable clone of the checkout, that Nx and Bazel rebuild exactly what an edit affects, with the same scenarios under both tools and the results side by side. `prepare` clones the tracked and untracked files into `--root` (default `/tmp/sinc`, `<drive>:\sinc` on Windows; refused when the root lies inside the checkout or contains it) and provisions the clone with its own `preflight`, `tools setup all` and `dependencies sync --all`; every edit of a scenario happens inside the clone, never in your tree.

```bash
repo incremental list
repo incremental plan --scenario=fw-ts-comment
repo incremental prepare
repo incremental run --scenario=fw-ts-comment --implementations=nx,bazel
repo incremental discard
```

- **Scenarios.** 17 scenarios in 7 areas (framework TypeScript, Rust crate, wasm component, Vite app, test target, lockfile, product activation) and 6 edit kinds (comment-only, body change, signature change, config, lockfile, unrelated sibling). Per implementation a scenario runs prime, `warm` (no edit: only uncached tasks may execute), `edit` (one line appended to one source) and `revert` (the exact original text is restored, also after a failure or a cancellation, and everything must come from the cache). `--phases=warm,edit,revert` narrows the phases.
- **Verdicts.** The expected set is computed from the declared inputs: tasks that read the file directly must run, dependents whose output changed must run, dependents whose output is provably identical may be spared, uncached tasks always run. `pass`; `stale` (a task that had to run did not); `excess` (a task ran that may not); `stale-excess`; `failed`; `skipped`. Bazel follows every dependency and cuts off on output digests, Nx follows only tasks that consume outputs: that is the one place the tools legitimately differ, and the report lists it as `only nx:` and `only bazel:`.
- **Observation and output.** Bazel is read from its compact execution log, Nx from its own run record. The report is English or German (`--lang`), `--json` prints JSON, `--file=<report>` writes it; progress is published per stage, every stage is a cancellation checkpoint, and the exit code fails when any run is not a pass.

**Prove concurrency with `repo concurrency`** ({{@concurrency}}). {{?concurrency::::The commands below belong to the staged set and do not exist until it is applied. }}The command is the acceptance test for everything in this chapter that claims "no waiting": N simulated agents work in the same disposable clone (the one `repo incremental prepare` provisions), edit different projects at the same time and run build, test and check through both tools; then the same sessions run once more in a row as the serial reference.

```bash
repo concurrency list
repo concurrency plan --workloads=all
repo concurrency run --workloads=disjoint-projects --sessions=3 --implementations=nx,bazel
```

- **Workloads.** `disjoint-projects` (a TypeScript, a Rust, a Go and a schema project, edited and built by one agent each), `same-target-cold` (one stale-making edit, three agents build the same target, started 250 ms apart), `warm-fanout` (primed, three agents build the same target) and `readers-while-edit` (an editor lists projects after a source edit while two readers do the same). `--sessions=<n>` takes the first n agents (default 3, at most 8). `plan` refuses a workload in which an agent edits a file that another agent's targets declare as an input.
- **Criteria** (`--criteria=default|warm|smoke` replaces a workload's own set). `default`: every step succeeds, results equal the serial run (exit code, declared output digests, stdout digest), no wait above 5 s for an unrelated project in any lock class (graph facts, dependency image, Bazel server, Cargo units, capacity admission, toolchain acquisition, other), and a task executes in one session only while the others execute no action for it. `warm` demands zero executed actions; `smoke` demands success and serial equality only. The report prints one verdict per owning executor: pass, fail, or unproven when only skipped rules remain.
- **What a wait measurement can prove.** Bazel's `Another command (pid=N) is running` and Cargo's `Blocking waiting for file lock` lines give exact waits. A lease publishes its progress row only every 15 s, so lease waits are a lower bound: a bound above the limit still fails a criterion, a bound below it leaves the criterion unproven. Exact lease waits need `SEMIO_LEASE_TRACE=<file>`, which the lease module writes for acquisitions that waited 100 ms or more (an optional part of the harness set). Nx writes one run record per cache directory, so concurrent Nx runs are matched by time window and marked `ambiguous` when they overlap.
- **Not a speed-up test.** The serial reference runs after the concurrent phase and is therefore warm: the harness proves equal results and absent waits, not a makespan.

**What an edit costs (Rust).** Static model over the repository: 240 Cargo workspaces, 3,374 crates and 12,134 inferred or authored cached targets.

| Edit | Crates and targets re-keyed today | With per-crate keys ({{@crate-keys}}) |
|---|---|---|
| comment or formatting in the root `Cargo.toml`, `.cargo/config.toml` or `rust-toolchain.toml`; a crate appended to the root `[workspace].members` | 3,374 and 12,134 | 0 and 0 |
| a root workspace dependency that one crate uses | 3,374 and 12,134 | 1 and 3 |
| a `Cargo.lock` package that one crate uses | 139 and 827 (Nx), 564 and 2,964 (Bazel) | 1 and 5 |
| `[build].rustflags` or the toolchain channel | 3,374 and 12,134 | 3,374 and 12,134 (correct) |
| an edit of a nested workspace manifest (`🧰️framework/Cargo.toml`: lints, dependency entries) | 0 and 0 (a stale hit) | 2,274 to 3,258 crates (a correctness fix) |
| a source edit in a leaf, median, 90th-percentile or hub crate | 1, 21, 2,233 and 3,080 crates | unchanged: the fan-in is intrinsic |

The fan-in is a property of the dependency graph. The corrected census (resolving workspace dependencies through the owning workspace) shows a mean reach of 363 crates (11.3 percent), a longest chain of 157 and 354 crates that reach 2,000 or more; an edit in the eight bottom crates of the value tower rebuilds about 2,915 of 3,213 crates because of the orphan rule. Splits of the hottest crates are planned, not applied.

**Faster loops.**

- Select narrowly: `repo run <project>:<target>` builds the closure of one task; check the plan first with `--dry-run`.
- For Rust use `repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib`: `check` is cheaper than `build`, and switching `prototype` and `release` re-keys everything.
- Stay on one tool and one mode while you iterate: each tool has its own cache, and the mode is part of every key.
- Do not edit the build tool's own files in a loop: every action is re-keyed once per edit.
- TypeScript units are a library engine; no `repo` command runs them yet, and the monolithic `typecheck` stays the release gate: on the measured program the unit-wise check reported 37 of 43 diagnostics, none extra, so it is a development gate until the six monolithic-only diagnostics are understood. Declaration emit must be able to name every exported type; the findings (a `brand` member, function-local classes with `private` members, inferred types that name `node_modules` paths) are owner fixes listed with the engine.

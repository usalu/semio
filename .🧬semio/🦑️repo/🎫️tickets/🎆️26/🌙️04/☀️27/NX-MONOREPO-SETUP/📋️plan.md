# Nx-only refactor plan for Semio

**The right refactor is to make Nx the sole repository-level orchestrator, preserve native incremental compilation underneath it, and give every generated artifact an explicit owner and retention policy.** Merely wrapping today’s build scripts in `nx run` would not address excessive compilation, oversized cache entries, duplicated outputs, or uncontrolled disk growth.

**Important audit limitation:** I could retrieve the branch’s overview, but attempts to retrieve the files at `aa72759d4183638d265b5c2cb3d409320ad72f4f` failed. I therefore cannot honestly present an exhaustive, file-and-line violation list for that commit, identify its installed Nx version, or claim to have measured the cause of the 100 GB growth. The branch overview is not a substitute for the pinned tree.

What follows is an end-to-end refactor design, a repository-specific audit map, and the verification gates required to complete the audit. **Paths identified below are inspection candidates, not confirmed violations. Proposed targets and mechanisms are explicitly new design elements.**

## 1. Define what “Nx for absolutely everything” means

Adopt this repository policy:

> Every supported repository operation must enter through Nx. Nx owns project selection, cross-project dependencies, scheduling, task-level caching, and execution reporting. Native tools perform their work inside Nx targets.

That covers development servers, dependency synchronization, code generation, formatting, linting, type checking, all test categories, builds, documentation, assets, packaging, publishing, deployment, environment lifecycle, benchmarks, diagnostics, and cleanup.

It does **not** mean implementing a compiler in Nx, or requiring Cargo’s internal compiler invocations to become separate Nx tasks. Nx supports tasks defined through package scripts, project configuration, and plugins; the key is which layer orchestrates the work. ([Nx][1])

### The boundary to enforce

| Layer                        | Responsibility                                                  | Forbidden responsibility                                           |
| ---------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------------ |
| CI, editor, developer, agent | Select and invoke Nx operations                                 | Run an independent build/test pipeline                             |
| Nx project/task graph        | Select projects, order tasks, cache results, schedule execution | Conceal another repository-wide scheduler inside one opaque target |
| Executors and native tools   | Compile, test, bundle, generate, or package the selected unit   | Independently rebuild unrelated repository projects                |
| Storage management           | Account for and reclaim explicitly owned derived state          | Delete arbitrary directories or active build state                 |

Two important consequences:

**Native commands inside a target are legitimate.** A target executing `cargo test` is not a violation. A GitHub workflow executing `cargo test` directly, bypassing the target, is.

**Nested orchestration is not the desired endpoint.** A target that launches another `nx run-many`, which launches package scripts that recursively build dependencies, has not produced one coherent task graph. Cross-project ordering belongs in the outer Nx graph.

There is also an unavoidable bootstrap boundary: a machine must acquire a runtime and Nx before it can execute Nx. Supply those through a pinned development image or a minimal, audited launcher. That bootstrap must not become an exception for installing application dependencies, generating code, or compiling the repository.

---

## 2. Audit the repository’s actual control surfaces

The accessible branch overview shows an existing Nx configuration alongside Bun, Cargo, uv, Go workspace, .NET solution, and CMake files. It also shows custom project/script/artifact metadata and development-tool configuration. Those surfaces make this a **polyglot build-system integration**, not simply a JavaScript script migration. ([github.com][2])

Start with this inspection map, validating every path against the pinned commit:

| Surface visible in the branch overview                                                          | Audit objective                                                                                       |
| ----------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `nx.json`, `.nxignore`                                                                          | Resolve effective targets, caching, inputs, defaults, discovery, and exclusions                       |
| `package.json`, `bun.lock`, `bunfig.toml`                                                       | Find package scripts, lifecycle hooks, workspace boundaries, and installation behavior                |
| `Cargo.toml`, `Cargo.lock`, `.cargo/`, `rust-toolchain.toml`                                    | Find native workspace boundaries, profiles, output directories, and compiler settings                 |
| `pyproject.toml`, `uv.lock`, `conftest.py`                                                      | Find Python environments, test collection, fixtures, native extensions, and packaging                 |
| `go.work`, `go.work.sum`                                                                        | Find Go modules, local module dependencies, and workspace-wide commands                               |
| `Monorepo.sln`                                                                                  | Find individual projects, build references, configurations, and platform requirements                 |
| `CMakeLists.txt`, `CMakePresets.json`                                                           | Find configure/build/test relationships and shared native build directories                           |
| `📋️project.json`, `📜️script.ts`, `🗿️artifact.ts`, `🔒️dependencies.json`, `🧅️layering.json` | Establish whether custom metadata defines projects, dependencies, artifacts, or a competing scheduler |
| `.github/`, `.devcontainer/`, `.vscode/`, `.storybook/`, `.gitmodules`, agent configuration     | Find every external entry point and nested repository boundary                                        |

Do not assume `📋️project.json` is either broken or already integrated. Standard Nx project discovery and custom metadata discovery are different mechanisms; a plugin can bridge custom configuration into Nx. The audit must establish whether that bridge exists and what it produces. ([Nx][3])

### Required audit deliverables

Produce four machine-readable inventories:

**Project inventory:** every buildable/testable package, native workspace, application, generator, asset collection, and submodule; its owning Nx project; platform; maintainer; and lifecycle targets.

**Command inventory:** every executable entry point, its callers, working directory, environment, subprocesses, and intended Nx target.

**Artifact inventory:** every generated directory or file family, producer, consumers, physical size, rebuild cost, cache policy, and cleanup mechanism.

**Violation ledger:** exact commit, path, line, rule, evidence, severity, replacement target/mechanism, owner, and regression test.

A useful violation record is:

```text
rule
commit
path
line
entry_point
resolved_command
owning_project
severity
evidence
replacement
regression_test
status
```

A text search is an initial discovery tool, not proof of completeness. It cannot reliably distinguish an executor’s legitimate compiler call from a bypass, and it misses dynamically assembled commands.

---

## 3. Establish an explicit violation taxonomy

The completed audit should evaluate all of the following. These are **rules to check**, not claims that the pinned commit violates them.

| Rule       | Violation                                                                                    | Required replacement                               |
| ---------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| ORCH-01    | Developer-facing build/test/dev command bypasses Nx                                          | Nx entry point                                     |
| ORCH-02    | CI or a composite action performs repository work outside Nx                                 | Nx target invocation                               |
| ORCH-03    | Editor task, debug prelaunch, hook, or agent instruction bypasses Nx                         | The same canonical Nx target                       |
| ORCH-04    | Scripts recursively orchestrate multiple projects                                            | Explicit graph relationships                       |
| ORCH-05    | An Nx target launches another repository-wide Nx pipeline                                    | One outer task graph                               |
| ORCH-06    | Install/lifecycle hooks silently build repository projects                                   | Explicit setup or build targets                    |
| GRAPH-01   | A package or native workspace is absent from Nx                                              | Project discovery/inference                        |
| GRAPH-02   | A real import, schema, FFI, asset, or runtime dependency is absent                           | Dependency edge with provenance                    |
| GRAPH-03   | Unrelated projects are coupled through wildcard dependencies                                 | Narrow, justified edges                            |
| GRAPH-04   | Native and Nx schedulers both build the same dependency closure unnecessarily                | One deliberate scheduling boundary                 |
| CACHE-01   | Cacheable target omits an output-affecting input                                             | Complete input contract                            |
| CACHE-02   | Unrelated root files invalidate nearly everything                                            | Scoped named inputs                                |
| CACHE-03   | Multiple targets own overlapping output directories                                          | Exclusive artifact ownership                       |
| CACHE-04   | Nx caches an entire mutable compiler/dependency store                                        | Cache deliverables, manage native state separately |
| CACHE-05   | Install, publish, deployment, mutation, or live-service checks are incorrectly cached        | Uncached side-effect target                        |
| CACHE-06   | Restoring a task result does not restore everything its consumers need                       | Complete, tested output contract                   |
| DISK-01    | Generated state has no owner or retention policy                                             | Artifact registry and bounded retention            |
| DISK-02    | Normal development performs broad clean/rebuild operations                                   | Incremental execution                              |
| DISK-03    | Build state is duplicated per package, profile, worktree, or container without justification | Deliberate cache/output topology                   |
| EXEC-01    | Nx concurrency multiplied by native workers oversubscribes resources                         | Coordinated limits                                 |
| EXEC-02    | Watchers/servers are orphaned or started twice                                               | Nx-owned process lifecycle                         |
| REPRO-01   | Toolchains, fetched assets, SDKs, or external fixtures float                                 | Pinned identity and verified acquisition           |
| TEST-01    | A green cached test depends on untracked external state                                      | Hermetic fixture or uncached execution             |
| RELEASE-01 | Publishing rebuilds instead of consuming tested artifacts                                    | Build once, verify, promote                        |

Include shell, PowerShell, JavaScript/TypeScript subprocess calls, Python subprocesses, Go process execution, Rust build scripts, CMake custom commands, and MSBuild custom targets.

Also inspect documentation and agent instructions. Leaving the old commands in those surfaces would preserve bypasses even after CI is corrected.

---

## 4. Measure the hours and gigabytes before choosing optimizations

Treat three problems separately:

**Execution cost:** what consumes CPU, waits on locks, downloads dependencies, or serializes the critical path?

**Retained development state:** what remains on disk after builds and tests?

**Published artifact size:** what users download or deployments carry?

A large compiler directory does not necessarily imply an oversized application bundle. Conversely, a small Nx cache does not imply healthy total disk usage.

### Add proposed diagnostic targets

```bash
nx run repo:audit
nx run repo:doctor
nx run repo:disk-report
nx run repo:perf-baseline
nx run repo:cache-verify
```

These targets need implementation; they are not claimed to exist today.

The baseline must identify the exact source revision, dirty-worktree state, submodule revisions, toolchain versions, OS/architecture, available memory, worker limits, and cache state.

Measure these workloads independently:

| Workload                                            | What it establishes                              |
| --------------------------------------------------- | ------------------------------------------------ |
| Fresh checkout and genuinely cold tool caches       | Initial provisioning cost                        |
| Dependencies available, compiler/task caches empty  | Actual cold build cost                           |
| Identical second invocation                         | Task-cache and incremental effectiveness         |
| One leaf application source change                  | Normal developer feedback loop                   |
| One shared schema/core change                       | Legitimate dependency fan-out                    |
| Test-only change                                    | Whether builds are coupled unnecessarily         |
| Switching between representative worktrees/branches | Cache reuse versus duplication                   |
| Repeated build/test cycles                          | Whether storage converges or grows monotonically |

For each task, record wall time, CPU time, peak memory, cache status, native compiler invocations, lock wait, output bytes, and retained-byte delta.

For disk reporting, include project directories, user-level tool caches, container storage, mounted volumes, worktrees, reports, downloaded fixtures, and temporary directories. Measure allocated space as well as apparent file size, and avoid double-counting linked content.

**Do not attribute the reported 100 GB to Nx without measurement.** Current Nx documentation provides a local task-cache size limit; that is distinct from controlling Cargo state, virtual environments, container storage, or materialized build outputs. ([Nx][4])

---

## 5. Build one complete polyglot project graph

### Preserve the repository’s layout

Do not start by moving everything into `apps/` and `libs/`, renaming emoji directories, changing package managers, or replacing every native build system.

Those changes would enlarge the migration and obscure whether the build-system improvements worked.

Use stable project identifiers independently of filesystem names. Preserve exact Unicode paths, and use argument arrays rather than constructing shell commands through string concatenation.

### Integrate custom metadata without creating another scheduler

For the custom project/dependency/artifact files, choose one authoritative integration:

**Prefer an Nx plugin that reads the existing metadata and produces Nx projects, dependencies, and target defaults in memory.**

Use generated standard configuration only when a concrete compatibility requirement makes it necessary. Do not maintain two manually edited descriptions of the same project graph.

Nx exposes project-graph extension mechanisms for adding language support and dependencies. The custom layer should implement those mechanisms, not independently schedule builds. ([Nx][5])

The plugin must:

* Discover every relevant project, including non-JavaScript projects.
* Attach dependency provenance: import, native manifest, schema, generated binding, runtime fixture, or explicit declaration.
* Validate duplicate identifiers, cycles, missing owners, and conflicting metadata.

Graph construction must be offline and cheap. It must not install dependencies, compile projects, recursively invoke Nx, or rescan every generated directory.

### Model dependencies beyond source imports

Explicitly account for:

| Relationship              | Example requiring an edge                                         |
| ------------------------- | ----------------------------------------------------------------- |
| Native package dependency | Rust path dependency, Go workspace module, .NET project reference |
| Generated code            | Schema → generator output → consumer                              |
| Cross-language interface  | Native library → binding/adapter → application                    |
| Runtime test dependency   | Service → integration or E2E suite                                |
| Shared test data          | Fixture producer → consuming test                                 |
| Packaging dependency      | Built binary/library → installer or extension package             |
| Asset transformation      | Source model/image/data → processed asset → application/docs      |

Do not make every frontend depend on every backend simply because they coexist in the monorepo. Model the actual implementation or test mode being used.

### Distinguish project edges from task prerequisites

A source dependency means a change can affect a consumer. It does not automatically mean a separate Nx `build` task must run first.

Cargo, MSBuild, and other native engines can already build their internal dependency closure. Adding blanket `^build` task prerequisites on top may duplicate work.

Create a task dependency only when it is necessary to establish an execution prerequisite or materialize an artifact.

---

## 6. Standardize lifecycle targets and eliminate hidden work

Adopt consistent target semantics:

| Target                                     | Meaning                                                 | Cache policy                                       |
| ------------------------------------------ | ------------------------------------------------------- | -------------------------------------------------- |
| `deps`                                     | Materialize the selected locked dependency environment  | Uncached, idempotent                               |
| `generate`                                 | Produce deterministic generated artifacts               | Cacheable after verification                       |
| `format-check`, `lint`, `typecheck`        | Read-only validation                                    | Cacheable after verification                       |
| `format-fix`                               | Modify source formatting                                | Uncached                                           |
| `test`                                     | Hermetic unit tests                                     | Cacheable                                          |
| `test-integration`, `test-contract`, `e2e` | Explicit higher-level suites                            | Cache only when inputs and fixtures are controlled |
| `test-watch`, `dev`                        | Long-running local process                              | Uncached, continuous                               |
| `build`                                    | Produce the project’s declared build deliverables       | Cacheable                                          |
| `package`                                  | Assemble an immutable distribution from built artifacts | Cacheable if deterministic                         |
| `publish`, `deploy`, `migrate`, `seed`     | External or mutable operations                          | Uncached                                           |
| `clean`, `disk-prune`                      | Reclaim explicitly owned generated state                | Uncached                                           |
| `bench`, `fuzz`                            | Performance or exploratory execution                    | Normally uncached; explicit result retention       |

Avoid making unit tests depend on a production bundle when they execute source directly. Likewise, starting one web application must not imply building native products, documentation, installers, and every test suite.

### A conservative Nx configuration direction

This is a **design sketch**, not a drop-in replacement for the unread `nx.json`:

```json
{
  "parallel": 3,
  "cacheDirectory": ".cache/nx",
  "maxCacheSize": "8GB",
  "targetDefaults": {
    "dev": {
      "cache": false,
      "continuous": true
    },
    "test-watch": {
      "cache": false,
      "continuous": true
    },
    "deps": {
      "cache": false
    },
    "format-fix": {
      "cache": false
    },
    "publish": {
      "cache": false
    },
    "deploy": {
      "cache": false
    },
    "clean": {
      "cache": false
    }
  }
}
```

The concurrency and size values are starting hypotheses, not measured recommendations for your machines. Enable caching on deterministic leaf targets only after their contracts pass verification.

Nx supports continuous tasks in version 21 and later. Validate this and all other configuration against the version actually pinned in the repository before introducing it. ([Nx][6])

### Prevent package-script recursion

Be particularly careful when changing package scripts into Nx aliases.

A script such as `build → nx run package:build` must not itself be inferred as the implementation of `package:build`. Either retain the script as the native leaf implementation invoked through Nx, or define an independent Nx implementation and exclude forwarding aliases from inference.

Inspect the **resolved** target configuration rather than judging only individual configuration files.

---

## 7. Make cache correctness an explicit engineering contract

A cache hit is useful only when it is equivalent to executing the task successfully.

### Inputs

Each target must account for its source closure, relevant configuration, generator implementation, dependency resolution, toolchain, build mode, platform, and output-affecting environment.

Use language-specific input groups. A Python-only dependency change should not automatically invalidate unrelated native products unless a real shared dependency requires it.

Do not globally include every root configuration file in every target. Conversely, do not narrow inputs simply to improve hit rates without testing invalidation.

Nx supports file, environment, runtime, and dependency-related inputs; runtime inputs can identify the tools actually executing the task. ([Nx][7])

For native targets, fingerprint the relevant compiler/SDK, target architecture, ABI, feature selection, linker flags, and build profile. Do not confuse “same source” with “compatible binary.”

Avoid timestamps, random run identifiers, and incidental working-directory paths in cache keys. Include revision identity only where the artifact really embeds or otherwise depends on it.

### Generated files

A generated directory needs a deterministic producer, explicit consumers, and a reproducible dependency relationship.

Do not assume an ignored generated file becomes a valid hash input merely because it appears in an input glob: Nx excludes gitignored files from ordinary source-file inputs. Model the producer’s inputs and dependencies, or use appropriate dependent-output inputs. ([Nx][7])

### Outputs

Every cacheable output path needs one owner.

A useful conceptual layout is:

```text
.cache/
  nx/                         Nx task results
  native/                     Mutable compiler state
  tools/                      Tool-managed caches, where appropriate

dist/
  <project>/<variant>/        Deliverables only

reports/
  <project>/<suite>/          Bounded test and diagnostic reports

tmp/
  <project>/<operation>/      Disposable working state
```

These are layout labels, not invented Nx interpolation tokens.

Never cache the workspace root, a shared parent `dist`, all `node_modules`, every virtual environment, or an entire shared native build tree as a project’s output.

Nx’s output configuration determines which files are stored and restored. It does not turn an arbitrarily broad directory into a safe artifact boundary. ([Nx][8])

### Required cache tests

For each cacheable target: execute it, record its deliverables, remove only those deliverables, restore from cache, and run its consumer.

Repeat on a clean compatible machine.

Then mutate each declared input category and confirm invalidation. Mutate an unrelated project and confirm the target remains reusable.

This must catch missing executable permissions, absent runtime libraries, incomplete test assemblies, omitted generated files, and incompatible native binaries—not just compare a “cache hit” log line.

---

## 8. Refactor each language at the correct boundary

### JavaScript/TypeScript and Bun

Retain Bun unless the pinned configuration or measurements demonstrate a specific problem.

Separate type checking, unit testing, bundling, Storybook, documentation, and packaging. Do not produce a distributable package for every source-only internal library merely because it exists.

Make applications consume source or built libraries intentionally. Avoid mixed conventions that build a library and then have the application compile the same source again.

Audit TypeScript project references, broad include patterns, repeated declaration generation, and overlapping tool inference. Nx’s TypeScript integration supports different project-linking and build approaches; choose one per dependency boundary rather than combining them accidentally. ([Nx][9])

Keep package installation out of ordinary test/build commands. Audit lifecycle scripts and trusted dependency hooks explicitly.

### Rust/Cargo

Focus on duplicate compilation variants, debug information, native cache topology, and repeated workspace compilation.

Use package-scoped Nx entry points where useful, but let Cargo own its internal compilation graph. Avoid automatically preceding every Rust test with separate Nx builds of every Rust dependency.

Share compatible native state within a deliberate Cargo workspace boundary. Across worktrees, prefer a supported compiler cache over blindly sharing mutable output directories and same-named executables.

Cargo distinguishes final artifacts from intermediate build state and supports configurable build locations. Its internal cache layout is not an appropriate stable artifact API for a custom Nx packager. ([Rust Documentation][10])

Benchmark reduced development debug information while preserving an explicit full-debug mode. Keep release optimization and linking settings out of routine development unless required.

Cargo profiles expose debug-information and incremental-compilation trade-offs: incremental compilation saves additional state to accelerate later builds, while more aggressive optimization can increase compile time. Do not disable incremental compilation globally merely to reclaim space. ([Rust Documentation][11])

Stage only binaries, libraries, and required runtime files into Nx-owned deliverable directories.

### .NET

Model actual projects and genuine native build boundaries, not just the entire solution as the default unit of work.

Separate dependency restoration, compilation, tests, and packaging where doing so produces meaningful reuse. Do not assume tests are execution-only: `dotnet test` normally builds before running, and behavior depends on the selected test platform. ([Microsoft Learn][12])

Use no-build/no-restore modes only when prior steps have established the exact required state. Restoring one DLL is not necessarily enough for a test or package consumer.

Give framework/runtime/configuration variants distinct ownership. Keep Windows- or host-application-dependent tests in explicit platform lanes rather than making them prerequisites of ordinary cross-platform work.

### CMake/C++

Keep configuration, native compilation, and staged installation under an explicit native-workspace owner.

Use presets for reproducible variants. CMake distinguishes configure, build, test, and package presets; map these deliberately rather than maintaining separate hand-written configurations in CI and editor scripts. ([CMake][13])

Do not assume a configured CMake directory is portable across machines or workspace paths. A cacheable native build target should restore its staged deliverables, not pretend that a missing local configure tree has been recreated.

Where appropriate, perform incremental configure/build operations inside one native Nx leaf rather than attempting to remotely cache fragile intermediate state.

### Python/uv

Make environment synchronization explicit and separate from test execution.

Choose environment boundaries according to dependency compatibility and developer workflows—not automatically one enormous environment or one duplicate environment per directory.

Keep uv’s dependency cache separate from Nx result caching. Use uv’s own cache-management commands instead of deleting its internals. The uv documentation also notes that locating cache and environment on different filesystems can force copying instead of linking. ([Astral Docs][14])

Audit Python test collection and shared fixtures carefully: a global fixture importing or building multiple implementations can conceal large costs behind a small test command.

### Go

Map Go modules and workspace relationships into Nx, while allowing Go to perform package-level compilation internally.

Use module/package-scoped test targets where they give useful feedback. Reserve full-workspace execution for explicit validation.

Keep Go’s build/module caches separate from staged deliverables. Include toolchain, platform, CGO settings, and relevant native dependencies in the task contract. Go’s build/test caching is a separate mechanism from Nx task caching. ([Go][15])

---

## 9. Bound disk usage without destroying useful incrementality

Introduce an **artifact registry**, ideally integrated with existing artifact metadata if its actual schema supports the purpose.

For each generated path family, record:

```text
owner
category
path
producer
consumers
platform/configuration
cacheability
portability
retention
size_budget
cleanup_handler
active_use_lock
```

Distinguish dependency stores, compiler intermediates, deliverables, reports, container state, and downloaded assets.

### Initial budget proposal

These are provisional allocations to test against measurements:

| Derived-state category                 | Initial allocation |
| -------------------------------------- | -----------------: |
| Nx task results                        |              8 GiB |
| Native intermediate state              |             20 GiB |
| Repository-owned container build cache |             10 GiB |
| Reports and temporary data             |              2 GiB |
| Materialized deliverables              |              5 GiB |

That is a **45 GiB derived-state planning budget**, not a promised hard ceiling and not a complete machine-storage budget. Dependency installations, source assets, and user data require separate accounting.

Build bursts need headroom. Retention limits alone do not guarantee that a running task cannot fill the disk.

### Safe reclamation

Implement:

```bash
nx run repo:disk-report
nx run repo:disk-prune --dry-run
nx run repo:disk-prune --apply
```

The proposed cleaner must resolve and validate paths, reject source/user-data locations, respect active-use locks, and delegate to tool-supported cleanup mechanisms.

It should reclaim stale reports, temporary data, obsolete variants, and unused generated state before discarding valuable active compiler caches.

Never make broad workspace cleaning, `cargo clean`, full dependency-cache deletion, or volume-pruning commands part of normal development.

For containers, use repository-owned builders and explicit BuildKit garbage-collection policy. BuildKit exposes age/space-based GC controls; Nx’s cache limit does not configure them. ([Docker Documentation][16])

### Worktrees, agents, and containers

Measure duplication across simultaneous worktrees and agent sessions explicitly.

Do not create an unbounded cache namespace for every task ID, timestamp, or branch. Also do not solve duplication by allowing multiple processes to overwrite a shared mutable output tree.

Share dependency/compiler caches only through their supported concurrency model. Keep staged deliverables isolated by their actual ownership and compatibility requirements.

### Published artifacts

Add per-product package-size checks separately from cache budgets.

Inspect accidental inclusion of source trees, development dependencies, debug symbols, test fixtures, model datasets, reports, or caches. Build release archives and container contexts from explicit allowlists or narrow staging directories.

Docker build-context exclusion controls which files are sent to the builder; use this to prevent repository caches and unrelated assets from entering image builds. ([Docker Documentation][17])

---

## 10. Control concurrency and long-running processes

Do not equate “more Nx parallelism” with “faster builds.”

Nx task concurrency multiplied by Cargo jobs, MSBuild workers, native compiler jobs, browser workers, and test threads can exceed memory or saturate storage.

Begin conservatively, then tune from measurements.

Nx supports disabling parallelism for tasks that require exclusive machine resources, but that is not a substitute for a cross-process, cross-worktree, or distributed lock. ([Nx][6])

Where measurements justify it, add a small resource-lease mechanism for shared native directories, ports, licensed hosts, or especially memory-heavy operations. Keep Nx as the scheduler; the lease mechanism should guard a resource, not create another task graph.

For development and E2E execution:

**One owner starts each server.** Do not let Nx, a package script, and the test runner each start another copy.

**Readiness is explicit.** A process existing is not equivalent to its service being ready.

**Cancellation owns the whole process tree.** Stopping Nx must terminate its children and release ports and leases.

**Watchers ignore generated state.** Writing a report or build artifact must not trigger another rebuild loop.

---

## 11. Refactor CI, devcontainers, editors, and release together

### CI

CI should provision the execution environment and invoke Nx. Repository-specific validation and build logic belongs in targets.

An intended validation entry point is:

```bash
nx affected -t lint typecheck test build \
  --base="$NX_BASE" \
  --head="$NX_HEAD"
```

Resolve the base/head SHAs from the actual workflow. Do not assume the repository’s integration branch is `main`, or use only the previous commit when intervening CI runs may have failed.

Nx uses Git changes and the project graph to compute affected projects; its guidance recommends considering changes since the last successful relevant CI baseline. ([Nx][18])

Use separate execution lanes for fast cross-platform checks, native platforms, integration/E2E, and release packaging. Select projects from graph metadata rather than maintaining a second manually curated list.

Keep scheduled full validation and deliberate cache-bypass checks. Affected execution is not evidence that the graph is complete.

### Remote caching

Introduce remote caching only after output contracts pass clean-machine restoration tests.

Protect the shared cache’s write boundary. Trusted branches may publish reusable results; untrusted contribution workflows must not receive equivalent write privileges. Nx documents different access modes, including branch-isolated behavior for read-only CI access. ([Nx][19])

Do not cache secrets, credentials, sensitive service responses, or artifacts whose sharing policy is unclear.

If adopting distributed execution, keep environment provisioning and other uncached side-effect operations outside the distributed cacheable task graph. Nx documents caching requirements for distributed task execution. ([Nx][6])

### Devcontainers and editors

Devcontainer creation should synchronize the selected environment, not automatically compile every product.

Attachment should be cheap and should not repeat installation or launch the entire monorepo.

Editor tasks, debug prelaunch hooks, test buttons where configurable, and agent workflows must invoke the same Nx targets as CI. Provide explicit development profiles for the actual product being worked on.

### Tests and reports

Separate fast unit tests, integration tests, contract tests, E2E, visual regression, and expensive/native-host suites.

Control browser workers and artifact retention. Playwright exposes worker, retry, reporting, and trace configuration; use distinct local and CI policies rather than retaining every large diagnostic artifact indefinitely. ([Playwright][20])

### Release

Adopt:

**build → verify → package → sign/publish/promote**

Publishing must consume the tested artifact identity, not silently rebuild it under a different environment.

Keep signing, publication, deployment, migrations, and external state changes uncached. Use Nx Release where its support fits the actual ecosystem; use explicit Nx targets elsewhere.

A cached Docker task must not merely restore a text file claiming an image exists. Its consumer must be able to obtain the actual image or immutable registry artifact.

---

## 12. Add enforcement so the refactor cannot regress

Introduce proposed targets:

```bash
nx run repo:policy-check
nx run repo:graph-check
nx run repo:cache-verify
nx run repo:artifact-check
```

**Command policy:** parse supported executable surfaces and reject bypasses, recursive orchestration, and forbidden lifecycle work. Distinguish allowed executor implementations from public entry points. Dynamic command construction should require explicit review rather than silently passing.

**Graph policy:** require every relevant manifest to have an owner; validate dependency provenance, unknown projects, cycles, and orphaned generators.

**Artifact policy:** reject overlapping outputs, broad cache roots, writes outside owned paths, and generated directories without retention rules.

**Runtime verification:** exercise representative entry points and record process/write provenance. Static analysis alone cannot prove what arbitrary shell or application code will execute.

**Documentation policy:** validate supported command examples and update agent instructions alongside target changes.

Protect these mechanisms with code ownership. During migration, allow narrow, named waivers with an owner and removal condition—not directory-wide permanent exemptions.

---

## 13. Execute the migration in reviewable phases

| Phase                         | Deliverable                                                                        | Exit gate                                                        |
| ----------------------------- | ---------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| 0. Establish evidence         | Exact pinned tree, inventories, violation ledger, timing/storage baseline          | Every relevant surface accounted for; unknowns recorded          |
| 1. Establish Nx entry points  | Developer, CI, editor, container, and agent entry points routed through Nx         | No new bypasses; temporary opaque targets explicitly tracked     |
| 2. Complete the graph         | Polyglot discovery and source/runtime/artifact dependencies                        | Representative change-impact tests pass                          |
| 3. Establish ownership        | Explicit inputs, outputs, variant paths, artifact registry                         | No overlapping outputs; clean restoration works                  |
| 4. Fix native execution       | Remove duplicate compilation, inappropriate release settings, and oversubscription | Cold/warm/leaf-change measurements improve without lost coverage |
| 5. Bound storage              | Budgets, retention, safe cleanup, container policy                                 | Repeated workloads converge instead of growing indefinitely      |
| 6. Optimize CI                | Correct affected baselines, trusted remote cache, platform lanes                   | CI matches local semantics; cache isolation verified             |
| 7. Remove compatibility paths | Retire legacy schedulers, scripts, aliases, and waivers                            | One supported orchestration model remains                        |

During equivalence testing, run old and new paths in separate bounded output locations. Compare test discovery, results, deliverables, runtime behavior, and packaging—not only exit codes.

Do not run two complete pipelines indefinitely “for safety.” That would reproduce the time and storage problem during migration.

---

## 14. Define success before declaring completion

The following should be release gates for the refactor:

| Scenario                      | Required result                                                              |
| ----------------------------- | ---------------------------------------------------------------------------- |
| Identical second invocation   | All eligible deterministic tasks reuse results; no unnecessary compiler work |
| Leaf frontend change          | No unrelated Rust, Go, CMake, or .NET work                                   |
| Test-only change              | No unrelated production packaging                                            |
| Shared schema change          | Every genuine consumer is invalidated                                        |
| Deleted deliverables          | Cache restoration reconstructs everything required by consumers              |
| Clean compatible machine      | Restored artifacts execute successfully                                      |
| Toolchain/platform change     | Incompatible native results are not reused                                   |
| Cancelled development session | No orphaned child processes or occupied ports                                |
| Repeated no-change cycles     | No monotonic retained-storage growth                                         |
| Cleanup                       | No source, user data, active state, or unrelated container volume is removed |
| Untrusted CI contribution     | Cannot contaminate trusted shared artifacts                                  |
| Release                       | Published artifacts match those that passed verification                     |

Set wall-time and storage service levels after the baseline identifies machine classes and workload sizes. The goal should be concrete—fast leaf feedback, predictable full validation, and bounded retained state—but an unmeasured “hours to minutes” promise would not be credible.

**My recommended priority is: complete the graph, eliminate hidden/repeated work, establish artifact ownership, then enable broader caching and distribution.** Doing those in the opposite order risks distributing redundant builds and caching the very directories responsible for the disk problem.

The outstanding part of your request is the **verified violation ledger at the exact commit**. The available branch overview established the inspection surfaces, but the pinned file contents and build measurements were inaccessible. A readable checkout or archive of `aa72759d4183638d265b5c2cb3d409320ad72f4f` is the missing input for converting this design into exact file-level changes and confirmed findings.

[1]: https://nx.dev/docs/features/run-tasks "https://nx.dev/docs/features/run-tasks"
[2]: https://github.com/usalu/semio/tree/%F0%9F%90%99ueli/%E2%9B%B3wip "https://github.com/usalu/semio/tree/%F0%9F%90%99ueli/%E2%9B%B3wip"
[3]: https://nx.dev/docs/concepts/nx-plugins "https://nx.dev/docs/concepts/nx-plugins"
[4]: https://nx.dev/reference/nx-json "https://nx.dev/reference/nx-json"
[5]: https://nx.dev/docs/extending-nx/project-graph-plugins "https://nx.dev/docs/extending-nx/project-graph-plugins"
[6]: https://nx.dev/docs/reference/project-configuration "https://nx.dev/docs/reference/project-configuration"
[7]: https://nx.dev/docs/reference/inputs "https://nx.dev/docs/reference/inputs"
[8]: https://nx.dev/docs/guides/tasks--caching/configure-outputs "https://nx.dev/docs/guides/tasks--caching/configure-outputs"
[9]: https://nx.dev/docs/technologies/typescript/introduction "https://nx.dev/docs/technologies/typescript/introduction"
[10]: https://doc.rust-lang.org/cargo/reference/build-cache.html "https://doc.rust-lang.org/cargo/reference/build-cache.html"
[11]: https://doc.rust-lang.org/cargo/reference/profiles.html "https://doc.rust-lang.org/cargo/reference/profiles.html"
[12]: https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-test "https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-test"
[13]: https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html "https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html"
[14]: https://docs.astral.sh/uv/concepts/cache/ "https://docs.astral.sh/uv/concepts/cache/"
[15]: https://go.dev/cmd/go/ "https://go.dev/cmd/go/"
[16]: https://docs.docker.com/build/cache/garbage-collection/ "https://docs.docker.com/build/cache/garbage-collection/"
[17]: https://docs.docker.com/build/building/context/ "https://docs.docker.com/build/building/context/"
[18]: https://nx.dev/docs/features/ci-features/affected "https://nx.dev/docs/features/ci-features/affected"
[19]: https://nx.dev/docs/guides/nx-cloud/access-tokens "https://nx.dev/docs/guides/nx-cloud/access-tokens"
[20]: https://playwright.dev/docs/test-configuration "https://playwright.dev/docs/test-configuration"

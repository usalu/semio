# 🗂️ Build And Test Cache Census (aud-caches-1020e)

Read-only audit for wave 1020e. No builds, no `bun repo`, no git, no sub-agents. Only this file was written.

## 0. Scope, Method, Deviations

Repo root `R` = `/Users/ueli/Documents/semio`. Path aliases used below (all relative to `R`):

- `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`
- `BUILD` = `LIB/🏗️build`
- `CACHING` = `LIB/⚡️caching`
- `PROC` = `🧰️framework/🔨️modules/🏃️process`
- `LEASES` = `PROC/📦️packages/🟦️typescript/🔒️leases/🟦️.ts` (the implementation is `PROC/🔒️leases/🟦️.ts`, 63 KB)

Tags: **VERIFIED-READ** = read or grep-confirmed in this session. **INFERRED** = from tool/runtime knowledge, not confirmed in this repo. **VERIFIED-LS** = directory listing only.

Deviations from the brief (disclosed):
1. One depth-3 `find` from the repo root for vite/vitest/playwright/tsconfig file names. This was repo-wide, which the brief forbids. It returned only `R/tsconfig.json` and `R/✏️s/tsconfig.json`.
2. Reads went beyond `BUILD` into `CACHING` and `PROC`. The brief names `.cargo/config.toml`, which points directly at `CACHING/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress`, and the partition budget lives in `CACHING`. Those reads were needed to answer the census.
3. Greps used `/usr/bin/grep` after the aliased `ugrep` silently failed on emoji-heavy minified files.

## 1. Physical Layout (the Cache Partitions)

Source of truth: `CACHING/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json:1-47` (VERIFIED-READ).

| Partition | Directory | Capacity | Logical max | Holds |
|---|---|---|---|---|
| rust | `.🧬semio/🦑️repo/⚡️cache/rust/fixed` | 16 GiB | 17178820608 B | cargo store, nx artifacts (see G4) |
| tools | `.cache/b/t` | 8 GiB | 64 GiB | Bazel executable and install |
| graph | `.cache/b/g` | 4 GiB | 4 GiB-1 MiB | repo graph facts (`inference`, `contexts/<id>`, `selections/<id>`) |
| dependencies | `.cache/b/d` | 8 GiB | 8 GiB-1 MiB | `node_modules` generations |
| products | `.cache/b/p` | 24 GiB | 24 GiB-1 MiB | Bazel workspaces, outputs, actions, server; receipts |
| diagnostics | `.cache/b/o` | 2 GiB | 2 GiB-1 MiB | diagnostics |
| nx | `.cache/b/n` | 1.5 GiB | 1.5 GiB-1 MiB, 512 MiB per artifact | nx graph namespaces |
| leases | `.cache/b/l` | 64 MiB | 60 MiB | lease `data` (`repositoryLeaseDirectoryV1`) |
| control (not a partition) | `.cache/b/c` | n/a | 1 MiB `maximumControlBytes` | SQLite lease catalogs, `claims/`, `*.queue/` |

- Global cap: `maximumBytes` 64 GiB (`policy:3`). Each partition is a fixed-size image: `.cache/b/<p>/fixed.dmg` plus `image.json` plus `m/` (VERIFIED-LS: `.cache/b/{d,g,n,p,t,o,l}` each contain `fixed.dmg`, `image.json`, `m`).
- Platform backing (VERIFIED-READ): `PROC/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` selects `darwin`→macos, `linux`, `windows` (the `"darwin"?"macos":...` expression). macOS uses a persistent APFS image (`🍎️image/`). Linux and Windows use a directory backend (`📁️directory/🟦️.ts`). That directory doc says kernel exchange on Linux and macOS, and a journaled write-through swap on Windows. Any other host is a refusal, not a fallback.
- Legacy and unbudgeted roots, all outside the 64 GiB sum (see G6): `.🧬semio/🦑️repo/⚡️cache/*`, `.cache/framework`, `.cache/held-javascript`, `.cache/bazel`, `.cache/boot-recovery`, `.cache/k0`, `.cache/ldta`.

## 2. Census

Columns: location, scope (per-checkout / per-agent / shared-in-folder / machine), concurrent-writer safety, incremental vs. all-or-nothing, eviction bound, content-addressed and cross-crate sharing.

### 2.1 Cargo / rustc / wasm

**Cargo shared store.** Location: `.cargo/config.toml:8-9` declares `target-dir` and `build-dir` at `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/shared/{target,build}`. Scope: shared across every agent in this folder, but only through the launcher.

- On disk (VERIFIED-LS): `m/cargo/shared` does NOT exist. The live cargo state is `m/cargo/{home,preparation,prototype}`. The `[build]` paths in `.cargo/config.toml` are therefore not the store in use. The comment at `.cargo/config.toml:1-3` says managed facades carry their own issued store paths.
- Writers: fine-grain locking (`.cargo/config.toml:14`, cargo unstable). Per-unit locks let different crates build in parallel, and the same-unit case serializes. Known wasm deadlocks (memory note) are the failure mode.
- Ingress gate: `[build].rustc` and `rustc-wrapper` (`.cargo/config.toml:6-7`) route every rustc through `CACHING/…/🚪️ingress/📜️script.ts`. That script throws unconditionally (`script.ts:4`, VERIFIED-READ) unless the launcher supplies its issued environment. Raw cargo is refused.
- Compiler charge: `repositoryCompilerReservationV1` takes an exclusive lease per build mode, `repo:compiler-charge:<image>:<mode>` (`CACHING/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:31`, VERIFIED-READ). It is short and per mode, not per crate. The compile body itself is retained shared (INFERRED from docstrings `🏪️`, `🧵️`).
- Config publication: immutable view written as `file.<rand>` with `wx`, then `renameSync`, under an exclusive lease. Collisions are checked by sha256 (`🚪️ingress/⚙️configuration/🟦️.ts:45`, VERIFIED-READ). This is content-addressed with CAS-style publish.
- Incremental: yes, per unit. `checksum-freshness = true` (`.cargo/config.toml:16`). `build-dir-new-layout` (`:15`) is unstable.
- Eviction: pruning is slice-based, not whole-dir. `CACHING/🧹️pruning/🟦️.ts:19` defines a slice as a compilation unit, an incremental crate dir, a stale finalized incremental session, an uplifted file, or a scratch dir. `📉️pressure` removes the oldest inactive slices (VERIFIED-READ docstrings). Bound: the 16 GiB rust partition, which nx also uses.
- Content-addressed: yes, cargo's unit hash in `build/<package>/<hash>` (VERIFIED-READ doc in `CACHING/🦀️cargo/🗂️directories/🟦️.ts` and `🧹️pruning`). Different crates share results only for common dependencies.

**rustc incremental.** Lives inside the cargo target tree (cargo default, INFERRED). Pruning compacts superseded finalized sessions (`CACHING/🧹️pruning/🔒️incremental/` docstring). A session's `.lock` file is measured with it (`CACHING/🧹️pruning/⏱️measurement` docstring family).

**Wasm.** Target-scoped rustflags: `wasm32-unknown-unknown` (`.cargo/config.toml:25-26`, 16 MiB stack) and `wasm32-wasip2` (`:43-44`, 512 MiB max memory, 8 MiB stack). Both are fixed per target, so fingerprints do not churn. Component artifacts sit under `CACHING/📦️artifacts/🕸️wasm` (directory exists, no docs, VERIFIED-LS only).

### 2.2 Nx

- Local artifact cache: `nx.json:91` sets `cacheDirectory` to `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts`. That is inside the **rust** partition (16 GiB, shared with cargo), not `.cache/b/n`. Enforced by `CACHING/…/🔌️nx/🟦️.ts:75` (`join(…,"m/nx/artifacts")`) and built by `CACHING/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts:46` (`nxCacheDirectoryV1`). VERIFIED-READ.
- On disk (VERIFIED-LS): `rust/fixed/m/nx` does NOT exist, so nx artifacts have not been materialized on this checkout, or they are created lazily.
- Writers: "serialized writer behind concurrent invocations" (docstring of `CACHING/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts`), plus daemon namespace retention (`useDaemonProcess: true`, `nx.json:6`). Safe, but serialized.
- Graph namespace: `.cache/b/n`, 1.5 GiB, per-artifact cap 512 MiB (`budget policy` nx entry). A per-artifact cap larger than 512 MiB is refused, so big outputs (bundles, binaries) are uncacheable (INFERRED consequence).
- Content-addressed: nx task hash including `compiler revision` (`📦️outputs/🦀️promotion` doc). Per-project granularity, so different crates or projects never collide.
- Input blindness: memory note says the nx hasher is blind to gitignored inputs. `nx.json:9-12` (`default` named input `{projectRoot}/**/*`) relies on git-tracked inputs (INFERRED from memory, not re-verified).

### 2.3 Repo Graph Facts (Inference Packs and Seal)

- Location: `.cache/b/g/m/{inference, contexts/<dataset-identity>, selections/<selection-identity>}` (`BUILD/📥️inputs/💾️storage/🟨️.mjs:104-107`, VERIFIED-READ).
- Writers: `withRepositoryBuildGraphFactStoreV1` (`🟨️.mjs:120`) calls `withGraphFactWriterV1` without a `namespace`. Lines 131-132 then take `acquireQueuedResourceLease({resource: namespace ? buildGraphWriterResourceV1(...) : "repository-graph-facts:"+resolve(workspace), mode:"exclusive"})`. **The default path is one workspace-wide EXCLUSIVE lock**, and this is the 2.5 h blocker in the brief. Sharding exists (`buildGraphWriterResourceV1`, `🟨️.mjs:111-113`, sha256 mod `writerShards`), but only when a namespace is passed.
- Optional leaves: quota and ledger (`🟨️.mjs:22-29`, `📒️ledger`). `lowWaterBytes` 512 MiB (`BUILD/📥️inputs/💾️storage/🧬️schema/🔣️.json`) and `minimumAvailableBytes` 2.5 GiB (budget schema, VERIFIED-READ by grep). `maximumSnapshotBytes` 512 MiB and `maximumRetainedBytes` 1 GiB (`CACHING/…/📦️budget/🕸️graph/🔣️policy.json`, VERIFIED-READ).
- Eviction: optional leaves reclaimed under exclusive maintenance (`prepareBuildGraphOptionalHeadroomV1`, `🟨️.mjs:75-78`). Writers defer while readers exist.
- Content-addressed: yes, by identity hash (`🟨️.mjs:112` regex `^[a-f0-9]{64}$`).

### 2.4 Dependencies (node_modules)

- Location: `.cache/b/d/m/j/81b8b51e9cf4b7e091f6/node_modules`, reached through the repo-root symlink `node_modules` (VERIFIED-LS, `ls -la R` shows the symlink). Generation-keyed by lockfile hash (path shape VERIFIED-LS; key derivation INFERRED).
- Writer: the producer holds whole-partition EXCLUSIVE custody across all installers and the Source transition. Docstrings in `CACHING/…/📥️opening/📤️publication/🟦️.ts`: "Completes only a fresh authenticated all-scope producer under retained funding and global Source custody" and "Plans only under the genuine current full funding, parent EX and complete Bun Source owner" (VERIFIED-READ). This matches brief blocker 2 (`native-fixed-image:.cache/b/d` EXCLUSIVE, > 1 h).
- Per-checkout: yes. Each install generation is a new subtree; old generations are retired by `rebuildUnready…`/reset logic.
- Vite and vitest caches (`node_modules/.vite`, `node_modules/.vitest`, INFERRED defaults) therefore live inside this generation: per-checkout, per-generation, not shared across generations.
- bun global install cache: `/Users/ueli/.bun/install/cache` exists (VERIFIED-LS), machine-wide, content-addressed by integrity (INFERRED). `bunfig.toml:1-4` sets `linker = "isolated"`, `globalStore = false`, and empties `hoistPattern`. The tarball cache is still shared, but no global store is used.

### 2.5 Bazel

- Output base: `bazelServerBaseV1` (`BUILD/🔧️tools/🏗️bazel/💾️storage/🟦️.ts:46-49`) returns `layout.server/<sha256(workspace)[0:16]>`. One output base per workspace, so every agent in this folder shares one Bazel server, and concurrent commands serialize on its lock (docstring: "Names the one output base whose server serves every command of a single workspace"). VERIFIED-READ.
- Startup flag `--output_base=` and disk cache flags: `bazelCacheFlagsV1` (`:51-52`). Command flags: `--disk_cache=<products>/bazel/actions` and `--experimental_disk_cache_gc_max_size` (`:52`), sized by `bazelDiskCacheMaximumBytesV1` (`:41-44`) to 40% of the products logical max (`schema` `diskCache.productsShareBasisPoints: 4000`, VERIFIED-READ at `BUILD/🔧️tools/🏗️bazel/🧬️schema/🔣️.json:733-734`). That is about 9.6 GiB.
- `.bazelrc:5-8` (VERIFIED-READ): `--experimental_convenience_symlinks=ignore`, disk-cache idle GC 10 s, disk-cache max age 30 d, repository contents cache max age 3 d. `MODULE.bazel` is one line (`module(name="semio",…)`). No `--output_user_root`, no `--repository_cache` pinned, so the repository cache uses Bazel's default location (INFERRED). `/Users/ueli/.cache/bazel` does not exist (VERIFIED-LS).
- Content-addressed: yes, Bazel action cache and CAS keyed by action digest. Shared across all workspaces on the same products partition (INFERRED Bazel behavior, consistent with the path).
- Remote cache: optional and off unless an endpoint is configured (`BUILD/🌐️remote-cache/🟦️.ts`, resolver doc "`undefined` means the remote cache is off"). Store: `BUILD/🌐️remote-cache/💾️store/🟦️.ts:34-35` (`RemoteCacheStoreV1`), atomic write (`:107`), content verification, LRU eviction to 90% (`:88`). Scope: cross-machine only.
- Orphan candidate: `.cache/bazel/install` (created Oct 14, VERIFIED-LS). Zero code references under `🧰️framework` (VERIFIED-READ grep).

### 2.6 Store Receipts

- `BUILD/🧾️store-receipt/🟦️.ts` (VERIFIED-READ): tasks declare a receipt, and cache hits whose receipt names missing or changed content are treated as misses.
- Physical receipts: `CACHING/…/📦️budget/📥️opening/🏪️store/🟦️.ts` writes `<label>.json` via `receipt+"~"+randomBytes` and `renameSync`, under exclusive `repository-cache-charge:<image.root>` (`:41,44`, VERIFIED-READ). Atomic publish, check-then-act inside the exclusive lease.
- Compiler receipts: `compiler-<mode>.json` (`CACHING/…/🏗️compiler/🟦️.ts:31-32`), exclusive per mode.

### 2.7 Lease Infrastructure (the Concurrency Substrate)

- Mechanism (VERIFIED-READ, `PROC/🔒️leases/🟦️.ts`): one SQLite file per resource, `<directory>/<sha256(resource)>.sqlite` (`:30`). Kernel-held SQLite locks. A FIFO arrival queue in `<resource-hash>.queue/<arrivedAtMs>-<pid>-<owner>` (`:251-257`). "An exclusive SQLite claim alone proves abandoned custody" (queue doc). No TTL. Shared-to-exclusive upgrade at original arrival position (docstring).
- Directories: `.cache/b/l/m/data` for partition leases (`repositoryLeaseDirectoryV1`, `CACHING/…/📥️opening/🟦️.ts:30`), `.cache/b/c` as the control dir (`CACHING/…/📦️budget/🟦️.ts:23,35`).
- Capacity gate (`PROC/🏗️build/🎟️capacity/`) and host sampling: `PROC/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts:3` imports `freemem` and `totalmem` from `node:os` (VERIFIED-READ). On macOS, `freemem` excludes reclaimable file cache, so available memory is under-reported (INFERRED; matches the brief's "mis-measured darwin memory").

### 2.8 TypeScript, Vite, Vitest, esbuild, Playwright, Bun Test

- tsc: `R/tsconfig.json:16` sets `"incremental": false`, and `:17` sets `noEmit: true`. No `tsBuildInfoFile`. Every run type-checks the whole `include` (`:19`, `**/*.ts`, `**/*.tsx`). Not incremental (VERIFIED-READ). `R/✏️s/tsconfig.json` was not read in full (scope).
- Vite: the library's `CACHING/🌐️vite/` is about product filesystem and compiler paths, not Vite's own cache. Vite's own cache follows the `node_modules` generation (INFERRED).
- Vitest: `package.json` lists `vitest` 4.x and `@vitest/coverage-v8` (VERIFIED-READ at `R/package.json:308,236`). Cache follows `node_modules` (INFERRED).
- esbuild: `package.json:410`. Used by the bundle pipeline (`CACHING/📦️bundle/`, VERIFIED-READ doc "Starts esbuild after admission…"). esbuild keeps no persistent disk cache (INFERRED).
- Playwright: `package.json:226,284` (`@playwright/test` and `playwright` 1.57). Browsers live in `/Users/ueli/Library/Caches/ms-playwright` (exists, VERIFIED-LS). Machine-wide, versioned, not per checkout. No `playwright.config.*` found in the depth-3 search (deviation 1).
- Bun test: no persistent test cache (INFERRED).

### 2.9 Inference Pass Output and Other Graph Packs

- `CACHING/📥️inference/` and `BUILD/🕸️graph/📥️inference/` compute in memory and persist into the graph namespaces in §2.3. There is no separate on-disk pack.

### 2.10 Legacy and Unbudgeted Roots

Counts from `ls -A | wc -l` unless noted (VERIFIED-LS). None are in the policy sum.

| Path | Entries | Code references under `🧰️framework` (VERIFIED-READ grep) |
|---|---|---|
| `.🧬semio/🦑️repo/⚡️cache/build-lock-closures` | 639 | 6 hits (`build-lock`), not inspected further |
| `.🧬semio/🦑️repo/⚡️cache/cargo-preparation` | 724 | used only as a resource-name string, not as a path |
| `.🧬semio/🦑️repo/⚡️cache/nx-lock-closures` | 587 | 0 |
| `.🧬semio/🦑️repo/⚡️cache/go` | 258 | not searched |
| `.🧬semio/🦑️repo/⚡️cache/leases` | 123 | only `PROC/🧭️routing/🎛️command/⚙️configuration/🟦️.ts:39` (`join(store,"leases")`) |
| `.🧬semio/🦑️repo/⚡️cache/edfix` | 101 | not searched |
| `.cache/framework/leases` | ~2.4k (link count 2423) | 0 |
| `.cache/held-javascript` | 92 | 0 |
| `.cache/bazel/install` | n/a | 0 |

Each `*-closures` or `*-preparation` directory looks like one directory per run, so growth is per invocation with no GC policy (INFERRED from counts).

## 3. Verdict For "Many Agents, Same Folder, Different Crates"

| Cache | Shares across agents? | Concurrent-safe? | Serializes? | Incremental? |
|---|---|---|---|---|
| Cargo (shared store via launcher) | yes | yes, per-unit locks | per unit and per compiler mode charge | yes |
| Nx artifacts | yes | yes | serialized writer | yes, per project |
| Repo graph facts | yes | yes, but default path is one EX lock | **yes, workspace-wide EX** | no, seal is all-or-nothing per graph |
| Dependencies (`.cache/b/d`) | yes | yes | **whole-partition EX during install** | no |
| Bazel | one output base per workspace | yes | **server lock per workspace** | yes (action cache) |
| tsc | per checkout | n/a | n/a | **no** (`incremental: false`) |
| Vite / vitest | per generation | n/a | n/a | yes if the tool supports it (INFERRED) |
| Playwright / bun global | machine-wide | yes (tool-managed) | no | n/a |
| Remote cache | off by default | atomic writes, verified | no | n/a |

Answer: different crates on the cargo path can build in parallel now. The graph seal and the dependency partition serialize the whole folder. Bazel serializes per workspace. tsc has no incremental state.

## 4. Gap Ranking

Ranked by impact on the brief's goal (many agents, same folder, different crates, shared and incremental).

**G1 (critical). Graph fact writers take one workspace-wide EXCLUSIVE lock by default.** `BUILD/📥️inputs/💾️storage/🟨️.mjs:120,131-132`. Any stale seal (triggered by any source edit) forces an exclusive rebuild that blocks every reader and writer. This is the 2.5 h hold. Fix: pass a namespace on every path (sharding already exists at `:111-113`), and let readers of a current seal stay shared.

**G2 (critical). Whole-partition EXCLUSIVE custody for long producers.** `.cache/b/d` installs hold the full partition EX (`📤️publication` docstrings; `📦️budget/📥️opening/🟦️.ts` "Queues exact whole-image EX"). A cold install stalls `show projects` and every other command. The queued-maintenance wait is the same mechanism (`CACHING/…/📥️opening/🏪️store/🟦️.ts:123-124`, "Queues exact whole-image EX"). Fix: split the dependency generation into per-package leases, or write into a staging generation and take EX only for the atomic swap, which is already the shape of `node_modules` → `m/j/<hash>`.

**G3 (high). Bazel serializes all agents on one output base.** `BUILD/🔧️tools/🏗️bazel/💾️storage/🟦️.ts:46-49`. Fix: one output base per agent or per target-set, backed by the shared disk cache (`--disk_cache`, `:52`), which is already content-addressed.

**G4 (high). Nx artifacts and cargo share one 16 GiB DMG.** `nx.json:91`, `CACHING/…/🔌️nx/🟦️.ts:75`, `CACHING/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts:46`, `policy:7-9`. Two producers with different eviction semantics compete for the same bytes, and the 512 MiB nx artifact cap sits on a different partition (`nx`). ENOSPC or pressure-driven evictions of cargo units are the likely outcome. Fix: give nx its own partition, or take `m/nx` out of the rust budget.

**G5 (high). The declared cargo store does not exist, and raw cargo is refused.** `.cargo/config.toml:6-9` names `m/cargo/shared/…`, which is absent (VERIFIED-LS). Every rustc call goes through the ingress script, which throws unless launched (`script.ts:4`). Any agent that bypasses `bun repo cargo --store=shared` fails. Fix: make the managed store path the single declared target, and remove the shared-path fallback so the config and the live store agree.

**G6 (high). Unbudgeted accumulators.** Section 2.10. About 3k entries are not in the 64 GiB policy, with no GC. This is the same pattern as the 926 GB incident in memory. Fix: register `.🧬semio/🦑️repo/⚡️cache` and the `.cache/framework`, `.cache/held-javascript`, `.cache/bazel` roots under the budget with a pruning policy, or delete them after confirming no reader.

**G7 (medium). FIFO leases with no TTL, plus FIFO capacity gating.** `PROC/🔒️leases/🟦️.ts:251-257` and `🎟️capacity`. One long holder delays every later ticket, with no progress-based fairness or preemption. Fix: priority or shared-cohort admission for read-only waiters, plus a progress timeout.

**G8 (medium). Host memory is mis-measured on darwin.** `PROC/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts:3` uses `freemem`, which excludes reclaimable cache. Fix: use `vm_stat` or `memory_pressure` counters on darwin and `MemAvailable` on Linux.

**G9 (medium). tsc is non-incremental.** `R/tsconfig.json:16` sets `incremental: false`, and `:19` includes the whole repo. Fix: per-package `tsconfig` with `composite` and `tsBuildInfoFile` under the per-package product directory.

**G10 (medium). Nx input blindness to gitignored inputs.** `nx.json:9-12` (INFERRED from memory). Cache keys miss generated inputs, so a cached result can be reused after an input changes. Fix: declare every generated input explicitly in `namedInputs`.

**G11 (low). Per-artifact caps block large outputs.** nx artifacts over 512 MiB and graph snapshots over 512 MiB are refused (`policy` entries). Plugin and bundle outputs may fall into this and rebuild every run (INFERRED).

**G12 (low). Platform coverage.** Linux and Windows directory backends refuse any unsupported libc or host (`📁️directory/🟦️.ts` doc). Devcontainer or Alpine (musl) builds may not open the store at all (INFERRED from the refusal; not run).

## 5. Open Questions (Not Resolved Here)

- Whether `rust/fixed/m/nx/artifacts` is created lazily or never populated. Only a run can answer it.
- The real on-disk size of `fixed.dmg` images versus their logical budget. Not measured (scope: `du` allowed only on `.cache/b` top-level, depth 1).
- Whether Bazel's default repository cache location is on the same volume as the products partition. Not verified.
- Whether `.cache/framework/leases` and `.🧬semio/.../⚡️cache/*` are referenced from outside `🧰️framework` (for example `✏️s`, `🌎️hub`). Not searched.

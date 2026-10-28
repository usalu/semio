# audit-nx-route-1026g (auditor aud-nx-g, wave 1026g)

Read-only audit of the Nx route (default build tool) against the brief `brief-wave-1026g.md`.
Method: code reads under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, the `nx@23.2.0` patch, the live process table and disk sizes (ps, ls, du; a snapshot at 07:32 CET), and one read-only node probe that imports the engine-closure module (no build, no Nx run, no repo command, no git, no kill). Probe scripts: `C/aud-nx-g-tools/engine-closure-*.mjs`. Result numbers from `result-nx2-d.md`, `result-nx3-d.md`, `result-nx4-d.md`, `result-nx-f.md` are cited as such.

Paths below are relative to the repo root `/Users/ueli/Documents/semio`. `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 0. Short answers

- Daemons: one finite-daemon pool per checkout (2 slots), shared by all agents on that checkout; not per agent. Idle exit after 10 min without requests. Measured one daemon at 876 MB RSS, detached (ppid 1). The host capacity governor does not see daemon memory.
- Graph: the default route builds the repo graph first and then Nx derives its own project graph per repo generation (namespace keyed by the pinned digest). This is duplicate work by construction, but it is once per generation per namespace, not once per run.
- Cache: task artifacts are shared per checkout under the rust partition; graph namespaces under `.cache/b/n`; toolkits under `.cache/b/t`. The toolkit store holds 149 identities (about 195 MB each as allocated) against an 8 GiB tools partition.
- Named inputs: every Nx target carries one runtime engine digest over 1,233 files (47 Bazel files, 3 scheduling files, 2 cache-gc files, 413 process files). A byte change in any of them re-keys every Nx task in the workspace. The `schedulingSourceExclusions` policy in `nx.json` is defined but not consumed by the Nx route.
- Patch: `nx@23.2.0.patch` is 139,811 bytes, 117 hunks, 36 compiled `dist/` files, +795 / -405 lines.
- Biggest blocker for the brief's multi-agent and 16 GB targets: the engine digest (problem 1), the daemon memory outside admission (problem 3), and the 4 GiB floor plus a generic 1.5 GB `run` row (problem 5).

## 1. Nx daemon lifecycle and RSS

Code:
- Limits and admission: `LIB/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🟦️.ts` (`nxDaemonLimitsV1`, `decideNxDaemonV1`, `nxDaemonAdmissionV1`), policy in `LIB/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧬️schema/🔣️.json` (`Policy.const`).
- Start: `LIB/⚡️caching/🚀️bootstrap/📜️script.ts` around line 449. The run asks admission only when `NX_DAEMON` is unset and the run is not `watch`. The result is `NX_DAEMON=true|false`.
- Slots: `nx:daemon-slot:<sha256(checkout root)[0:16]>:<finite|watch>:<index>` (same file). The slot key is the checkout, not the agent. A namespace with a live daemon is reused without taking a slot (`decideNxDaemonV1`, reason `reuse`).
- Lifecycle: `LIB/⚡️caching/🕸️graph/🏁️startup/🧬️lifetime/🟦️.ts` (phases awaiting-acquisition, acquired, ready, draining, terminal; complete drain before release), `LIB/⚡️caching/🕸️graph/🏁️startup/🚦️service/🟦️.ts` (service birth and namespace binding).
- Patch: `shutdown-utils.js` (25 added, 72 deleted lines) rewrites the drain path. The repo daemon service counts the other live holders of its namespace lease as open requests (`otherLeaseHoldersV1`, nx3-d), so an attached client keeps the daemon alive.

Policy numbers (`Policy.const`):

| knob | value | effect |
|---|---|---|
| finite | 2 | finite daemons per checkout; `min(finite, RAM/4/2 GiB)` = 2 on both 16 GB and 32 GB |
| watch | 6 | dev watch daemons per checkout, never idle out |
| maximumSlots | 16 | hard ceiling for `SEMIO_NX_DAEMONS` overrides |
| idleMilliseconds | 600000 | finite daemon exits after 10 min with no open request |
| yieldCheckMilliseconds | 5000 | an eviction of the graph pool drains idle finite daemons within 5 s |
| drainMilliseconds | 60000 | drain watchdog; the patch makes a stuck drain exit |
| megabytesPerDaemon | 2048 | only used to compute the slot count; not charged to the host |

Measured (ps, 07:32 CET, read-only): daemon pid 64500, ppid 1 (detached), RSS 876,320 KB, elapsed 13:45. It runs from `.cache/b/t/m/nx/<toolkit>` with `bun --conditions development`. Other agents' loader children were at 2.98 GB and 3.2 GB RSS (pids 73242, 69857); not touched.

Findings:
- No heap cap on the Nx daemon or on the Nx child. `bazelSessionStartupV1` (`LIB/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts:142`) passes `--host_jvm_args=-Xmx<n>m` and `--max_idle_secs` to Bazel; the Nx route has no equivalent (grep for `max-old-space`, `NODE_OPTIONS` in the Nx route: no hit).
- The governor (`LIB/🏗️build/🌡️host-capacity/🟦️.ts`) has no daemon accounting (grep `daemon` in that file: no hit outside a comment about the dashboard). The Nx run asks the generic `run` row, `fixedMegabytes: 1536` (`LIB/🏗️build/🔣️policy.json`, `hostCapacity.operations[0]`). A daemon that outlives its run holds memory that no grant covers.
- Shared per checkout, not per agent: two daemons serve all agents of one checkout. A third concurrent namespace gets `daemon.slots-exhausted` and the run goes daemonless (`NX_DAEMON=false`, rerun once, nx3-d). That run recomputes the Nx graph in-process.
- Namespace churn holds slots: a daemon whose namespace is superseded by a newer digest keeps its slot until the 10-min idle rule fires, because superseded is not idle.
- Drain watchdog: the 60 s bound is law-green (nx3-d) but was not seen live; nx3-d recorded a 10-min hang in `completeNxDaemonShutdownV1` before the bound existed.

## 2. Nx graph construction versus the repo's own generations

Flow on every graph-consuming Nx run (`LIB/⚡️caching/🚀️bootstrap/📜️script.ts`, `runWithToolingStore`, around lines 404-409):
1. `loadBuildGraphV1` loads the repo build graph (the shared generation machinery; writer lease `repository-graph-facts:<root>:shard:7`, `LIB/🏗️build/📥️inputs/💾️storage/🔒️writer/🟨️.mjs`).
2. `withSelectedRustBuildGraphV1` then `withRepositoryBuildGraphArtifactV1` pins a graph artifact and sets `SEMIO_BUILD_GRAPH_PATH` and `SEMIO_BUILD_GRAPH_DIGEST`.
3. Line 250: a set digest forces `NX_FORCE_REUSE_CACHED_GRAPH=false`.
4. Line 448: the Nx namespace is bound with `(toolingIdentity, SEMIO_BUILD_GRAPH_DIGEST, implementationIdentity)`, so the namespace key is the digest.
5. Lines 525-532: the Nx generation claim decides reuse. If the Nx generation for this digest already holds in this namespace, `NX_FORCE_REUSE_CACHED_GRAPH=true`. Otherwise Nx derives its project graph again. The membership walk (`nxGraphSourceMembershipV2`, line 527) and the seal check (line 530) run only when no digest is set, so they are off on the default route.

So a repo generation costs one Nx project-graph derivation per namespace, on its first run. Later runs of the same digest reuse it. Sizes (result-nx2-d): the print project graph was 497 MB JSON (550 projects) before sharing, 113 MB after; course 161 MB to 59 MB. The Nx graph is a pure projection of the repo graph (`projectBuildGraphForNxV1`), so the second derivation is duplicate work.

Other duplicate or serialising points:
- Every graph-consuming run, Nx or Bazel, takes `repository-graph-facts:<root>:shard:7` exclusively in the path above. nx-f measured eight runs queued for hours behind Bazel `run-many` loaders holding shard 7 (21103 for 2,502 s). A run whose generation is already sealed still waits for the writer.
- The retry path (lines 560-566) forces `NX_FORCE_REUSE_CACHED_GRAPH=false` and relaunches up to three times when a source changes during the run. Under the fleet's edit rate this is a main source of long Nx runs (nx3-d: five print:test builds of 37 min in total reported "Taking a new snapshot (1 of 3)", then exit 75).
- Graph memory on the repo side is unchanged by Nx: the brief records builder 5-13 GB RSS and 60-120 min for the framework-edit case. Nx does not reduce that; it adds a second graph per generation.

## 3. Nx cache location and sharing

| store | location | size now | shared by |
|---|---|---|---|
| task artifacts | `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` (`NX_CACHE_DIRECTORY`) | 230 MB, 135 entries | all agents of one checkout; content-addressed by the Nx task hash |
| TMPDIR, native | `…/rust/fixed/m/nx/tmp`, `…/nx/native` | inside the rust partition (48 GiB budget) | same |
| graph namespaces | `.cache/b/n/m/<40 hex>` (`NX_WORKSPACE_DATA_DIRECTORY`) | 7 namespaces, 677 MB | per (workspace, toolkit, digest, implementation[, watcher]) |
| toolkits | `.cache/b/t/m/nx/<64 hex>` | 149 identities, about 195 MB each as allocated (about 29 GB) | per tooling identity (nx version + patch + authorizers + binaries) |

Findings:
- The task-artifact cache is shared across agents of one checkout, but the Nx task hash includes the engine digest (problem 1), so a peer edit to any engine-closure file invalidates the shared artifacts of every agent.
- Two caps describe the same Nx store: `LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` gives partition `nx` 1,610,612,736 bytes (1.5 GiB) with `maximumArtifactBytes` 512 MiB; `LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json` gives `maximumPoolBytes` 939,524,096 (896 MiB), `maximumNamespaceBytes` 512 MiB, `minimumFreeBytes` 1 GiB, `capacityWaitMilliseconds` 900000. The pool at 677 MB sits at 75 % of its 896 MiB cap.
- The toolkit store: 149 identities against the tools partition `.cache/b/t` (`capacityBytes` 8 GiB, `maximumLogicalBytes` 64 GiB). About 29 GB allocated exceeds the 8 GiB capacity. Either retention is not bounding the store or APFS clones hide the cost. Not resolved here (needs `repo cache status`).
- Across checkouts: no sharing unless a remote Nx cache is configured. Whether the default contract enables one was not checked.
- Every namespace is a funded native child with its own `native/` tree and `.native-birth-*.jsonl` journal (`ls .cache/b/n/m/*`). Namespace churn therefore also costs child creation and mount, not only graph derivation.

## 4. Named inputs width

- Target inputs (`LIB/🟨️.mjs`, `projectBuildTargetForNxV1` at line 85 and the per-project map at line 95): non-noop targets get `inputs: [...(properties.inputs ?? ["default","^default"]), engineNamedInput]`. Each project gets `namedInputs[engineNamedInput] = engine`. `default` is `{projectRoot}/**/*` plus `sharedGlobals`. A target without a declared closure therefore hashes its whole project root and every dependency root (`^default`).
- Engine closure (`nxBuildEngineClosureV1`, `LIB/🟨️.mjs:46`): 1,233 files, 2 packages. Breakdown from `C/aud-nx-g-tools/engine-closure-count.mjs`: 413 under `🏃️process`, 132 under `⚡️caching/🚀️bootstrap`, 85 under `🏗️build/🕸️graph`, 76 under `⚡️caching/💾️storage`, 47 under `🏗️build/🔧️tools/🏗️bazel` (including `🌡️host-capacity`, `🎬️executor`, `🌐️platform`, `🌐️remote-cache`), 3 under `🏗️build/📥️source/🔏️closure/🗓️scheduling`, 2 under `🏗️build/🧹️cache-gc`.
- The engine is one runtime input (`nxBuildEngineDigestV1`, sha256 over current bytes) and it is in every target's inputs. A byte change in any of the 1,233 files re-keys all Nx tasks of the workspace.
- `nx.json` `namedInputs.schedulingSourceExclusions` (4.2 KB, prefixes such as `🏃️process/🏗️build/🌡️host-capacity/**`) is defined at `nx.json:44` and referenced by no named input or target of `nx.json`, and by no `project.json`. The Bazel and native route uses the same policy from `LIB/🏗️build/🔣️policy.json`. So the Nx engine closure does not honour the scheduling exclusions that exec-f relied on for action keys.
- Per-command widths (result-nx4-d, measured by `closure-product.ts`): print compile script 1,730 to 427 files; fonts/toolchain 2,356 to 387; bundle 2,355 to 386; tests 405 to 0 (narrowing done at source). Still wide: `@semio-tech/ui-styling-tokens:generate` 2,444 files (413 tests) because its router script imports the union of all its commands.
- `commandSources` groups from the repo's own inference (`LIB/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:333-473`) list 2,800-3,000 paths each, about 430 KB of instruction per task (result-nx2-d). The Nx policy allows 4 MiB per instruction and 256 MiB aggregate (`LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🧬️schema/🔣️.json`).
- Width is also a time cost: each file in each task's closure is hashed on every run that reaches the hasher.

## 5. The nx@23.2.0 patch (117 hunks)

File: `LIB/⚡️caching/🩹️patches/nx@23.2.0.patch`. 139,811 bytes, 2,173 lines, 117 hunks, 36 files under `dist/` (compiled JS; no source-level diff). +795 / -405 changed lines (headers excluded). Applied by Bun at install through `patchedDependencies` (`LIB/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts`, `nxToolingInstallFilesV1`); the patch file is copied to `patches/<sha256>.patch`, so its bytes are part of the toolkit identity. `assertToolingPatchV1` (`LIB/⚡️caching/🚀️bootstrap/🛠️tools/🩹️patch/🟦️.ts`) checks hunk counts and paths only.

Largest files (hunks, +added, -removed):

| file (under `dist/src/`) | hunks | +/- | change |
|---|---|---|---|
| `daemon/client/client.js` | 17 | +110/-25 | `getSemioFilesystemJournalV1`: RPC to the repo membership journal (`GET_SEMIO_FILESYSTEM_JOURNAL`), budgets, abort |
| `daemon/server/project-graph-incremental-recomputation.js` | 15 | +43/-17 | recomputation fed by the repo journal |
| `project-graph/nx-deps-cache.js` | 8 | +57/-141 | replaces Nx's deps cache with repo-owned storage |
| `project-graph/project-graph.js` | 8 | +14/-21 | graph assembly |
| `daemon/server/server.js` | 7 | +28/-3 | server lifetime hooks |
| `tasks-runner/cache.js` | 5 | +47/-18 | output custody on put and get (`withOutputCustodyV1` path) |
| `command-line/graph/graph.js` | 5 | +8/-7 | graph command |
| `daemon/client/daemon-environment.js` | 4 | +45/-8 | daemon admission environment (`SEMIO_NX_*`) |
| `daemon/server/watcher.js` | 3 | +113/-3 | journal-fed file watcher; Nx's native watcher is kept |
| `daemon/server/shutdown-utils.js` | 3 | +25/-72 | drain and shutdown rewrite |
| `utils/handle-import.js` | 1 | +51/-0 | loads repo-owned authorizer modules |
| `project-graph/build-project-graph.js` | 1 | +66/-44 | projection |

Other files: `hasher/native-task-hasher-impl.js` (engine digest), `tasks-runner/task-orchestrator.js`, `run-command.js`, `create-task-graph.js`, `batch-process.js`, `daemon/logger.js`, `daemon/tmp-dir.js`, `utils/workspace-context.js`, `utils/db-connection.js`, `utils/cache-directory.js`, three `bin/*.js` and `.d.ts` files. Debug lines (`[DEBUG]`) were removed (0 remain, verified by grep).

Risks:
- Every upstream bump needs a hand re-port of 117 hunks against compiled output.
- The patch is part of the toolkit identity: each edit creates a new toolkit (about 195 MB), a cold namespace and a fresh daemon (sections 1 and 3).
- The patch copies repo logic into Nx internals (journal RPC, output custody, deps cache). Nx's native watcher still runs next to the journal (problem 10).

## 6. Where Nx and Bazel duplicate machinery

| concern | Nx route | Bazel route | assessment |
|---|---|---|---|
| bounded heavy-process pool | daemon slots `nx:daemon-slot:*` (2 finite, 6 watch) in `🏁️startup/🚏️admission` | `🔧️tools/🏗️bazel/🧵️sessions` (servers, service and growth slots) | two slot allocators with different rules |
| idle exit and heap cap | daemon idle 600 s (patched), no heap cap | `--max_idle_secs`, `-Xmx` (`bazelSessionStartupV1`) | same job, different guarantees |
| capacity admission | generic `run` row (1,536 MB), no Nx rows | `🏗️build/🌡️host-capacity` plus a 3.7 KB Bazel wrapper | one core, Nx has no rows |
| graph store | `🔌️nx/📦️graph` pool (896 MiB) and partition `nx` (1.5 GiB) | repo graph partition `graph` (6 GiB) and generations | two stores for graph material |
| output promotion and put slots | `🔌️nx/📦️outputs/` plus `🎟️put-slots` (4 slots) | repo artifact cohorts under `📦️artifacts/📋️native-orchestration` | parallel implementations of cohort custody |
| garbage collection and retention | `⚡️caching/🧹️pruning` (109 KB), `🏗️build/🧹️cache-gc` (64 KB), `⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention` (147 KB) | same three | three retention implementations |
| file change tracking | patched Nx native watcher plus repo membership journal worker (`🕸️graph/🔏️seal/📥️membership/📥️journal`) | journal only | two trackers in the Nx daemon |
| warm processes | Nx daemon | `🏗️build/🏡️resident` warm pool | two warm-process designs |
| source closures | `commandSources` from project-assembly | same | shared, good |
| graph loading | `loadBuildGraphV1` then Nx derivation again | `loadBuildGraphV1` | Nx adds a second derivation |
| parity | `🏗️build/🪞️parity` compares both routes | | by design |

## 7. Top 10 problems for multi-agent use on 16 GB (ranked)

1. Engine closure keys every Nx target. 1,233 files including 47 Bazel, 3 scheduling and 2 cache-gc files; a byte change in any of them re-keys all Nx tasks of all projects. `nx.json` `schedulingSourceExclusions` is not consumed. Fix (one line): in `nxBuildEngineClosureV1` drop the `🔧️tools/🏗️bazel`, `🗓️scheduling` and `🧹️cache-gc` prefixes (or key the engine per target family), after checking with the closure law that no Nx-invoked command loads them.
2. Duplicate Nx graph per repo generation. Namespace key is the pinned digest (script.ts:448), so each new generation and each new digest starts cold (7 namespaces on disk). Fix (one line): key the Nx namespace on (toolkit, workspace, implementation) and stamp the Nx graph with the repo digest, so an unchanged task slice reuses it.
3. Daemon memory invisible to admission and no heap cap. One daemon measured at 876 MB detached; the policy's 2 GB per daemon only sizes the slot count. Fix (one line): hold a 2048 MB host-capacity lease from daemon start to terminal drain (`nxDaemonAdmissionV1` plus the release in the drain path) and pass a heap cap in the daemon environment (`--max-old-space-size`, as Bazel does with `-Xmx`).
4. Superseded namespaces hold daemon slots for up to 10 min, so a third namespace on a checkout runs daemonless and rebuilds the Nx graph in-process. Fix (one line): treat a daemon whose namespace is superseded by a newer digest as idle in `nxDaemonIdleV1`.
5. The governor cannot price Nx runs. The 16 GB floor is 4 GiB (policy: 20 % of RAM clamped to [4, 8] GiB; the 6.5 GB quoted in the brief and result files is the 32 GB value), and the Nx run is charged the generic `run` row of 1,536 MB, while the brief records Nx runs admitted at 3.5 to 5.6 GB each. Fix (one line): add `nx-run` (about 4096 MB) and `nx-graph` rows to `LIB/🏗️build/🔣️policy.json` and check the floor at 16 GB.
6. Two caps for one Nx store: partition `nx` 1.5 GiB against pool 896 MiB, with 677 MB used and 15-minute capacity waits. Fix (one line): derive `maximumPoolBytes` from `repositoryCacheBudgetV1().partitions.nx.capacityBytes`.
7. Toolkit identities accumulate: 149 identities (about 195 MB each) against an 8 GiB tools partition, and every patch edit creates one. Fix (one line): keep the two newest toolkit identities plus any with a live daemon in `🧹️retention`.
8. The graph-facts writer is exclusive for every graph-consuming run, so runs queue behind one another even when the generation is sealed (nx-f: 8 runs behind a 2,502 s holder). Fix (one line): take `repository-graph-facts:<root>:shard:7` shared when the pinned generation is sealed and wait only for an unsealed build (the graph-f phase-2 rule, applied to the Nx path).
9. Wide inputs. Targets without a declared closure fall back to `[default, ^default]` (whole dependency roots); router scripts union all commands (`ui-styling-tokens:generate`: 2,444 files); `commandSources` groups list 2,800-3,000 paths. Fix (one line per router): give each command its own script entry so its closure is per command (the next step nx4-d names), and make the fallback `inputs` the declared closure only.
10. Two change trackers and full-tree walks. The Nx watcher is patched (+113 lines in `watcher.js`) but not removed, next to the repo membership journal; the non-pinned path walks the whole tree per run (`nxGraphSourceMembershipV2`). Fix (one line): when the journal service is live, start the Nx daemon with its native watcher disabled and consume only the journal RPC.

Honourable mentions (not in the top 10): the 117-hunk patch over compiled `dist/` (section 5) and the retry-on-churn path that forces a full graph recompute up to three times (section 2). Both would move up if the brief's churn target gets tested under load.

## 8. Brief targets the Nx route cannot meet by design (or meets only through the repo)

- No-op under 10 s and under 1.5 GB for the whole tree. Cannot be met with a daemon: the daemon alone measured 876 MB and is allowed 2 GB, and the loader and the Nx child add to it. Time is achievable through the reuse path (section 2, lines 525-532), but the RSS target is at risk by design.
- Graph generation after a peer edit, only affected owners re-inferred, under 2 min and under 2 GB. The repo graph can do the affected-owner part. The Nx route cannot: each new digest re-derives the whole task-slice Nx graph and starts a new namespace, so nothing in Nx is incremental per owner. By design it adds a second full derivation per generation.
- Peer edits to unrelated files, including scheduling code, cause 0 re-executions. Fails today (problem 1). It can be met after narrowing the engine closure, except for files that really are inside a product's own closure.
- No failed run during peer churn. Nx has no snapshot isolation of its own. The repo wrapper (`Taking a new snapshot`, exit 75 after 3 attempts) is the only protection, and under constant edits it fails (nx3-d). This part cannot be met by the Nx route alone.
- Three agents on 16 GB with fair queueing and no swap storm. The Nx route cannot meet this as built: daemon memory is outside admission and daemon slots are per checkout, not per agent (problems 3, 4, 5).
- Dashboard one-line edit under 2 min. Cargo-bound, not Nx-bound. The only Nx dashboard build on record (nx3-d, dash5) spent 8 min 46 s in compile. Not measured warm; cannot be judged from these files.
- Disk under a declared budget. Toolkits and namespaces are outside the per-identity budget (problem 7); fixable only by retention.

## 9. Not verified (no build, no Nx run allowed)

- Daemon RSS over a long run and under load; only one live sample (876 MB).
- The claim that different product selections produce different digests and therefore different namespaces. Inferred from `withRepositoryBuildGraphArtifactV1` (script.ts:409) and the namespace key (script.ts:448); not checked against `.cache/b/n/m/*` metadata.
- Retention behaviour of `.cache/b/t` (149 identities against an 8 GiB cap); whether APFS clones hide the allocation.
- Nx graph derivation time per digest for the 550-project print slice. Only sizes are on record.
- Whether a remote Nx cache is part of the default contract.

## 10. Evidence index

- Daemon: `LIB/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🟦️.ts`, `…/🚏️admission/🧬️schema/🔣️.json`, `…/🧬️lifetime/🟦️.ts`, `…/🚦️service/🟦️.ts`; `LIB/⚡️caching/🚀️bootstrap/📜️script.ts` (lines 249-251, 257-258, 404-409, 448-449, 525-532, 560-566).
- Engine and inputs: `LIB/🟨️.mjs` (lines 46, 67, 71-72, 85-95); `nx.json` (namedInputs, line 44).
- Graph pool and bounds: `LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json`; `LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🧬️schema/🔣️.json`; `LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts` (lines 59-69, 84).
- Budget and capacity: `LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`; `LIB/🏗️build/🔣️policy.json` (`hostCapacity`); `LIB/🏗️build/🌡️host-capacity/🟦️.ts`.
- Patch: `LIB/⚡️caching/🩹️patches/nx@23.2.0.patch`; `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts` (`nxToolingInstallFilesV1`, line 128-134 region); `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🩹️patch/🟦️.ts`.
- Bazel comparison: `LIB/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts` (lines 142-143), `…/🧵️sessions/🧬️schema/🔣️.json` (heapMegabytes 2048).
- Probes (read-only): `C/aud-nx-g-tools/engine-closure-probe.mjs`, `C/aud-nx-g-tools/engine-closure-count.mjs`, `C/aud-nx-g-tools/engine-closure-list.mjs`.
- Field results: `C/result-nx2-d.md` (graph sizes, fault P, Q, N), `C/result-nx3-d.md` (daemon slot race, exit 75 churn, drain hang), `C/result-nx4-d.md` (closure narrowing numbers, `ui-styling-tokens:generate`), `C/result-nx-f.md` (writer-lease queue, 2,502 s holder).

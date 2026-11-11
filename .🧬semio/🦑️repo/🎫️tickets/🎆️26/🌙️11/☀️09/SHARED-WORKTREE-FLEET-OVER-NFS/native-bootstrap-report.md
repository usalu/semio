# Native bootstrap on a fresh machine

## What the provision scripts do now

Both scripts (shell and PowerShell) finish the fleet setup first — mounts, limits, environment, time sync, join, daemon, compaction timer, relay and cache on servers, all through the standalone launcher — and run the native bootstrap as the last step. A failure there does not undo or fail the fleet setup.

| Exit code | Meaning |
|---|---|
| 0 | fleet ready, native build complete |
| 1 | error, nothing after the failing step ran (usage errors 2) |
| 3 | fleet ready, native build pending: marker `.cache/f/native-bootstrap.done` absent, log in `.cache/f/native-bootstrap.log`, next run retries |

`preflight` has a `native-build` check: `ok` with the marker, a warning without it.

Reason: a fix for a broken build arrives through the shared worktree, so a machine must already be syncing when its native build fails.

## Executed evidence

- Topology suite 67 pass (shell-level test with a stubbed failing `repo`, PowerShell mock harness branch), shellcheck clean, `bash -n` clean.
- Fresh Ubuntu 24.04 arm64 VM (Lima, vz), worker of g1 over kernel NFS, provisioned with the final script: `machine.json`, `state.json` (270k files) and `daemon.lock` present, `semio-fleet-replica@semio.service` active and running, `semio-fleet-compact@semio.timer` waiting, marker absent, log written.

## Root cause of the native failure (reproduced 2026-11-10, fresh Ubuntu 24.04 arm64 Lima VM, 4 vCPU, 8 GiB, tree = files not ignored by the root `.gitignore`, 269,395 files, on the VM's own disk, `SEMIO_LANGUAGE=en` as the provision script sets it)

Not one defect but a chain; each one hid the next. The resident service log never showed any of them because `runHostForegroundV1` is started without `log` (`📚️library/🏗️build/⌨️command/🌐️host/🟦️.ts:25`, default `log=()=>{}` at `🏡️resident/🏗️composition/🌐️host/🟨️.mjs:15`), so `plan-failed` (`🏡️resident/🏛️service/🌐️host/🟨️.mjs:144`) is dropped although the client says "the details are in its log". For the diagnosis a `log` writing to stderr was patched into the VM copy only; not fixed in the repository (open, one line: pass a `log` that writes to the service log).

| # | Symptom | Cause | State |
|---|---|---|---|
| 1 | `Cannot find module 'typescript'` | the bootstrap ran `workspace:setup` without installing the dependency scopes; `node_modules` is ignored and never replicated, the planner imports `typescript` from the installed generation. CI runs `repo dependencies sync --all` before anything else. | fixed earlier the same day: `🐚️.sh` `repo_bootstrap` runs `repo dependencies sync --all` before `workspace:setup` (about 35 min on 4 vCPU; plan generation needs it). Verified: no `typescript` error any more. |
| 2 | `Original captured runtime entry provider required` (`🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts:34`, called from `🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:178`, branch `current !== "mounted"`) | The planner of the resident host loads the graph in a slice worker process (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts`). Every other process root that touches the native-build storage wraps its body in `withRepositoryRuntimeEntriesV1` or `withRepositoryCompilerRuntimeEntriesV1` (command entry, graph publication, Nx daemon service); the slice worker did not. On a machine whose fixed native storage is already mounted the branch is not taken, so the missing provider never showed (warm ignored state). On a fresh machine the storage is prepared inside the worker and needs the captured helper. | **Fixed**: `👷️worker.ts` loads the graph inside `withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control,…)` (the compiler role: the worker is not started from a captured loader, so the admitted role fails with `Actual admitted Source resolution map required`, seen in the VM). Regression test `🏡️resident/🧪️tests/👷️slice-worker-scope/🟦️.ts` (passes). In the VM the failure moved past this point. |
| 3 | `plan-failed` with a bare `Error` at `🕸️graph/📥️inference/🔎️owner-discovery/🕸️wasm-outputs/🟦️.ts:18` (`packageContract`, message "Physical authored Wasm package metadata required") | `wasmCompileOutputIndexV1` (new in commit 876c3b15944, 2026-11-10) throws for the whole graph when one declared Wasm compile output has no authored `package.json` named like its specifier. The project `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust` declares `@semio-tech/flow-core` (outputs under the ignored `🕸️bindings/`), but its own `package.json` is `@semio-tech/flow-core-build` and no authored `@semio-tech/flow-core` manifest exists anywhere in the tree. Independent of the machine: the same call fails on the host repo (`wasmCompileOutputIndexV1` over the ten `wasmPackageSourceV1` owners: only this one throws). It is reached only when a script import resolves to a generated path, which is why a warm tree can avoid it. | **Fixed 2026-11-11** (see Update below): the manifest is not missing, it is published too late. In the VM it had been bypassed locally to see what follows. |
| 4 | `Slice worker exited with code null` | OOM kill (`dmesg`: `Out of memory: Killed process … bun … anon-rss:6469840kB`): the slice worker grows from 0.6 GB to 5.9 GB in 15 s while the cold inference of all 4,754 projects runs (the governor logs `no memory peak learned … 256 MB assumed`, and no cgroup limit applies on this VM, so the limiter is silently absent: `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs:49`, `limiter.create(...).catch(()=>undefined)`). The repo README documents it: "A cold graph is the slowest command ... bound by garbage collection of a 7 to 8 GB heap" (README line 547 of the repo docs). | **Mitigated 2026-11-11, not fixed at the root** (see Update below): a fresh machine has no graph memo (`.cache` is ignored and not replicated), so `workspace:setup` needs a cold all-workspace inference with a 7 to 8 GB heap; it does not fit an 8 GiB machine with a 6 GB resident limit, and at the documented 6 to 33 projects per minute it takes hours even where it fits. |

Consequence for the fleet: `repo` itself works natively (`repo-mcp:install` and `repo --help` pass in the VM), but the last bootstrap step `repo run workspace:setup` cannot finish on a fresh machine of 8 GiB, and the marker `.cache/f/native-bootstrap.done` is not written (exit code 3 path of the provision script, as designed). Not proven in this run: the bootstrap to completion, because of 3 and 4. Decisions needed from the build owners: (a) fix or relax 3; (b) either give fleet machines at least 16 GiB (and measure the cold graph there) or let them take the graph memo from a peer (cache service of the group server) instead of inferring it; (c) decide whether `workspace:setup` (all dependency scopes, browsers, cargo, dotnet, wasm) is the right "native build complete" for a worker.

Reproduction of 3 on any tree: `bun <script>` that imports `wasmCompileOutputIndexV1` from `…/🕸️wasm-outputs/🟦️.ts` and calls it with owner `{name, root, manifest}` of the flow project above and a `read` that returns file contents or `undefined`: throws at `🟦️.ts:18`.

Not verified: the same steps on a fresh macOS or Windows machine.


## Update 2026-11-11: A, B and the cache path fixed, the cold graph still does not fit

### A. The Wasm index: the manifest is a published bootstrap source, the graph loaded before it was published

`@semio-tech/flow-core` is not an authored manifest and must not become one: `🎁️browser-package/🕸️bindings/package.json` is the static workspace manifest that `bootstrapFlowCorePackage` writes from `flowBrowserPackageManifestV1` (declared in `🏗️build/🚀️bootstrap/🌱️sources/🔣️.json`, "required by `bun install`"; the directory is ignored and generated, so no authored file can live there, and the index is right to demand a physical manifest as the input of the edge). The declared contract of `publishBuildBootstrapSourcesV1` is "before any dependency installation or graph import". Two entries broke it: the slice worker of the resident host loaded the graph first (a task run publishes only later, in `ensureTaskDependenciesV1`), and `repo dependencies sync` (mode sync) never published, so a fresh machine had neither the manifest nor the workspace link. Fix: `👷️worker.ts` publishes the bootstrap sources before `loadBuildInvocationGraphV1`, `SyncScript.run` before the installation. Tests: `🕸️wasm-outputs/🧪️tests/🌱️flow-manifest/🟦️.ts` (the flow index throws "Physical authored Wasm package metadata required" on a tree without the manifest and indexes its three specifiers once `bootstrapFlowCorePackage` ran; the order of both entries). Not verified end to end: the VM never got past the cold graph, so the first successful plan on a fresh machine is still unseen.

### B. The host log

`runHostForegroundV1` now gets `log:writeHostServiceLogV1` (stderr, redirected to the service log by the detached start). Test `🏡️resident/🧪️tests/🪵️service-log/🟦️.ts`. Seen live: the VM's `host-*.log` carried the governor and graph lines.

### C. Why a fresh machine never used a cache, and what exists now

1. The remote cache of the build tools carries Bazel and Nx task results only. The project graph (`.cache/b/g`, generation documents) had no remote layer: nothing was ever uploaded, so there was nothing to fetch.
2. The provision script ran the native bootstrap through `sudo -u <account> env ...`, a non-login shell: the `SEMIO_REMOTE_CACHE_*` variables of step 5 (profile files) and the token (`~/.profile`) never reached it. Measured: before the change the bootstrap had no cache variables; after it, the process shows the four (endpoint, access, instance, token) and the server counted the graph lookup (`ac.misses: 1` in `/status`). Now the script writes them to a `0600` file in `.cache/f` that the bootstrap sources and the script removes (never in argv).
3. New: graph exchange (`🕸️graph/🕰️generations/🌐️remote-exchange`, schema beside it, README "Generations over the remote cache"). Generation documents are named by the sha256 of their bytes, so they are CAS blobs; one action result under `sha256([domain, registry identity, workspace root])` lists them and carries a note. Builder: upload after publishing; any client with write access offers a served generation once per process if the cache has none; never replaces a larger entry. Empty store: download inside the builder custody before inferring, verified against names and note, pointer last; the normal fact verification (stamps are hints, sha256 decides) leaves only differing projects to infer. Failures only print a message; after the first one a process stops asking. Tests: 4 in `🌐️remote-exchange/🧪️tests` (real cache server, token-less) incl. the real `loadGenerationGraphV1` with a wiped store and edited inputs, 1 in `🔌️load/🧪️tests`.
4. Memory: provisioning gives Linux machines swap up to 16 GiB of RAM plus swap (`ensure_swap`, step 4; 8461 MB on the 8 GiB VM), so a cold build is slow instead of killed. Tests in the topology suite (69 pass, shellcheck clean). **Not done: bounded slices of the cold inference.** `discoverPart` is not chunkable without restructuring the environment part of a generation, and a measurement (below) shows the live set, not garbage, is the problem.

### Measured (Lima vz, Ubuntu 24.04 arm64, host 16 GB, 12 cores)

| What | Result |
|---|---|
| Server VM 8 vCPU, 11 GiB RAM, 16 GiB swap, `provision --server --first-machine` | fleet ready after about 3 min (seed of 269,401 files, cache and relay units), then toolchain and dependencies in about 8 min, then the cold graph load; I stopped the graph worker by its pid after 31 min 37 s in total (exit 3, the fleet stays ready) |
| Cold graph of 4,754 projects on it | 7 projects in about 20 min (one at a time, "no memory peak learned", headroom 0 MB); slice worker 9.4 GB resident plus 8.1 GB swap (18.2 GB virtual), 78 CPU minutes on 5 cores, no progress between minute 7 and 20: at this rate about 160 hours; the warm-up cannot finish on this host |
| Worker VM 4 vCPU, 8 GiB, empty tree | joined in about 2 min, swap file 8461 MB created, `native-environment` mode 600 with 4 variables; the native bootstrap was stopped while installing dependencies |
| Exchange across the two VMs (cache server of the group with tokens, real generation engine, synthetic workspace of 1,500 projects, tree copied by tar, different inodes) | server: cold build 1.0 s, upload 1,504 documents (1 MB) in 1 s. Worker with an empty store and 5 edited files: restored 1,504 documents, 5 projects inferred again, 1.17 s wall, peak RSS 153 MB. Worker with a wiped store and no edits: 0 inferred, 0.62 s, peak RSS 121 MB. Server counters: ac hits 3, misses 2, puts 2; cas hits 4,504, puts 1,512 |

### Still open / unproven

- The warm-up of the real graph on a group server and a fresh 8 GiB worker that completes the native bootstrap with the marker `.cache/f/native-bootstrap.done`: not reached, because no machine here can build the cold graph. The exchange is proven on a synthetic workspace through the real engine and the real cache server and clients on two VMs, not on the real generation (size, row mix, the first real fact verification with moved stamps).
- The cold graph itself: the live heap of the all-workspace inference exceeds 16 GB on this host (README says 7 to 8 GB). A real bound needs an owner-sliced inference with a release between slices, or a smaller live set; until then the first generation of a group has to be built on a machine with enough memory (or the server needs about 32 GB and many hours) and uploaded by it.
- A generation key contains the absolute workspace root: every machine of a group must use the canonical worktree path.
- Windows: the PowerShell script already exports the cache variables into its own session; unchanged and not run.

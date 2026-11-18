# R169: the disk wall of g1-m01 (97.1 % used) and one disk budget per machine (living report)

Wall (read-only, `/Users/Shared/semio-store/v1/origins/g1-m01/ready-log-workspace-failed.txt`): 00:51 UTC "Partition / is 97.1% used ... only 6.77 GiB free"; at 01:21 UTC the same text with **97.7 % and 5.32 GiB** (attempt 67 of stage `workspace`, `ready.json` state `backoff`): the machine (Mac mini, 16 GB, 256 GB disk, about 231 GiB volume) gets worse while it retries. The store holds no disk facts of g1-m01 (no free bytes, no sizes of any root); the refusal text is the only disk evidence it publishes.

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R169/` (`land.py`, `patches.json` anchored hunks, `stage.py`, `preload.ts` bun plugin that serves the staged files, `run.sh`). Staged copies and measurements live in the scratch directory of the session, nothing was deleted anywhere (all measurements read-only, `nice du`, `ls -s`, `df`, `hdiutil imageinfo`).

## 1. What the gate does today, why it freed too little, what is not eligible

Chain: `withBuildCapacityV1` (heavy commands only) -> `admitBuildDiskV1` (`🌡️host-capacity/💽️disk`) -> `createCasGuardV1.admit` -> `decideCasEnforcementV1` with the declared shares `partition.high 0.90 / low 0.80 / critical 0.97` of `🗄️cas/🧬️schema` -> `collectCasV1`.

- "freeing the least recently used cache objects" = `collectCasV1` over exactly **three things**: the host CAS (`~/.cache/semio/cas`, cap 32 GiB), the graph-memo directories of the user's worktrees (cap 4 GiB, 7 days) and the build-input closures (cap 8 GiB, 7 days). It evicts least recently accessed objects older than `minAgeMs` 10 minutes, not pinned, not leased, down to the low share 0.80 (it is told the partition, so it aims to free `used - 0.80 x size`).
- Single flight and "collected once": after one collection (`state.collectedAt` within `minAgeMs`) the guard decides with `collected: true`, so at >= 97 % it **refuses at once** instead of trying anything else. A machine whose CAS is small (a worker that never served much) frees a few hundred MiB and refuses.
- The refusal names `repo cache cas gc`, which collects only that CAS. `repo cache gc` (ledger areas + host store) is the bigger command and was not named.
- The 97 % share is a **fixed percentage**: on a 256 GB disk it means 7.7 GB free, on a 4 TB disk 120 GB. A Cargo build declares 4 GiB (`EXPECTED_GROWTH_BYTES_V1`), the `dependencies` stage 6 GiB (R146 `diskRequirements`): the gate refused a 6.77 GiB machine for a command that needs 6 GiB, and admitted nothing for a 4 TB disk until it was 120 GB short.
- NOT eligible for this collection (and never touched by it): the images in `.cache/b` and everything inside them (the guard does not know them), `.cache/f` (waves, `lost`, graph state, cache server), dependency generations, TMPDIR, cargo units, closure store layers, ticket `generated` folders, the leaked staging directories and sandbox images of R146/R123, the replica store.
- The biggest single fact (finding F1, measured on this Mac): **the fixed images never give blocks back**. `fixed.dmg` is UDRW; `hdiutil compact` does not apply; freed blocks inside are reused but the host allocation only grows. Here the 8 images hold 86.1 GiB on the host and 29.1 GiB of live data: **57 GiB (66 %) is slack no collector returns**. The machine budget (`🎚️machine`) already counts `allocatedBytes` (`hostAllocationV1`) but only reports it; no collector acts on it.

## 2. Inventory of everything the build system writes (this Mac as the model: 926 GiB volume, 47 GiB free, 95 % used)

Sizes: host-allocated bytes unless "inside" is said. `gate` = whether the 97 % gate (old) triggers a collector for it. New = after landing, see section 3.

| Root | Size here | Growth mechanism | Bound | Collector | Old gate | New |
|---|---|---|---|---|---|---|
| `.cache/b/r` image (rust: unit cache `u`, cargo store, Nx artifacts) | 45.1 GiB allocated, 10.4 inside (`u` 8.9, cargo 1.3) | cargo units, incremental sessions, Nx task outputs | `u` 24 GiB, cargo-home 3 GiB, ledger | `repo cache gc` areas `unit-cache`, `cargo-store` (delegate `cargoIncrementalReclaim`) | no | `cargo-incremental` (writes 10, image relief only) |
| `.cache/b/d` image (dependencies, 12 generations of ~1.5 GiB du) | 28.2 GiB allocated, 13.2 inside | one generation per lock/package change (R152: 12 local) | retention grace 6 h, rollback 24 h, `maximumCollectedPerRun` 16, **only runs in dependency admission when the image has < 25 % free** (here 37 % free, so never) | `collectJavascriptDependencyGenerationsV1` | no | `dependency-generations` (50) |
| `.cache/b/t` image (tools: native 1.2, `r` 0.85, nx 0.3, bazel, sdk, downloads) | 6.8 GiB, 3.1 inside | pinned tool installs, SDK derivatives | ledger `nx-tooling` 2 GiB (soft 2.5 GiB free), others by retirement | `nxToolingReclaim`, `repo cache gc` | no | `tool-generations` (20) |
| `.cache/b/g` image (graph: inference 1.8, closures) | 5.3 GiB, 1.9 inside | optional graph facts, closure stores | `graph-inference` 2 GiB, closures 1.25 GiB | `reclaimGraphOptionalV1` (soft 1.75 GiB free) | no | `closure-layers` (30) |
| `.cache/b/n`, `o`, `l`, `p` images | 1.0, 0.1, 0.1, 0.1 GiB | Nx namespaces, diagnostics, leases | 1.5 GiB / 2 GiB / 64 MiB | `retireNxGraphNamespaceV1`, test-runs reclaim | no | via ledger |
| **image slack** (all 8 images) | **57 GiB** | blocks freed inside are never returned to the host | none | **none** (manual detach + recreate of an idle recomputable image) | no | registered as `image-slack` (manual, lists the command) |
| `~/.cache/semio/cas` (host CAS) + graph memo + closures areas | 2.5 GiB | content-addressed adoption of bun/tool downloads | cap 32 GiB, areas 4 + 8 GiB | `collectCasV1` | **yes (only this)** | `cas-lru` (70, last) |
| `.cache/f/cache-server` | 1.27 GiB | remote cache store | FX-12 2.5 GiB | own LRU | no | outside; listed by size only |
| `.cache/f/lost` (files/stages the replica overwrote; 72 dirs + a 755 MiB parked `old-repo-g1-server`) | 0.88 GiB | one directory per overwrite, **no retention** (only `index-*` stages are capped, `KEPT_STAGES`) | **none** | **none** | no | `replica-lost`: 30 days, newest 16 stay (`repo cache gc`) |
| `.cache/f/owner-pool-waves` | 0.13 GiB (8-10 dirs, 6-10 MiB each) | kept wave per pool start; the runner removes only dirs outside `kept` | **none** (R146 swept `semio-owner-pool-*`, not these) | **none** | no | `stale-waves`: 24 h without a write, newest 8 stay |
| other `.cache/f` (graph-state 48 MiB, remote-execution 34, graph-server 12, sidecars 11, fetch-verified 3.5) | 0.1 GiB | fleet state | own limits (remote-execution 512 MiB, remote-cache 3 GiB ledger areas) | own | no | outside |
| `.🧬semio/🦑️repo/⚡️cache` loose ledger root (`tools` 3.3 GiB incl. playwright/elan, `🗃️bin`, `implementation`, `🎛️dashboard`) | 3.5 GiB | pinned downloads, built binaries, sealed implementations | 25 GiB root, per area (playwright 2 GiB/90 d ...) | `repo cache gc` | no | `compile-caches` (62, spawn of `repo cache gc`) |
| `.cache/scratch`, `test-sandbox`, `boot-recovery`, `test-admission` | 18 MiB | per-process scratch, sandbox checkouts | ledger (512 / 32 / 16 MiB) | `repo cache gc` | no | via ledger |
| TMPDIR (`/var/folders/.../T`), 89 `semio-*` entries | 6.9 GiB total, `semio-*` 1.67 GiB | farm sessions, graph-farm dirs (113 MiB each), warm dirs, owner-pool staging, closure/receipt dirs | OS purge only (macOS, 3 days idle); R146 sweep only for `semio-owner-pool-*` at pool start | R146 sweep; `semio-read-verdicts-<uid>` prunes its own old entries **only when a new read round starts** (2 h) | no | `leaked-staging`: dead-pid names after 1 h, pid-less after 7 days (60) |
| `$TMPDIR/semio-read-verdicts-501` | 0.81 GiB (46 round dirs) | verdict files per validation round | 2 h, lazily | lazy only | no | `read-verdicts` (42, 2 h) |
| `$TMPDIR/semio-footprint` | 25 MiB, one `<pid>.log` per process | footprint trace per process, capped per file (`traceBytes`) but **count unbounded** | per file only | **none** | no | `footprint-logs`: newest 256, none older than 3 days (41) |
| test sandbox images (`.cache/test-sandbox/r*`, `gf-*` nested images) | 0 now | `hdiutil attach` per private test repository | gate maximum 32 attaches | R123 sweep (by owner pid + start) | no | `leaked-images` (61) |
| non-semio test leftovers in TMPDIR (`r121au-*` 0.7 GiB, `uf-mtime-*` 0.5 GiB, `tmp.*`) | ~1.5 GiB | agents and test tools outside the product | none | none (agents, not the product) | no | not registered (not ours) |
| ticket `🗑️generated` folders (13 of them) | **14.3 GiB**: DEMONSTRATOR 08/28 10.1, GET-CODEBASE 10/18 1.8, LAYOUT 09/16 1.3 | per-run nx workspace caches and logs of agents | CLAUDE.md says delete when done | the agent (manual) | no | out of scope of the gate: these live in the replicated worktree, **find how many workers carry them** (not knowable from the store) |
| replica store `/Users/Shared/semio-store` (server role) | 81.5 GiB (blobs 59.6, bootstrap 9.6, fleet 8.0, snapshots 2.4) | content-addressed fleet blobs | **none found** | **none found** (`v1/trash` is empty) | no | out of scope here; measure on the server before the fleet grows (finding F4) |
| cargo `target/` in the checkout | none (`.r`, `target` absent; Cargo writes into the rust image) | | | | | |

What cannot be known for g1-m01 from the store: any size of any root, the allocation vs content of its images, its TMPDIR, its CAS, whether the 14 GiB of ticket `generated` folders are replicated to it (the replica ignore rules were not read), who consumed 1.45 GiB between 00:51 and 01:21. After landing, the refusal text itself names the five largest consumers by root with sizes, so the next `ready-log-workspace-failed.txt` answers this.

## 3. Fix by construction (staged, tested, lands in the 02:00 UTC window)

One disk budget per machine, derived, no fixed share:

- `🗄️cas/🧬️schema` gets the definition `Budget`: `largestNeedBytes` 6 GiB (the largest stage need of R146's table; the law test asserts it is >= every `diskRequirements` entry and >= the 4 GiB Cargo growth), `cushionDivisor` 4, `minimumTotalFactor` 2. The reserve is **largest need x the safety factor of the workspace scale family** (`📏️workspace-scale` `Family.safetyFactor` 2, read, not copied) = 12 GiB; cushion 1.5 GiB; hysteresis one largest need.
- `casPartitionBudgetV1(total)` / `casPartitionPolicyForV1(policy, partition)` (`🗄️cas/🌡️partition`): critical = 1 - cushion/size, high = 1 - reserve/size, low = 1 - (reserve + hysteresis)/size. A 238 GiB disk and a 3.7 TiB disk keep the same bytes free; a volume below 2 x (reserve + hysteresis) = 36 GiB keeps the declared shares (every existing test uses 1000 MiB volumes and is unchanged). The guard (`createCasGuardV1`) judges and collects with this policy.
- `machineDiskBudgetV1({total, free, footprint})` = `min(total, footprint + free) - reserve`: what the build system may hold, derived from the volume and what it holds now (replaces the declared 198 GiB `defaultTotalBytes` as the model; wiring it into `repo cache gc --budget` as source `derived` is **not done**).
- New member `🌡️host-capacity/🧹️disk-recovery/` (schema first: `Policy`, `Bounds`, `Writers`, `Messages` EN+DE): the **registry of 13 writers** with priority, class, relief (`host` returns bytes to the volume at once, `image` only inside an image, `none`), root, collector kind, the bound in EN/DE and the exact command. Order (ascending): cargo-incremental 10, tool-generations 20, closure-layers 30, stale-waves 40, footprint-logs 41, read-verdicts 42, dependency-generations 50, leaked-staging 60, leaked-images 61, compile-caches 62, cas-lru 70 (least recently used last), replica-lost 80, image-slack 90 (manual, last).
- `recoverDiskV1`: the gate judges `need + cushion` (must) and `need + reserve` (target). Below the target it frees the `host`-relief writers in priority order, re-reads the volume after each one (the claim of a collector is not trusted), stops as soon as the target fits, never lets a failing or slow collector (10 min budget, `AbortSignal`) stop the next, honours cancellation between collectors and reports progress lines in the language of the environment. Between cushion and reserve it runs **in the background and the command goes on**; below the cushion the command waits and, if everything automatic is spent, is refused with the typed `disk-recovery.low-space-consumers` refusal (EN/DE): volume, free, used percent, what the command needs (own need + cushion) and the **five largest consumers by root with sizes and the exact command to free each**. Wired into `admitBuildDiskV1` (`recoverBuildDiskV1`, single flight per store root, skipped in a child of the recovery via `SEMIO_DISK_RECOVERY_CHILD`).
- Real collectors (`🔌️real`): `repo cache gc` as the product's own collector (one spawn per recovery, clean environment, budget 10 min), `collectCasV1` with the derived policy, R146 `ownerPoolStagingSweepV1`, R123 `sweepLeakedSandboxImagesV1`, and the new bounds.
- `repo cache gc` now also applies the machine bounds after the host store (`collectMachineBoundsOfCommandV1`, lines `Machine bound <id>: <bytes> freed` / `würden freigegeben` on a dry run, `machineBounds` in `--json`), so the command named in the refusal really frees them.

### Laws (`🌡️host-capacity/🧹️disk-recovery/🧪️tests/🟦️.ts`, 19 tests, staged 19 pass)

- registry closed (unique ids/priorities, commands, manual last, derivable < LRU < user files < slack); every named command exists (areas are in the ledger, or `repo cache gc`, `repo cache cas gc`, `repo doctor --fix --only=host`, manual image removal);
- budget derived from the scale family and the declared stage needs; 256 GB vs 4 TB; 10 GiB volume keeps fixed shares;
- **the wall**: at 97.1 % with 6.77 GiB free the old shares refuse (asserted, this is the red-before proof: `decideCasEnforcementV1` with `CAS_POLICY_V1` answers `refuse`, the derived policy admits; a 6 GiB command still waits); the guard admits at 6.77 GiB, collects once and refuses below the cushion;
- **law with a fake facts provider**: a machine at 97.1 % with collectable artefacts admits after the collection (derivable writers run first by priority, image-internal and manual writers never run, LRU only when needed, stops when the target fits); a command with a declared need continues until need + cushion fits; a machine with nothing collectable refuses with the five largest consumers sorted, in English and German, with the exact commands;
- failure isolation (a throwing collector, a collector that never ends within its budget, cancellation between collectors, unmeasurable volume);
- gate end to end through `admitBuildDiskV1` with injected writers (admit after collection, refuse with the list, background collection between cushion and reserve);
- Python oracle (`oracle.py`, independent) agrees on the judgement and the machine budget for 8 volume cases (including the 20 GiB and 40 GiB edge cases);
- `repo cache gc` hook in both languages, dry run removes nothing, JSON carries `machineBounds`.
- Staged runs of the existing suites: cas 36 pass (incl. the Python enforcement oracle), `💽️disk` 7 pass, host-capacity 17 + quiet 3 + phases 6 + agents 2 + cargo-scaling 2 pass, cache-gc budget 7, directory 2, quota 3, verify 4, launch 9, future 3 pass.

## 4. Unbounded growers and the bound each now has

| Store | Old | New bound | Law |
|---|---|---|---|
| `.cache/f/owner-pool-waves` | none | 24 h without a write, newest 8 kept (`Bounds.waves`) | `bounds` selection test + real directory test + gc hook test (12 waves -> 8) |
| `.cache/f/lost` | none (only `index-*` capped) | 30 days, newest 16 kept (`Bounds.replicaLost`); user files, so last but one in the order | selection test (x0, x15 kept, x39 gone) |
| `$TMPDIR/semio-footprint` | per file only | newest 256 logs, none older than 3 days | 300 logs -> <= 256 |
| `$TMPDIR/semio-read-verdicts-<uid>` | lazy at next round | 2 h, also by the gate and `repo cache gc` | selection test |
| `semio-*` temporary directories | R146 only for owner-pool | made temporary directories only (`semio-<tag>-<random6>`): dead pid in the name after 1 h, pid-less after 7 days; sockets, files, foreign names and the long-lived roots (`semio-repo`, `semio-footprint`, `semio-scratch`) never | selection test with live/dead pid |
| dependency generations | image pressure only | registered at priority 50 with its own command; host pressure still reaches it only through `repo cache gc` | registry law |
| generations kept: no new N invented. The existing retention (grace 6 h, rollback 24 h) is kept; the fix is that the collection is no longer gated on the image's own 25 % free. **NOT changed in code**: the 12 local generations stay until the dependency stage runs or `repo cache gc --area=dependencies` is run | | |

Removal is confined: a bound only deletes direct children of its root whose real path stays inside the root, never follows links, refuses separators and `..` (test with a link out of the root and `../x`).

## 5. Landing log and watching g1-m01

- 01:31:40 to 01:31:46 UTC (hold 6 s): applied, the native runs under the lock were red (cas, disk, budget, quiet: `Cannot find module .../scratchpad/R169/mirror/...`), restored at once, shas of all six files equal to the base, new directory removed, lock released. Cause (mine, a test-harness fault): Bun's runtime transpiler cache (`~/Library/Caches/bun/@t@/*.pile`) kept 9 entries of my staged runs with the staged module graph's resolved scratch paths; the landed files had the same content, so the native run read the poisoned entries. The 9 entries (all and only those containing my scratch path) were deleted, every later scratch run uses `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`.

- 01:40 UTC: coordinator measurement freeze 02:05-03:30. Full battery re-run in scratch with a clean transpiler cache (0 poisoned entries, cache disabled): new 19, cas 36, disk 7, cache-gc 33, host-capacity 26, all green. Native run still unproven (only the red above), and a restore would restart the workers, so the 02:00 landing is **cancelled and queued for 03:30**: `python3 window.py` from `🧰️scripts/R169/` (needs the scratch mirror; rebuild with `mkpatch.py` first). Nothing of R169 is live.
- Task 4 (watch g1-m01) is therefore pending until after 03:30. To be filled after landing: the gate text in `ready-log-workspace*.txt`, whether `Collector ... freed ...` and `the command starts` lines appear, or the refusal with consumers.

## 6. Not verified / open

- Not run on a worker or on a machine below 12 GiB free; the real recovery (spawn of `repo cache gc` from inside the gate, CAS collection with the derived policy) is only covered by injected-writer laws. The dynamic collectors (`runLedgerCollectorV1`, `sweepLeakedSandboxImagesV1`) are not exercised in tests (they would touch the real machine).
- F1 image slack: no collector exists; the biggest relief (57 GiB here) needs an image retire-and-recreate of an idle recomputable partition (graph, nx, products, rust per the ledger `recreate` rows). Only the manual command is listed. Decision needed: a `repo cache` operation that retires an idle recomputable image under the lease protocol (the reset protocol `♻️reset` exists internally, no command).
- F2 `.cache/f` stays undeclared in the ledger (its entries are protected only because they are always recent); declaring areas for waves/lost was tried and rejected (the `native` root budget of 768 MiB is documented in the repository README and checked by the model law; the bounds live in the new member instead).
- F3 machine budget as `--budget` default (source `derived`) not wired.
- F4 replica store 81.5 GiB on the server has no collector found.
- The 12 dependency generations are not touched.

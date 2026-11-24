# SC1: Stage cache across play driver restarts (read-only audit of `/Users/ueli/.sinc/play`)

Scope: read-only. No edits, no builds, no git writes. Snapshot code paths are relative to `/Users/ueli/.sinc/play`; `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build`, `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `SF` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/☁️stage-fleet/🟦️.ts`.

## Short answers

1. **Warm generation (the default). Stage outputs persist across restarts.** Each attempt gets a new run dir `.cache/b/r/m/components/runs/w<id>` (the `SEMIO_NATIVE_BUILD_GENERATION_ROOT` in rustc env). The stage store is NOT in the run dir: it is the shared `.cache/b/r/m/components/stages` (`NB/🧩️component-pass/🟦️.ts:124`, `stages: resolve(cacheRoot, warmRoot)`; `cacheRoot = store.root/components`, `PL/🏗️build/🧩️driver-stages/🟦️.ts:116`). On disk it holds 790 MB: `transpile` 631 MB, `optimize` 158 MB, `describe` 1.2 MB. Only the fresh generation (`SEMIO_NATIVE_BUILD_GENERATION=fresh`, set by `🆕️fresh-build/🟦️.ts`, not used by the logged site builds) gets a new cold `<generation>/s` per attempt.
2. **Fleet tier is engaged in the snapshot build, and it resolves its own settings.** The stage tier reads `store.environment` plus `fleetRemoteEnvironmentV1(environment, workspaceRoot)`, which reads the local fleet facts when the env is empty (`SF` `stageFleetClientV1`, `unit-cache/🚪️launch/🟦️.ts:92`). It does not need the unit-cache wrapper env that N1 fixed. Evidence: `Stage transpile of semio_hub_dag_component was shared with the fleet cache (32 files, 211275413 bytes)` and `Stage describe of semio-hub-dag-component was shared ... (2 files, 356342 bytes)` in `pinned/site-build.log` (run wkuNcIZ); the same lines in `pinned/site-build-181007.log` (run wUysekt). No `served` (fleet hit) line exists in any pinned log. Other runs print `not shared` / `(fault)` skips (`site-build-115054.log`, `site-build-100131.log`).
3. **A restart counts nothing as staged early; it restores rather than recomputes, if keys match.** `staged` (`NB/🧩️component-pass/🟦️.ts:779`) counts only the link-staging copy/publish of a component. Post-link stage counters (`local / fleet hit / miss`) are per-process and start at 0, so `0/73 staged · 0 local` right after a restart is expected. A matching key restores the files by copy plus a sha256 re-verify of every file (`NB/🗃️stage-cache/🟦️.ts` `restore`), so a hit costs roughly one read of the output bytes (seconds, not measured). A miss re-runs describe/transpile/optimize (minutes; see the measured bound below).
4. **What makes reuse happen:** nothing new is required for warm builds. Keep the generation warm (do not set `SEMIO_NATIVE_BUILD_GENERATION=fresh`) and keep the tool binaries identical: the describe `emitter` digest and the transpile `native-transpile` digest. Those are the only keys that changed in the evidence (section 3). Changing the fresh generation to a persistent stage root needs a code change, not an env value (section 5).

## 1. Evidence: on-disk stage entries (`.cache/b/r/m/components/stages`)

Each entry is `<stage>/<digest>/🧾️entry.json` (`{version, key:{stage,schemaVersion,inputs,tools}, digest, files}`). Keys read from the manifests:

| mtime (CET) | stage | schema | component input | tool digest that changed | notes |
|---|---|---|---|---|---|
| 11:31 | transpile 7b33 (cad) | v3 | 03f4c107 | native-transpile 9dfe | 224,063,390 B core; slot still listed in `.slots` |
| 11:32 | transpile (cad, same) | v3 | 03f4c107 | native-transpile 9dfe | entry.json |
| 11:35 | describe 869f (cad) | v2 | 03f4c107 | emitter 7a1f | still listed as a `.slots/describe` row |
| 11:49 | optimize b175 (cad) | v2 | core 158f17db | wasm-opt binary fa49 | 165,475,327 B; publish ~17 min after cad transpile (upper bound on wasm-opt time, the core was linked later) |
| 17:12 | transpile e9e3 (cad) | v3 | 03f4c107 | native-transpile 982d | key flip vs 11:32 |
| 17:14 | describe 6087 (cad) | v2 | 03f4c107 | emitter 0d20 | key flip vs 11:35 |
| 22:55 | transpile 3538 (dag) | v3 | 785dd676 | native-transpile 9dfe | 210,422,390 B core |
| 22:55 | describe 66b9 (dag) | v2 | 785dd676 | emitter 7bce | |

Reading:
- The cad component's wasm digest (03f4c107) is identical across 11:32, 11:35, 17:12 and 17:14. Cargo output is stable. Every miss came from a tool digest: the emitter changed three times (7a1f, 0d20, 7bce across attempts), and native-transpile changed 9dfe to 982d and back to 9dfe.
- Stored schemas match the snapshot code: transpile v3 (`🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:108`), optimize v2 (`:113`), probe v2 (`:205`), describe v2 (`🖨️describe/🧩️component-stage/🟦️.ts:110`, `prepareActor` on). So the stored entries are valid for this snapshot's key format.
- Superseded entries are not removed until their slot is requested again. `.slots/transpile/06b7…json` still lists 7b33 (11:31) beside e9e3 (17:12). Slot supersession runs only for a request of that slot (`SF` `run`, `settleStageSlotV1`, grace 600000 ms).

## 2. Evidence: runs and the progress line

- `runs/`: four warm run dirs, 2.8 GB in total: w5AAUcq 905 MB, wUysekt 934 MB, wjPtp2e 48 MB, wkuNcIZ 981 MB. Each holds `w/` (component.wasm copies), `cs/` (source witness), `l/` (logs), `r/timeline.jsonl`. The timeline has resource samples only, no per-stage records, so per-stage wall time is not logged.
- Latest log `pinned/site-build.log` (run wkuNcIZ, started about 20:41): `warm generation …/runs/wkuNcIZ: 27 pass(es), 73 component build(s), 14 job(s)`. Staged count 0/73 until about 134 min, then `1/73 … stage cache 0 local / 0 fleet hit(s) / 2 miss(es)`. The 2 misses are the dag describe and transpile, published at 22:55.
- Earlier runs (wUysekt in `site-build-181007.log`, wjPtp2e in `site-build-204049.log`, w5AAUcq in `site-build-115054.log`) show the same pattern and `0 local / 0 fleet`. No log line in any pinned file reports a stage-cache local hit (`grep "stage cache [1-9]"` finds nothing).

## 3. Per-component stage cost

- Not logged per stage. Bounds from entry publish times: cad optimize was published about 17 min after cad transpile (upper bound on wasm-opt for a 165 MB core). Describe and transpile for dag were published in the same 30 s progress window as the 1/73 staged transition, so their cost sits inside the cargo wait, not beside it.
- A hit restores (copy with COPYFILE_FICLONE, then sha256 of every file). That is seconds for 0.6 to 0.8 GB, not minutes. Not measured.

## 4. Evidence: fleet tier

- Engaged: the `shared` uploads above. Client state: `stages/.fleet/fl/down` holds one fault at 11:33 CET. `downNow()` needs `downFaults` recent faults (`🛰️fleet/📡️client/🟦️.ts:56`), so the breaker is not blocking now.
- Gap, not explained: the cache server on this machine (192.168.178.70, the coordinator; store `.cache/b/p/m/fleet/cache-server/v1`) shows no stage payload. `mirror/` has 2,799 blobs, none newer than 22:50 and none with the dag sizes (210,422,390 / 799,953 / 278,967 / 77,375 B). `main/ac` and `semio/ac` hold unit results only (`unit.json`). `claims.jsonl` has no claim with a stage expected time (30000 / 60000 / 90000 ms). Either the uploads went to another store, or they were pruned (`.retention/sweeps.jsonl` is active). Needs a read-only probe of the endpoint before anyone relies on the fleet tier for stages.
- `SEMIO_STAGE_CACHE_REMOTE` is read only in `SF` (`stageFleetClientV1`, and tests). No play file sets it. The switch is off by default, so the fleet tier is on whenever fleet settings resolve.

## 5. Exact settings for reuse across restarts (and what is key-neutral)

- Warm (current default, works now): no env change. Required: the generation stays warm, and the emitter and native-transpile binaries are the same as the stored entries (the current digests are not computed here; check them against 7bce / 9dfe with `sha256` of the binaries the resolvers pick: `withDescriptorEmitterTaskV1`, `nativeTranspileBinaryV1`).
- Fresh (`SEMIO_NATIVE_BUILD_GENERATION=fresh`): no env can point its stage root at a persistent directory. `nativeGenerationLayoutV1` (`NB/🧩️component-pass/🟦️.ts:124`) fixes `stages = <generation>/s`, and the generation root is a new `mkdtemp` per attempt. Reuse needs a code change: fresh `stages` resolves to `join(cacheRoot, warmRoot)`. Key-neutral: yes. `StageKeyV1` has no path or generation field (`🗃️stage-cache/🟦️.ts:24`), entries are restored by copy and verified per file, and the fleet key is namespace + platform + digest (`SF` `stageRemoteKeyV1`).
- Fleet tier: no setting needed; keep `SEMIO_STAGE_CACHE_REMOTE` unset and `SEMIO_UNIT_CACHE_REMOTE` unset. Key-neutral: yes. Safety: every restored file is sha256-verified (`restore`), and the remote path falls back to local compute on any fault (`SF` header comment).
- Safe: a shared root is safe for concurrent drivers. Publish is an atomic rename, and the second publisher keeps the first entry (`stageCacheV1`).

## Open items

- Confirm the current emitter and native-transpile digests equal 7bce / 9dfe (section 5), else every stage misses on the next restart.
- Find where the 22:55 stage uploads landed (section 4). Probe the endpoint read-only.
- Stale slot rows (7b33, 869f, 60850) keep ~250 MB until their slot is requested again. Do not delete `components/stages` while the play chain runs.
- Not checked: whether the runs dirs (2.8 GB) are reclaimed by `relieveUnitStorageV1`.

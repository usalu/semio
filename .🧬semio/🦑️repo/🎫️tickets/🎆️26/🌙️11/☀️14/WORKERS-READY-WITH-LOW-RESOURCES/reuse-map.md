# Reuse map: what stops a package-level unit from being built twice in the fleet

Date of the read: 2026-11-15, about 18:40 CET. Read-only. Nothing was built, tested, started, locked or written except this file. No git write. Store and cache-server files were read with `ls`, `du`, `cat` on small JSON, and Python counting over `provenance.jsonl` and `claims.jsonl` (no blobs read or hashed).

Abbreviations (all paths relative to `/Users/ueli/Documents/semio`):
- `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`
- `C` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`
- `UC` = `C/🦀️cargo/🗃️unit-cache` (Rust rustc wrapper)
- `T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`
- `W` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES`
- `S` = `/Users/Shared/semio-store/v1`
- `CS` = `/Users/ueli/Documents/semio/.cache/f/cache-server` (the cache server data dir on this machine, g1-server; 728 MB)

## Short answer

Sharing exists for one kind of unit only: Rust rustc units (check, build, test, wasm) through the `UC` wrapper, via a fleet cache server with a claim table. It is wired and has run: the server holds 63 257 action-cache rows and 340 375 CAS rows from 10 origins. Nothing else is wired end to end. TypeScript/Nx tasks have a remote cache route but only 20 Nx rows exist. Graph owner records have a claim protocol that has never been exercised by a real wave. Remote execution is wired for rustc offload but declines most units. There is no per-consumer record, so "origin A produced, origin B consumed" cannot be shown from the store. The only direct evidence of the same unit being built on two origins is the divergence log (921 distinct keys on 2026-11-15).

## 1. Per unit kind

### 1a. Graph owner record (project inference, `N/W`: the wave that infers each Rust owner)

- Key: `projectRecordKeyV1(head, root, json, selectedTargets, selectedInputs)`, filed as `projectRecordStateV1` in `B/🪪️graph-state/📦️project-records/🟦️.ts`. The machine root is replaced by a token, so the key is machine independent by construction (R49 section 1, `result-R49.md`).
- Store: fleet cache, namespace of the graph state, via `B/🪪️graph-state/🛰️exchange/🟦️.ts:93` (`decideGraphEntryV1`, look up, else claim) and `:123` (`tryGraphEntryV1`).
- Other machine can fetch: yes by design. A server wave already consumes a record another machine stored (`B/👁️observation/🟨️.mjs:962`, `port.gate`). Evidence: none. R49 section 4: "No worker published an owner record the server consumed. Not shown." The owner pool is wired on every machine (`B/🌐️remote-execution/🛰️sidecar/👷️agent/🟦️.ts:41`, a `pool` child), but the status files show it idle: `S/origins/g1-*/pool.json` at 18:33 to 18:39 all say `state: idle`, `published: 0`, `stored: 0`, `units: 0`.
- Platform: not split by platform in the key (inferred from the key text). Linux and Windows are not tested (R49 section 4).
- Claim: yes, the same `B/🌐️remote-cache/🎟️claims/🟦️.ts` table (TTL 30 s, heartbeat 8 s, R49 section 1).
- Duplicate risk: every machine that runs the graph child infers its own owners unless the pool is used (inferred from R49 section 4: the pool is a puller only; the wave code is not run by workers).

### 1b. Rust check, build and test units (rustc wrapper, `UC`)

- Local key: `unitKeyV1` at `UC/🔏️key/🟦️.ts:110`. Inputs: toolchain, normalised rustc args, source sha256, extern artifact sha256 (`--extern` bytes), env.
- Fleet key: `remoteKeyV1` at `UC/🛰️fleet/🪪️identity/🟦️.ts:131` = sha256 of namespace, fleet namespace, the platform object and `portableKeyV1` (line 125). Units whose bytes contain a real path also get that path in the key (`sensitive`, same line), so they are shared only between machines with the same paths.
- Platform object: `platformOfV1` at `UC/🛰️fleet/🪪️identity/🟦️.ts:63`: os, arch, host triple, digest of `rustc -vV`, launcher toolchain digest, linker (`darwin-<major>`, only for linked kinds), and CPU identity (only for native-CPU builds). Declared in the same file at line 16.
- Host-independent units: `hostIndependentV1` (line 52) with `targetPrefixes: ["wasm32-"]`, crate types `lib`, `rlib` (`UC/🛰️fleet/🧬️schema/🔣️.json`). These are shared by all platforms.
- Slot (content-free identity, for supersession): `slotIdV1` at line 136. The design says supersession is not built (`T/design.md` decision 4); I found no code path that deletes superseded keys (not verified by reading all of GC).
- Storage: local unit store `UC/💾️store/🟦️.ts`. Fleet: upload queue `UC/🛰️fleet/📤️uploader/🟦️.ts` posts to the cache server as `ac` entries (kind `ac`, namespace `main`). Server write: `B/🌐️remote-cache/🖥️server/🟦️.ts:204`.
- Fetch by another machine: yes, through `obtainFleetUnitV1` (`UC/🛰️fleet/🟦️.ts:91`) called from `UC/📜️script.ts:164`. Settings come from `SEMIO_REMOTE_CACHE_*` env (`UC/🛰️fleet/📡️client/🟦️.ts:13`), which the launcher fills from the group facts file `S/fleet/cache.json` (endpoint `http://192.168.178.70:6094`, instance `main`, access `read-write`, generation 7, published from this machine). Facts path: `B/🌐️remote-cache/🛰️fleet/🟦️.ts:38`.
- Claim: yes. `obtainFleetUnitV1` (`UC/🛰️fleet/🟦️.ts:91-119`): look up; on miss claim at the server (`claim.state` `granted` → compile and hold a heartbeat; `held` → long-poll for the upload, bounded by `waitFactor` times the holder's expected cost; `stored` → fetch; lapse → claim ourselves). Policy values in `UC/🛰️fleet/🧬️schema/🔣️.json` line 28: claim TTL 30 000 ms, heartbeat 8 000 ms, minimum wait 30 000 ms, maximum wait 1 200 000 ms. The server side is `B/🌐️remote-cache/🎟️claims/🟦️.ts` (table at line 45, route answer at 146-168) and `B/🌐️remote-cache/🖥️server/🟦️.ts:130-132` (claim route), `:204` (`claims.stored` on put).
- Local single-flight in addition: `acquireUnitClaimV1` on a per-machine lock (`UC/📜️script.ts:156-159`). It only covers two processes on one machine.
- Fail-open: every "not granted" outcome compiles locally (`UC/🛰️fleet/🟦️.ts:96` fault, `:98` no-claims, `:104` claims-full and claims-fault, `:107` holder-slow, `:116` rounds, `:117` error). Design rule 4 (`T/design.md`) requires this.
- Path-dependent output: not shared. `UC/📜️script.ts:186-188` (`leakedPathsV1`, `remote-skip` with `path-dependent` reason).
- Rust build and test units: same wrapper, same key, same server namespace. The interface cutoff (R39, `result-R39.md` verdicts 1-2) only applies to metadata-only check units; code units are keyed by bytes. Whether a `build` unit and a `check` unit can be told apart on the server: not in the provenance (kind is `ac` for both).

### 1c. TypeScript build (Nx tasks)

- Key: Nx task hash (content hash of the project and its path dependencies, `T/design.md`; `result-R39.md` section 1). Not machine-specific by design, but I did not verify that the Nx hash includes the platform (inferred: it does not include platform, so Nx keys would be shared across operating systems if the remote cache were used).
- Store: Nx self-hosted API on the cache server: `B/🌐️remote-cache/🖥️server/🟦️.ts:78` (`/v1/cache/<hash>`, kind `nx`). Front and single flight: `B/🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts` (lease, heartbeat, circuit breaker), `🏁️task-finish`, `🪢️slot-settle` folders beside it.
- Fetch by another machine: the route exists. Evidence of use: 20 `nx` rows in the provenance log (origins g1-m01 1, g1-m03 8, g1-m04 11). That is almost no use.
- Claim: the Nx single-flight is the claim. It is not connected to the unit claim table (`grep` in `B/🟦️typescript` finds no fleet, remote-cache or claim references outside tests).
- Platform: not a factor in the key (inferred, see above).

### 1d. Wasm plugin build

- The wasm32 rustc units are the Rust units of 1b with `hostIndependentV1` true, so they share across platforms (`UC/🛰️fleet/🪪️identity/🟦️.ts:52`, schema `Policy.hostIndependent`).
- Wasm packaging and reading (`C/📦️artifacts/🕸️wasm/`, `👓️reading`, `📋️contract`): no fleet or claim references found in the non-test files (`grep`). Inferred: the packaging steps are local per machine.

### 1e. Tool installs and dependency installs

- Install records: `S/fleet/install/inst-<key>.json`, 18 files (6 `linux x64`, 6 `darwin arm64`, 6 `linux x64-gnu`, bun 1.4.2; 12 created 11-14, 6 created 11-15). Each has `key`, `projectionSha256`, `runtime` (platform, arch, bun, recipe digest), `installed`, `shape`. There is no origin field in these files, so the producer cannot be read.
- Code: `C/🚀️bootstrap/🛰️fleet-fetch/📦️install/🟦️.ts` (`callClaim` at line 413, claim, renew and release; 10 claim references). The install path does have claims.
- Other fleet-fetch paths: `🦀️crates/🟦️.ts`, `🐍️uv/🟦️.ts`, `🪞️mirror/🟦️.ts`, `🟦️.ts` (fleet-fetch root), with 0 claim references each. Duplicate downloads are possible there (inferred from the absence of claims; not measured).
- Store size: `S/fleet/install` 5.3 GB, `S/fleet/fetch` 699 MB (256 shards, 1 620 files), `S/fleet/setup` 12 MB.
- Platform: installs are platform specific by runtime (the three runtime keys above).
- R25 (dependency funding, `W/result-R25.md`): "STATUS: PREPARED, NOT LANDED, NOT WIRED". It shortens the exclusive hold of the native dependency funding on one machine. It does not change cross-machine reuse.

### 1f. Rust unit offload (remote execution of one rustc unit)

- Key: same as 1b. Offload ships the unit to a worker, which compiles it and uploads the result to the cache (`UC/📜️script.ts:168`, `offloadUnitV1`; output post-processed locally, `UC/📜️script.ts:169-173`).
- Decision: `OFFLOAD_POLICY_V1` and `offloadModeV1` in `B/🌐️remote-execution/🚀️offload/🟦️.ts` (imported at `UC/📜️script.ts:24`). Mode from `SEMIO_OFFLOAD`, default `auto` (`T/landings.md`, line 96). The requester decides by its own capacity (`B/🌡️host-capacity/🟦️.ts`); the scheduler picks the worker (`B/🗓️scheduler/🟦️.ts`, load-aware dispatch per `T/landings.md` line 120).

## 2. Remote execution: what exists

- Wired on this machine: the build sidecars run under the replica (`CS/health.json`: `build-agent`, `build-scheduler`, `build-cache`, all `running`, restarts 0 since 1794760253756 ms).
- Who decides: the requester's `UC` wrapper (offload mode) and capacity admission; the scheduler chooses a machine (`T/landings.md` 120, 297).
- Real run (landings, live proof, `T/landings.md` line 297, 11-14): `--offload=always` of `semio-plugin-testing-laws` (234 unit misses): 1 offloaded, 130 fell back (`incomplete-inputs` for 70 + 24 + 16 + 13 first-dependency crates; `infrastructure`, 94, when disk fell below 25 GiB; `no-idle-machine`, 8; `inputs-too-large`, 1). Later run: 15/25 always, 19/25 auto, 0/25 calm, 150/150 artifacts byte-identical (`T/landings.md` line 365).
- Counters on this machine (`CS/../offload-counters`, and `CS/../build-counters.json`): out 139, fellBack 512, refused 560. Reason files (count of markers): unmapped-path 601, worker-policy 202, no-idle-machine 21, unreadable-input 21, incomplete-inputs 20, path-dependent-output 1. The reasons do not add up to the two counters; the exact accounting is in `B/🌐️remote-execution/🚀️offload/` (not read).
- Other machines: their counters are local files on each machine and are not in the store. Whether other machines offload: not shown.
- The older map (`T/map-remote-execution.md`, 11-13) says nothing else submitted work to the REAPI server. The later landings wire rustc offload. The Bazel backend is still the other client.

## 3. Evidence in the store and logs

Files read:
- `CS/fleet/provenance.jsonl`: 403 652 lines, 0 unparsable. One line per stored entry: namespace, kind, key, origin `o` (producer), time `t`, optional fingerprint `f`. No consumer field. Capped at 500 000 lines (`B/🌐️remote-cache/🧾️provenance/🟦️.ts:24`, MAXIMUM; oldest dropped beyond it). It is 81% full, so older rows will be lost.
- Rows by kind and producer origin (time span of `ac` rows, UTC):

| origin | ac rows | cas rows | ac first to last (UTC) |
|---|---|---|---|
| g1-server | 22 713 | 129 397 | 11-13 23:39 to 11-15 17:41 |
| g1-m07 | 5 988 | 65 217 | 11-14 01:56 to 11-15 17:39 |
| g1-m03 | 9 382 | 57 186 | 11-14 03:02 to 11-15 14:44 |
| g1-m01 | 7 070 | 24 626 | 11-14 03:31 to 11-15 17:40 |
| g1-m05 (Windows) | 6 528 | 12 654 | 11-14 12:26 to 11-15 17:41 |
| g1-m04 (Linux) | 2 652 | 19 055 | 11-14 03:37 to 11-15 17:39 |
| g1-m08 | 4 421 | 9 746 | 11-14 11:19 to 11-15 03:56 |
| g1-m10 | 2 733 | 9 394 | 11-15 04:25 to 11-15 15:36 |
| g1-m02 | 1 291 | 5 692 | 11-14 02:03 to 11-15 17:36 |
| g1-m06 | 492 | 7 508 | 11-14 16:04 to 11-15 06:04 |
| nx (all) | 0 | 0 | nx rows: g1-m01 1, g1-m03 8, g1-m04 11 |

  Totals: 63 257 ac rows, 340 375 cas rows, 20 nx rows.
- `CS/fleet/divergences.json`: 1 000 rows (the file is capped at 1 000, `ROWS=1000` at provenance line 24; the real count is at least 1 000). Kind `ac`: 1 000 rows, 921 distinct keys, all dated 2026-11-15, namespace `main` 989, `semio` 11. Most frequent pairs (kept origin to refused origin): g1-server to g1-m10 462; g1-m01 to g1-m10 147; g1-m07 to g1-m10 96; g1-server to g1-m03 87; g1-m01 to g1-m03 72; g1-m01 to g1-m05 36. A divergence means the same key was written with different bytes by two origins, so the second origin had its own bytes for that unit. The refused origin must have produced or offered the unit locally (`UC/🛰️fleet/🟦️.ts:121` offer and `:129` share, inferred from the code path), so this is a unit that two machines built or held. Meaning of the byte difference is not known (inferred: path or nondeterminism; `T/design.md` says identity must hold; R39 section 7 item 5 says SVH equality across machines is unverified).
- `CS/v1/claims.jsonl`: 392 lines; 335 `set` and 61 `drop`, all `ac` addresses. Owners: g1-m05 Windows `DESKTOP-6NSLEUO` 156 sets; g1-server 119; g1-m05 (second process name) 46; g1-m01 14. This shows claims were granted to at least three origins. It does not show waits, holds or outcomes. The count of claimed addresses (396) is small against 6 932 local misses on g1-server (below), so the journal may be compacted or claims may be rare (not resolved).
- `CS/../build-counters.json` (g1-server only, wall 53 153 445 ms): unit `localHit` 7 844, `remoteHit` 566, `miss` 6 932, `stored` 4 488, `skipped` 997. `remoteHit` is the only fleet-consumption counter in any file read, and it has no origin. Some of the 566 must come from entries that other origins produced (inferred: g1-server has 22 713 ac rows of its own, so not all can be its own, but the split is not in the files).
- `S/origins/g1-*/pool.json`: all ten pullers `idle`, `published` 0, `stored` 0, `held` 0 (18:33 to 18:39). Consistent with R49.
- `S/origins/g1-*/attest.json` and `head.json`: show that the replica is alive on all ten origins; `g1-m06` has no `pool.json`; `g1-m09` has no content.
- `S/heads/`: 10 head files, one per origin.

Direct answer to item 3: the store and logs show that origin A produced ac and cas entries that the cache server stored (provenance). They do not show that origin B fetched any of them. The only per-consumer number is g1-server's `remoteHit` 566, which is not split by origin. The only record of two origins building the same key is the divergence log. Consumption by origin B: none in the files.

## 4. Gaps, ranked by duplicate work allowed

Ranking is by how much duplicate work each gap allows. Sizes marked (inferred) are not measured.

1. TypeScript and Nx tasks are effectively not shared. Only 20 Nx rows exist, and the Nx single flight is not connected to the unit claim table. Every machine runs its own TypeScript builds (inferred from the 20 rows; volume not measured). Close at `B/🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts` (wire it to the shared claim table and the Nx remote read on every replica) and the Nx route in `B/🌐️remote-cache/🖥️server/🟦️.ts:78`.

2. Rust units built twice on different origins, with divergent bytes: at least 921 keys on 2026-11-15 (`CS/fleet/divergences.json`). Two causes are visible in the code: the fail-open branches in `obtainFleetUnitV1` (`UC/🛰️fleet/🟦️.ts:96-117`: `no-claims`, `claims-full`, `holder-slow`, `rounds`, `error`) compile locally by design, and the claim covers only the fleet key, not the local key. Close at `UC/🛰️fleet/🟦️.ts` `obtainFleetUnitV1` (wait budget and claim policy in `UC/🛰️fleet/🧬️schema/🔣️.json:28`), and find the byte difference first, at `UC/🛰️fleet/🪪️identity/🟦️.ts:63` (`platformOfV1`) and `:131` (`remoteKeyV1`). Until then the server's first-write-wins (`B/🌐️remote-cache/🖥️server/🟦️.ts:204`) keeps one of two different outputs.

3. Owner records are inferred per machine. The claim protocol exists (`B/👁️observation/🟨️.mjs:962`), the pool is supervised on every machine, but no wave has published a record and no worker runs the wave (`S/origins/*/pool.json` idle; R49 section 4 and 6). Inferred: every machine infers the same Rust owners. Close at the wave publish and consume path in `B/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/` and the wave hook at `B/👁️observation/🟨️.mjs:962`.

4. Rust units that are path-dependent or contain real paths are never shared. `UC/📜️script.ts:186-188` skips them (`remote-skip`, `path-dependent`). The count is the `skipped` counter (997 on g1-server, reason not split). Close with the remap work in `UC/🧭️remap/` (not read in depth).

5. Remote execution declines most units. Out 139 against refused 560 and fellBack 512 on g1-server; `unmapped-path` (601 markers) is the largest reason (`CS/../offload-counters/reason/`). Compute is not moved to idle machines in practice (inferred from the counters). Close in `B/🌐️remote-execution/🚀️offload/🟦️.ts` (path mapping) and `B/🗓️scheduler/🟦️.ts`.

6. Tool installs: `fleet-fetch/🦀️crates`, `🐍️uv`, `🪞️mirror` and the fleet-fetch root have no claims (0 claim references each). Install (`📦️install`) does have claims (10 references). Duplicate downloads are possible (inferred). Close at `C/🚀️bootstrap/🛰️fleet-fetch/🦀️crates/🟦️.ts` and `🐍️uv/🟦️.ts` by reusing the `callClaim` of `📦️install/🟦️.ts:413`.

7. Platform partition. The seven macOS machines (g1-m01, m02, m03, m07, m08, m10 and g1-server; `attest.json` `platform: macos` for g1-m01) share among themselves only, if their OS major matches (`platformOfV1` `linker` field, `UC/🛰️fleet/🪪️identity/🟦️.ts:67`). The Linux machine (g1-m04) and the Windows machine (g1-m05) have no peer for native Rust. g1-m05 still uploaded 6 528 ac rows and 156 claims that no one else can use (inferred from the platform key). This is by design (`T/design.md` decision 7), not a bug, but it is the largest permanent gap for those two machines.

8. No consumer record and capped evidence. Provenance has no consumer field and is 81% of its cap (`B/🌐️remote-cache/🧾️provenance/🟦️.ts:24`). The divergence list keeps 1 000 rows. Per-machine counters stay on each machine. Close by adding a `consumedBy` row to the server's `get` path in `B/🌐️remote-cache/🖥️server/🟦️.ts` and by making the status route return per-origin hit counts, so the "A produced, B consumed" evidence can be read from `S/`.

9. R25 funding is not wired (`W/result-R25.md`, "NOT WIRED"). Dependency installs hold one exclusive lock per machine. It is not a cross-machine duplicate, but it serializes installs on each machine.

## 5. Notes for whoever reads this

- Not verified: any run of the fleet on the workers; the other machines' `.cache/f/build-counters.json`, `offload-counters` and `cache-facts.json` (not in the store, on each machine); Linux and Windows key equality.
- Results R39, R40, R49 and R50 are "prepared" or "landed" packages. R50 (`W/result-R50.md`) says the four packages landed in one window on 2026-11-15 17:59 CET. Their effect on the counts above is not measured (R40 section 4 gives re-key counts, not reuse counts).
- The fleet design (`T/design.md`) lists the work packages A to E. The code shows A (facts through the store, `B/🌐️remote-cache/🛰️fleet/🟦️.ts`) and B (unit claims) as built; C (slot supersession) and E (offload under pressure, `B/🌐️remote-execution/🚀️offload/`) are partly built. D (load reports into the scheduler) is partly built (`T/landings.md` line 120); the per-machine load reports were not read.

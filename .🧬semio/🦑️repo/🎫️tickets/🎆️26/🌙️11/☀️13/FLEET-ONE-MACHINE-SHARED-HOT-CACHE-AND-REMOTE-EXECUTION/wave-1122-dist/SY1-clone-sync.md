# SY1: consolidated clone-sync list for the play snapshot

Snapshot = `/Users/ueli/.sinc/play`, source = main `/Users/ueli/Documents/semio`. Scratchpad (all verification artifacts): `/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/sy1/` (`ov2/` = APFS clone of the snapshot `🧰️framework` with the set applied; nothing was written to the snapshot or to main outside T).

## Result: 17 files copied whole from main + 3 patched files (snapshot content + only the wanted hunks)

### A. Copy whole from main (in this order: data first, leaves before dependents; use cp to a temp name + mv, ideally with the chain stopped)
```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧬️schema/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/📌️pins/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🫂️holders/🧮️plan.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🫂️holders/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🪦️reclaim/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧹️collector/🧮️plan.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧹️collector/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🌳️inputs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧮️phase-ledger/🟦️.ts
```

### B. Copy from `T/wave-1122-dist/SY1-patched/` (NOT from main; main's versions of these three carry key changes or unrelated lanes)
Order: `store.ts`, then `component-pass.ts`, then `script.ts`. Destination = path on the right, relative to the snapshot root.
```
SY1-patched/store.ts           -> 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts
SY1-patched/component-pass.ts  -> 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts
SY1-patched/script.ts          -> 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts
```
Each was cut from the snapshot's file as of this run. Base sha256 (first 16): store `7e0194381fa925a0`, component-pass `2efdf572dc251d58`, script `2c4cae7a03bd1838`. If the snapshot file's sha differs from its base (the play session edited it since), do not overwrite: apply the matching `SY1-patched/<name>.patch` instead (`cd /Users/ueli/.sinc/play && patch -p1 < .../SY1-patched/<name>.patch`; tested, applies cleanly and reproduces the patched file byte for byte).
Hunks in the patched files:
- `store.ts` (CAS2): +2 imports (`registerCasHolderSyncV1` from holders, `CAS_POLICY_V1`), `HELD_REFERENCES_V1` + `holdHostObjectsV1`, 4 calls (`lookupUnitV1` hit, `writeEntryV1`, `lookupBuildRunV1` hit, `writeBuildRunV1`), the dead `rmSync(<root>/o)` removed + docstring. NOT taken: the `buildDependencyPathsV1` import (needs the re-keyed `🔏️key`), `retention.self/units`, `BuildEntryV1.retention`, `buildRunNeutralV1/RestoreV1` dependency tokens (K5/K6).
- `script.ts` (H2 learned peaks): one added line before `const publish=`, `writeUnitPeakV1(settings.root,crate,provenance.peakRssBytes/1048576)` via lazy import of the offload admission module, after the compile, key already computed. NOT taken: `sourceValueV1` env values, `markProducerKeyV1`/`producerKeyV1`, `buildRunKeyV1`, `rawExterns/producers` retention (K4/K5/K6, key-feeding and slot metadata).
- `component-pass.ts` (V1 counters): import of `createUnitReuseTallyV1`, tally creation, progress line `R reused (A local, B fleet)`. NOT taken: provenance file name now hashing `generation.report` (unrelated lane, harmless to keys but out of scope).

### C. Optional (not in the verified minimal set, verified to load)
- `LIB/🏗️build/🗄️cas/⌨️command/🟦️.ts`: `repo cache cas status` listing holders (CAS2 verification aid); its text also describes CAS1 cap/refusal behaviour that the unsynced guard does not implement, so the status text overstates. Bundles fine with set A.

## Per fix
| fix | files | state in snapshot before |
|---|---|---|
| O9 closure (`dependency closure` declines) | `offload/🌳️inputs`, `offload/🗺️paths` | missing; main has it |
| O6 remainder | `offload/🗺️paths` (only the `bindgen!/generate!/from_witx!` WIT-binding scan is left; env-read names, serde_core exact reads, dependency-row skip are already in the snapshot) | env/serde_core part present |
| CAS2 holders | `cas/🫂️holders` (2), `cas/🧹️collector` (2), `cas/🪦️reclaim`, `cas/📌️pins`, `cas/🧬️schema` (2), `gc/🪢️supersession`, patched `store.ts` | absent |
| V1 content retry (503/Retry-After) | `🗄️content`, `offload/🟦️`, `offload/🧬️schema`, `cas/📌️pins` (content imports `CAS_LEASE_HEADER_V1`) | `retries:0` |
| V1 reuse counters | `🧮️phase-ledger`, patched `component-pass.ts` | absent |
| O8 client | nothing to sync: `🔌️client/🟦️.ts`, `offload/📊️counters`, the scheduler O8 code are byte-identical in the snapshot; the only O8-adjacent difference is in `offload/🟦️.ts` (deadline cause counters, `linked` instead of `peak>0`), which is in set A | present |
| H2 requester | `offload/🟦️`, `offload/📐️policy`, `offload/🚦️admission`, `offload/🧬️schema` (`policy.peak`), patched `script.ts` (learned peak writer) | static 6144 for wasm linked only |

## Excluded, with reasons
- S1 (`🔏️key/🟦️.ts` `sdkIdentityV1`; also the docstring in `offload/🧰️toolchain`): EXCLUDED. `sdkIdentityV1` feeds `machineValueV1("SDKROOT")` and so the key of every linked native unit: proven by the real function: the snapshot returns `MacOSX27.2.sdk` for the CLT path, main returns `macosx27.2` (keys of linked units re-key once). `🔏️key/🟦️.ts` is byte-different from the snapshot and must stay so out of the set.
- K4/K5/K6, anything in `🔏️key/`, `🪪️extern`, `🏗️buildscript/🔑️run-key`, `🏗️buildscript/📦️run-wire`, `🏗️buildscript/🟦️`, `unit-cache/🧬️schema/🔣️.json` (retention declaration), `🛰️fleet/🪪️identity`: not touched (all key-feeding or key-adjacent). The remaining hunks of `store.ts`/`script.ts` above are the cannot-sync-whole case (see below).
- H2 worker/scheduler side (`🌐️remote-execution/🟦️.ts` Task `peakMegabytes`, `🗓️scheduler` 1 line, `👷️worker`, `🛰️agent`, `🌡️load` + new `🌡️load/🧬️schema`, `🪑️stage-claim`, sidecar `🚦️ready`, `🏃️execution/🧬️schema/🌐️remote-execution.json`): requester side only was asked; these run in worker/scheduler processes (restart rules apply).
- CAS1 server side (`cas/🚦️guard`, `cas/🗃️store`, `cas/🆘️full-disk`, `cas/🧲️adopt/🔒️private`, sidecar `🗄️cache`, remote-cache server/store/collector, `offload/📜️action` input-lease intent): not asked. Consequence: offloaded input blobs are not leased (no behaviour change versus today).
- Unrelated landings in the same dirs (graph, observation, fleet-status, storage images, `native-build/🟦️.ts` log-failure guard, tests): not asked.

## Files that cannot be synced whole without re-keying (handled by patch)
- `unit-cache/📜️script.ts`: key-relevant hunks = import `sourceValueV1` and its use in `keyInput.environment` (OUT_DIR keyed as `⟨out⟩<crate>`), `producerKeyV1`/`markProducerKeyV1`/`buildRunKeyV1` (retention/slot key side), `shim(...,key)` argument. They need the re-keyed `🔏️key/🟦️.ts` (`outDirectoryIdentityV1`, `buildPathIdentityV1` in `keyEnvironmentV1`, new `sdkIdentityV1`). Copying main's file alone fails with `No matching export "sourceValueV1"` (negative control reproduced); copying `🔏️key` too re-keys. Consequence of not syncing: learned peaks only through the patched line (provided).
- `unit-cache/💾️store/🟦️.ts`: imports `buildDependencyPathsV1` from `🔏️key/🟦️.ts` (new in main), and changes build-script-run neutralization (`DEP_*` path tokens) and entry content. Patch carries only the holder registration.

## Verification (snapshot bun 1.3.14 `~/.bun/bin/bun`; clone `ov2/` of the whole snapshot `🧰️framework` + set A + patched B)
1. Differing files found with `diff -rq` over `🏗️build`, `⚡️caching`, `native-build`: 264 + 36 rows; selection by reading each diff (main vs snapshot, offload/RE diffs are 1 to 40 lines).
2. Export names: no export of any replaced file is removed (regex export-diff over all 20 files; only additions).
3. `bun build --target=bun` (resolves every import, rejects missing exports) of: `UC/🏁️entry.ts` (244 modules), `UC/📜️script.ts`, `UC/💾️store`, `offload/🟦️`, `🗄️content`, `cas/🧹️collector`, `🫂️holders`, `🪦️reclaim`, `📌️pins`, `gc/🪢️supersession`, `component-pass` (474 modules), `phase-ledger`, `native-build/🟦️.ts`, `🎟️admission` (native-build admission), `cas/⌨️command`, `cas/🚦️guard`, `cas/🗃️store`, remote-cache server/store/collector, sidecar `🗄️cache`, `🛰️agent`, `🗓️scheduler`, `👷️worker`, and the whole repo CLI `🏗️build/📜️script.ts` (2155 modules): all bundle, 0 errors. Negative control: main's `script.ts` dropped into the clone fails with `No matching export in 🔏️key/🟦️.ts for import "sourceValueV1"`.
4. Real imports (bun) of the changed modules: all load. Smoke run on scratch roots: `holdHostObjectsV1` writes `v1/h/unit-cache-<hash>.json` indexing `uc/e` and `uc/r` with `outputs.*.sha256,stdout,stderr`; `readCasHeldV1` sees 1 holder; new collector dry-run on a scratch store reports `holders:1`; `OFFLOAD_POLICY_V1.peak` (2048, 25 %, 64..16384, 137, x2) and `limits.contentRetries 3 / 2000`; `offloadUnitPeakMegabytesV1` unknown 2048, learned 1000 gives 1250, wasm unknown 6144; `writeUnitPeakV1/readUnitPeakV1` round-trip; `dependencyClosureV1` with a missing directory returns an empty map (no decline); reuse tally counts 2 local + 1 fleet and ignores the partial last line. The unpatched snapshot lacks all of it (the same smoke fails there).
5. Wrapper seal: `sealedUnitCacheV1` over the clone seals 345 files (snapshot: 343; +2 holders), limit 2000, no `🏗️build/📜️script.ts` in the sealed tree, so the sealer's closure law still holds with the lazy `admission` import (it was already in the closure).
6. Keys unchanged: (a) the closure of `🔏️key/🟦️.ts` (3 files) and of `🏗️buildscript/🔑️run-key` (4) contains no differing file; (b) the 16 differing files in the wrapper closure (234 inputs of `🏁️entry.ts`) are exactly the offload/cas/store/script set, and `🔏️key/*`, `🪪️extern/🟦️.ts`, `buildscript/*` are byte-identical; `🪪️extern` pulls `store` only for `digestFileV1` (unchanged function); (c) the one `script.ts` line added runs after the key is computed and only writes `<root>/c/<crate>.peak`; (d) `unitKeyV1` on a fixed input prints the same sha256 (`de553fd787a7…`) with snapshot and clone code. The unit-cache schema JSON is unchanged (no namespace change).
7. Source hashes: every set-A file in the clone is byte-identical to main at the end of the run (17/17 `same`); all destination paths are 137 to 183 bytes.

## Behaviour notes (not key related)
- H2: every offloaded unit now declares `semio-peak-mb` (>= 2048, wasm linked 6144 as before) instead of only wasm linked units, so with workers whose `freeMegabytes` is still the raw available memory (old worker code) the scheduler places stricter; the platform property is part of the REAPI action digest (not of the unit key), so remote action-cache entries made with the old properties are not reused for non-wasm units. Also the offload memo of declined `no-idle-machine` now keys on `linked` (same meaning as before).
- CAS2: collector side evicts only when the cap still needs bytes after unreferenced objects are gone (then LRU unit-cache entries together with their objects), protects held objects always; registration runs in every wrapper from its first entry write/hit.
- V1: the cluster content client now retries 408/429/5xx (not 507) up to 3 times, waiting max(backoff, Retry-After capped at 2 s).

## Restart needed
- Wrappers (rustc shim -> `🏁️entry.ts`, offload client, store registration, peak writer, `paths`/`inputs` planner): each rustc call is a new process; a changed tree gives a new sealed copy (`sealed/<digest>`), so no restart, only avoid copying while a unit is mid-flight: stop the chain, copy, start it (mixed old/new files within one rustc call is the only risk; the list is ordered data first for that).
- Native-build driver (counters: `component-pass.ts`, `phase-ledger`): loaded once per driver process, so the chain (`native-chain.sh` driver) must be restarted to print `R reused (A local, B fleet)`.
- CAS holder PROTECTION also needs a collector running the new code: the cache sidecar (:6094, collects with the old code today) restarts at its next natural restart or by the play session's decision; until then registration is harmless (writes `v1/h/*.json`) but the sidecar still ignores it. `repo cache gc`/guard collections from the clone use the new collector immediately only through the synced `cas/🧹️collector`; the unsynced `cas/🚦️guard` keeps calling it without `incremental`.
- Nothing in set A or B needs a scheduler/worker restart.

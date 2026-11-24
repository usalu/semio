# SY2: combined clone sync (SY1 + O10) for the play snapshot

Snapshot `/Users/ueli/.sinc/play`, source main `/Users/ueli/Documents/semio`. Scratch: `/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/sy2/` (`ov3/` = APFS clone of the snapshot `🧰️framework` with the whole set applied, `ov0/` = pristine clone, `ov3cap/` = ov3 with cap 6). Nothing was written to the snapshot or main outside T.

## Snapshot state found: SY1 NOT applied
All 3 SY1 base files have the base sha of SY1 (store `7e0194381fa925a0`, script `2c4cae7a03bd1838`, component-pass `2efdf572dc251d58`); the 16 set-A files that exist differ from main, the 2 holders files are missing; driver-stages is `97972b2ad87c7ea5` (O10 base), no `🧵️job-slots` dir. Set-A files in main are byte-identical to what SY1 verified (17/17). So the full list below applies, in this order.

## Final ordered list (23 files: 17 whole from main, 2 new from main, 4 patched)
Prefix `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build`, `UC` = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache`. Copy to a temp name then `mv`; ideally with the chain stopped (see restart).

```
# A. WHOLE COPY from main (17), data first, leaves before dependents
LIB/🏗️build/🗄️cas/🧬️schema/🔣️.json
LIB/🏗️build/🗄️cas/🧬️schema/🟦️.ts
LIB/🏗️build/🗄️cas/📌️pins/🟦️.ts
LIB/🏗️build/🗄️cas/🫂️holders/🧮️plan.ts          (new file in snapshot)
LIB/🏗️build/🗄️cas/🫂️holders/🟦️.ts              (new file in snapshot)
LIB/🏗️build/🗄️cas/🪦️reclaim/🟦️.ts
LIB/🏗️build/🗄️cas/🧹️collector/🧮️plan.ts
LIB/🏗️build/🗄️cas/🧹️collector/🟦️.ts
LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts
LIB/🏗️build/🌐️remote-execution/🚀️offload/🧬️schema/🔣️.json
LIB/🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts
LIB/🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts
LIB/🏗️build/🌐️remote-execution/🚀️offload/🌳️inputs/🟦️.ts
LIB/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts
LIB/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts
LIB/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts
NB/🧮️phase-ledger/🟦️.ts

# B. NEW from main (2), stable copies in SY2-patched/
SY2-patched/job-slots.ts           -> NB/🧩️component-pass/🧵️job-slots/🟦️.ts
SY2-patched/job-slots-schema.json  -> NB/🧩️component-pass/🧵️job-slots/🧬️schema/🔣️.json
   (mkdir -p both dirs; the .ts imports ./🧬️schema/🔣️.json, so copy schema first; tests dir NOT synced)

# C. PATCHED (4): copy the file from T/wave-1122-dist/SY2-patched/ (NOT from main), order: store, script, component-pass, driver-stages
SY2-patched/store.ts           -> UC/💾️store/🟦️.ts                     (SY1; base sha 7e0194381fa925a0 -> 06d2180a699bebfe)
SY2-patched/script.ts          -> UC/📜️script.ts                       (SY1; base 2c4cae7a03bd1838 -> 03f0b256994534d2)
SY2-patched/component-pass.ts  -> NB/🧩️component-pass/🟦️.ts           (SY1 reuse tally + O10 job slots MERGED; base 2efdf572dc251d58 -> 831b9fbbb5493f5b)
SY2-patched/driver-stages.ts   -> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts   (O10; base 97972b2ad87c7ea5 -> 44df7226fd59c6e1)
```
Fallback if a snapshot file's sha differs from its base (play edited it): `cd /Users/ueli/.sinc/play && patch -p1 < <T>/wave-1122-dist/SY2-patched/<name>.patch` with `component-pass.patch` (combined snapshot -> SY1+O10, 11 hunks) or `driver-stages.patch`; `store`/`script` use `SY1-patched/{store,script}.patch`. All four patches proven on the pristine clone `ov0`: `patch -p1` applies cleanly and the result is byte-identical to the shipped file (`cmp` OK for component-pass and driver-stages). Merge proof: `SY1-patched/component-pass.ts` + `O10-patched/component-pass.patch` == `O10-patched/component-pass.ts` (cmp identical), and snapshot + `SY2-patched/component-pass.patch` == the same file. Only `component-pass` is touched by both lanes; no other file overlaps (driver-stages, job-slots only O10; store, script, set A only SY1).

## Verification (snapshot bun 1.3.14 `~/.bun/bin/bun`, clone `ov3/`)
1. Import closure: `bun build --target=bun` 0 errors for: wrapper entry `UC/🏁️entry.ts`, offload client `🔌️client`, `offload/🟦️`, native-build admission `🎟️admission`, `component-pass` (+ job-slots), `driver-stages` (2,567 modules), `job-slots`, `native-build/🟦️`, `phase-ledger`, `store`, `UC/📜️script.ts`, cas collector / holders / reclaim / pins / `⌨️command`, `🗄️content`, and the whole repo CLI `LIB/🏗️build/📜️script.ts` (exit 0). The only message is the snapshot's own pre-existing warning `Duplicate key "portableSourceAnalysisOwner"` in `👁️observation/🧬️schema/🔣️.json:179` (also in the pristine clone, a warning, not an error).
2. Key neutrality: `unitKeyV1` on the fixed input (`SY1` `key.ts`) = `de553fd787a7c56c3453d2f2cfe5dc036427abeb73584c569833a897e071ee65` with pristine clone `ov0` and with `ov3`; `sdkIdentityV1` unchanged (`MacOSX27.2.sdk`). `🔏️key`, `🪪️extern`, `buildscript`, unit-cache schema are not in the list. O10 touches only scheduling of cargo processes (no key input).
3. Wrapper seal on `ov3`: 345 files (snapshot 343, +2 holders), no `🏗️build/📜️script.ts` in the sealed tree, holders and admission present: the job-slots module is outside the wrapper closure (driver only), so the sealed digest of SY1 and SY2 is the same set.
4. SY1 smoke repeated on `ov3`: holders registration, peaks (2048/1250/wasm 6144), retries 3/2000, peak file round trip 800, closure of a missing dir `Map {}`, reuse tally `{local:2, fleet:1}`: all as in SY1.
5. Job plan smoke in `ov3` (`planComponentJobsV1`, local 1, 2048 MB unit): fleet slots 11 -> 12 jobs (1 local + 11, `jobs-all`); 22 -> 23; 40 -> 24 (`jobs-cap`); fleet down (`null`) -> 1 (`jobs-local-only`); 0 slots -> 1; warn pressure -> 6 (1 + 5); critical -> 1; low memory 8 GB -> 12. EN text line prints. `nativeBuildPassOffloadableV1`: `wasm32-wasip2` true, `aarch64-apple-darwin` false.
6. Path lengths: longest new path 149 bytes.

## Limiting the concurrent component jobs for the first run
There is NO environment variable and no policy flag for the component job cap. The only knob in the snapshot code is the schema constant (grep: `job-slots/🟦️.ts` has no `process.env`/`environment`; `driver-stages` and `component-pass` pass only `local`, fleet slots, memory, pressure into the plan). The job count counts local + remote:

- File (after sync): `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧬️schema/🔣️.json`, line 12: change `"componentJobsMaximum": 24` to `"componentJobsMaximum": 6`.
- Verified in `ov3cap/` (same edit, plan run): 11, 22 or 40 fleet slots all give `{"jobs":6,"local":1,"remote":5,"reason":"jobs-cap"}`; fleet down stays 1. The JSON is imported at module load (`with{type:"json"}`), so the edit must be in place before the driver process starts. The value is not a cache-key input. (One `sed -i '' 's/"componentJobsMaximum": 24/"componentJobsMaximum": 6/' <file>` after copying `job-slots-schema.json`; the report copy `SY2-patched/job-slots-schema.json` keeps 24, the authored value, because main must not diverge.)
- Other bounds that already apply on top (all read by the code): `jobSlots = min(plan.jobs, tokens + fleetRemote)` (`component-pass/🟦️.ts:515`), where `fleetRemote` comes from `planFleetJobsV1` (own schema cap `fleetJobsMaximum` 48, `🏗️build/🎟️capacity/🧮️units/🧬️schema/🔣️.json:23`); mode `never` of the offload setting (`OFFLOAD_POLICY_V1.mode.environment`, `offloadModeV1(environment)==="never"`) makes `fleetIdleSlotsV1` return null so the plan is the local grant (1 job): a total off switch, not a cap. Local `tokens` (`SEMIO_BUILD_PARALLEL`) only changes the local share, not the cap.
- Recommendation: set cap 6 for the first run (O10 saw one lock cycle at 12 concurrent wasm cargos on a shared build dir), raise once clean.

## What must be restarted
- Wrappers (rustc shim -> `🏁️entry.ts`, offload client, store registration, peak writer, paths/inputs planner): new process per rustc call, new sealed copy `sealed/<digest>`; no restart, only avoid copying mid-unit (stop chain, copy, start).
- Native-build driver: `component-pass/🟦️.ts` (tally + job slots), `driver-stages/🟦️.ts` (componentJobs callback), `phase-ledger`, `job-slots` are loaded once per driver process: the play chain driver must be (re)started to use them (prints `R reused (A local, B fleet)` and `[component-jobs] ...`). The cap edit (above) also needs this restart.
- Cache sidecar (:6094): CAS2 holder protection needs a collector running the new code (SY1 note: sidecar already restarted 17:07 with CAS2; clone re-registers once it runs the new store).
- Nothing needs a scheduler or worker restart. O10's placement/scheduler peak-aware edits are NOT in this list: an old scheduler ignores `peak` so fleet slots are action slots only (over-optimistic, placement still gates, jobs wait), which is why a cap such as 6 is prudent.

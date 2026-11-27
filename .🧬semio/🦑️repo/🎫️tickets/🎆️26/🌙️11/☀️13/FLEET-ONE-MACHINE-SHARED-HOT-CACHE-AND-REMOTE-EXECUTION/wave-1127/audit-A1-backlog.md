# Audit A1: Open-Items Backlog (fleet, from wave 1125)

Lane: A1 of wave 1127 (read-only, foreground). Inputs: `wave-1125/HANDOVER-1125.md`, `wave-1125/coordination.md` (all), every `wave-1125/result-F*.md` and `findings-F*.md` (sections Open, Not verified, Not done, Follow-up, Remaining, Round 2, Post-seal), `landings.md` (grep for closure), `wave-1127/brief.md` (live state 00:00 11/27).

Status legend: OPEN = no landing or evidence closes it. LIKELY OPEN = evidence suggests still open, not re-run. PARTLY = core landed, named remainder open. VERIFY = probably closed by a later landing, check before acting. `[USER]` = only the user can do it.

## 1. Live state and graph seal (highest priority)

- G1 OPEN - Whole graph NOT sealed. Last seal read restarted 18:45 11/26 (graph8), no "seal done" entry in landings.md. Owner in wave 1127: H9. Source: coordination.md 18:45; HANDOVER-1125 "Open 1".
- G2 OPEN - After seal: `repo run repo-mcp:build` then `repo run repo-mcp:install`, then restart the repo MCP server. Source: HANDOVER-1125 State 21:07 and 23:28; coordination POST-SEAL (1).
- G3 OPEN - Seal read dies on churn/conflicts (torn owner registry, BUILD_SOURCE_CONFLICT, "Original Graph format identity changed", SIGKILL at store phase). Only partly fixed by F53, F58, F113, F115, F118. Source: coordination 01:21, 06:07, 06:56, 18:45.
- G4 OPEN - TODO from coordinator: classify "Original Graph format identity changed" as source-transient (bilingual, auto-retry once) instead of Internal error. Source: coordination 18:45 TODO.
- G5 PARTLY - Graph read SIGKILL (EXC_BREAKPOINT, 3 crashes). Death-note (F118b) landed; root cause not reproduced. Open risk: gate waits once per 120 s; policy decision (over-grant hard state, gate tier above 100 %, no heal while swap >80 %) not taken. Source: result-F118.md "Open risk" and "Follow-up 2"; findings none.
- G6 OPEN - BUN_JSC_forceRAMSize lead not confirmed; next lever is one real read with the ramSize raised to the grant. Source: result-F118.md "Follow-up: BUN_JSC_forceRAMSize".
- G7 OPEN - Schema catalog regeneration not done (stalled on graph read). Source: result-F105.md Not done.
- G8 OPEN - Heap per read still unproven flat (retainer fixes F55/F100 round 3 landed, but no whole read re-measured). Source: result-F100.md Not verified; coordination 15:30.
- G9 OPEN - Read parent heap census and per-wave cost; F130 tier thresholds (32 MB/s, 1024 MB, critical) are guesses to tune from the next read. Source: result-F130.md Not verified.
- G10 OPEN - `repo cache dedupe` (runDedupeV1) walks and clones below mounted images with no shared lease (outside the lease law). Source: result-F121.md NOT verified; result-F124.md.
- G11 OPEN - Worker agent's own writes to its home (blob fetch, private roots, self-retirement) take no image lease. Source: result-F127.md NOT verified.
- G12 OPEN - Cargo, nx-tooling and graph owners still read/delete in images with their own custody (not under the shared lease). Source: result-F127.md.
- G13 OPEN - Law allowlist keeps 13 documented unrouted entries; `unrouted` means the law is green while readers are unleased. Source: result-F124.md; landings F116 correction.
- G14 VERIFY - Direct readers bypassing the shared lease (sidecar uploader, progressAtMs, head adoption) listed in F114; landing "F116: every direct reader takes its shared lease" suggests closed. Source: result-F114.md; landings.md "Wave 1125 F116".
- G15 OPEN - Schema-module-id gate stuck at 99 %; 1673 schema-module-id-missing; 1985 definition modules without row. Source: result-F119.md Not done; result-F107.md Not verified.
- G16 OPEN - `schema generate` still non-atomic writeFileSync. Source: result-F107.md notes.
- G17 OPEN - Rust checker: 1983 residual rust incomplete rows (other macro forms, real absences). Source: result-F111.md Not done.

## 2. POST-SEAL QUEUE (from coordination.md 13:10, 15:30; 11/26)

- Q1 OPEN - repo-mcp build + install, then "seal done" to F82: Rust tests `cache_retirement_tests` (engine), compile of `semio-plugin-host-guest-runtime`, clippy not run. Source: coordination 13:10 (1); result-F82.md Not verified; no landing with that name in landings.md.
- Q2 OPEN - Land F74 lease reentrancy fix (verify takes SH under the same task's EX in cleanup/retain, `cleanup/🎛️command/🟦️.ts:190`); staged only. Also Nx fixture trim-paths unverified; units test rerun. Source: coordination 06:58 (POST-SEAL), 13:10 (2); result-F74.md item 1; `wave-1125/../🧰️scripts/F74/stage-mapping-reentrancy.md`; landings.md line ~8867 "Not landed".
- Q3 OPEN - F75 owed tests: closures main (VERIFY, closed by F76: 27/0), storage journal tests, nx journal tests, publication tests not run. Source: coordination 06:58 and 13:10 (3); result-F75.md Not verified; result-F76.md.
- Q4 OPEN - Native-dependencies last law needs a real cargo compile; output overflows the 4 MiB owned-capture cap. Source: coordination 13:10 (4); result-F92 (reading/derivative/native deps); result-F99.md Still red (`Owned data capture output limit exceeded`).
- Q5 OPEN - Cargo testing tests (`🏃️process/🧪️testing/🦀️cargo/🧪️tests`, lock) need SEMIO_SCRIPT_PROVIDERS from the suite/project env declaration. Schema has the Environment const, no evidence the suite declaration is wired. Source: coordination 15:30 (5); result-F102.md.
- Q6 OPEN - F74 round-1 and F72 rows 7, 8 (another mapping question) and row 5 (dead-API decision) still open. Source: result-F72.md Not verified; coordination 06:25.

## 3. Tests red (current, from wave 1125 results)

- T1 OPEN - Budget tests `unknown`, `key-reference`, `inference`, `publication-fallback` (one 4096-byte block short); groups 12/1. Source: HANDOVER-1125 Open 4; coordination 21:48 (F39).
- T2 VERIFY - Headroom load timeouts; step-overdue was 18/4, later 22/0 per F57 (coordination 03:11). Source: HANDOVER-1125 Open 4.
- T3 OPEN - `🎯️acceptance/🧪️tests/🎯️goal-gate` fails before first test (Ajv draft-07 `$schema`). Source: result-F107.md Not verified.
- T4 OPEN - Native reading 7 ("Store settlement isolated process laws", frozen exit code) and derivative 5 (live store guard). Source: result-F92.md Remaining reds.
- T5 OPEN - Native-dependencies boundary regression: `📥️inputs` imports opaque-source helpers from `🔍️discovery/`; fix is to move the helpers out. Source: result-F92.md Remaining reds (commit 8d17d8ba399).
- T6 OPEN - extracted-function ceiling ratchet red (12 new `new Function(` in peers). Source: result-F92.md.
- T7 OPEN - `📦️artifacts/🧪️tests`: 2 fails (native receiver shell `.cache/b/l` holding `m/data`). Source: result-F93.md Not verified.
- T8 OPEN - `⚡️caching/🧪️tests/📦️native-dependencies`: output limit (see Q4). Source: result-F99.md.
- T9 OPEN - Admission test file 17/2 (real-cargo timeouts at 600 s); graph admission 19/1 (`selected Source discovery yields original control`). Source: result-F64.md Open; result-F83.md.
- T10 OPEN - `🧯️available/🎫️broker` 12/1 (sourceKlocV1 fallback expectation 5 vs 2). Source: result-F71.md Not verified.
- T11 OPEN - `operations` test 14/2 (`cache gc --reclaim` scoping; help test timed out at 120 s under load). Source: result-F70.md; result-F56 (pid list). Known.
- T12 OPEN - ~93 product type errors left (89 in `🤝️shared-worktree`, incl. child.stdout nullability in live replica daemon code). Source: result-F98.md "The 93 left".
- T13 OPEN - `ac1018` editor check and unit-ceiling 9/2 (carving list changed by a peer); admission 17/2. Source: coordination 11-26 05:06; result-F60.md.
- T14 OPEN - Generations load test "progress counters" 113/1 (`build:graph-phase` resource moved to state.detail). Source: result-F53.md Not verified.
- T15 OPEN - Sealed/signed-fleet tests unrun after F47 (signed-fleet cluster); sidecar signed-scheduler 6/1 and `slotQueueMilliseconds` fixture needs pid+startMs tickets. Source: result-F47.md 5; result-F52.md Not done.
- T16 OPEN - `native receiver` test `🏗️build/🏡️resident/🧪️tests/🌐️host` fails: file over 64 KiB pin (peer). Source: result-F116.md NOT verified.
- T17 VERIFY - Type errors: library 0 (F95), product 372 then 93 left (F98), declaration gaps F66/F88/F91, declarations `reservationOwner` product error (F70). Source: results F66, F70, F88, F91, F95, F98.
- T18 PARTLY - Test-file type errors: `⚡️caching` 84, `🎛️commands` 21, graph 154 and build-outside-graph ~290 at F88 time; later lanes cut library errors to 0 (F95). Source: result-F88.md Still open.
- T19 OPEN - `whatwg-url` in wasm-outputs test: keep as oracle; no action unless user decides. Source: result-F91.md Notes. (Decision, not bug.)
- T20 OPEN - Step-overdue 1 failing test (verified-memo watch) and pre-existing `inventory` semantics. Source: result-F25.md; result-F41.md.
- T21 OPEN - Test fixtures typed via declarations; `🧪️testing` typed-fixture loader for the remaining ~201 test-local errors not started. Source: result-F66.md Remaining.

## 4. Low-resource workers, warden, memory

- L1 OPEN - Over-grant state: child stands 1.5-2 GB above grant for the last phase of inference; policy (stop admitting projects while footprint >100 % of grant, release between waves, no cooldown) not taken. Source: result-F118.md Follow-up 2.
- L2 OPEN - Underlying pressure not removed: 16 GB workers swap at 20-146 MB/s while 3.6 GB slice worker and cargo run together. F128 only makes the stage back-pressure safe. Source: result-F128.md Not done.
- L3 OPEN - Warden libraries above their 1536 MB ceiling not on the carving list (e.g. process3d host 1750 MB on m01); swap-high on m03 m04 m07 m08 m10. Source: wave-1127/brief.md live state; result-F15 (dwg/binary_io/sourcing not split).
- L4 PARTLY - Carving: dwg_snapshot_objects split landed (F24); `binary_io_registry` split not done; sourcing (4273 MB) not analysed; puzzle not analysed; consumer workspaces not compiled. Source: result-F15.md Section 4 and Not verified; result-F24.md.
- L5 OPEN - `unit-ceiling` remeasure after build, remove old carving entries. Source: result-F24.md Not verified.
- L6 OPEN - F13 cad/puzzle beyond-host placement: no live fleet queue run; scheduler placement on coordinator depends on free memory. Source: result-F13.md; result-F60.md Not verified.
- L7 OPEN - Admission grant growth: waiter arriving while cargo small does not re-evaluate growth. Source: result-F64.md Not done.
- L8 OPEN - ~30 unit-cache wrappers holding :6094 connections, cargo at 0 % CPU (wedge, pids 69285/69392); cause not analysed. Source: result-F64.md Open.
- L9 OPEN - Broker 38325 never answered admit for syn; needs broker-side watchdog. Source: result-F69.md Open.
- L10 PARTLY - Broker deadlines landed (F71), Windows reader thread cannot be cancelled by closing the named pipe. Source: result-F71.md Not verified.
- L11 OPEN - Memory-retention of graph per-node witness arrays unconfirmed. Source: result-F100.md Not verified.
- L12 OPEN - Slice pre-inventory 7 min cost; resumable per-attempt inventory; path-journal re-verification not built. Source: result-F5.md Not done; result-F12.md Not done.
- L13 OPEN - Pre-inventory window unlabelled (F20 found the fix, F25 landed labels) - VERIFY that labels now show; host-wait phase live unmeasured. Source: result-F20.md NOT done; result-F25.md Not verified.
- L14 OPEN - m05 binding cost 9-14 s per binding unexplained; batching not done; m05 post-landing behaviour unobserved. Source: result-F14.md Not changed.
- L15 OPEN - Windows m05 supervisor wedged (ready.json stale since ~23:30): F59 fallback is USER-facing. Source: coordination 11-26 02:32; result-F59.md NOT verified.
- L16 OPEN - Slice-worker heap handovers (F19 paged facts) if handovers persist: next retention is pass result holds facts (~195 MB remaining). Source: result-F19.md Not verified.
- L17 OPEN - Workspace-stage failures on m01/m14 (`framework-rs:generate:prototype`, `framework-actor-rs:typegen`, `norm-din4108-rs:build:prototype`, Nx 130) and m03 `nativeImageShardMountedV1 is not defined`. Source: wave-1127/brief.md live state; F33 m01 vanished-directory race OPEN (result-F33.md Not verified).
- L18 OPEN - m05 Windows: EPERM unlink of esbuild.exe; partition capacity exceeded. Source: wave-1127/brief.md live state; result-F10.md; result-F17.md.
- L19 OPEN - m02/m15 BUILD_SOURCE_CONFLICT (replica churn): retry landed (F58), not proven live. Source: coordination 11-26 02:32; result-F58.md.
- L20 OPEN - m12 "Exact live repository partition cache owner required" / "Actual child funding journal descriptor" (F22, F32) landed; cause unproven. Source: result-F22.md; result-F32.md; coordination 18:08.
- L21 OPEN - Closure index descriptor changed on m12 (F75 landed; live recheck unverified). Source: coordination 06:31.
- L22 OPEN - Ready count below 16 (Ready 9/15 at 23:28; 8/16 at 00:00 11/27). Source: wave-1127/brief.md; HANDOVER.

## 5. Remote cache (unit cache, CAS, Nx)

- R1 OPEN - Unit-cache cross-origin key divergence unproven (no same-slot pair with components yet; itoa slot lists 0 keys). Next: grep `[units] miss` in S/origins/*/ready-log-workspace*.txt or diff-slot.ts itoa. Source: result-F40.md NOT done; coordination 19:54 and 20:07.
- R2 OPEN - wasmparser rmeta divergence (rlib equal, rmeta differs by 87 bytes): not diagnosed; per-user cargo home suspect. Source: result-F11.md Open class.
- R3 OPEN - Nx hit rate 0.2 % (Nx hash proof missing); Nx hashes with spawn-only rows effect unverified. Source: result-F11.md; result-F16.md Nx unproven; result-F38.md Not verified.
- R4 OPEN - Key spelling for AC key: why both spellings equal not traced. Source: result-F11.md.
- R5 PARTLY - Namespace `semio` writers (m06, m07, g1-server) still exist; F16 added namespace main + refusal; writer unification not verified. Source: result-F11.md; result-F16.md.
- R6 OPEN - Cache credentials: m11/m12 `unit-cache-denied` (448/409 units expired unshared), g1-server read-only (5053 units unshared). Owner H2 in wave 1127. Source: wave-1127/brief.md live state.
- R7 OPEN - Cache counters at 00:00 11/27: skipped 77k (spool-full 27.9k, upload-entry-gone 18.0k, upload-failed 14.7k, upload-expired-down 7.9k, key-incomplete 7.3k). Source: wave-1127/brief.md.
- R8 OPEN - Remote-cache entry size: values >4 MiB stream; 'remote-cache' edit landed without lock; no direct test for cache-gc runner, store lock, doctor ports. Source: result-F56.md; landings line 69.
- R9 PARTLY - CAS 507 (volume 96 % full) and admission deadline 15 s vs client 20 s: F79 landed; relief only in fleet cache sidecar (F82), other machines via readiness supervisor. Source: result-F82.md Not verified; coordination 10:30.
- R10 OPEN - Fleet cache facts endpoint list still probe-and-failover; `resolveFleetEndpointV1` switch is follow-up. Source: result-F44.md Not verified.
- R11 OPEN - Wasmtime cwasm cap and store tmp sweep: gap 1 is a Rust edit in the wasm host (needs its own lane with cap + prune test). Source: result-F80.md (Recommended follow-up lane).
- R12 OPEN - Unit cache misses after rebuilds: grep `[units] miss` (F40 round 2 logs landed, no lines yet). Source: HANDOVER-1125 Open 5; coordination 20:07.
- R14 OPEN - Duplicate fleet-service `[units]` miss logging needs origin ids (m10/m13/m14 share a host name). Source: result-F40.md Notes; closed by F47 origin ids once hostnames applied (see U5).

## 6. Remote execution (offload, sidecar, scheduler)

- E1 OPEN - Offload: out 6531, refused 895, fellBack 3029; reasons local-slot 12.8k, cheap 5.1k, unmapped-path 3.3k; queue-timeout 176; no-idle-machine 180; pin-failed 676. Source: wave-1127/brief.md live state.
- E2 OPEN - Live offload proof: only one completed offload since scheduler restart 21:52 (23:20). Source: coordination 23:13 (F50); 23:28 (F52) "live offload 1 completed".
- E3 OPEN - Remote-execution headroom: CPU ceiling 1.5/core + 2 reserved server slots landed; unit offload before-fallback queue `queueSaturatedMilliseconds` 120 s (may need raise). Source: result-F13.md Not verified; coordination 17:12.
- E4 OPEN - Sidecar restart drain fields (`drainMaximumMilliseconds`) and Windows restart-on-failure (conhost loses exit code, RestartOnFailure never fires): check. Source: result-F10.md Not done; result-F91.md.
- E5 OPEN - Scheduler: multiple registered workers per origin (restarts leave old ids), expiry only by lastSeen; `no-endpoint` 21:56 not reproduced. Source: result-F50.md Follow-ups.
- E6 OPEN - Worker supervisors restart on replicated code; Windows launch path (`--%`, `-EncodedCommand`, CREATE_NO_WINDOW) unproven. Source: result-F17.md NOT verified.
- E7 OPEN - Go suite slow (not hung); launcher TestLauncherReadsItsStateBelowTheDeclaredDirectory fixed by F29, Go suites green 18:27. Source: coordination 18:27. VERIFY.
- E8 PARTLY - Offload `unmapped-path`, `not-portable`, `worker-policy`, `incomplete-inputs` reasons not analysed. Source: wave-1127/brief.md live state.
- E9 OPEN - Remote offload of cad/puzzle: no live run; F60 beyond-host gate covered by unit tests only. Source: result-F60.md.
- E10 OPEN - Lease self-deadlock (3 tests in F72; F74 fix not landed). See Q2.

## 7. Monitoring and fleet status

- M1 OPEN - Monitoring readers `source-failed` for `storage` and `workers` (no answer in 5 s), generation null; H5 in wave 1127. Source: wave-1127/brief.md live state.
- M2 OPEN - Fleet status host-name conflict display: gate state only in addresses.json; closed schema. Source: result-F47.md 5.
- M3 PARTLY - Host name column landed (F48); g1-server still publishes old label. Source: coordination 23:02.
- M4 OPEN - Live truth for stall counts (`ready-timings.json` stalled counts) and worker outcomes after F5/F25/F26 unmeasured. Source: result-F5.md Not verified.
- M5 OPEN - Monitoring: heap census names retainer after next read; F36 diagnostics now (F55) silent-fix done; census not re-read. Source: coordination 11-26 02:34; result-F36.md.
- M6 VERIFY - Monitoring ticket 26/11/26 FLEET-MONITORING closed with summary.md (M8 done). Source: coordination 06:02. Likely closed.
- M7 OPEN - F130 thresholds to tune from next read. See G9.
- M8 OPEN - Remaining landings verification for the fleet-status `stale readiness beat` line (F59) on m05. Source: result-F59.md NOT verified.

## 8. USER-only actions (cannot be done by agents)

- U1 USER - m04 Linux inodes: `df -i /home/deck/semio/.cache` and `sudo du --inodes -x -d3 /home/deck/semio 2>/dev/null | sort -n | tail -20`. Cold inode need 428,796 vs ~122k free. Source: HANDOVER-1125 USER; result-F4.md Round 4; coordination 17:22.
- U2 USER - m04 service-bootstrap sudo (privileged step). Source: wave-1122/result-E64.md; HANDOVER-1125 USER.
- U3 USER - m04 EACCES fallback only if it recurs: `chmod -R u+rwX .../bun-cache` and check owner. Source: result-F3.md USER.
- U4 USER - g1-m09: run `prompt-g1-m09.md` on that Mac (never provisioned). Replacement g1-m15 is in workspace. Source: HANDOVER-1125 USER; coordination 23:13.
- U5 USER - Hostname: run `repo fleet hostname apply` on the coordinator and on m08/m10/m13/m14 (shared `Uelis-Mac-mini` LocalHostName, dialogs). Source: HANDOVER-1125 Update 23:28; coordination 22:07; result-F51.md (sudo path not run).
- U6 USER - Re-provision with name-based store mount (result-F44.md, then result-F47.md); avahi on Linux. Source: HANDOVER-1125 USER.
- U7 USER - m05 Windows: Defender real-time exclusion for `C:\semio`; Windows task checks; confirm popups gone. Source: result-F10.md; result-F17.md; result-F14.md; coordination 16:45.
- U8 USER - m05 fallback if readiness supervisor still wedged (F59). Source: result-F59.md.
- U9 USER - Archivist: clear 2738 staged stray index entries (replica/sa, -inf) at next commit. Source: result-F8.md; coordination 16:50.
- U10 USER (optional, not stale-proven) - Docker data 70 GiB (daemon off since Nov 16), Zotero 22 GiB, Claude VM bundles 11 GiB on the coordinator. Source: result-F80.md; coordination 10:30.
- U11 USER - macOS dialog cleanup "local hostname in use" (user dismisses; optional sudo scutil per duplicate Mac). Source: coordination 22:07.
- U12 USER - Confirm Ready staying for m05 and m04 after inodes; m02/m03 workspace outcomes are agent work. Source: HANDOVER-1125 Open 2.

## 9. Likely closed by later landings (verify only, do not reopen)

- C1 VERIFY - F1 `#[path]` debug mounts (landed; crate-refused gone fleet-wide per coordination 16:43).
- C2 VERIFY - F10/F17 Windows spawns hidden (landed; not run on Windows).
- C3 VERIFY - F31 50 build-owners rows landed in F35 train; F53/F115 owner registry writer.
- C4 VERIFY - F41/F42/F45 pid+startMs identity; F56 baseline 63 -> 1.
- C5 VERIFY - F47/F50/F52 resolver, origin naming, per-launcher lease.
- C6 VERIFY - F76 closures main 27/0; F81 units test; F93/F94 native artifact and product pool.
- C7 VERIFY - F97 inventory via change journal (warm 0.15-0.42 s).
- C8 VERIFY - F99 library typecheck 0 (F95).

## Summary of counts
OPEN, LIKELY OPEN or PARTLY: roughly 90 items (dedupe by section). USER-only: 12 (section 8). Most urgent for the coordinator: G1-G2 (seal + repo-mcp), Q2 (F74 lease reentrancy), Q4 (native-dependencies cap), L1 (over-grant policy), E2 (live offload proof).

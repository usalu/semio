# Result E4: unit-cache skips, upload spool, peer tier

Paths: `C` = `…/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache`, `B` = `…/📚️library/🏗️build`. Landing record: `../landings.md` (section E4). Edits were made before I read the 00:56 per-area lock addendum; locks were taken afterwards (disclosed in landings.md).

## Measured (this machine, `unit-cache stats` over 45 lanes, 44 h)
- Skip events (remote-skip etc.): breaker (cache down/denied) 4631, fault 744, upload retries 1270, upload-dropped 2296, path-dependent 83, diverged 6, upload-failed 77, publish refused 21, bypass no-artifact-messages 59. Counter `skipped` 5677 and each fleet machine's `cacheSkipReasons` were `{}` because the reason files did not exist when those skips were counted (older wrapper) .
- Class distribution: ~80% of skips are outage related (units compiled while the cache breaker was open were never queued; the uploader also dropped queued jobs while the breaker was open). Only ~1% (path-dependent) needs compiler-output work (E3).
- Misses exceed stored entries (251k misses vs 114k entries): consistent with ~100k units never uploaded, not with key mismatch (platform key is identical for all macOS workers; key construction is path-portable and covered by existing cross-path tests).

## Changes
1. Reasons: skips already counted per reason; now guaranteed to add up: `accountedSkipsV1` (C/🛰️fleet/📊️counters) shows counts without a recorded reason as `unrecorded`; counters schema declares `skipReasons`. Live: `readBuildCountersV1` now returns `{"bypass-no-artifact-messages":1,"upload-failed":1,"unrecorded":5677}` (sum = skipped 5679); fleet status shows it per machine once those machines run the code.
2. Avoidable class removed at the root (outage skips): `prepareFleetV1` no longer refuses while the breaker is open; the compiled unit is queued (`shareFleetUnitV1` no longer requires a usable client); the uploader no longer drops jobs while the cache is unreachable but queues them again (due after the retry delay, no attempt used), exits after its idle time and counts only jobs past their age (`upload-expired`, `upload-expired-denied|down`); reasons `breaker-*`/`upload-dropped` are gone. `unsharedReasonV1` is now `read-only` or `spool-full`; `fleetSkipReasonV1` became `fleetUnidentifiedReasonV1` (`no-compiler-identity`). Fleet-status alert `unit-cache-denied` keys on `upload-expired-denied` (en/de text adjusted).
3. Client bug found by the e2e test: a 401 on PUT never marked the breaker `denied` (only GET did), so a wrong credential burned all upload attempts in 60 s; `call` now marks denied on 401.
4. Peer tier: not redundant with the server CAS; it is the cross-group read-through for topology groups g2..g5, which do not exist live (mounts `/Volumes/semio-peers/g2..g5` are empty, no attested facts). Every read asked four unrouted peers, counted errors, paused them, and counted `skipped` (12683). Now a transport has `routed()` (direct: has an endpoint; relayed: store mount holds `v1`); unrouted peers are never asked and counted in the new status field `absent` (schema `ServerStatus.peers.absent`); misses/errors/skipped only count routed peers. No code removed: the tier is needed once a second group exists.

## Tests
peers 15 pass (new: absent peers); unit-cache fleet 18 pass (updated denied-credential test: units wait in the spool, none dropped or counted, uploaded with the right credential); uploader 3 pass (new: outage defers, upload on return, expiry counted); portable 26 pass (new: unrecorded accounting, new unshared reasons); fleet-status 15 pass (TS and Python oracle); dependants: graph-state lookup-fault 2, remote-cache restart 3, owner-pool early-wave 7. Live: `repo cargo check -p semio-framework-pack-json-source --store=shared` ok; counters read back as above.

## Not verified
- The server's live peers counters will change only after the group server restarts on the new code; not observed.
- Cross-origin key comparison from receipts: no unit receipts or other origins' lookup logs exist in the store; by-code review and existing cross-path tests only. To compare, set `SEMIO_UNIT_CACHE_TRACE_KEYS=1` on two origins and diff with `unit-cache churn`.
- No measurement yet of fleet hit-rate after the change; no Windows/Linux run.
- Root-keyed units during an outage still count `root-key-failed`.

# Round 2

Locks: caching-cargo, execution, fleet-status, remote-execution taken before editing; `fleet-status` released as soon as its part was tested (E8 asked), the others after verification. Landing record: `../landings.md` (sections "E4 round 2").

## 1. Cross-origin key diagnosis (done, live)
- New member `C/🛰️fleet/🔑️key-samples`: each origin's upload worker publishes, at the end of every drain, the digests of the key inputs of its most recent misses (newest miss per unit = crate + slot, at most 400 units): one digest per component (`toolchain`, `source`, `platform`, `roots`, `extern:<name>`, `env:<NAME>`, `args.<class>`), no path, argument text or value (schema `KeySamples`, pattern-checked as hex; a path is rejected). The document is one content-addressed blob in the fleet cache; only its reference `{blob,count,atMs}` travels through the counters path (`build-counters.json` -> load report `MachineCounters.keySamples` -> scheduler table). A read-only machine publishes nothing; nothing is sent when no component changed. `slot` is now recorded in the lookup log of every miss.
- View: `repo fleet status` reads every reference, fetches the blobs (size, sha256 and shape checked, a bad one is left out), compares the same unit across origins and shows `fleet.keys` (origins sampled, units seen on several, units with different keys, differing components by count) plus one line per differing unit (en/de), and a warn finding `unit-key-machine-dependent` for units whose source is the same everywhere but whose key differs (the machine-dependent case), naming the components.
- Proven: unit tests (collect, validation against the schema by Ajv, naive cross-check of the comparison, publish/republish/read-only, tampered blob), fleet-status TS + Python oracle with the closed schema. Live: g1-m02, g1-m10, g1-m12 and g1-server published (64, 61, 64, 1 units), the collector read all four (`keys.origins = 4`).

## 2. Comparison of >= 3 macOS origins (NOT achieved)
- The four published samples share no unit (`units: 0`): each machine's recent misses are different crates, so no unit was missed on two origins in the window (the 64-unit samples cover minutes). The bound was raised to 400 units to widen the window (landed 01:55); machines republish as they compile. Check later with `repo fleet status` (or `…/⚡️caching/🦀️cargo/🗃️unit-cache` `unit-cache churn`): any `unit-key-machine-dependent` finding names the component to fix. Until then no machine-dependent component is confirmed; by code review the platform key (toolchain, macOS major for linked units) is the only host-dependent input and all workers report the same macOS platform key. Nothing for E3 yet.

## 3. root-key-failed during outages (done)
A unit that holds a directory of the machine is now keyed locally (the leak scan and the root paths need no cache). While the cache is unreachable it is queued under its root key with its root hint (`SpoolJob.hint`); the upload worker publishes the hint (`publishRootHintByKeyV1`) before it uploads the unit, and drops the job only when the fleet names other directories (`root-key-conflict`). Counter `root-keyed-deferred` replaces `root-key-failed` in that case. Test (rooted, e2e): producer builds with a dead cache, hint+unit upload later, the second machine at the same root compiles nothing and hits the root-keyed unit.

## 4. Hit ratio trend (foreground snapshots 01:23:48 and 01:53:58, `repo fleet status --json`)
Hit = local hits + fleet hits as the scheduler reports them, deltas over 30 min:
| origin | hit | miss | skipped | hit ratio |
|---|---|---|---|---|
| g1-m02 | 132 | 446 | 21 | 0.23 |
| g1-m05 (Windows) | 0 | 125 | 25 | 0.00 |
| g1-m10 | 267 | 553 | 2 | 0.33 |
| g1-m11 | 2231 | 2948 | 25 | 0.43 |
| g1-m12 | 4190 | 2622 | 1083 | 0.62 |
| g1-server | 1 | 10 | 4 | 0.09 |
| g1-m01, m06, m07, m08, m13 | 0 | 0 | 0 | idle or not reporting |
| g1-m03 | (absent in the first snapshot; its absolute 21665 / 29575 is not a delta) | | | 0.42 cumulative |
Fleet delta without g1-m03: 6821 hits, 6704 misses (0.50); cumulative before 0.362. Skipped fell sharply (about 2200 skipped, and 1083 of them on g1-m12 whose reasons are still pre-change `breaker-down`; machines on the new code add almost none). Only 5 of 11 machines compiled in the window, so the trend is not yet representative.

## Not verified
- The fleet-wide effect of 400-unit samples and of the deferral on the hit ratio (needs hours of builds on the new code).
- Windows/Linux publication of samples.
- The scheduler on the group server was started before round 2 (00:19); it accepted the new counters field during the window (all four origins visible), so no restart was needed.

# Round 3

Locks taken per landing and released after each (caching-cargo, fleet-status); landing records in `../landings.md` ("E4 round 3", 3b, 3c).

## 1. Overlap and machine-dependent components
- No overlap for 20 min with 64-unit samples of recent misses (02:02-02:17). Widened as asked: samples now also hold, for the crates of a fleet-wide rotating bucket (sha256 of the crate name, 8 buckets, 30-minute window), the newest lookup (hit or miss) of every unit, up to 400 units. At 02:22 five origins overlapped (39 units; later 269 units over 6 origins).
- Result (all 265 compared units have the same source and differ anyway): `args.metadata` 223, `args.diagnostics` 127, `args.unstable` 2 and `env:CARGO_ENCODED_RUSTFLAGS` 2 units (externs differ downstream of those). Pairwise: g1-m10 vs g1-m12 share 194 units and all differ only in `args.diagnostics`; g1-m02 differs from m10/m11/m12 only in `args.metadata` on 104/109, 41/44 shared units.
- Fixed: the rendering flags of the compiler messages (`--color`, `diagnostic-rendered-ansi`/`diagnostic-short` in `--json`) no longer enter the key (`normalizeRustcArgumentsV1`, Python oracle, test). Consequence: every key changes once, so the fleet cache is cold once.
- Not fixed, root cause not proven: `args.metadata` of g1-m02 (cargo hashes package id, features, profile, rustflags and all dependencies' metadata; a configuration outside the tree is the suspect: this machine has a `~/.cargo/config.toml` that the repository does not carry and Cargo concatenates it), and the rustflags/`-Z` difference of the wasm host units on m06/m11/m12. To make them provable the samples now carry the flag texts (`values`) of the small classes and the rustflags environment (paths replaced by `<path#digest>`) and `repo fleet status` prints them per differing component; the other machines had not republished with values by 02:36. Details and next step in `findings-E4.md`.

## 2. Windowed hit ratio (snapshots 01:53:58 and 02:35:28, 41 min)
| origin | hit | miss | skipped | ratio now | ratio 01:24-01:54 |
|---|---|---|---|---|---|
| g1-m01 | 0 | 695 | 7 | 0.00 | idle |
| g1-m02 | 579 | 2175 | 136 | 0.21 | 0.23 |
| g1-m06 | 824 | 1795 | 498 | 0.31 | idle |
| g1-m10 | 837 | 742 | 2 | 0.53 | 0.33 |
| g1-m11 | 1586 | 1097 | 24 | 0.59 | 0.43 |
| g1-m12 | 5200 | 1627 | 96 | 0.76 | 0.62 |
| g1-server | 2 | 16 | 0 | 0.11 | 0.09 |
| m03, m05, m07, m08, m13 | 0 | 0 | 0 | idle | |
Fleet: 9028 hits, 8147 misses, 763 skipped = 0.53 (previous window 0.50 without g1-m03, 0.44 with it; cumulative before 0.36); skipped per minute fell from about 73 to 19. The key fixes of this round land after the window, so the ratio should rise once machines run them; g1-m02 stays low (its metadata differs).

## Not verified
- The effect of the rendering-flag fix on the hit ratio and that the diagnostics difference of m10 is really colour/ansi (the value texts were not yet published by the other machines).
- The cause of m02's metadata difference.
- Pre-existing failure in the key tests (SDKROOT neutralisation) and in the churn tests (proc-macro extern identity): not touched by E4.

# Round 4

Locks: `process` for the registry change (released); `caching-cargo` was held by E3 (round 5, unit key over all module files), so my unit-cache edits this round are limited to the test oracle/test files of the key and churn tests (no source of the key was touched). Landing record: `../landings.md` ("E4 round 4").

## 1. Hermetic Cargo (done, not yet verified on the fleet)
- Cause found: Cargo reads `<ancestor>/.cargo/config.toml` of every parent directory of the working directory, and the repository lives below the home directory, so `~/.cargo/config.toml` of a machine is merged into the repository's configuration (array values such as rustflags are concatenated). Experiment on the pinned nightly: a hostile `~/.cargo/config.toml` adds `--cfg hostile_user` to the compiler line; `__CARGO_TEST_ROOT=<repo>` removes it. `repo cargo` already uses a repo-owned `CARGO_HOME` (`.cache/b/r/m/cargo/home`, no config file), so the home directory was the only leak.
- Change (one choke point, `routeRegisteredCommandV1` -> `toolEnvironmentV1`, which every owned `cargo` spawn passes through): the `cargo` registry entry now denies the ambient compiler-flag and Cargo-override variables (`RUSTFLAGS`, `RUSTDOCFLAGS`, `CARGO_ENCODED_RUSTFLAGS/RUSTDOCFLAGS`, `CARGO_BUILD_RUSTFLAGS/RUSTDOCFLAGS/TARGET/RUSTC*`, `CARGO_TARGET_*_RUSTFLAGS/LINKER/RUNNER`, `CARGO_PROFILE_*`, `CARGO_ALIAS_*`) and sets `__CARGO_TEST_ROOT` to the repository root through a new schema field `rootVariables`, so the configuration search stops at the repository. The launcher's own variables (`CARGO_HOME`, `CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR`, `CARGO_NET_*`, `RUSTC_WRAPPER`, `SEMIO_CARGO_*`) are kept. The wrapper needs nothing extra: it only sees what the scrubbed Cargo passes.
- Test `🧰️tools/🧪️tests/🦀️hermetic-cargo`: real Cargo, a hostile `config.toml` in a temp HOME above the repository (rustflags, `[env]`, `[alias]`) and hostile `RUSTFLAGS`/`CARGO_BUILD_RUSTFLAGS`: the unscrubbed run compiles with them, the registry run compiles with the repository flag only. Registry, funnel, resolve, scan suites pass; live `repo cargo check` ok.
- Limits: a Cargo run without `CARGO_HOME` still reads `~/.cargo/config.toml` (the default `CARGO_HOME`); every repository entry point sets it. `__CARGO_TEST_ROOT` is an undocumented Cargo variable; the hostile-config test fails loudly if a Cargo update drops it. The existing `~/.cargo/config.toml` on this machine stays on disk (not touched).
- Verification with samples: pending. The samples (02:40-02:56) are older than the change and still show the old picture (g1-m02 vs m11: `args.metadata` on 104 of 109 shared units; m10 vs m12: `args.diagnostics`). Re-run after the workers restart their Cargo processes: any remaining `unit-key-machine-dependent` finding in `repo fleet status` names the component.

## 2. g1-m01 hit ratio 0.00 (cause not proven)
- Facts: g1-m01 counters 2781 hits / 6466 misses at 01:24 and 2781 / 7161 at 02:35 (695 new misses, no hit at all, 7 skips); it is an offload worker (`offloadedIn` 23 -> 68, worker `refusing: no-headroom`), publishes no key sample (no reference in its load report) and its skip reasons show only expired uploads while the cache was down (1) and 3 `bypass`. The cache server (restarted at about 02:45, counters since then) shows g1-m01 reading 3466 cache objects / 200 units, so it does reach the cache.
- Reading: the reads are the inputs of offloaded actions; the 695 misses are units that are new to the fleet (nobody had them), compiled by m01, plus possibly its own cold lane. Both give 0.00 without a fault. The alternative (its fleet tier unusable, breaker down) is contradicted by the server-side reads but not excluded for the unit wrapper. A decisive check needs m01's own `unit-cache stats` (fleet breaker events, lookups by outcome), which this machine cannot read; with the key samples on m01 (first drain with a successful sample publication) the comparison will show whether its keys equal the others'.

## 3. Test failures
- SDKROOT key test: real issue in the test oracle, not in the code: the key code neutralises `SDKROOT` as `⟨sdk⟩<directory name>` (another lane, `sdkIdentityV1`), the independent Python implementation still returned the raw path. Oracle updated (key environment `SDKROOT`), key tests 8/0.
- Churn test (proc-macro extern identity): already fixed by its owner (version-2 identity files) when I re-ran it; the one failing churn test now was mine (colour as an input) and is updated: colour and `--json` rendering are no input of the key since round 3 (churn 9/0).

## Not verified
- Effect of the hermetic Cargo on g1-m02's `args.metadata` and on the wasm rustflags differences (needs restarted Cargo processes on the workers and fresh samples).
- g1-m01's cause (see 2).

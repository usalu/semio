# Beyond-host evidence and claim deadline

## Root causes
1. Per-unit peak RSS (`.cache/b/r/m/u/c/<crate>.peak`) was keyed by crate only, so a stale peak of the former fat profile (5,784 MB) classed the lean-profile unit "beyond host".
2. A beyond-host unit waited up to `beyondHost.timeoutMilliseconds` (1 h) in the fleet queue; a worker that accepted the connection but never executed stalled the build.

## Fix
- `offload/🪪️profile`: `unitProfileIdentityV1` parses opt-level, lto, codegen-units, debuginfo, target, target-cpu, target-feature from the rustc args (last flag wins, `-O`, `-g`, joined spellings). `unitPeakKeyV1` = `<sha256[:10]>_<crate>`; the `_component` family suffix is preserved. `readUnitExpenseV1` and `readFamilyPeakV1` take the profile key. Offload and unit-cache script read/write the keyed evidence. Old unkeyed `.peak` files are ignored (no migration).
- Schema `Policy.beyondHost`: `claimMilliseconds` 20000 (max 60000), progress 5000, retry 2000; reason `beyond-host-refused` replaced by `beyond-host-local`; texts `beyondHost.fallback` and `beyondHost.alone` (en/de).
- `awaitBeyondHostV1`: bounded by the claim deadline, then answers `beyond-host-local` (also at once for reasons no worker can fix). `offloadRustcUnitV1` aborts an attempt not at EXECUTING within the deadline (maps to `queue-timeout`), cancellation honoured.
- unit-cache script: on `beyond-host-local` the unit compiles locally via `acquireAloneV1` (new in lock module: one claim plus every compile slot, SIGINT/SIGTERM abort, no leftover claims), under the existing memory warden.

## Tests
- Profile: JSON fixture + Python oracle + TS (stale fat peak is no evidence for lean profile).
- Beyond-host: JSON fixture cases, 300 ms unclaimed-worker test, fallback texts, cancellation.
- Alone: slots held, cancelled wait leaves nothing.
- Typecheck passed; unit-admission and gate tests still green.

## Open
`driver-stages` (component pass scheduling) still reads crate-only peaks (seeded from history); it does not know the rustc profile. Follow-up: key it by the build profile.

## Follow-up: component-pass scheduling keyed by profile
- `driver-stages/🪪️peak-profile/🟦️.ts` (new): `componentPeakProfilesV1` resolves the Cargo profile of each pass with the profile policy (`renderCargoProfileConfigurationV1`, target and package composition, so the lean release compositions get their own settings) and keys each crate's peak by `unitPeakKeyV1(crate, unitProfileKeyV1(unitProfileArgumentsV1(settings)))`. `componentPassPeakV1` is the one scheduling peak (seed, median, family of the same profile) used by both `componentJobs` and `fleetSlots` in `driver-stages/🟦️.ts`; the duplicated crate-only reads are gone.
- One identity function: `unitProfileIdentityV1` (offload `🪪️profile`) stays the only parser; `unitProfileArgumentsV1` only renders resolved settings into the rustc arguments it parses. The identity now defaults `opt-level` and `debuginfo` to rustc's `0`, so an omitted flag and an explicit default give one key (Cargo omits default flags).
- Tests: profile fixture gained `settingsKeys` (JSON, Python oracle checks it); driver test `🪪️peak-profile/🧪️tests` (lean vs final keys differ and equal the unit cache's key; a stale fat peak is ignored, own evidence is read, seeds go to the profile's own key). Red first (module absent), then green. Typecheck of driver-stages, the new test and offload: pass.
- Full offload suite (all 118 test files under remote-execution): 107 pass, 11 fail. None of the 11 files references the changed symbols; `fleet/restart` passes when run alone (load flake). Failures: offload timeouts, cluster fleet-store (stored slot count), native-link cluster (xcrun cache sandbox), eligible (`fleetLinkedSlotsV1`), fleet, bazel and bazel flags (Bazel extraction / endpoint resolution), sidecar wave-costs, priority and tools-grace handoff. I could not prove them pre-existing without a baseline (git state must not be touched), so they need a look by an owner of the fleet/sidecar area.

## Follow-up: failure triage of the full offload suite
Stale references: none left in the framework for `beyond-host-refused`, `beyondHost.refused`, `beyondHost.timeout`, `timeoutMilliseconds` of `beyondHost` (grep clean).
Causality, from the assertions:
- `eligible` (not caused by the policy change): the host's own origin (`.cache/f/machine.json`, `g1-m01`) equals the hardcoded worker origin of the test, so `eligibleFleetWorkersV1` drops it as "own". Fixed: test workers renamed `x1-m01`.
- `timeouts` (pre-existing client bug): `executeOffloadV1` never enforced the action budget while the scheduler refuses connections instantly, because the 1 ms re-armed timer was cleared by every retry before it fired; only the 60 s ride-through ended it. `🔌️client/🟦️.ts` is unchanged by me and by HEAD. Fixed: the outage branch returns `declined` once `submitted + execute` is spent.
- `fleet-store` (pre-existing): commit 8d17d8ba399 (manifest-publish) stores a second slotted entry, the input manifest, per unit; the test still counted one. Fixed: `slottedPerUnit = 2` (unit and manifest).
- `fleet`: `freeMegabytes` 53248 instead of 55904 comes from the uncommitted host-capacity decision rewrite of another agent (925-line diff in `🌡️host-capacity`), not from the beyond-host policy or peak keys. Not changed, to avoid freezing an in-flight policy into the test.
- sidecar `wave-costs` (13312 vs 13976), `priority` (python `jsonschema` module missing on this host), `tools-grace/handoff` (missing `.stop` file), bazel (Bazel sandbox run) and bazel flags: no reference to my symbols; sidecar files carry other agents' staged changes.
Rerun (117 files, native-link cluster excluded): 111 pass, 6 fail (the six above). `timeouts`, `eligible`, `fleet-store` are green.

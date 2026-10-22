# result-describe-rs: `@semio-tech/os-plugin-describe-rs:build` (last failing play/demo activation action)

STATUS 10-22 22:05 (wind-down): FIVE root causes found; (1)-(5) fixed LIVE, NO green run yet. Previous: 21:15: four root causes found and fixed LIVE; slice-play3 loop3 attempt 2 (20:19-21:05) already passed (1) and (2) in the real describe-rs build and then hit (4); green run after the (4) fix still pending (see "Verification").

`L` = `🧰️framework/🔨️modules/🏃️process/🔒️leases`, `K` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation`.

## (3) "Actual compilation copy extent exceeds remaining budget"
Already fixed by slice-demo2 (19:21): schema field `maximumRetainedCompilationBytes` = 1.5 GiB (`process/artifacts/publication/sources` schema), used by retention snapshot/reading/copy, native-build, component-pass, describe emitter. loop2.6 (20:06) and loop3.1 (20:18) both passed the compile + retention with it. Nothing further changed.

## (1) "Prepared manifest changed: …🔌️plugin/📦️packages/🦀️rust/Cargo.toml"
- Source: `K/🟦️.ts` `withPreparedCargoDependencyPairV1` (the cargo dependency preparation the action runs first), NOT Bazel input staging: the action is a host-class `task` that runs cargo in the live checkout (schema decision `Actions.kinds.task.sandbox=false`), so the bzl-snapshot cannot give cargo planned bytes. Making it hermetic would hide the live manifests cargo itself reads.
- Cause: the replan policy only retried a manifest edit seen BEFORE the first dependency command (`retryBoundary: before-dependency-commands`); an edit by a peer while `cargo update --workspace` / `fetch --locked` ran (a minute on this tree) was thrown. Update + locked fetch are idempotent and converge on the current manifests.
- Fix: the replan boundary is now the receipt publication: `dependenciesStarted` removed; a changed manifest/roster at any `current()` check replans (bounded by `maximumAttempts` 4, same deadline/cancellation). Schema `K/🔁️replan/🧬️schema/🔣️.json` (`retryBoundary: before-receipt-publication`, case kind `always-changing-after-update`, `dependencyCalls` max 4), corpus `K/🔁️replan/🧪️tests/🔣️.json` (`changed-after-update` now succeeds with 2 identity / 3 dependency calls; new `always-changing-after-update` refused after 4/4), test `K/🔁️replan/🧪️tests/🟦️.ts` edits the manifest after the first update only (resp. after every update).

## (2) "SQLiteError: unable to open database file (SQLITE_CANTOPEN)" at `L/🟦️.ts:168 initialize`
- Measured (watcher `C/describe-rs-tools/leases-watch.py`, log `C/🗑️generated/describe-rs/leases-watch.log`): the 63 MiB leases volume (`.cache/b/l/m`) normally sits at 65-87 % but during the post-compile publication of describe-rs it filled to 97.2 % (1.8 MB free) at 20:18:44 and loop3 attempt 1 died at 20:18:46 with `SQLITE_FULL` (database or disk is full) in the same `BEGIN EXCLUSIVE` of a new lease database (a full disk on journal creation surfaces as CANTOPEN or FULL depending on the step). Claims grew 900 -> 2300 in 40 s.
- Cause: every queued acquisition leaves a settled 12 KiB claim database behind; they are retired by `retireResourceQueueClaimsV1` which a process starts at most once per 30 s (`gate.claimSweepIntervalMilliseconds`) and which retires at most 128 claims per turn. A burst of leases (cargo provenance admissions, retention copies) outruns that on the bounded volume.
- Fix (pressure-driven sweep, schema-first): `L/🧭️order/🧬️schema/🔣️.json` + `L/🧭️order/🔣️policy.json` get `gate.claimSweepPressureFreeRatio` (0.2) and `gate.claimSweepPressureIntervalMilliseconds` (1000); `L/🟦️.ts` new `leaseStoreFreeRatioV1(directory)` (statfs) and `claimSweepIntervalV1(gate, freeRatio)`: while less than 20 % of the store is free, a queue acquisition may sweep every second instead of every 30 s (callers with timeout 0 still never sweep). `LeaseOrderPolicyV1` type + the order fixture + the two process-test gate literals carry the new fields (0 = no pressure).
- Diagnostics: a `SQLITE_CANTOPEN` during acquisition is now rethrown as `Resource lease database cannot be opened: <path> (create probe …, database … bytes, free … bytes … files)` (`explainUnopenableLeaseV1`), so a recurrence names the physical cause.
- Test: `L/🚪️admission/🧪️tests/🟦️.ts` + corpus `🔣️.json` `pressure` rows (5; sqlite `select case when …` as the independent oracle) + free-ratio test: 16 pass. Order fixture suite 68 pass.

## (4) NEW at 21:01 (loop3 attempt 2): `FileObservationErrorV1: Physical file observation: deadline: …/captures/cargo-artifacts-9tMkYG/q/30dba3…`
- Where: retention snapshot `verify` -> `observePhysicalFileV1` under the cargo provenance physical control (`K2/🟦️.ts` `cargoProvenancePhysicalControlV1`, `K2` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt`). That control carries a flat `deadlineMs` 600000 started when the compiler terminated; the observation (digest ~1 GB / 126k work units of inputs, copy + verify 757 MB of retained compiler bytes, rechecks) took 100-200 s on a quiet host but more than 600 s at load ~20 with 8 concurrent cargo builds (compile ended ~190 s into the action, the deadline fired at ~790 s).
- Fix: the budget is schema-declared now: new `K2/🧬️schema/📏️physical-budget.json` (maxBytes, maxWork, chunkBytes unchanged; `deadlineMs` 1800000 = 3x the slowest measurement, description records the measurement); `CARGO_PROVENANCE_PHYSICAL_BUDGET_V1` reads it. Test in `K2/🧪️tests/🟦️.ts` (Ajv validates the budget against the schema, a smaller deadline is refused, deadline >= 3x slowest measured). 2 physical tests pass.
- Lease volume during this attempt (describe-rs action window 20:52-21:01): peak 80 % used (was 97.2 % without the pressure sweep) -> fix (2) works under the real load.

## (5) NEW at 21:57 (loop3 attempt 3, after 1670 s): `Finite physical artifact Source required` in `stageRepositoryArtifacts`
- With (1)-(4) fixed, attempt 3 got through compile + observation + retention (the 1800 s deadline was needed: the action ran 1670 s at load 70-100) and died publishing the emitter: the retained compilation (757 MB) exceeds the shared repository artifact protocol `maximumOutputBytes` 512 MiB (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧬️schema/🔣️.json`, `Protocol`); the manifest cap 1 MiB could not list 16384 blobs either.
- Fix: `maximumOutputBytes` 2 GiB (>= retained compilation 1.5 GiB + index), `maximumManifestBytes` 2 MiB (>= 16384 canonical names). Test `…/📦️artifacts/💾️storage/🧪️tests/📏️extent/🟦️.ts` (2 pass). Written after attempt 3 ended, so UNVERIFIED in a real run.

## Verification (OPEN)
- Next step: rerun `SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel run @semio-tech/os-plugin-describe-rs:build --configuration=prototype` (or slice-play3's loop; the action is not cached yet, expect 15-30 min at current host load). Not started by me (wind-down; slice-play3 loop3 pid 6995 still owns the play activation).
- Unrun: `K/🔁️replan/🧪️tests/🟦️.ts` (needs `bun repo exec` invocation; it queued behind the memory gate and I killed my pid 43388). Parse-checked only.
- Watcher `describe-rs-tools/leases-watch.py` stopped (stop file); log `🗑️generated/describe-rs/leases-watch.log`.

## Other failure of the same attempt (not mine): `@semio-tech/plugin-registry:session-draw:prototype` "error: Original relative directory facts changed" (line 1173 of loop3.2.log) - a peer edit under the staged session tree during the run; rerun.

## Files
`L/🟦️.ts`, `L/🧭️order/{🔣️policy.json,🟦️.ts,🧬️schema/🔣️.json,🧪️tests/🧫️fixtures/🔣️.json,🧪️tests/📥️processes/🟦️.ts}`, `L/🚪️admission/{🔣️.json,🧬️schema/🔣️.json,🧪️tests/🟦️.ts}`, `K2/{🟦️.ts,🧬️schema/📏️physical-budget.json,🧪️tests/🟦️.ts}`, `K/🟦️.ts`, `K/🔁️replan/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/🔣️.json,🧪️tests/🟦️.ts}`. Tools: `C/describe-rs-tools/leases-watch.py`.

## Notes
- `L/🧭️order/🧪️tests/📥️processes/🟦️.ts` "a waiter does not hold the directory admission gate while it waits…" times out at bun's default 5 s under the current host load (load ~17) with plain bun 1.3.14; the other 20 pass. Not touched by this change (gate literals only gained zero-valued fields).
- My edits landed while slice-play3 loop3 was running (attempt 2 was in the capacity queue); any process that loaded the old policy keeps it.

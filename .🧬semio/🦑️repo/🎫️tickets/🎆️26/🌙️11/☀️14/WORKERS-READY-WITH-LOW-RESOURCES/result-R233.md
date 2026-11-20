# R233 Mid-Read Owner Registry Repair

An owner added halfway through the 2026-11-18 18:12 UTC whole-graph read left the enumerated registry out of date. The runner refused after 24,784 seconds when a dependency named the new crate, even though a peer had already registered it. The first R233 staging left its helper and tests in the shared tree while the loader and refusal schema were still at their previous revisions.

The graph loader follows a named native-owner refusal when the on-disk registry differs from the attempt's snapshot. It also uses the scoped owner reconciler when the registry is unchanged and the named manifest appeared or vanished. The retry clears the authority snapshot and keeps the loaded implementation pin. It reuses persisted owner records, bounds its follow-up rounds, and preserves cancellation and unrelated exceptions. The new refusal names the roots and original cause in English and German. The graph implementation authority includes the retry helper. Reusing a pinned graph proceeds directly to the graph reader.

## Verification before landing

- The original partial live staging passed four registry-follow tests and failed the typed-refusal check because its schema definition was absent.
- With that schema supplied, the inherited-pin regression alone failed on the previous helper. It reported the new root as inconsistent after zero follow-up rounds because both reads used the same snapshot. The other five registry-follow tests passed.
- The staged fix passed all 28 tests in registry-follow, pending-crates, adopt-registry, repair-rounds and seal-under-churn. Both inherited-pin variants read two snapshots. SQLite independently projected the roots from the fixture registry JSON and matched the second read.
- The generation-protocol suite had 45 passes and two failures on the live baseline. Both failures trace to the existing selected-inference-switch expectation, once in the TypeScript check and once in its Python oracle.
- The baseline CLI selected `@semio-tech/math-rank-rs`, inferred one project, and sealed and returned its selected graph. Disk maintenance delayed the command outside the graph inference itself.

## Live landing

The guarded landing completed at 12:30:18 UTC. All 28 focused tests passed on the shared tree, and the graph entry imported. The pre-existing generation-protocol failures reproduced unchanged. The old 01:10:04 UTC R219 lock had no living landing holder and was recovered under the documented age rule. The landing ledger records the old owner bytes and before/after SHA-256 for each file. Other processes, including the automatic whole-graph reader, were left running.

## Worker-farm failure route

The later automatic read contained 2,988 set-asides naming exactly two native roots. The window-config contract accounted for 2,981 and was already registered by the time of the audit. The WFC grid3d transient crate accounted for seven and still lacked its row. The other three set-asides were farm no-record failures without an undeclared dependency root.

The worker IPC already carries the full message and code. The farm was converting those replies into generic `graph.farm-job-failed` after three tries, with the message shortened to 200 characters during prepare or 70 during observe. The project loop treated that generic code as recoverable unavailability and sealed or continued without those owners. The follow-up changes the farm to propagate only the two known native registry fault forms immediately. A wave-local signal retires the stale farm, stops warm and take work, and releases memory admissions. The compute path passes the original exception out of the generation refresh so the bounded registry retry can re-enumerate. Normal farm failures keep their existing retry and set-aside behavior. A whole-read owner record keys its registry contribution by catalog providers, so adding a registry row does not by itself change unrelated record keys.

On the old farm, both new end-to-end IPC regressions failed. With the staged fix, 42 tests passed across nine focused farm and graph suites. The synthetic worker reports a message longer than the old truncation bound. Prepare starts one child job, preserves the cause, and causes a second pinned graph attempt to follow the new row. Observe also forwards after its first reply.

The 12:30 change directly imported the physical coordinates module from the retry helper. The normal Nx plan sealed its graph slice, then the bundled authorizer attempted to load the target-execution schema relative to its cache staging path and failed with ENOENT. The follow-up performs the physical read through the existing lazy inference producer, so the helper no longer pulls a static coordinates module into the compiled authorizer. The earlier baseline selected command returned its project data and was interrupted during outer disk collection after graph completion.

## WFC owner correction

The normal owner reconciler proposed one new row for `semio-s-artifact-wfc-grid3d-transient`. Its manifest is the existing Cargo.toml below the WFC grid3d transient Rust package. The guarded scoped update compared the full registry immediately before the atomic replacement, with every pre-existing byte identical after removing the proposed row. It added one row (4,775 to 4,776) and the owner law reported zero findings for that scope. The fresh selected graph inferred one owner, sealed quietly and published to the fleet. A requested private report path was rejected by the CLI report-name policy after that graph completed. A second normal invocation was started with stdout capture.

The second fresh `repo show project semio-s-artifact-wfc-grid3d-transient` invocation exited 0. Its JSON reported the same canonical root and the `build`, `check` and `test` targets. The root and its Cargo.toml were checked again against the current registry. A refreshed scan found only the now-registered WFC and window-config roots in named set-asides. The three no-record farm failures contained no owner root to add.

## Farm follow-up landing

The follow-up reached the shared tree at 13:00 UTC. Under the guarded lock, all 42 focused tests passed on the live files and the normal graph module imported. The original research fixture and generated overlays were cleaned up after testing. A whole automatic read that started before the landing continues with its earlier pinned code, so its outcome cannot verify the new repair.

The scoped `repo tools setup nx --lang=en` completed with exit 0 after compiling and validating the authorizer derivative, then reported local Nx 23.2.0. This exercised the compiled path that had failed after the first landing. A new whole-workspace fleet seal using the follow-up remains unverified.

## Owner Lookup Fault Counters

A whole coordinator read reported 11,902 failed lookup attempts while inferring about 107 owners that the fleet had listed. The earlier report did not distinguish which exchange failures produced those attempts.

Slice-memory telemetry now exposes `sources.owners.looks.outcomes`, with ordinary `misses`, `cancelled` looks, and eight bounded counters under `faults`. The periodic adoption line includes the nonzero fault reasons. The counters describe completed lookup attempts, so a retried owner can contribute several failures.

- `unavailable` covers a cache tier skipped by its breaker or known to be unavailable. `transport` covers an unusable response, including network, status, or read-body failures for which the client does not retain a finer reason. `denied` records a response that refused access.
- `record` covers a refused action note or digest and a missing primary record in a received entry. `blob-missing` covers a missing blob that a present entry promised, while `blob-integrity` covers its failed byte extent or SHA-256 check. `payload` covers a refused compressed document or packed primary record.
- `unknown` covers a failure where the lookup has no typed evidence. The report carries fixed categories and numbers. An action-key miss and an aborted look each have their own count.

Verification and fallback decisions still use the original take outcomes. The diagnostics use checks already made on each response and introduce no new reads or retries. A real loopback route exercised 12 response cases through the normal exchange client and owner take scope, independently checked the counts in SQLite, and imported no refused payload. The final focused run passed 13 tests with 540 assertions, including cross-origin reuse, local fallback, and the active-signal handoff. The earlier rollout's 11,902 attempts cannot be reclassified from those old logs. No whole fleet read was started for verification.

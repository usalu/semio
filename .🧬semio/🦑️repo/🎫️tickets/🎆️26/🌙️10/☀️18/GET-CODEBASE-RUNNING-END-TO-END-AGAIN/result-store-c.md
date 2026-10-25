# Result store-c (executor store-c, wave 1023c)

STATUS 09:10 (10/24): PAUSED at the coordinator's request. NOTHING of mine is running (my queued `check -p db-artifact-contract` wrapper 89721/89723/89724, lstart 08:48:05, was killed after 21 min in the capacity queue; verified lstart before kill). No unattended chains. Everything below is WRITTEN, NOT COMPILED (the 08:00-09:06 edits had no build slot).

## Handover
- Tools: `C/store-c-tools/runw.sh <log> <crate> <max-test-binary-min> [test filter]` (one crate, `--test-threads=4`, watchdog kills only my own test binary + writes `<log>.hung`), `sum.sh <log>` (compile errors / results / failed tests), `chain.sh`/`run.sh`/`wait.sh`. Logs of the 22:12-03:01 chain: `C/🗑️generated/store-c/c1-*.log` (full verdicts; table below).
- HUB (db-artifact-contract, 770 errors): cause = the durable-group generic types gained bounds (`ArtifactCanonicalJsonTree`, `ColdMutation<P>`, `OpBinary + OpText`, `P: ArtifactStoreSnapshot`) that the contract crate's impls did not state. First `check --tests` after my port: 778 -> 28 errors; then I fixed those 28 statically (the `verify_fixed_three_edits` where-clause, `OpBinary/OpText` on the mutation bounds, test `absorb` moved to `ColdMutationDiff`, dev-dep `serde` + Cargo.lock). Files: `db/🗿️artifact/🧬️contract/{🛡️durable-group-recovery,🧾️receipts-and-checkpoints}/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, `Cargo.lock`. NEXT: `bun repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml -p semio-framework-os-kernel-db-artifact-contract --tests` (the lib itself had no errors in hub-test8 04:07; only the test target was red); the hub's remaining blocker is then mcp-c's list (zip-editor = plug-mz).
- NEXT (store, in this order, ONE crate at a time with runw.sh): tests-03 (biggest: replay issuer + presence Deref fix + batch `Blocked`), then 05, 08, 06, sync-tests-1, sync-tests-2 (a test binary hung 49 min there: find it via `<log>.hung`), tests-04 (compile after the prelude testkit feature), 09, 10, 01 (+ filter `retained_clone_tests::` with a 8-min watchdog), 02, then durable-group-tests, member-open-tests, os-store-engine, world_model (renderer-contracts carve fix written, downstream renderer crates never compiled), pack-record-encode-cursor.
- Open/unknown after the fixes: tests-10 `resident_consumed_envelope_keeps_catalog_until_exact_terminal_release` (box close > 1024 turns; suspect another alias-vs-registry or catalog-alias interplay, now with the extra replay alias), tests-06 `retained_hydrators_refuse_actorless_history` (first row accepted instead of refused), tests-01 retained-clone stalls (`unit_generic_enum_and_recursive...`; `recursive_depth_envelope...` hang-or-slow), tests-03 `retained_clone_preparation_store_lifecycle_matches_neutral_oracle` (batch publication Blocked, phase Preparing), sync-tests-1 `the-first-archive-fills-an-absent-folder` fixture assertion.
- Risk to watch in the first rerun: the cold-authority restore in `EditReplay::step_with_operation_cap` (progress.done accounting) and the member producer's exact birth quote (`cold_replay_birth_bytes`; the `📦️catalog` allocation-law test compares quote vs actual allocation).


PREVIOUS STATUS (08:50):
STATUS 08:50 (10/24): RESUMED. c1 chain was killed by the coordinator (03:02); I read ALL c1 logs (they hold full verdicts, see table). Fixes below are WRITTEN, awaiting reruns (one crate at a time, `C/store-c-tools/runw.sh <log> <crate> <max-min> [filter]` = cargo test --test-threads=4 + watchdog that kills only my own hung test binary and writes `<log>.hung`). Now running: `check -p semio-framework-os-kernel-db-artifact-contract --tests` (hub blocker, 770 -> 28 errors after bounds port; log `C/🗑️generated/store-c/chk-dbac3.log`).

## c1 verdicts (pre-fix, from the logs)
| crate | compile | result | dominant cause |
|---|---|---|---|
| tests-01 | ok | 53+ pass, 1 FAIL source-scan, retained_clone 1 FAIL + 2 hang | pub authority fields; stale tiny grants |
| tests-02 | died on peer launcher fault | rerun needed | |
| tests-03 | ok | ~35 FAIL | `native replay requires its retained original preparation issuer` (catalog lacks replay issuer) + presence retirement Deref trap + batch publication Blocked |
| tests-04 | 4 errors (`test_support::SerdeValue` testkit-gated) | - | prelude lacked `testkit` on test-support |
| tests-05 | ok | 62 ok / 17 FAIL | text loader replay has no issuer |
| tests-06 | ok | 36 ok / 2 FAIL | replay issuer; `retained_hydrators_refuse_actorless_history` (accepted first row) |
| tests-07 | ok | 29 ok, GREEN | |
| tests-08 | ok | 28 ok / 13 FAIL | replay issuer |
| tests-09 | ok | 34 ok / 1 FAIL | denied-grant test stale (copy-0 no longer denies) |
| tests-10 | ok | 22 ok / 1 FAIL | `resident_consumed_envelope_keeps_catalog...` turns > 1024 (open) |
| sync-tests-1 | ok | 36 ok / 13 FAIL | replay issuer (+ `the-first-archive-fills-an-absent-folder` fixture assertion) |
| sync-tests-2 | ok | many FAIL, a test binary hung 49 min | replay issuer |
| durable-group-tests, member-open-tests, os-store-engine, world_model, pack-record-encode-cursor | not reached | | |

## Fixes written 08:00-08:50 (all UNVERIFIED until rerun)
1. ROOT of ~100 reds: `EditReplay` live range refused every replay without an installed preparation issuer (696 removed the cold `replay_operation` path); the text loader (`settle_parsed_envelope` -> `loaded_history_replay(..).run`) and the unscheduled `step`/`run` API have no issuer and no grant. `🔁️replay/⏩️replay-owned-state-operations`: a replay stepped with neither an original grant nor an issuer is the explicit COLD authority again (`cold_authority_grant()` unbounded envelope + `replay_operation`); granted native stepping without an issuer still refuses.
2. Store test doubles: `fixture_authoring_catalog` (= `demo_closable_store_owners`) installs `cold_demo_replay_preparation_factory::<DemoMutation>()` via `with_replay_preparation`; `fixture_authoring_member_owners<M>` uses `admit_source_constructor_with_semantic_preparations` with the cold replay factory and `fixture_authoring_birth_demand` quotes its 4 arcs (`cold_replay_birth_bytes`).
3. Deref-vs-trait trap #2 (same class as store-b's snapshot/read fix): `reads.terminal_is_empty()` on `SnapshotReadRegistryAliasRetirement` now resolves to the ErasedSnapshotRetirement impl (= alias closed), not the registry: presence `♻️retirement` close_step and ephemeral transient store (4 sites) -> `(**reads).terminal_is_empty()`. Explains the 3 presence-retirement WorkLimit reds.
4. `ArtifactStoreOneItemLiveAuthority` fields private (+2 external readers ported to accessors).
5. test-support `testkit` feature enabled from the testing prelude (tests-04 compile).
6. tests-09: denied-grant list derived from the quoted grant (copy 0 no longer denies); tests-01 utf8 reserve grant 7 + turn bound; retained_copy stall message prints the demand.
7. hub blocker `semio-framework-os-kernel-db-artifact-contract`: the durable-group types gained bounds (`ArtifactCanonicalJsonTree`, `ColdMutation<P>`, `OpBinary/OpText`, `P: ArtifactStoreSnapshot`); `🛡️durable-group-recovery` + `🧾️receipts-and-checkpoints` where-clauses ported; test `absorb` moved to `ColdMutationDiff`; dev-dep `serde` (+Cargo.lock).
8. renderer contracts `ProgramFault.create_app_source` (see below, earlier).


OLDER NOTES:
STATUS 00:56 (10/24): PAUSED at the coordinator's request (usage pacing). HANDOVER:
- Left running on purpose: chain wrapper `sh store-c-tools/chain.sh c1 ...` pid 87163 (lstart Fri Oct 23 22:12:29 2026), logs `C/🗑️generated/store-c/c1-<crate>.log` + `.log.status`, `c1.chain.out`. It is now compiling/testing os-store-tests-03 (compile phase, ~18 min in; carries my private-fields edit of ArtifactStoreOneItemLiveAuthority, so a compile error there = a missed external field read; fix by switching to the accessor). Remaining chain: 04..10, sync-tests-1/2, durable-group-tests, member-open-tests, os-store-engine, world_model (semio-framework-os-infinite-world-model), pack-record-encode-cursor. Test binaries run with `--nocapture`: a stuck test binary (0 % progress, >10 min at high CPU) is a child of that chain; kill it (verify ps lstart) and the chain advances (I did so for tests-01 pid 64533).
- Next steps: (1) read each c1-*.log.status/log with `sh C/store-c-tools/wait.sh <log>`; (2) rerun os-store-tests-02 (it died on a peer's launcher-schema edit; stall fix + cold_demo_replay_preparation_factory unverified); (3) rerun tests-01 with my fixes (private authority fields, utf8 reserve copy 7, stall message now prints next demand) and investigate `retained_clone_tests::unit_generic_enum_and_recursive_records_remain_grant_bounded` + whether `recursive_depth_envelope_accepts_boundary_and_rejects_the_next_box` is a hang or just slow (run alone, bounded); (4) world_model_unit: the contracts carve fix is written, downstream renderer crates never compiled (nr1 died at contracts), expect more errors.
- Capacity: every cargo request queued 20-60 min (free memory gate; bun 12 GB hog seen 22:50 from framework-os-dev activate).
Previous status: 23:27 (10/23): tests-01 still queued for capacity since 22:13 (75 min, 1 ahead; free memory is the gate, not CPU). Original STATUS 22:12: chain c1 launched headless (wrapper pid 87163, started 22:12:29; tools `C/store-c-tools/chain.sh c1 Cargo.toml <crates>`; logs `C/🗑️generated/store-c/c1-<crate>.log(.status)`). Order: tests-02, 01, 03..10, sync-tests-1/2, durable-group-tests, member-open-tests (= "companions"), os-store-engine, world_model_unit (package semio-framework-os-infinite-world-model), pack-record-encode-cursor. Queue contention: rcore-b orphan chain 82303 (cargo 84041) holds compile capacity.

BLOCKER NOTE 22:55: my cargo (tests-01) has queued 27 min for capacity (need 3584 MB, spare ~1 GB, 5 running / 7 ahead). Biggest memory holder: bun pid 21052 (lstart 22:27:12, 12.6 GB RSS, 22 % CPU) = child of `bun repo --tool=bazel run @semio-tech/framework-os-dev:activate-s-react-prototype` (pid 86494, lstart 22:11:55, ppid 1). c1 tests-02 died at 22:25 with `Invalid repository Cargo launch Policy: /parallelism must equal its const` (cargo launcher schema `Policy.parallelism` const "planned" vs Configuration "available", edited by a peer 22:24, since fixed live); run.sh now retries on it; tests-02 needs a rerun (appended after c1).
Static fix 22:30 (world_model_unit blocker, WRITTEN UNVERIFIED): renderer wgpu contracts `ProgramFault.create_app_source` was `Option<crate::kernel_runtime::CreateAppSourceFailure>` (pre-carve path; contracts cannot depend on kernel-protocol which depends on it) -> now `pub create_app_source: Option<CreateAppSourceRefusal>` (rendered text captured at conversion, defined in contracts `📥️lifted/🌉️ProgramBridge/🦀️.rs`); the `From<CreateAppSourceFailure> for ProgramFault` impl moved into kernel-core `🐣️create-app/🎟️source/🦀️.rs`.

| crate | compile | pass/fail |
|---|---|---|
| os-store-tests-02 | died 22:25 on launcher schema fault (peer edit); RERUN NEEDED | - |
| os-store-tests-01 | COMPILES (first time) | 53+ ok, 1 FAIL (source-scan law: LiveAuthority fields were pub) + retained_clone: 1 FAIL (`unit_generic_enum_and_recursive...` stall) + 2 HANG (`utf8_copy_is_boundary...`, `recursive_depth_envelope...`; I killed the test binary after 22 min) |
| os-store-tests-03..10 | queued (c1 chain, tests-03 waiting, 13 ahead) | ? |
| os-store-sync-tests-1/2 | ? | ? |
| durable-group-tests, member-open-tests | ? | ? |
| os-store-engine | ? | ? |
| world_model_unit | ? | ? |
| pack-record-encode-cursor | ? | ? |

## tests-01 findings + fixes (00:10, WRITTEN UNVERIFIED, rerun after the chain)
- `one_item_publication_source_denies_generic_or_unbounded_shortcuts`: `ArtifactStoreOneItemLiveAuthority` had `pub operation/generation/base_revision/base_applied_edit_count/next_sequence_number/next_clock/actor` (law: apps cannot fabricate them; accessors exist). Fields made private in `🏪️store/🎮️one-item/📬️publication/🦀️.rs`; the only two external field reads ported to accessors (`📦️prepared/🔗️binding/🦀️.rs` `next_clock()`, `📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs` `actor()`).
- `utf8_copy_is_boundary_paged...` hang: the string cursor's per-turn copy demand is now `min(body, page)` rounded to a char boundary (value control `retained_clone_grant_admits`, since 10-20), the test's `reserve` grant allowed 2 copy bytes against body 7 -> `Progress(default)` forever in an unbounded loop. Test: reserve copy 7 + turn bound.
- `unit_generic_enum_and_recursive...` stall: stall message now prints the next demand (diagnostic, to find which demand axis exceeds the grant); not yet understood. `recursive_depth_envelope...` ran > 20 min: unknown whether hang or slow; needs a bounded run.

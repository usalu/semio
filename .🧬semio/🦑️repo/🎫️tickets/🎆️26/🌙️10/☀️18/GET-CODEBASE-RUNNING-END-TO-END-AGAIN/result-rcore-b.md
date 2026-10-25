# Result rcore-b (executor rcore-b, wave 1023b)

STATUS (newest first)
- 18:3x per-crate check `chk2` (18 crates, `--tests`) showed the blind fixes landing: db-io-registry, flow-extensions (lib, incl. the DomainOwner Sync fix), post-turn-relay lib, tool-run lib now compile; it also exposed latent reds (db-io-core `testkit` must imply `claimed-backend-laws` -> fixed in its Cargo.toml; mcp bridge `#[cfg(test)]` helpers are stale -> my feature attempt reverted; production `host/shard/child instantiate_budget` literal lacked `work`/`retained_policy`/`work_fuel` -> rebuilt with `Budget::original`). Applied rustc `use` suggestions mechanically from census2 (`applysug.py`, 115 insertions, 31 files, ranked by crate priority). Re-check `chk3` (24 crates) running, pid 9931. Open: visibility-widening for tests moved out of their crate (~250 private type/field/method errors; needs a policy: in-crate `#[cfg(test)]` mounts vs `#[doc(hidden)] pub`), dense one-line unit tests of the 696 API (`registry` guest-call test, `owned-runtime` interpreter test, `shard identity` test), os-spr `field_set_mutations!`, flow-vcs-laws/flow-extensions-laws white-box tests, host-test-component cdylib (73 errors), os-store* (store-b).
- COORDINATOR RULE (compiler-writer starvation): no workspace-wide `build --workspace --tests` while play/demo activation is red; only per-crate `check -p` / `test -p`. My census2 (`census2.out`, json) had already ended (rc 1) when the order arrived; no process of mine is running.
- flow-extensions (ts-b blocker): E0277 DomainOwner not Sync root-caused = the `RetireOwned` derive emits a shared-original cursor impl with a `Name: Sync` bound (SealedShared accounting), so every owner reachable through `OriginalTurnFailure` must be Sync. Fix WRITTEN (unverified, no cargo yet): `…/🧵️job/⚠️failure/📦️domain/🦀️.rs` `DomainOwner: Send + Sync`, `OriginalDomainFailure: … + Sync`, `unsafe impl Sync for DomainFrame<T>` (shared access reads only the immutable original value; the merely-Send retirement cursors are touched via `&mut self`). E0308 in `📥️request` was already rewritten by someone at 11:42 (`to_dsl_value`). Verification rides on census2 (chain 78060).
- 12:xx root census done (log `🗑️generated/rcore-b/census.out`, table below: ~70 red units / 1687 unique errors, mostly stale tests vs the 696 API + moved modules; plus ~13 os-store* units owned by store-b). Wrote blind fixes (UNVERIFIED, no cargo yet): `kernel::Budget::original` / `Budget::from_retained` constructors + 66 literal call-site rewrites (codemod), `protocol::Mutation::diff|inverse` -> `ColdMutation` (3 files), db-io-registry private types `pub(super)`, companion roots tests-01/02/05/06/17 extern-crate aliases (lib mode) + `app_testing` alias, `store::pack::snapshot` -> `semio_framework_pack::snapshot`, 12 dev-dependencies (Cargo.toml), transient-preparation-law `advance(grant, control)`, scene dev-dep `testkit`. Detached chain 78060 = `cargo update --workspace --offline` then census2 (`build --workspace --tests --keep-going --message-format=json`, log `census2.out`) so rustc's `use` suggestions can be applied mechanically (`rcore-b-tools/applysug.py`).
- 08:xx host-effects 16/16 GREEN and guest-contract paired_admission GREEN (he7a: guest-contract 7 pass + 1 filtered, host-effects 16 pass). failure_publication test root cause = TEST BUG: `drop(entry)` inside the second `caller.turn` closure moved the pinned `OriginalActorGuestEntry` (descriptor address check saw a 247 KB-shifted payload); entry now `ManuallyDrop` (no move) - written, unrun. Chain pid 46219 (headless) now runs `census` (`build --workspace --tests --keep-going`, log `C/🗑️generated/rcore-b/census.out`) after he7b.
- 05:2x (coordinator order: host-effects first) host-effects 15/16 green after fixes below; remaining `revoked_capability_*` fix written (compute `run_job` slept 1 ms after every progressing step, never waking under `ManualRuntime`; now runs up to 256 progressing steps before one poll sleep). guest-contract: `paired_admission…` stalls (zero progress from turn ~100, diagnosing with remaining-grant print), `failure_publication` test aborts the lib binary (drop assert during unwind: guarded with `thread::panicking()` in `BorrowedProjectedPackCursor::drop`). Verifying in `he6a/he6b` (chain pid 81110). Root census (`build --workspace --tests --keep-going`, `cargo test --keep-going` does not exist) NOT yet run: queue waits are 30-60 min per cargo.
- 03:0x root census `test --manifest-path Cargo.toml --workspace --no-run --keep-going` launched detached (pid 95719, `C/🗑️generated/rcore-b/root-census.out`), waiting in the cargo capacity queue (6 running, 2 ahead, >20 min).

Tools: `C/rcore-b-tools/run.sh LOG cargo-args…` (retry wrapper, nice 10), `wait.sh`.
- 03:5x host-effects hang root-caused: router close loops (`drive_router_effect_job`, `close_unadmitted_router_effect` in `…/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs`) slept via `runtime.sleep_until(now+1)` on every zero-progress close turn; a `Pending` turn with zero progress is a pure state transition (BeginClose -> Fault page), only `Blocked` waits. Under `ManualRuntime` (clock never ticks) the sleep never wakes -> 0 % CPU hang. Fix: sleep only on `Blocked`. [DEBUG] print of rust-reds-c removed. Verifying (he2.out, pid 54199, watchdog `rcore-b-tools/dog.sh`).

## Fixes so far (host-effects hang + neighbours)
| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs` | close loops sleep only on `Blocked` (was: every zero-progress turn; hang under ManualRuntime); removed rust-reds-c [DEBUG] print; `for _` for unused loop vars |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧰️compute-pool/🦀️.rs` | `Ok(None)` from `checked_out_outcome` after a pumped step (job step returned `Ok(None)` = keep going) is a resume, not `WorkerLost` (fixed `router_effect_runs_through_the_retained_compute_session`); immediate continue for up to 256 progressing steps (const `COMPUTE_IMMEDIATE_STEPS_PER_YIELD`) |
| `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs` | Drop assert skipped while panicking (no abort that hides all other tests) |
Temporary [DEBUG]: `🔨️modules/🧵️job/📬️outcome/🎟️descriptor/🦀️.rs` (matches), guest-contract `🧪️tests/🤝️admission/🦀️.rs` (publish turn print): remove before finishing.

## More fixes
| file | change |
|---|---|
| guest-contract `…/🌉️call/🧬️schema/🔣️.json` + `🧪️tests/🧫️fixtures/🔣️.json` | `entryOriginalAuthority` copy 4096 -> 16384: payload builder initialization alone is `JOB_PAYLOAD_OPERATION_PAGES (256) x 32 B` = 8192 copy bytes (one paid slot per turn), so 4096 could never publish a fault (stall at remaining copy 25 after 128 turns); proven by the per-turn remaining-grant print |
| host-effects `🧪️tests/🔬️unit/🦀️.rs` `revoked_capability_*` | ticks the manual clock until both completions arrive (the 256-slot payload init needs >256 job steps, so the 1 ms poll sleep after 256 progressing steps must see time advance, same as the sibling test) |
| guest-contract `📤️failure/🧪️tests/🦀️.rs` | entry in `ManuallyDrop` (see status) |

## Root compile census (`build --workspace --tests --keep-going`, 10-23 ~11:00, log `🗑️generated/rcore-b/census.out`)
1687 unique errors in 209 files, 85 red units. Dominant causes: stale tests vs the 696 retained-turn/StepContext/Grant API (missing `work` field, `Budget.deadline_ms`, `close_step(cx)`), moved/renamed modules (E0425/E0433 ~800), private types/fields. Lib (non-test) reds block downstream units, so more are hidden.

| unit | errors |
|---|---|
| semio-framework-artifact-flow-flow-copy lib test | 1 |
| semio-framework-artifact-flow-flow-mutation-laws lib test | 10 |
| semio-framework-artifact-infinite-dag lib test | 49 |
| semio-framework-artifact-workflow-run-tests lib test | 22 |
| semio-framework-artifact-workflow-workflow-schema lib test | 1 |
| semio-framework-artifact-workflow-workflow-schema-laws lib test | 1 |
| semio-framework-os-command-tests lib test | 3 |
| semio-framework-os-config-tests-1 lib test | 4 |
| semio-framework-os-dsl-module-tests lib test | 3 |
| semio-framework-os-dsl-tests lib test | 13 |
| semio-framework-os-flow-extensions lib | 3 |
| semio-framework-os-flow-extensions lib test | 21 |
| semio-framework-os-flow-vcs-laws lib test | 66 |
| semio-framework-os-host-space lib test | 17 |
| semio-framework-os-infinite-board-normal-host lib test | 9 |
| semio-framework-os-infinite-board-normal-scene lib test | 1 |
| semio-framework-os-infinite-world-interaction lib test | 1 |
| semio-framework-os-infinite-world-model test "world_model_unit" | 2 |
| semio-framework-os-kernel lib test | 6 |
| semio-framework-os-kernel-db-io-core-laws lib test | 2 |
| semio-framework-os-kernel-db-io-registry lib | 110 |
| semio-framework-os-kernel-db-io-registry lib test | 110 |
| semio-framework-os-mcp-consent lib test | 3 |
| semio-framework-os-spr-laws-tests lib test | 26 |
| semio-framework-os-store-durable-group-tests lib test | 27 |
| semio-framework-os-store-member-open-tests lib test | 21 |
| semio-framework-os-store-replay lib test | 1 |
| semio-framework-os-store-sync-tests-1 lib test | 8 |
| semio-framework-os-store-sync-tests-2 lib test | 19 |
| semio-framework-os-store-tests-01 lib test | 9 |
| semio-framework-os-store-tests-03 lib test | 21 |
| semio-framework-os-store-tests-04 lib test | 29 |
| semio-framework-os-store-tests-05 lib test | 5 |
| semio-framework-os-store-tests-06 lib test | 2 |
| semio-framework-os-store-tests-08 lib test | 26 |
| semio-framework-os-store-tests-09 lib test | 18 |
| semio-framework-plugin-app-builder lib test | 11 |
| semio-framework-plugin-decl-surface lib test | 5 |
| semio-framework-plugin-extension lib test | 1 |
| semio-framework-plugin-host-guest-runtime lib test | 20 |
| semio-framework-plugin-host-operation lib test | 2 |
| semio-framework-plugin-host-post-turn-relay lib | 2 |
| semio-framework-plugin-host-post-turn-relay lib test | 9 |
| semio-framework-plugin-host-shard lib test | 2 |
| semio-framework-plugin-host-shard-executor lib test | 27 |
| semio-framework-plugin-host-shard-laws lib test | 91 |
| semio-framework-plugin-host-test-component lib test | 102 |
| semio-framework-plugin-instance-dispatch lib test | 5 |
| semio-framework-plugin-instance-docs lib test | 1 |
| semio-framework-plugin-interpreter-runtime lib test | 20 |
| semio-framework-plugin-job lib test | 10 |
| semio-framework-plugin-operation lib test | 1 |
| semio-framework-plugin-ownership lib test | 2 |
| semio-framework-plugin-reactor-jobs lib test | 17 |
| semio-framework-plugin-runtime-instances lib test | 5 |
| semio-framework-plugin-runtime-ops lib test | 2 |
| semio-framework-plugin-store-stage-laws lib test | 13 |
| semio-framework-plugin-surface-fixture lib | 10 |
| semio-framework-plugin-surface-fixture lib test | 38 |
| semio-framework-plugin-testing-fixtures lib test | 5 |
| semio-framework-plugin-testing-fixtures-publication lib test | 2 |
| semio-framework-plugin-tests-01 lib | 114 |
| semio-framework-plugin-tests-01 lib test | 142 |
| semio-framework-plugin-tests-01-2 lib test | 18 |
| semio-framework-plugin-tests-02 lib | 8 |
| semio-framework-plugin-tests-02 lib test | 8 |
| semio-framework-plugin-tests-02-2 lib test | 2 |
| semio-framework-plugin-tests-03 lib test | 6 |
| semio-framework-plugin-tests-03-2 lib test | 44 |
| semio-framework-plugin-tests-05 lib | 67 |
| semio-framework-plugin-tests-05 lib test | 64 |
| semio-framework-plugin-tests-05-2 lib test | 44 |
| semio-framework-plugin-tests-06 lib | 241 |
| semio-framework-plugin-tests-06 lib test | 178 |
| semio-framework-plugin-tests-06-2 lib test | 46 |
| semio-framework-plugin-tests-17 lib test | 42 |
| semio-framework-plugin-tests-18 lib test | 145 |
| semio-framework-plugin-tests-19 lib test | 48 |
| semio-framework-plugin-time-travel lib test | 13 |
| semio-framework-plugin-tool-run lib test | 1 |
| semio-framework-plugin-window-config lib test | 3 |
| semio-framework-plugin-window-config test "window_config_apply_edit" | 12 |
| semio-framework-test-transient-preparation-law lib test | 1 |
| semio-framework-test-typed-input-law lib test | 1 |
| semio-framework-value-derive-resources lib | 1 |

## Census 2 (10-23 16:19, after the first blind fixes; json, unit = package/target kind; counts include duplicates between lib and lib test)
68 red units, 1877 diagnostics. Compile order now reaches downstream crates, so counts are not comparable one to one with census 1. os-store* rows belong to store-b.

| unit | errors |
|---|---|
| semio-framework-artifact-flow-flow-copy (lib) | 1 |
| semio-framework-artifact-flow-flow-mutation-laws (lib) | 10 |
| semio-framework-artifact-infinite-dag (lib) | 48 |
| semio-framework-artifact-workflow-run-tests (lib) | 10 |
| semio-framework-artifact-workflow-workflow-schema (lib) | 1 |
| semio-framework-artifact-workflow-workflow-schema-laws (lib) | 1 |
| semio-framework-os-config-tests-1 (lib) | 4 |
| semio-framework-os-flow-extensions (lib) | 12 |
| semio-framework-os-flow-extensions-laws (lib) | 77 |
| semio-framework-os-flow-host-read (lib) | 12 |
| semio-framework-os-flow-vcs-laws (lib) | 66 |
| semio-framework-os-flow-wasm-domain (lib) | 26 |
| semio-framework-os-host-space (lib) | 1 |
| semio-framework-os-infinite-board-engine (test) | 19 |
| semio-framework-os-infinite-board-normal-host (lib) | 9 |
| semio-framework-os-kernel-db-io-core-laws (lib) | 6 |
| semio-framework-os-kernel-db-io-registry (lib) | 72 |
| semio-framework-os-mcp-bridge (lib) | 3 |
| semio-framework-os-mcp-plugin-runtime (lib) | 3 |
| semio-framework-os-renderer-wgpu-contracts (lib) | 6 |
| semio-framework-os-renderer-wgpu-host-base (lib) | 2 |
| semio-framework-os-run (lib) | 8 |
| semio-framework-os-spr-laws-tests (lib) | 16 |
| semio-framework-os-store-durable-group-tests (lib) | 73 |
| semio-framework-os-store-member-open-tests (lib) | 14 |
| semio-framework-os-store-tests-01 (lib) | 5 |
| semio-framework-os-store-tests-05 (lib) | 1 |
| semio-framework-os-store-tests-08 (lib) | 2 |
| semio-framework-plugin-app-builder (lib) | 11 |
| semio-framework-plugin-decl-surface (lib) | 1 |
| semio-framework-plugin-extension (lib) | 1 |
| semio-framework-plugin-host (bin) | 4 |
| semio-framework-plugin-host (lib) | 13 |
| semio-framework-plugin-host (test) | 1 |
| semio-framework-plugin-host-guest-laws (lib) | 18 |
| semio-framework-plugin-host-guest-runtime (lib) | 1 |
| semio-framework-plugin-host-post-turn-relay (lib) | 3 |
| semio-framework-plugin-host-routers (lib) | 1 |
| semio-framework-plugin-host-shard-executor (lib) | 9 |
| semio-framework-plugin-host-shard-laws (lib) | 27 |
| semio-framework-plugin-host-test-component (cdylib) | 73 |
| semio-framework-plugin-instance-dispatch (lib) | 5 |
| semio-framework-plugin-interpreter-runtime (lib) | 20 |
| semio-framework-plugin-operation (lib) | 1 |
| semio-framework-plugin-reactor-jobs (lib) | 6 |
| semio-framework-plugin-runtime-instances (lib) | 5 |
| semio-framework-plugin-runtime-ops (lib) | 1 |
| semio-framework-plugin-store-stage-laws (lib) | 13 |
| semio-framework-plugin-surface-fixture (lib) | 48 |
| semio-framework-plugin-tests-01 (lib) | 245 |
| semio-framework-plugin-tests-01-2 (lib) | 18 |
| semio-framework-plugin-tests-02 (lib) | 16 |
| semio-framework-plugin-tests-02-2 (lib) | 2 |
| semio-framework-plugin-tests-03 (lib) | 6 |
| semio-framework-plugin-tests-03-2 (lib) | 40 |
| semio-framework-plugin-tests-05 (lib) | 129 |
| semio-framework-plugin-tests-05-2 (lib) | 43 |
| semio-framework-plugin-tests-06 (lib) | 378 |
| semio-framework-plugin-tests-06-2 (lib) | 43 |
| semio-framework-plugin-tests-17 (lib) | 34 |
| semio-framework-plugin-tests-18 (lib) | 83 |
| semio-framework-plugin-tests-19 (lib) | 47 |
| semio-framework-plugin-time-travel (lib) | 13 |
| semio-framework-plugin-tool-run (lib) | 5 |
| semio-framework-plugin-window-config (lib) | 3 |
| semio-framework-plugin-window-config (test) | 1 |
| semio-framework-repo-dashboard (lib) | 1 |

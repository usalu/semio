# r11-store: retirement refactor completion (commit 677 fallout)

Owner design understood and applied; no compat shims, no re-added removed types. Gate label `r11-store`, logs `T/🗑️generated/r11-store/` (T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`). Sub-agent reports: `r11-exec-store-{a,b,t2,p1,p2,p2b,p2c,p3,m,inf,f}.md`.

## Design (as implemented)
- Currency: `RetainedCloneGrant` (5 axes) in, `RetainedCloneProgress` out, `RetainedCloneStep::{Progress,Complete}`; every owner quotes a `RetirementDemand` first; a grant axis below the quote yields `Progress(default)`, depth below the quote is an error, one item per call, receipt must `fits(grant)`, `Complete` only with the terminal-empty witness. Imports live under `semio_framework_value::retained_clone::`.
- Store: `DocumentStoreOwners` catalogs are funded (`funded_*_store_owners`, `install_unscheduled_catalog`, `complete_unscheduled_catalog`); displaced snapshots retire through `ArtifactStoreDisplacedSnapshotRetirement` (grant-based `SnapshotRetirementFactory::retire`); `ArtifactStore::close_owned_unscheduled()` for cold teardown; authoring identity threaded (`EntityIdentityAuthority`, plugin macro `crate::with_authoring_identity!`).
- Plugin ladders: close ladder (`🪜️close-ladder`) is a rung enum, each rung = (quote, granted step); maintenance ladder (`🪜️maintenance-ladder`, agent m) restores the 26-stage rotation incl. store replacement, archive loads, envelope ingress, peer roster; envelope frontier (`🪜️envelope-ladder`). Frontiers without a rung fail closed with `UnsupportedOwner` (never silently leaked).
- Z-close bug (demand answered 1 while a partition owes up to 32 KB): `window_config_store`/transient/retired-window frontiers now have rungs (`DirectIngress`, `OwnedStores` lanes 6/7, `WindowRetirements`) and `close_terminal_is_empty` terms match the rungs.

## Compile state (through the gate)
| target | result |
|---|---|
| `semio-framework-os-kernel --lib` (store, spr, io, ...) | 0 errors |
| `semio-framework-plugin --lib` (+ dag, playbook crates) | 0 errors, borrowck included |
| `semio-framework-os-infinite --lib` and `--lib --tests` | 0 errors (agent inf); tests: 516 pass, 3 fail serially (pick/GPU, not retirement), 56 fail in parallel (global mesh claim, pass with `--test-threads=1`) |
| `semio-framework-artifact-flow-flow --lib` | 0 errors |
| `semio-framework-os-flow --lib` | in migration (agent f): host/wasm/extensions/vcs/mesh |
| `semio-s-artifact-stdio-contract` | blocked by a concurrent half-written edit in `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🎬️media-export/🦀️.rs` (duplicated `pub struct RetireOwnedSnapshotDisposer`), not retirement related |
| `semio-s-artifact-bim-model --lib --tests` | not reached yet: chain stops at os-flow and stdio-contract |

Tests: value crate (own manifest) `cargo test`: 218 pass, 2 fail (`retirement_turn_original_queue_preserves_full_grants_...`, `retained_fixed_list_depth_quotes_...`) in the untouched value crate, i.e. already failing at the owner's HEAD. Kernel/store/spr unit tests (`cargo test -p semio-framework-os-kernel`): migration by agents t1/t4 in flight; before: 303 lib errors + ~1400 test errors (nothing ran); after: see update below.

## Files migrated (high level)
- store `🏪️store/🦀️.rs` (+ `📦️prepared/♻️retirement`), os-kernel callers (`⚛️`, `📡️spr` command/touched paths kept additive), 2d font, io serialization, pack.
- plugin crate root + submodules (sub-agents p1/p2/p3), new modules `🪪️identity`, `🪜️close-ladder`, `🪜️envelope-ladder`, `🪜️maintenance-ladder`.
- dag, playbook generation, vcs/retained/copy, flow registry (neural `Operator` retire API), infinite board jobs + world + dag leaf (restored `DagScreenHit`/`Value` import dropped by the 677 board refactor, removed dead `force_graph` re-export).

## Open items
1. `os-flow` and `stdio-contract` (above); then the BIM crate itself (report only: BIM `Snapshot/Mutation` types need `#[derive(RetireOwned)]`, mounted_job hooks use the new `mounted_job_{close,maintenance}_{demands,step}(instance, ...)` shapes, `build_*_store_owners() -> Option<Result<DocumentStoreOwners, ValueError>>`).
2. Plugin crate `--lib --tests`: ~1500 test-file errors (old shapes), plugin `cargo test` not run.
3. Maintenance quotes that the job/store crates cannot give yet (worker-job pump, archive hydration step) quote depth only (see `r11-exec-store-m.md`).
4. Pre-existing owner failures: value crate 2 tests (above); infinite pick tests (3).

## Update (os-flow landed, stdio crates in flight)
- `semio-framework-os-flow --lib`: 0 errors (agents f, w + my `🕸️wasm` fixes; stale tail in `🕸️wasm/🦀️.rs` trimmed). Added `'static` bounds on `artifact_pair_snapshot`, `ArtifactDocumentPayload::{snapshot, settled_snapshot}` (store parse functions now need them; BIM callers use concrete types).
- stdio `🎬️media-export` syntax breakage was a peer's mid-edit and is gone. `💾️binary` migrated (RetireOwned derives). `🗜️deflate` (38), `🔤️txt` (9) and further stdio crates in the BIM closure: agent s (`r11-exec-store-s.md`).
- Kernel lib is green; unit tests compile but fail on library gaps (replay drop custody, authority tuple, children returning Complete early, depth 64 quote): agents l1, l2 (`r11-exec-store-l1.md`, `-l2.md`); inventory in `r11-exec-store-t1.md`, `-t4.md`.
- Check command for the chain: `cargo check --keep-going --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib` (via `🚦️gate.sh`); `--keep-going` surfaces all independent failing crates in one run.

## Update 2: chain reaches the BIM crate (0 framework errors)
`cargo check --keep-going --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --tests` (gate, log `🗑️generated/r11-store/bim10.txt`): every framework, flow, infinite, dag, playbook and stdio crate compiles; the only failing crate is `semio-s-artifact-bim-model` itself: lib 3 errors, lib-test 87 errors, all BIM-side (missing `spaces`/`diagnostics`/`element_solids`/`model_graph` modules, `ModelInference::infer` (47), `phase` field, `document_pack`, `UiLabel::as_str`, `DiffAlgebra::between`) = in-flight WP work, not retirement API.
- Value crate: `impl RetireOwned for [T: RetireOwned; N]` (FixedArray cursor) replaces the Copy-only impl; `RetainedClone for [T; N]` additionally requires `T: RetireOwned`. Value tests 218 pass, same 2 failures as before the change (pre-existing in the owner's HEAD).
- stdio closure migrated (agent s, pdf, step): see `r11-exec-store-{s,pdf,step}.md`; open decision there: svg-tiny variant rename `restore-non-tiny` -> `reinstate-non-tiny` was applied in code/leaf JSON/oracles, aggregate schema catalog still says the old kind.
- os-flow `--lib --tests` compiles; `host::` tests 59 pass / 79 fail on the missing Flow operation wire source (agent f adding the Flow preparation factory; every artifact must install one, see `PlainAuthoringFactory` refusal).
- Still running (reports follow in their own files): l1 (replay custody, plain test store wire source, hangs), l2 (authority tuple, children `Complete` while not terminal, depth-64 quote), f (os-flow tests). Kernel unit tests: compile green; pass counts before/after are in `r11-exec-store-t1.md`/`-t4.md` (before: 233/400 store unit, 38/117 t4 modules) and will be superseded by l1/l2 reports.
- Not done / deferred (coordinator): plugin crate `--lib --tests` (~1500 old-shape test errors), `brep` extension owner + its tests, `md` stdio `splice-source` verb, `🛢️db`/hub callers.

## Update 3: svg-tiny ruling applied; BIM lib compiles
- `restore` is in `APPROVED_VERBS` (`("restore","Restored")`, `spr/🎮️command/🦀️.rs:162`), so `restore-non-tiny` stays the single name: the 677 half-rename (`ReinstateNonTiny` variant/struct, `ReinstatedElement/Attribute`, oracle/leaf-JSON `reinstate-non-tiny`) was reverted to `RestoreNonTiny`, `RestoredElement`, `RestoredAttribute`, `restore-non-tiny` across code, leaf JSON, oracles; schema catalog already said `restore-non-tiny`.
- Gate `cargo check --keep-going … -p semio-s-artifact-bim-model --lib`: exit 0 (BIM lib compiles with the whole framework chain). `--lib --tests` still had BIM-side test errors at the last run (87).

## Final counts (helpers l1, l2, f finished; details in `r11-exec-store-{l1,l2,f}.md`)
| suite | before | after |
|---|---|---|
| os-kernel `--lib` / `--tests` compile | 303 lib errors / ~1400 test errors | 0 / 0 |
| store unit module (`os_store::component::tests`, 400 tests, one process each) | did not compile | 233 first run -> 360 pass, 40 fail |
| whole `os_store::` (l2 run, 637 tests) | did not compile | 434 first run -> 567 pass, 70 fail |
| other kernel modules (t4 set, 117 tests) | did not compile | 38 pass (before l1/l2 fixes), re-run not done |
| value crate (own manifest) | 218/2 | 218/2 (same two pre-existing failures) |
| infinite `--lib --tests` | did not compile | 516 pass, 3 fail serially (pick/GPU), 56 parallel failures from the global mesh claim (pass with `--test-threads=1`) |
| os-flow `--lib --tests` | did not compile | 259 pass, 7 fail (3 host dag/fixture, 2 vcs 13-vs-12 operations fixture, 2 `wasm_session::domain_laws` in the wasm file) |
| flow extension wasm (`extensions::wasm`) | did not compile | 8 pass, 0 fail |

Remaining library failures (kernel): `SpaceHost` meta store and codec-reduction (`apply_ops_binary`) stores lack a production operation wire source (7); 4 store-close deadlocks and 7 member-close stalls on a shared displaced snapshot retirement (value crate refuses to retire an `Arc` while a `Weak` exists; needs an owner decision); hot_path close livelocks in disposer phase `Complete` (envelope retirement of a 1000-edit history); `replay_retirement` corpus law (encode failure at the first operation); 2 process aborts; 7 `os_dsl`/`os_pack`/`os_spr` tests. `plain_document_store_owners` test wire source leaks the op bytes (test-only). Plugin crate `--lib --tests` (~1500 old-shape test errors), artifacts/flow own tests, brep extension owner and `md` splice-source verb remain deferred.

## Update 4: Weak ruling, next waves
- Weak ruling implemented: value crate `FactoryTicket`, `SharedControlledRetirement` (lease and exclusive paths), `FactorySharedRetirement` no longer wait on `Arc::weak_count`; the Arc frame counts as released only when no Weak remained (else it frees with the last Weak: quote 0). Same in store (presence peers, backbone queue/wake, group visibility) and plugin (group visibility owner). Law tests rewritten to the new law (`shared_unique_original_arc_retires_despite_weak_aliases_and_credits_actual_frame`, `factory_preborn_tickets_never_wait_on_weak_aliases_and_copy_only_body_truthful`, `shared_factory_custody_*`, `shared_full_grant_owns_original_aliases_weak_backing_*`): value crate tests 218 pass / same 2 pre-existing failures.
- Value crate also: 4-tuple `RetireOwned` controlled support, std collections declare controlled retirement (l2), `[T; N]` for `T: RetireOwned` (FixedArray).
- Helpers running: l3 (store-close deadlocks, hot-path livelock, 2 aborts), l4 (replay corpus law, SpaceHost/codec wire source, os_dsl/os_pack/os_spr), plugin test migration pt1..pt7 (plugin `--lib --tests`: 1283 errors in ~45 files at census, split by file group; pt7 = interaction submodules + tool-run + authority, waiting for a free subagent slot). Rule from the coordinator: `os-kernel --lib` stays green after every edit (restored at the last l4 fix).

## Update 5: after the session-limit stop
- Libs re-checked through the gate after the reset: `semio-framework-value --lib`, `semio-framework-os-kernel --lib` and `semio-framework-plugin --lib`: 0 errors each (Finished).
- Helpers resumed via SendMessage on their intact transcripts: l3 (store-close deadlocks, hot-path livelock, 2 aborts), l4 (replay corpus law, SpaceHost/codec wire source, dsl/pack/spr), pt1 (builder-contract tests), pt2 (composition), pt3 (typed-command operation), pt4 (mutation fixtures), pt6 (extension/child registry), pt7 (interaction submodules + tool-run + authority). pt5 (time-travel/reload: 8 files) had reported: migrated, compile-clean in its files at typeck, borrowck/runtime not yet verified because upstream crates were red during its runs. Compile-atomic rule in force for all.

## Update 6: l4 finished (`r11-exec-store-l4.md`)
Kernel combined run (`os_dsl::`, `os_pack::`, `os_spr::`, `space_`, `canonical`, `sqlite_snapshot`, `document_codec`, `replay_retirement`, `runtime_seed`; needs `SEMIO_TEST_ARTIFACT_DIR` set): before 660 pass / 8 fail and 138 / 10, after **668 pass / 0 fail**. Production wire sources added (`ArtifactCanonicalAuthoringFactory`, canonical JSON for the six `SpaceHistoryMutation` leaves, `HybridLogicalTimestamp::to_value_controlled`, `ArtifactCodec::{of_authoring, bare_authoring}`); the leaking test wire is replaced by a non-storing borrowed view. Open for l3: store-close deadlocks, hot-path livelock, 2 process aborts.

## Update 7: l3 report (`r11-exec-store-l3.md`), plugin test helpers' interim status
- `os_store::` (637 tests, one process per test): 581 pass at l3's start (Weak ruling in) -> 603 pass / 34 fail (k5). Fixed: the 4 store-close deadlocks (`SharedValueRetirementFactory::retire` now uses lease custody: a displaced snapshot no longer waits for an alias that retires only in a later disposer phase), `hot_path_tests::local_steps_cost_o_change...` (not a livelock: exponential demand re-quoting `4^depth` per turn in nested owners + per-element fixed-array cursors; `[T; N]` of trivial elements now retires as bounded copy work: 80-edit close 80 893 -> 29 966 turns, 10.6 s -> fast), the 2 process aborts (assertion inside `Drop` while unwinding; `thread::panicking()` guards added in `ArtifactHistoryLedger::drop` and `MemberStoreOpenRetained::drop`; they now surface their real assertions).
- Still failing in `os_store::` (causes in the l3 report): fixture ceilings below real quotes (e.g. ledger slots 393 216 B vs `admissionBytes` 262 144 in member-open / completed-registry / resident-backing laws: an owner decision whether to raise the fixture ceilings), `retained_member_common_root_copies_history_under_grant` (95 k turns vs helper cap 65 536), `erased_member_snapshot_read_releases_its_alias_before_the_live_current_root` (one-turn terminal law vs factory capability turns), `retained_genesis_aliases_retire_each_decoded_allocation_once` (count 1 vs 2). Debug probe `debug_close_scaling_probe` in `store/🧪️tests/⚡️hot-path` is marked `[DEBUG]` and must be deleted.
- Plugin tests: pt5 (8 test files) and pt6 reported, pt2, pt3 migrated their files; none could run tests yet: repeated upstream breakages (io-sqlite-snapshot WIP, value retained-clone) kept the plugin `--tests` build from completing. pt1, pt4, pt7 running.

## Update 8: l3 final (`r11-exec-store-l3.md`)
- `os_store::` (one process per test, 637-643 tests): start 581 pass / 56 fail -> k5 603 pass / 34 fail -> last sweep k13 594 pass / 49 fail; the k13 regression (16 tests: 11 `retained_clone_tests`, `retained_read_retirement::captured_store_source…`, `snapshot_clone_preparation::handoff::partial_paged_cursor…`, 2 `member_owned_batch_admission_*`, 1 member publication wrong-owner) came from a peer's in-flight `RetainedCloneSource<T,A>` rework ("original source birth requires complete capacity and alias-copy admission"), not from l3.
- Fixed by l3: 4 store-close deadlocks (lease custody in `SharedValueRetirementFactory`), hot path (one-pass `ErasedSnapshotRetirement::next_demand` instead of 4^depth re-quoting: 80-edit close 10.6 s -> 0.31 s; `PlainArray` bulk cursor), 2 process aborts (`thread::panicking()` guards).
- OWNER DECISIONS pending: (1) the edit-message ledger needs one 384 KiB slot allocation but the member-open / completed-registry / resident-backing / replay-preparation fixtures admit 256 KiB (`admissionBytes: 262144`): raise the admission ceilings (fixtures + `.ts` laws + schema pin) or shrink the ledger slot allocation; (2) `erased_member_snapshot_read_releases_its_alias_before_the_live_current_root` expects terminal after ONE step but `FactorySharedRetirement` needs several turns (dyn factory Arc cannot be released race-free in one turn); (3) `retained_member_common_root_copies_history_under_grant` needs ~95k turns vs the helper's 65 536 cap (raise the cap or accept).
- Housekeeping: delete the `[DEBUG]` probe `debug_close_scaling_probe` in `store/🧪️tests/⚡️hot-path`.

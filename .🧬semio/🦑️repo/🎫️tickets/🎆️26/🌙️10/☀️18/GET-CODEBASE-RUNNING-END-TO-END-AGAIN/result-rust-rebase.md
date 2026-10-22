# Result rust-rebase (executor rust-rebase, wave 1022a)

Tools: `C/rust-rebase-tools/{run.sh,check.sh,root1.sh,fw2.sh}` (one detached `bun repo cargo --store=shared --configuration=prototype -- test|check --manifest-path <ws> --no-fail-fast` per run, retried on capacity/lease refusals, `RUST_MIN_STACK=32 MiB`). Logs in `C/🗑️generated/rust-rebase-logs/` (`fw6-final.log` = last framework run, `root5.log` = last root run, earlier attempts kept as `*-attemptN.log`). Never more than one cargo of mine at a time; no process of mine is left running; no git-modifying command used.

## 1. Decision on commit daa678dd3dc (SealedShared over SharedOriginalStorage): INTENDED
Evidence (design, schema, oracle, consistency):
- New module `🌱️value/♻️retirement/🔗️arc-custody/📦️original/` with its own closed schema, fixtures and TS/SQLite oracle ("stable original aliases and field retirement agree with independent SQL").
- Design note `DASHBOARD-LAUNCH-COCKPIT/oct21-stable-original-custody.md`: the old final unwrap quoted 5128 copied bytes against the 4096 copy limit; the new owner is an opaque counted allocation (atomic alias count + `MaybeUninit<T>`), constructor and release quotes use the real `Layout`.
- `SealedShared` unit tests (`semio-framework-value-shared-issuance` lib) and the code are consistent with it and were green in the first run.
Accounting formula (every re-baseline below is derived from it, not copied from output):
- birth of `SealedShared<T>` = `arc_bytes::<Allocation<T>>()` = 16 (Arc header) + 8 (alias counter) + `size_of T`, padded: `String` 40 -> 48, the 16-byte `SourceRoots` wrapper 32 -> 40.
- non-last alias duplicate / retire: copy 0, depth 1.
- last-alias election of a type without the stable-field capability (honest by-value path): copy = `size_of::<Option<SharedOriginalStorage<T>>>()` + 2 x `size_of::<Option<T>>()`. For the 24-byte `RetainedCloneLeaseOwner`: 16 + 2 x 32 = 80. Independently derived by layout (`Option<ManuallyDrop<Option<Arc>>>` has no niche left = 16 B; `Option<Lease>` = 32 B) and equal to the observed `80 != 0` of the pack test. Types with the capability (derive(RetireOwned) records) pay capacity instead of copy.
- last-alias release = birth bytes; `original_transfer_demand` = issuer transfer copy (0 for the source roots) + birth bytes.
- consequence for fixtures: one `SealedShared<String>` birth +8 (receiving ledger: 7 shared births = +56: sealed 40 -> 48, scaffold 9392 -> 9440, final 9450 -> 9506; the SQL ledger oracle sums 7 + 11 + 48 + 9440 = 9506).
The peer edit `Cursor::next_work_byte_demand` (quote now equals the step requirement) in `sealed/🦀️.rs` was already in the tree; I kept it.

## 2. Table: target -> decision -> status
Status = result of the last run (`fw6-final.log` framework, `root5.log` root). GREEN = whole target binary passes.

| target | class | decision / fix | status |
|---|---|---|---|
| value-shared-issuance (lib) | daa | reference implementation, unchanged | GREEN |
| job `original-payload-builder` | daa-adjacent (builder charges init/seal/close copy) | test funds each turn with `max(policy, quote)`, metadata turns assert receipt == quote, payload copy == source length, close receipts == quote; fixture policy (copy 17) kept | GREEN |
| job `original-paged-source` | same | close loop funded from `retirement_demands()`, init/append/close/cancel loops bounded (it spun forever) | GREEN |
| job-publication `original-source-publication` | same | funded turn = `max(policy, advance_demands)`, fill/metadata turn classification, close loop bounded | GREEN |
| job `original-fixture-caller` (2) | not daa: commit 696 removed the clock/deadline gate from `retire_step` (retirement is clock free, `retirement_should_yield` ignores the deadline once original retirement) | tests expect the paid root unwrap (1 item, release 40 = 16 + 8 + 16), no further turn after it; fixture + schema + TS oracle `rootRetirement` | GREEN |
| job `original-page-return` | not daa: native construction needs the original Job decoder issuer (696) | `OriginalTestPayload` carries an `OriginalJobDecodeOwner`, turns use `turn_with_decoder`, `close()` closes the decoder | GREEN |
| pack `original-owned-wire` (4) | daa (80 B lease election) + REAL BUG | grants funded by quote (`funded(copy, quote)`); fixture/schema/TS `leaseElection` (16 + 2 x 32 = 80 < 8 x 64, never a wide copy). REAL BUG: `RetireOwned for BorrowedProjectedPackCursor` retired only `symbols` and leaked `frames`, `chunks`, `expression` (4488 B at stop=3); now one physical sequence via `PhysicalBorrowedPack` + `RetireOwned` for `RetainedExpressionCursor` and the two leaf `Frame` types | GREEN |
| replication-mutation-retained `retained_central_diff_forwards...` | daa | fixture + schema: sealed 48, scaffold 9440, final 9506 (see 1) | GREEN |
| replication-mutation-retained `retained_key_lookup...` | daa (non-last alias handoff is copy free) | copy denial only where the quote has copy | GREEN |
| replication-mutation-retained-field (4: indexed x2, record x2) | REAL BUG, not daa (4165 = 4096 + 67 + 2, 4132 = 4096 + 34 + 2) | the advance/demand path did not subtract the nested headers from `body` (the close path did): `decision_demand/decide/inverse_demand/inverse` (field), `demand/advance` (indexed, record) now pass `body - META` down, so every quote fits its body | GREEN |
| replication `retained_diff_sequence` (2) | isolation: process-wide `PROJECTIONS` counter shared by 3 tests | `PROJECTION_GATE` mutex in `run()` | GREEN |
| tool-run lib `payload_stream` (3) | carve: lib has no heap-witness allocator, receipts (176/112) vs (0,0) | cfg(test) `HeapWitness` global allocator in the package glue | GREEN |
| tool-run-tick-wire | - | unchanged | GREEN |
| value-derive `neutral_owner` (`paged_native_recursive_clone...`) | daa-adjacent | clone frames are 5 per level + 3 (measured 8/13/18/88 for depth 1/2/3/17, linear); fixture `cloneFrames {perLevel 5, base 3}`, grant depth = 5 d + 3 and asserted equal to the deepest quote; mixed copy+capacity quotes (120/120) are funded exactly by quote | GREEN |
| async-tests companion lib-test | carve | `wasm-pool-host` feature (Lane: Deserialize), wrapper imports (`RetirementCursor`, `RetireOwned`), 17 types made `pub #[doc(hidden)]` in async (the 177 hidden "private type" errors behind the first 10), `ThreadWakeRetirement` re-export instead of alias, generated typegen header `repo run` | GREEN |
| dsl-record, job, pack, value-derive remaining binaries | - | no change | GREEN |
| pack-record-encode-cursor lib `borrowed_projected_pack_original_expression_ast...` | pre-existing, not daa, not touched | expression cursor prints `--2` for the corpus row `-(-2)` | OPEN |
| os-store-read `original_history_store_retirement...` | REAL BUG | `ArtifactStoreHistoryLedgerRetirement` set `closing_factory` but never quoted/stepped it (terminal never reached): `demands` and `close_step` handle it first | GREEN |
| os-store-space-history (3) | carve | cfg(test) `RequestedAllocator`; fixture `owner` = `semio_framework_os_store_space_history::io::sqlite::snapshot` (was the pre-carve path) | GREEN |
| os-store-member-open | carve | cfg(test) `RequestedAllocator` | GREEN |
| os-store-initialization-replay (2) | not daa: `HistoryPageStack` releases directory chunk and page in two steps (694) | tests loop per lane (exactly 2 steps), close bound 12 | GREEN |
| os-store-one-item-batch `mutation_apply` | environment: stray `🧰️framework/.🧬semio` made the repo-root probe pick the wrong ancestor | probe requires `.🧬semio` and `🧰️framework` | GREEN |
| os-store-codec-registry | dev-dep | `semio-framework-job` dev-dep gets `testkit` | GREEN |
| os-store-engine lifted store (compile) | rust-port artefact | removed the duplicate cfg(test) `use ColdMutation` | compiles |
| os-store-backbone, -canonical-json, -ledger, -one-item-publication, -retirement, -snapshot-clone | - | green with the allocators added by rust-tests | GREEN |
| os-store-engine `original_derived_semantic_task_admits_copy_and_capacity_as_independent_currencies` | carve + API drift | allocator added (the first half and all heap receipts now pass); cancellation tail still fails: `ArtifactDerivedOperation` retirement needs a decoder control with an installed retirement recipient (`NativeDecodeControl::new` has none) | OPEN |
| os-history `retained_native_history_fold_obeys_the_neutral_law` | NOT daa (`SharedUtf8` is Arc based, fold uses no SealedShared) | unlocated stall of the coroutine at work 1 / byte 1: pending copy 241 = 2 x 120 + 1, items 0, depth 1, grant admits it, neither `FoldIndexRead` nor `FoldIndexInsert` is the unadmitted future (instrumented, removed again). Next step: instrument `FoldByteRead/Copy`, `vector`, `track` polls | OPEN |
| os-store-space-schema `every_leaf_shape_has_a_canonical_tree...` | NOT daa | canonical tree of `CommitSpaceCheckpoint` omits `index` (`Option<u64>`, `skip_serializing_if`) while the value document has it; derive `MutationLeaf` canonical tree issue | OPEN |
| os-store-tests-02 (`history_transition_unit`) | NOT daa | does not compile: 3 x E0599 (`ArtifactStore::derive_reference_provisional_snapshot` / `snapshot_owner` / `owned_snapshot_retirement_factory` unavailable for `DemoMutation`; `M: Mutation<P>` bounds lack `ColdMutation<P>`), so the history test could not be evaluated | OPEN (blocked, rust-port area) |

## 3. Changed files (all small anchored edits, re-read before editing)
Framework (`🧰️framework/🔨️modules/`):
- job: `🧵️job/📬️outcome/📦️builder/🧪️tests/🌍️public/🦀️.rs`, `📖️source/🧪️tests/🦀️.rs`, `📤️publication/🧪️tests/🦀️.rs`, `⏱️context/🧪️testing/🎟️caller/{🧪️tests/🦀️.rs,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}`, `🧪️testing/📬️payload/🦀️.rs`
- pack/dsl: `🎒️pack/🌱️value/🛫️encode/🫳️borrowed/📨️wire/{🧪️tests/🦀️.rs,🟦️.ts,🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}`, `⏳️cursor/🦀️.rs` (physical retirement), `🗣️dsl/🧬️schema/🛫️encode/🧮️expression/🦀️.rs` (RetireOwned)
- replication: `🎮️mutation/🧳️retained/🔎️key/🧪️tests/🦀️.rs`, `🔺️diff/🫴️receiving/{🧪️tests/🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}`, `🪡️field/🟰️change/{🦀️.rs,📋️indexed/🦀️.rs,🎚️record/🦀️.rs}`, `🔺️diff/🪢️sequence/🧪️tests/🦀️.rs`
- tool-run: `⏯️tool-run/📦️packages/🦀️rust/🦀️.rs`
- value: `🌱️value/🧪️tests/🧩️neutral-owner/🦀️.rs`, `🌱️value/📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json`
- async: `⏳️async/🧪️tests/🥣️companion/{📦️packages/🦀️rust/Cargo.toml,📬️publication-claim/🦀️.rs,🧵️thread-wake/🦀️.rs}`, `🤖️generated/⏳️async/🟦️.ts`, visibility in `⛔️cancel-token`, `♻️cancel-return/📬️claim`, `🔔️maintenance`, `🛑️cancel/🚦️waiters`, `🔔️worker-parking`, `🔔️deferred-wake`, `🖥️native-pool`, `🔔️wake/🧵️thread`
Root (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/`): `⚙️engine/🦀️.rs`, `⚙️engine/📥️lifted/🏪️store/🦀️.rs`, `🔁️replay/🎮️operation/🌱️derived/🧪️tests/🦀️.rs`, `🧩️composition/🚪️open/🦀️.rs`, `📦️codec/📦️packages/🦀️rust/Cargo.toml`, `📜️space-history/{🦀️.rs,🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🚪️ownership/🔣️.json}`, `🔗️read/{🧮️cursor-ledger-and-vcs-retirements/🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs`, `📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs`.
Temporary `[DEBUG]` instrumentation (fold, index, pack wire, neutral-owner) was removed again; `git diff` shows no `[DEBUG]` of mine.

## 4. Commands and outcome
- `fw1` (framework, 10 packages): 11 red targets (this was the diagnosis run); `root1-attempt*` (root, 15 packages): compile blockers fixed one by one (duplicate import, dev-dep testkit, a peer's transient duplicate Cargo key) then 8 red.
- final `fw6` = `bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml --no-fail-fast -p job -p job-publication -p pack -p pack-record-encode-cursor -p dsl-record -p replication-mutation-retained -p replication-mutation-retained-field -p replication -p tool-run -p value-derive -p async-tests`: 1 red target (pack-record-encode-cursor, pre-existing expression corpus), everything else green.
- final `root5` (`-p os-history os-store-{read,engine,space-history,space-schema,backbone,canonical-json,codec-registry,initialization-replay,ledger,member-open,one-item-publication,retirement,snapshot-clone,one-item-batch}`): 3 red (os-history fold, engine derived tail, space-schema), 13 green.
- `cargo check --tests -p semio-framework-async-tests`: green (4 iterations to remove the hidden private-type errors).

## 5. Open / notes for the coordinator
- Open (not accounting): os-history fold stall, engine derived-op cancellation tail (needs a retirement-recipient decode control), space-schema canonical tree `index`, os-store-tests-02 compile, pack-record-encode-cursor expression `--2`.
- `🧰️framework/.🧬semio/🦑️repo/⚡️cache/rust/fixed/` (16 GB sparse `fixed.dmg`, created 16:01 by a `bun repo cargo` run that resolved the repository root to `🧰️framework`, not by my runs: they always used `Cargo.toml` / `🧰️framework/Cargo.toml` from the repo root cwd) is a stray native cache image and a second `.🧬semio` marker; whoever owns it should remove it (I did not touch it).
- The fixed-image lane `a1op65zrdbr` is shared by all agents of this session: concurrent cargos run in an overflow lane that starts empty (cold, slow); capacity waits of 10-20 min were common.
- The sealed accounting changes need one more independent oracle if the by-value fallback ever changes: `original_demands` in `sealed/🦀️.rs` and the three fixtures `leaseElection` (pack wire), `rootRetirement` (fixture caller), receiving ledger (sealed 48) are the places that encode it.

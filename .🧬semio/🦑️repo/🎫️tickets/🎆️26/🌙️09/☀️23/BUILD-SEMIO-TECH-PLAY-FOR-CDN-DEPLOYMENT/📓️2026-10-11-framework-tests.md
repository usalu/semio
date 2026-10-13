# 2026-10-11 Framework Test-Target Health After The Grant-Based Retained-Clone Migration

Executor: fw-tests (Sonnet). Scope: os-kernel `canonical_edit` stall, replication `--lib`, plugin crate `--lib` test target, shooting artifact (not reached). All cargo ran in the foreground through `🚦️cargo-slot.sh` with the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/fw-tests/target` (deleted at the end; intermediates are the shared build dir). No wasm32 cargo, no git-modifying commands.

## Per-target table

| Target | Compile before | Compile now | Passed / failed / other | Fixes | Remaining (owner) |
| --- | --- | --- | --- | --- | --- |
| os-kernel `canonical_edit` | ok | ok | 31 / 0 / 0 (whole filter runs in ~3 s) | sealer stall (2 causes), reader accounting + bulk fill, test rewrites | none |
| os-kernel `--lib` (all 1463, one process per test, 6 in parallel) | ok | ok | 1346 ok / 105 failed / 8 abort / 4 timeout | store unit grants (512 -> 4096) turned 5 livelocks into assertion failures | see "os-kernel remaining" |
| replication `--lib` (+ `paged_seal_digest`) | 12 errors | ok | 340 / 0 / 0 and 1 / 0 / 0 | 5 test files migrated, 1 product fix, 3 fixtures + TS oracle re-derived | none |
| plugin `--lib` (1227 tests, one process per test, `RUST_MIN_STACK` 64 MiB) | 33 errors in the first pass, 0 after | ok | 759 ok / 420 failed / 46 abort / 2 other | 3 product fixes (below), ~30 test files migrated | see "plugin remaining" |
| plugin `window_config_replace_reaches_prepared_through_the_real_sealer_and_closes_under_quotes` | compile | ok | passes: Prepared in 322 turns, peak live 47 087 B of declared 131 072 B | none needed | the old note "WindowConfigReplaceEdit still declares 65536" is moot: `CONFIG_EDIT_RETAINED_BASELINE_BYTES` is 131 072 and the real-sealer peak is far below it |
| shooting artifact `--lib` | 46 errors | 46 errors (not started) | not run | none | stale unit-test template, same API drift as gisterrain/vcs/space `✏️editor/🧪️tests/🔬️unit` (`new_app_with_registry`, `close_registered_fixture_app`, `handle_action`, `dispatch` all gained `mounted_policy` + `identity`); owner: the artifact unit-test debt wave (see `project-plugin-unit-test-debt-playbook`) |

## 1. os-kernel `canonical_edit` and the cancel-at-turn-0 stall

Root causes of `canonical_store_native_sealer_mounts_original_sources…` (never settled, both turns of the close loop made no progress):

1. TEST bug, not product: the test's `RootFactory` retired the post snapshot with `admit_shared_retirement(.., lease_only = false)` while the test itself still held a strong alias of that `Arc`. A non-lease shared retirement waits for uniqueness forever. Production (`BoundedArtifactRetirementFactory`) already uses `true`. Fixed in `🧳️source/🧪️tests/🦀️.rs`.
2. PRODUCT contract bug (surfaced as soon as 1 was fixed, cancel_turn=257): `ArtifactStoreOneItemSealer::advance` checked `grant.permits_one()` (items and depth) before answering a Prepared sealer, so the free "query" turn of a prepared sealer was `Blocked` under a depth-0 grant although it spends nothing. Reordered in `🧵️canonical-edit/🦀️.rs`: cancelled/closing -> Blocked, prepared -> `Prepared`, then the funding check.
3. `…all_cancellation_frontiers` (`turns < 1000000`) is green again with the unchanged product code: it was a victim of the same stall family, no separate cause.

Reader / borrowed encoder (`reader::*`, `borrowed_tests::*`): three real defects in the legacy borrowed encoder that the plugin interaction capture still uses.

- `ArtifactCanonicalEditEncoder::encode_chunk` produced one byte per call regardless of the chunk grant (a 4 925 byte document needed thousands of turns, the 100 000 turn tests died). It now fills the output up to `min(output, copy grant)` in one turn, `copied_bytes` = bytes written, partial bytes reported on error.
- `ArtifactCanonicalEditEncoder::close_step` dropped frames (`Box<ArtifactCanonicalJsonCursor>` = 3 184 B, boxed iterators) with an unreported physical release (`(0, 3184)` heap vs receipt). It now reports `released_bytes` per dropped frame and quotes it (`next_close_release_byte_demand`, wired into `ReaderState::demands`).
- Tests rewritten to the native architecture: `borrowed_tests` (no `owner.edit`; quoted grants through `preparation_demands()`; `restore_checkpoint` now asserts the designed `canonical-edit.native-checkpoint-unsupported`; the legacy fixed-depth-stack refusal became "a deep map seals under the depth grant"; `MapMutation` gained path-based `canonical_json_node`/`key` over its tree), `reader::tests` (frame copy floor 64 B, no iterator-lifetime asserts for the Indexed walk, sealer failed-prefix test drives the native sealer and asserts no prepared authority + exact close). The legacy iterator-lifetime laws (`active_iterators`) are unreachable on the Indexed path by design.

Counts: `cargo test -p semio-framework-os-kernel --lib canonical_edit` = 31 passed, 0 failed (was 20 passed / 5 failed / 3 hung). Whole lib: see below.

### os-kernel remaining (1463 tests; counts above)

Clusters from the isolated run (message -> likely owner):

- fixture catalogs install `ArtifactCanonicalAuthoringFactory` ("fixture authoring catalog prepares no retained gesture", ~8) or fixture factories whose tickets are not all admitted ("installed original catalog requires every admitted factory ticket", 9) -> store fixture owner (fd-window-edit / os-domains).
- `durable_group` ×4 "unprepared factory ticket lost its original source" -> already listed in corrections (`FactoryTicket::preparation_demands` with an alias taken), store owner.
- sqlite space-history baseline ×9 ("close made no funded progress", "producer exceeds remaining physical grant") -> stdio B / store.
- `member_*` ×~20 (history dictionary abort `next_copy_byte_demand <= grant`, owned batch constructor refusals, genesis member envelope) -> store member-open owner.
- 4 timeouts at 600 s under fleet load: `retained_clone_tests::{production_snapshot_read_lease…, retained_vector_and_completed_optional…}`, `tests::{an_ephemeral_one_item_publication_reports_monotone_progress…, artifact_store_batch_cancel_mid_flight…}`: a livelock or a very slow debug walk; reproduce with a quoted `--test-threads=1` run, store owner.
- The 7 `ArtifactStoreOneItemGrant { 512,512,512,64 }` unit grants were below the new sealer constructor floor (copy 2 384, capacity 3 760) and looped forever in `advance_apply_batch`; raised to `{4096, 4096, 262144, 4096}`. That converted 5 timeouts into the real failures above.

## 2. replication `--lib`

`cargo test -p semio-framework-replication` does not work from the root (crate is outside the root workspace): use `--manifest-path 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml`.

- `project_owned` now takes a grant and returns `(projection, progress)`: identity, authority and canonical-field tests pass an alias grant (`RetainedCloneBinding::alias_copy_bytes()`).
- `RetainedCloneSource` constructors need the alias copy (`constructor_copy_bytes`) in the birth grant (4 tests).
- Seal tests moved to `take_authority`/`RetainedCloneSourceTake` (`take_owner` and `Phase::Unwrap` are gone); the dictionary decode source test had one un-annotated `sum()`.
- PRODUCT: history fold `SharedActorAdmission`/`SharedActorLease` charged their documented native word-layout copy (`COPY_BYTES`) in the preflight but the receipt came from `SharedUtf8::admit` (copy 0); the fold now adds its own word-layout copy to the receipt it debits (fixture `native-owner-word-layout` stays true).
- Local-interaction root retirement: since `String` payload retires by release only (value copyBytes 0), the copy-byte oracle is the ordered-map KEY corpus: root 73 -> 36, large root 33 265 -> 19 950, cancel 101 -> 36 (+ the 5-byte domain key once the candidate map owns it, `cancelDomainKeyBytes`). Fixtures `🌳️root/🧫️fixtures`, `🌳️root/🩹️update/🧫️fixtures` and the TS oracle (`rootKeyBytes`) updated together; `bun run` of the source-contract script passes. DECISION NEEDED (value owner): map keys charge copy while `String` values do not; if that asymmetry is unintended the fixture constants change again.

## 3. plugin crate `--lib`

Compile: 33 first-pass errors (+ a second typecheck layer of ~45) -> 0. Test-code migrations: reactor driver comma, `io_run` removed (local `run_route`/`run_route_controlled` over `io_run_with_snapshot_control`), duplicate `terminal_is_empty` merge residue, `quoted` helper, `DirectIngress.addresses`, SharedUtf8 vs String, `with_registry`/`ingest_operations_text`/`pump_until` identity+policy, `InteractiveJob` outcome borrows (member job rewritten on `OriginalToyUnit` + `emission`), `ArtifactNativeSnapshot` for the declaration-fixture snapshots, `ArtifactSqliteSnapshotCodec` import/export signatures, `ExtensionInvokeStep` handlers, `RestoredCheckpoint` owner, tool-run outcome slot close, `drive_framework_reserved_spawn_like_host` (the old "Done in 2 steps" law is obsolete: the reserved job is a multi-turn bounded job; steps now `> 1`), `MountedTypedCommandFullOperation` initializers.

Product fixes (all native-checked with `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib`):

1. `artifact_owned_tool_handoff_grant()` funded zero copy bytes, so `reserve_live` refused every gesture-capture alias ("original gesture capture alias is not funded") and the half-built `ArtifactOwnedToolJobContext` was then dropped un-retired (masking panic "context root retains its original projection or Store issuer", 75 tests). The handoff grant now funds one sealed alias handle (`ARTIFACT_OWNED_TOOL_HANDOFF_COPY_BYTES = 64`). FOLLOW-UP: a `?` between building the context and handing it to the request still drops it un-retired on any error; every such early return should retire the context (owner: pl-c).
2. `WindowRegistry::insert` reserved one page and then `push_reserved(..).unwrap_or_else(panic)`; the migrated `PagedList` may need more than one `reserve_one` step (root + leaf), so a cold insert panicked ("reserved original registry entry remains admitted", 58 tests). It now loops until the slot exists and asserts each step progresses.
3. `fixture_close_receipt` returned `?` on a `ValueError` in a `Result<_, Fault>` fn (cfg(test) compile error), mapped.

### plugin remaining (759 / 420 / 46 / 2 of 1227): root-cause buckets, each owner-routable

| Count | Message | Cause | Owner |
| --- | --- | --- | --- |
| ~80 | "authoring requires the installed original operation wire source" (config lanes, time-travel, tool-run) | `Store::authored_operation_rows`/`authored_operation_identity` need `factory.operation_wire_source()`; `config_apply_preparation_factory`, `mutation_apply_preparation_factory` and the whole `PagedOneItemPreparationFactory` family return `None`. Fixtures that use `bounded_config_store_one_item_preparation_factory` (refuses by design, corrections #39) or the raw `config_apply_preparation_factory` therefore cannot author. SAME GAP IN PRODUCTION: energy and raster install `config_apply_preparation_factory`/`mutation_apply_preparation_factory` unwrapped, so their config-lane authoring will refuse at runtime. | store / fd-window-edit: give the generic factories a canonical-JSON wire (`ArtifactPreparedOperationSource::CanonicalJson`) or require `operation_wire_preparation_factory` at install; then migrate fixtures |
| 49 | `child_complete_*` assertion `left == right` | tests assert the old logical String drain (`ProcessedBytes` = backing bytes); `String` retirement is now release-only (copy 0) | core-tests (value) + fixture refresh |
| 43 | `fatal runtime error: stack overflow` (with the 64 MiB test stack) | NOT diagnosed: either a frame that grew past 64 MiB in debug or an unbounded recursion; first sample `time_travel_tests::noted_shell_commands_keep_every_locale_and_undo_matches_its_chord` | pl-c |
| 41 | "batched inputs require their original mutation retirement authority" | (`store/🦀️.rs`: `mutation_retirement_factory` is `None` on the document store) the time-travel `ToyHistoryApp` catalog is installed without it; not verified beyond the guard | store / pl-d (testkit) |
| 29 + 10 + 9 + 8 + 8 | "original fixture close receipts", "actual native lifecycle turn", "no installed owner disposer", "increment: Fault …" | partly verified: `VcsArtifactApp::close_step` returns `AwaitingInput` while a constructor receipt is unclaimed (`take_constructor_receipt` is only called by `new_registered_app`), so fixtures that construct through `with_registry` cannot close (9 tests show that exact message); the rest are not examined | pl-c / pl-d |
| 21+ | "the parent declares the member it is about to own" | same authoring wire gap as the first row, document lane of the member fixtures | store |
| rest (~130) | single-digit buckets | mixed: pack source declared-value authority, depth limit on retirement frontiers, `subtract with overflow` in `child_member_registry_tests::original_composition_pin_retirement…` (4 tests, not examined), presence roster, download seam | per test, see `plugin-each/results.tsv` |

Raw per-test results: `.🧬semio/🦑️repo/⚡️cache/play-fleet/fw-tests/plugin-each/results.tsv` (`status<TAB>test<TAB>md5-of-name`, log = `<md5>.log`), os-kernel in `kernel-each/`.

## Running these targets (for the next owner)

- Test binaries run outside cargo need `RUST_MIN_STACK=67108864` (the repo `.cargo/config.toml` `[env]` sets it only under cargo); without it ~380 plugin tests abort on stack overflow.
- The plugin and os-kernel libs abort the whole process on some failures; run one process per test (`--exact <name> --test-threads=1`, xargs -P6): the full plugin lib takes ~35 s, os-kernel ~12 min (dominated by the slow/hung tests).
- Debug [DEBUG] prints added during diagnosis were removed again; nothing temporary remains in the tree.

## Files touched

Product: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/{🦀️.rs,🧵️borrowed/🦀️.rs,📖️reader/🦀️.rs}`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/{🦀️.rs,🪟️window/🎚️config/🗂️registry/🦀️.rs}`, `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs`.

Tests/fixtures: canonical-edit `{🧳️source,🧵️borrowed,📖️reader}/🧪️tests`, store `🧪️tests/🔬️unit/🦀️.rs`; replication `{🎮️mutation/🧵️canonical/{🪪️identity,🛂️authority}/🧪️tests, 🚪️io/📝️text/🧵️canonical/{🧪️tests,🔏️seal/🧪️tests}, 📖️dictionary/🛬️decode/🧾️source/🧪️tests, 📡️wire/🏠️local-interaction/{🌳️root/🧫️fixtures, 🌳️root/🩹️update/{🧫️fixtures,🧪️tests}, 🧪️tests/🧪️source-contract/🟦️.ts}}`; plugin (≈30 test files under `🧪️tests`, `🪟️window`, `⚛️reactor`, `🧩️composition`, `🕹️interaction`, `🛂️describe`, `⏯️tool-run`, `🧵️retained-command`).

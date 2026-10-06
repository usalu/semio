# 📓️ S5 kernel law run — 2026-10-05 16:19–16:35 (S5-STORE, after the 11:34 reboot, build B2 live, channel 23)

One kernel test binary, every owner's filters on it. Private `CARGO_TARGET_DIR` + `CARGO_BUILD_BUILD_DIR`
(`⚡️cache/cargo/target-nde-s5-store`), `CARGO_BUILD_JOBS=4`, `CARGO_INCREMENTAL=0`, `RUST_MIN_STACK=268435456`, one cargo at
a time; the filters ran on the built executable from the package directory with
`SEMIO_TEST_ARTIFACT_DIR=T/🗑️generated/s5-store/test-artifacts`. Raw outputs: `T/🗑️generated/s5-store/kernel-run-*.txt`.

## Builds

| when | command | result |
|---|---|---|
| 16:19–16:21 | `cargo test -p semio-framework-os-kernel --lib --no-run` | exit 0, 1 m 49 s, 897 warnings, 1193 tests |
| 16:25–16:26 | same, after wave LO (below) | exit 0, 1 m 18 s, 1194 tests |
| 16:30–16:33 | `… --lib --features sync --no-run` | exit 0, 3 m 29 s, 1282 tests |
| 16:34 | `cargo test -p semio-framework-replication --lib -- causal` | exit 0, 58 / 0 |

## Table (final state, binary of 16:26 unless noted)

| owner | filter | passed / failed | first failure |
|---|---|---|---|
| STORE | `viewer_head_tests supersede_law_tests` | **13 / 0** (9 + 4) | — (first run 16:21: 11 / 1, see "Red of mine") |
| STORE | §22.28 `a_content_revision_names_a_head_whatever_line_its_replica_stood_on` | ok | — (was RED before wave C: live ≠ own reload) |
| STORE | CL `a_commit_on_an_alternative_waits_for_the_alternative` | ok | — |
| STORE | merge laws (`a_read_back_pair_merges_its_log_and_never_moves_the_reader`, `two_peers_on_one_folder_converge_through_an_open_history_edit`; no test is named `merge_persisted`) | ok, ok | — |
| STORE | renumber dirt (the mirror oracle inside `the_viewer_head_corpus_matches_two_stores`) | ok | — |
| STORE | canonical-edit: `os_store::component::canonical_edit edit_digest_chains borrowed_map canonical_reader canonical_sealer canonical_authority` | **25 / 0** | — CHANNEL's reseal agrees with the sealer and the store digest |
| STORE | DAG pending rule: replication crate `causal` (`a_dependency_that_is_buffered_but_pending_keeps_its_dependent_pending`) | **58 / 0** | — |
| STORE | `--features sync -- the_folder_archive_presence_corpus` | **1 / 0** | — |
| STORE | `--features sync -- os_store::sync::` (whole sync module, incl. backbone parity) | **88 / 0** | — |
| CHANNEL | `os_spr::` (channel suite, member rows, handshake corpus) | **343 / 0** | — (without `SEMIO_TEST_ARTIFACT_DIR`: 342 / 1, `os_spr::io::native::tests::retained_history_and_tail_follow_caller_cancellation_after_original_context_drops` panics on the missing variable — a harness precondition, not a defect) |
| CHANNEL | `durable_group` (20 tests, inside `os_store::`) | 20 / 0 | — |
| LOAD | `the_document_archive_load_host` | **1 / 0** | — |
| regression | `os_store::` | **514 / 1** | `…snapshot_capability_tests::native_encoding_tests::sqlite_snapshot_native_physical_pack_record_preserves_literal_table_and_numeric_tags` |
| regression | whole binary, parallel | **1188 / 6** | the six below |

## Every red

| # | test | message | whose | evidence |
|---|---|---|---|---|
| 1 | `os_store::…::sqlite_snapshot_native_physical_pack_record_preserves_literal_table_and_numeric_tags` | byte 283 on: the product emits the object's members ordered by key bytes (`\0\\"\n\r\t` first), the law expects stored order (`nan64_text` first) | Codex peer (intrinsic object order, in flight) | test file committed (10-04 18:27, no diff); `💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` uncommitted (4 + / 38 −, 05:52) removes the sort in `encode_dsl_value` only — this path still sorts |
| 2 | `os_pack::value::intrinsic_media_tests::intrinsic_media_wire_direct_helpers_settle_complete_allocator_requests` | actual lists `"\0ä"` first, expected `"z"` first | Codex peer, same work | same: law (committed) expects actual order, product path still orders by key |
| 3 | `os_pack::…::intrinsic_media_wire_direct_helpers_share_semantic_depth_and_typed_control` | same shape | Codex peer | same |
| 4 | `os_pack::…::intrinsic_media_wire_preserves_actual_order_duplicate_keys_and_full_literal_words` | "ordinary and controlled grammar": bytes differ at the first map entry | Codex peer | same |
| 5 | `os_dsl::component::tests::controlled_refusal::os_controlled_field_derive_retains_all_refusal_categories_and_position` | `"value.refused field"` ≠ `"refused field"` (a path prefix in the refusal message) | not a fleet wave — Codex peer or already red at HEAD | test `🗣️dsl/🧪️tests/🪆️refusal/🦀️.rs` committed, no diff; the only fleet change under `🗣️dsl/✨️derive` (S5-GATES 10:09, withdraw-only leaf) does not touch record refusals; `🗣️dsl/🦀️.rs` carries + 65 uncommitted lines (operation page codec) |
| 6 | `os_dsl::component::canonical_record_producer_consumers::sqlite_snapshot_native_schema_lazy_recursive_physical_text_never_materializes_ordinary_metadata` | `RECURSIVE_ORDINARY_CALLS` 5 or 6 ≠ 0 | test isolation (sqlite-snapshot ticket, Codex peer): passes ALONE, fails only in a parallel run — a process-wide counter other tests increment | solo run 1 / 0 |

Deterministic reds: 5 (1–5), none in a file a fleet wave changed; 1 isolation flake (6). No red of the fleet is left.

## Red of mine, fixed forward in this run — wave LO (16:25)

`the_viewer_head_corpus_matches_two_stores` (first run): case `finalizes-on-both-replicas-keep-their-own-heads`, arrival 5:
listed alternatives `["", "blue", "red"]` where the corpus (and the replica's own reload, and every peer) says
`["", "red", "blue"]`. Cause: `adopt_history_facts` materialized the history fold into the change / checkpoint /
alternative ledgers by keeping known facts in place and appending new ones — ARRIVAL order — while the fold, and therefore
a reload, lists them in log order (`(hybrid clock, id)`). Unmasked by wave CL, which let the corpus law reach its fourth
case. Fix `T/🧪️s5-store-ledgers-follow-log-order.py` (apply-only hold, train line 16:25): `order_ledger_as` exchanges
entries in place after adoption (slots, keys and reservations untouched; one comparison pass when the order already
holds, which is every local authoring) + the focused law
`the_ledgers_list_their_facts_in_log_order_whatever_order_the_events_arrived_in` (two authors, reversed + every rotation,
live ledgers == what the replica's own pair restores). Product change in `🏪️store/🦀️.rs` (2 hunks), no format change.
The train lanes are not running: LO has NO `--lib` closure verdict from the train — only my kernel test builds (which
compile the kernel lib); the change adds one private function and three calls, no signature changes.

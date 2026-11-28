# Curation Rust workspace, 2026-11-27

Driver: `repo cargo --store=shared --offload=never --configuration=prototype -- <args>` from `🗿️artifacts/🗂️curation`, `-j 4`. Logs: `🗑️generated/curation-rust-check1..5.log`, `curation-rust-test1..10.log`.

## Result (resumed after disk was freed)
- `check --workspace --all-targets --keep-going`: exit 0 (after the three compile fixes below).
- `cargo test --workspace --no-fail-fast -j 4 -- --test-threads=1` (native fallback: `repo cargo` queued >10 min / JSON parse error; own `CARGO_TARGET_DIR`, `bun` on PATH, `SEMIO_CURATION_IO_OUT` set): first full run 216 passed, 10 failed, 0 ignored in 44 test binaries. After the fixes below and targeted re-runs: **223 passed, 3 failed**.
- python oracle `🚪️io-curation-1/🐍️.py check ../🧫️fixtures` against the Rust-written `archive.zip`, `snapshot.json`, `snapshot.txt`: "oracle agrees" (3 files re-read, sqlite 3.54.0, 19 cases). `repo test-file` itself fails on the host (runs remotely: "locked Python environment is not synchronized: run `repo dependencies ensure`"), so I ran `python3 🐍️.py check` locally.
- Bridge `🏭️bridge`: `cargo test --all-targets` builds and passes (0 tests exist).

## Test fixes (this round)
| Failure | Cause | Fix |
| --- | --- | --- |
| `curated_state_round_trips_through_zip_2_0`, `io_fixture_zip_archive_members_match_and_restore_the_kit` | tests assumed the `s.stdio.zip` payload is a raw zip; by convention it is the zip artifact's pack (`\x89SEM`) | tests in `🧪️tests/🔬️unit/🦀️.rs` decode the pack and `encode_zip` it for the PK check and for the oracle's `archive.zip` |
| `every_shipped_module_typology_segment_and_kind_has_an_english_and_a_german_label` | German name of `beam-kvh-c24` equalled English | `🧬️schema/🧾️domain/🦀️.rs` catalogue: "Konstruktionsvollholz C24 100×200" |
| 4 x `sqlite_snapshot_curation_*` (NotFound) | oracle spawns `bun`, not on PATH natively | environment only; pass with bun on PATH |

## Remaining failures (3, not fixed)
- `sqlite_snapshot_curation_cancel_reaches_known_stock_and_ordinal_frontiers`: `DecodeNative` error at the cancel frontier (known sqlite encode allocation effect item).
- `history_edits_end_to_end`: `value.refusal: operation output payload extent exhausted` (typed-operation settlement, framework).
- `history_conflicts::conflict_history_edits_end_to_end`: fails in the same history-acceptance driver.

## Fixes (3 files, all in sourcing)
| File | Fix |
| --- | --- |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs` | `crate::schema::curation_set` to `crate::schema::domain::curation_set` (function moved by the carve) |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🧪️tests/🔬️unit/🦀️.rs` | same path fix, 3 call sites |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️harness/🎬️dispatch/🦀️.rs` | `use semio_plugin::PluginApp;` (typed-operation outbox, local-query and `export_media` are trait methods; 6 E0599) |

One transient: `test3` showed 4 E0308 in `artifact-store-preparation` / `config-store-preparation` (`ColdHeapQuote::reference(...)`) while a peer edited those files (mtime during my build); `check5` before and the rebuild after compiled them, so not a standing error. Worth a re-check once the test build runs.

## Notes
- Earlier attempts (ENOSPC, warden kills, peer edits restarting cargo preparation) are history; the host is now at 48 GB free.
- `repo test-file` for python needs `repo dependencies ensure` on the executing machine.

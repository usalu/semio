# Schema crates sweep (2026-11-24)

Status: interim; the sweep is incomplete because shared build infrastructure timed out or killed most runs (see table).

## en1997

- Bare `-p semio-s-artifact-norm-en1997-schema --offline`: compiles, `test result: ok. 0 passed; 0 failed` (twice: lib and doc). The schema crate has no tests of its own; the compliance tests are `include!`d by the family crate `semio-s-artifact-norm-en1997` (`-p semio-s-artifact-norm-en1997`), which is where the reported errors lived.
- `semio_s_plugin_norm_mechanics` did not move: the family crate simply never listed it. Fix: dev-dependency `semio-s-plugin-norm-mechanics = { workspace = true }` in `en1997/📦️packages/🦀️rust/Cargo.toml`.
- `OriginalSnapshotRefusal` is now `pub struct OriginalSnapshotRefusal(pub String)` (a stage label, no `kind`, no `Display`). The sqlite snapshot test now reads `error.0.contains("Canceled")` / `.0.contains(&format!("{kind:?}"))` and prints `{error:?}` (same as en1999).
- Further family test compile errors found and fixed: `z: f64` annotation and borrow-split of `doc.anchors/lab_tests` rows and a named `placed` fn (entity-check-laws), `cpt_nkt: 0.0` in `SoilLayer` (field-meta-and-perturbation-laws), `use protocol::{ColdDiffAlgebra as _, ColdMutation as _}` (diff unit test), `&p` for `export_sqlite`/`import_sqlite` (sqlite snapshot test).
- Family run (`-p semio-s-artifact-norm-en1997`): compiles; `test result: FAILED. 97 passed; 26 failed`. `checks_compliance::example_snapshot_validates_against_json_schema ... ok` (annex no longer lowercased). The 26 failures are en1997 content/snapshot work in flight (for example `norm native list required`, `UnverifiedBasis` instead of `Pass`, native extent 3682 vs 3192 bytes, `plugin-assembly.artifact-owner`), not touched here; names in `🗑️generated/en1997-failures.txt` while the ticket is open.

## Sweep


Command per family: `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-<family>-schema --offline` from the family dir, hard timeout 40 min, one cargo at a time (`schema-crates-sweep.sh`, `schema-crates-sweep-all.sh`). Logs and exit lines: `🗑️generated/sweep-<family>.log`, `🗑️generated/sweep-status.log`.

| family | result |
| --- | --- |
| en1990 | `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (lib and doc) |
| en1997 | `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (lib and doc) |
| vdi3805 | `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (lib and doc) |
| din4108 | no result: 40 min timeout (exit 124) while 61 crates compiled behind host-capacity queueing |
| din16798, din18599, iso16757 | no result: exit 101, `semio-framework-tool-typing-edit` could not compile (`settle_deferred` private). Another agent's in-flight framework edit in `tool-machine/typing`; it is `pub` now, so these need a rerun |
| en1991 | no result: exit 101, rustc killed with signal 137 (memory pressure) in `en1991-mut-wind-faces` and `-wind-bridges` |
| en1992, en1994, en1999 | no result: 40 min timeout (exit 124). en1999 passed bare earlier (per `test-serde-schema-required-2026-11-24.md`) |
| en1996, en1998 | no result: exit 1, "the host warden cancelled processes to keep this machine from running out of memory" |
| en1995, en1993 | not reached yet by the first pass |

No test failure was seen in any schema crate that ran. The failures are all build-infrastructure (timeouts, memory warden) or the now-fixed framework edit, none are in the schema crates or the dev-dependency edits.

A second pass (`schema-crates-retry.sh`) is still running detached: it waits for the first pass to end, then reruns every family without `exit 0` up to three rounds, appending to `🗑️generated/sweep-status.log` and writing `RETRYDONE` at the end. Read the final `test result` lines from `🗑️generated/sweep-<family>.log` (`grep "^test result"`). The `🗑️generated` folder is therefore not deleted yet.

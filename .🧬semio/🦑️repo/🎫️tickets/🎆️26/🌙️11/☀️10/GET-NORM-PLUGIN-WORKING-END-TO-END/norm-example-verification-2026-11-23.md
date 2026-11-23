# Norm Example Test Verification 2026-11-23

Runs via `test-family-examples.sh <family>` (detached, one family at a time) and one direct `repo cargo ... -p semio-s-artifact-norm-en1997-editor --offline` run. No source files were edited. Raw logs: `🗑️generated/examples-<family>.log`, verdicts in `🗑️generated/rerun-<family>*.out`, en1997 editor in `🗑️generated/rerun-en1997-editor.log`.

## Results

| Family | Crate(s) | Exit | Passed | Failed | Outcome |
|---|---|---|---|---|---|
| 🧱️din4108 | semio-s-artifact-norm-din4108-examples | 0 | 4 | 0 | Pass |
| ⚡️din18599 | semio-s-artifact-norm-din18599-examples | 0 | 15 | 0 | Pass |
| 📇️iso16757 | semio-s-artifact-norm-iso16757-examples | 0 | 4 | 0 | Pass |
| 🏋️en1991 (run 1) | semio-s-artifact-norm-en1991-examples | 101 | - | - | Compile error E0599 `heap_bytes` not found for `VcsArtifactApp<A, M>` in `semio-plugin-instance-construction` (lib). Framework code. No tests ran. |
| 🏋️en1991 (run 2) | semio-s-artifact-norm-en1991-examples | 101 | - | - | Compile error E0063 `statements` missing in `variant::NativeVariantSpec` in `semio-framework-value-native-owned` (lib), 2 errors. Framework code. No tests ran. |
| 🏋️en1991 (run 3) | semio-s-artifact-norm-en1991-examples | 101 | 2 | 1 | Compiled and ran. Failure: `example_subjects_unit::committed_example_assets_decode_to_their_named_subjects`, `assertion left == right failed` comparing two `En1991Snapshot` values (both print the same leading fields; the difference is further down the struct). |
| 🏭️vdi3805 | semio-s-artifact-norm-vdi3805-examples-failing, -failing-blatt-4-to-19 and the rest of the family | 101 | 28 | 3 | Failures: `blatt_16_fail`, `blatt_19_fail`, `blatt_5_fail`, each panicking with "expected ≥2 fails" (failing-example assertion in `🧪️tests/🎩️example/🦀️.rs:9:5`). |
| 🌍️en1997 editor | semio-s-artifact-norm-en1997-editor (`integration` test target) | 101 | - | - | Compile failure, 5 errors: E0433 `store` and `protocol` not found (`🧪️tests/🧩️integration/🔬️unit/🦀️.rs:130-131`), E0599 no `examples` on `En1997PlayApp`, and "queries overflow the depth limit". No tests ran. |

## Notes

- en1991 run 1 was a fresh compile error, not the SIGTERM cancellation seen in the earlier run. Run 2 and run 3 followed the coordinator's fixes (04:28, 05:37). Run 3 is the first that compiled.
- vdi3805: the earlier "33 of 32 files" source-boundary refusal is superseded by the crate split (`T/vdi3805-source-boundary-2026-11-23.md`). This run has no source-boundary error.
- Framework-code compile errors (en1991 runs 1 and 2) were recorded as-is; no fixes were applied here.

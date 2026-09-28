# WP-U6 — stdio Lib-Test Drift (bcf/svg/semio/txt/wav) + Dead docx/xlsx Diff Code (Window-3 T2 Set)

Session 14c slice U6 (spawned 2026-09-28 23:3x, Opus 5.5). Coordinator = main. Scripts/inputs `wp-u6/`, captures `wp-u6/generated/`
(expendable) — since the 01:14 sweep (rule 26) captures, private target and scratch live under `.🧬semio/🌐hub/s14-u6-{logs,target,scratch}`. Native cargo only via `📜️fleet-mutex.sh native u6`
(build-fleet-b, nice 15, `CARGO_INCREMENTAL=0`); the docx/xlsx scratch mirror builds with a PRIVATE build-dir inside the scratch.

## Session 14

| # | Item | Status |
|---|---|---|
| A | stdio lib-test compile drift (rule 22, test-only): bcf 1, svg 2, semio 3, txt 1, wav 10 (L1 baseline `wp-l1/generated/t1-base-native-1.txt`) | **written 23:29** (`wp-u6/u6-test-drift.py --write`, 7 test files / 19 hunks, dry run 0 problems, all 7 rustfmt-clean, backups `w3-backup/test-drift/`); proof queued (native lane, `generated/a1-check-test.txt`) |
| B | Window 3 set: `u6-dead-docx-xlsx` — dead fns/types in docx/xlsx after the OPC `xml_parts` migration, ON TOP of LB2 p5 | scratch mirror + p5 applied (scratch only; live p5 dry run still `files=12 already-applied=0`); compiler dead-list run queued (native lane, private build-dir, `generated/b1.log`) |

#### Log

- 23:19 start. Load 45–57, 1 rustc, 125–128 GiB free. Native lane held by h13 (hold 15), queue l1 (priority) / s18 ×3 / h14 ×2 / s20 / g12.
- 23:2x Task A root causes read against the current production API (no production change needed for any of the 17 errors):
  bcf `render(document, locale)`; svg crate has no `serde_json` → own `pack::parse_json` (framework's serde_json replacement, same
  Index/PartialEq/Display surface, no new dependency); semio `Fault` has no `Display` by design → `Fault::describe()`; txt
  `Mutation::apply` gone → `MutationDiff::apply(Mutation::diff(..).diff(), ..)` (mp4/wav idiom); wav `set_snapshot` leaf imported by
  the test, `UiMap` iterates owned pairs → owned argument reads, `LocalizedLabel::resolve(Terminology::Native, locale)`, window
  coordinates `u32`. The window test's three unformatted helpers are rewritten rustfmt-clean in the same hunks.
- 23:2x found in passing (T1, not mine): txt editor `🦀️.rs` — the removed `build_document_store_initialization_job` override left its
  docstring + `#[allow(clippy::result_large_err…)]` attached to `command_id` (misattached docs, compiles). Owner: the retire-load-request
  / initializer set (S20).
- 23:3x Task B: scratch mirror `scratchpad/u6-sc` (root manifests + `.cargo/config.toml` with private build/target dirs, every other
  entry symlinked, docx + xlsx artifact dirs copied); LB2 p5 applied there by a path-redirected copy of its script (backups inside the
  scratch — LB2's own `generated/p5-backup` untouched); `cargo metadata --locked` resolves docx/xlsx to the copies.
- 23:3x `u6-scratch-check.sh` (lib WITH the feature, lib-test unit via `--profile test` WITHOUT it — cargo's JSON cannot tell lib from
  lib-test messages in one `--lib --tests` run) queued with the stamp of its first queueing (re-queued once after that fix).
- 23:4x `u6-dead-docx-xlsx.py` written (analyze → payload rounds; `--dry-run/--write/--revert`; span-keyed exact blocks, exactly-once +
  `//#region` guard; dead types take their `impl … for T` blocks; tidy pass removes emptied `impl {}` / regions). Pipeline rehearsal on
  a text-only copy with the baseline's 311 dead items as fake diagnostics: 317 edits, −2 771 lines over 6 files (docx `🔺️diff` 3 189 →
  1 584 lines), rustfmt state unchanged per file (before = after), 4 kept (named in feature-gated editor tests: `xlsx_flat_cells` ×3,
  `xlsx_cell_revision`). The real list comes from the compiler run.
- 23:3x–00:2x both lane tickets waited (queue 8–10 deep); the machine panicked (00:31/00:41/00:48, rule 25) → both waiters, the scratchpad
  mirror and every capture died; the 01:14 sweep removed `wp-u6/generated` + `w3-backup` (scripts survived).
- 01:34 resume: set A still in the tree (auto-committed) — `u6-test-drift.py --dry-run` reports all 19 hunks "already applied"; L1's
  T1 re-check (`T1-native-2`, swept) saw the five crates' tests green. Own proof re-queued: `.🧬semio/🌐hub/s14-u6-logs/a2-check-test.txt`
  (check `--lib --tests` + lib tests, build-fleet-b, target `s14-u6-target`). Scratch mirror rebuilt at `.🧬semio/🌐hub/s14-u6-scratch`
  (no `.🧬semio` link inside — no walker cycle), LB2 p5 re-applied there (live p5 dry run still `files=12 already-applied=0`);
  fixpoint run queued (`s14-u6-logs/b1.log`).


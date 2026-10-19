# Validation Evidence and Output Hygiene Audit

Read-only evidence audit on 2026-10-19 during concurrent final course and React runs. Only this report was written. No tests, native build, deployment, installation, Git mutation, cleanup, ticket transition or goal transition was executed by this audit. Findings are a snapshot; the coordinator owns subsequent process exit evidence and cleanup.

## Current Conclusion

Deployment readiness remains draft pending the full gates. No current native frontend release, Rust release, image construction, Compose runtime, browser rehearsal or offline bundle proof was observed. The goal is not complete. Installed Node/Vitest and native source-helper diagnostics establish their bounded assertions; they do not substitute for the canonical managed release pipeline.

The actual canonical preflight log records exit 1 and explicit refusal before store allocation on Windows x64. Required Debian 12/Linux, CAP_SYS_ADMIN, storage, loop devices and native tools are absent or insufficient. This is actual admission refusal evidence, not an allocation or deployment pass.

## Reviewed Results and Counting

- workspace-fence-final.log ends with 40 passed, zero failed, 947 assertions, 7.92 seconds. These are source-only helper regressions, not a production compiler run.
- course-fence-final.log and the source reports identify the selected launch/resolution/deployment diagnostic as 44 tests across three files. Its deployment subset is 23 tests; that subset must not be added to the 44 again.
- workspace-fence-regression-green.log actually ends with 35 passed, seven failed and 937 assertions. Its filename is misleading. The final source report correctly cites workspace-fence-final.log instead; retain the failed run as history without counting it as a pass.
- course-unit-current.log ends with 594 passed and one failed test across 595 executed tests; two files failed, including launch collection with ENOENT on .vscode/launch.json and the old missing providedContext map. It is a failed historical run, not the final current result.
- At inspection, course-unit-final.log contained ownership progress only and ui-full-react-final.log contained ownership progress plus the Vitest RUN header only. Neither contained a final test summary or successful exit proof. The coordinator must record their eventual summaries and process exit before claiming full-suite success.
- Repeated focused UI counts (61 presence tests, 89 hints, and the 261-test nine-suite run with two prior failures) overlap. They must not be summed. ui-repair.md explicitly avoids that and leaves the configured 38-file gate pending.
- current-source-typecheck.log explicitly records eight diagnostics in the generated UI-axes prerequisite closure. This is not a passing typecheck even though repaired course React modules have no diagnostics in that program.
- dependency-lock.md correctly distinguishes actual installed Node tar-stream oracle execution from the unmaterialized direct course edge and canonical dependency synchronization. Archive decoding is not image production.

## Report Interpretation

final-audit.md and runner-prerequisite.md preserve the remaining native/runtime boundary. ui-repair.md preserves the pending full React gate and eight type diagnostics. No unsupported deployment-complete claim was established in these current reports.

readonly-audit.md and deployment-final-audit.md are earlier source snapshots; final-audit.md records repairs superseding their original blockers and rollback/platform findings. Read them chronologically and do not report their stale findings as current defects without revalidation. The UI report's statement that owned repair verification is complete is explicitly bounded by its following pending full-gate statement.

## Output Hygiene

The generated directory had 110 files when inspected. No generated file path exceeded 256 UTF-16 characters. The longest observed was 219 characters, the nested vitest-course-react-cache/vitest results.json path.

The inspected course node_modules/.vite-temp directory contained only the zero-byte file named 🟦️.ts.timestamp-1791987873230-497f03f1bf4ed8.mjs. Its creation and modification timestamps are both 2026-10-14T16:24:33.2382783+02:00. This is historical Oct14 output, not attributable to this task; leave it untouched. The inspected framework node_modules/.vite-temp directory contained no files. No task-specific outside-ticket output was established at these two locations. This is a bounded location check, not a whole-repository provenance assertion.

Native config loading is recorded in the source reports as the current means of avoiding future bundled config artifacts. Existing ticket generated logs/caches remain for the coordinator's final evidence review and removal; this audit deleted nothing.

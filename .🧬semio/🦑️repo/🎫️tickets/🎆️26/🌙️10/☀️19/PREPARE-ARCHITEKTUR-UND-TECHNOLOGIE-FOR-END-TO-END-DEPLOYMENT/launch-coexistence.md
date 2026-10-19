# Required Editor Launch and Dashboard Coexistence

The latest human-provided instructions explicitly require developer commands in `launch.json`. Concurrent repository edits introduced a dashboard-only documentation schema and a test forbidding any launch catalog. Those assertions conflicted with the current task authority. No AGENTS file was edited. Root restores the actual sixteen course launch entries and their fixture/tests; the companion owner repairs the dashboard parity case. Existing dashboard target metadata, readiness, original-mode defaults and compounds remain intact.

This scoped documentation repair changed only:

- `🎓️teaching/🏛️architecture/📚️course/README.md`: restored required Run and Debug instructions after a concurrent dashboard-only rewrite, and added a concise paragraph explaining that the dashboard metadata describes the same canonical owner, targets, readiness and original modes as the editor launch configurations.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎛️dashboard/🔣️.json`: replaced only `definitions.Documentation.allOf` negative editor clauses with positive checks for Run and Debug, the escaped `.vscode/launch.json` path, dashboard and `@teaching/architecture-course`.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎛️dashboard/🟦️.ts`: renamed only the fourth documentation case and changed its independent regular-expression loop to check the positive patterns. The first three cases were preserved for the companion owner's separate parity repair.

The edits used exact before-image patch contexts. A SHA256 check of the entire parsed dashboard `const` metadata before and after the fixture edit remained `3f879a0aaafc8adea00a9574b306a2b969c6350430638faa6e6a4adaf4e0db22`.

## Actual Red and Green

The initial probe labelled `red` exited 0 because a concurrent writer had already restored dashboard-only README text; this is not claimed as a red result. After restoring the human-required editor instructions, the actual old documentation schema probe exited 1 and reported all four negative editor clauses rejecting that README. After the positive neutral contract edit, the probe exited 0. Installed Ajv and an independent JavaScript regular-expression check agreed in both actual red and green runs. Evidence is `🗑️generated/coexistence-red.json` and `🗑️generated/coexistence-green.json`; the retained diagnostic input is `coexistence/📜️script.ts`.

The fourth test's old negative clauses and the third test's no-IDE assertion were observed reappearing after earlier repairs. This report records actual before-images and checks rather than assuming concurrent source stability.

The companion owner also observed the actual third-case RED with the restored launch file (one failed, three skipped), then a parity-only GREEN (one passed, three skipped). Evidence is `🗑️generated/dashboard-launch-red.log` and `dashboard-launch-green.log`. The guarded case replacement reads the actual launch command targets and compares them to all current dashboard targets, then checks each explicit default compiler mode against the original launch environment. It preserves the first two dashboard tests, all dashboard metadata and the fourth documentation case. After another concurrent reversal, the coordinator reapplied only this parity case with an exact current before-image guard and removed the then-unused existsSync import.

## Final Complete Verification

The coordinator owned and observed both final processes through completion. The complete course diagnostic exited zero with **13 files and 603 tests passed**, including the restored launch suite, all four dashboard tests and the edited documentation contract (`🗑️generated/course-unit-coexistence.log`). The full current-source TypeScript program then exited zero with **zero diagnostics and 3,176 reads** (`🗑️generated/current-source-typecheck-coexistence.log`). No files were emitted. The Low-agent fresh source check confirmed sixteen exact launch entries, both suites included, canonical target/mode parity and matching positive README patterns.

Two High-agent continuation turns failed with model capacity before the final executions; the coordinator completed the remaining edit and tests. No different model, native compiler grant, host-policy change, Git mutation or AGENTS edit was used. Native release proof remains blocked by the separately documented admitted-runner prerequisite.

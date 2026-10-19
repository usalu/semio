# Architektur und Technologie Deployment Readiness

Current assessment on 2026-10-19: **the end-to-end objective remains incomplete**. Current source diagnostics are substantially repaired, but the canonical release cannot be admitted on the observed host and no admitted alternative runner has been established. The ticket remains open. Historical artifacts and prior binaries are not used as current release proof.

Final shared-checkout reconciliation is verified: a concurrent dashboard migration removed the launch file and its tests despite the current human and on-disk AGENTS requirement. The sixteen course entries and their neutral regression are restored alongside the new dashboard metadata and contract. The contradictory launch-absence assertion now verifies canonical target/mode parity; documentation checks positively require both control surfaces. The expanded thirteen-file course run and source TypeScript rerun both exited zero. See launch-coexistence.md.

## Executed Current Evidence

| Gate | Actual result | Evidence and limits |
| --- | --- | --- |
| Complete course diagnostics | 13 files, 603 tests passed; process exit 0 | `🗑️generated/course-unit-coexistence.log`; final complete rerun includes the current launch, dashboard, documentation and public-export contracts. Node Vitest with native config loading and controlled current teaching public export resolution. The already installed Nx tar-stream dependency supplied the independent archive decoder through diagnostic NODE_PATH. |
| Complete Course React diagnostics | 38 files, 1,032 tests passed; process exit 0 | `🗑️generated/ui-full-react-root.log`; root-owned Node Vitest process, current React config and public exports. This supersedes failed historical baselines and the interrupted agent-owned full run. |
| Controlled workspace cancellation/freshness | 40 tests, 0 failures, 947 assertions | `🗑️generated/workspace-fence-final.log`; actual cancellation during revalidation, cumulative progress, current membership refusals and unrelated-source acceptance. See workspace-cancellation.md. |
| Teaching manifest/lock parity | Root plus all 119 current owned workspace records matched | `🗑️generated/lock-source-parity.log` and `lock-roster-verified.log`; source-only publication and comparison, no installation. |
| Developer command registration | Current launch JSON and all 16 course entries validated | `🗑️generated/launch-coexistence-restored.log` and the final complete course run; sixteen course entries, five release-intent environments, exact dashboard target/mode parity and positive documentation checks. Historical 28-entry and subsequent absent states are superseded. See launch-coexistence.md and launch-integration.md. |
| Source TypeScript diagnostic | Zero diagnostics; process exit 0 | `🗑️generated/current-source-typecheck-coexistence.log`; final authored course tsconfig, current controlled teaching public exports and existing declarations, no emit, 3,176 reads. Earlier eight public export errors and four shared Windows FFI generic errors are repaired. This is a diagnostic source program, not the canonical managed typecheck gate. See manifest-export-repair.md and native-system-typing.md. |
| Canonical native admission | Refused before fixed-store allocation | `🗑️generated/current-preflight-final.log`; the actual final canonical route exited 1 on Windows x64 before allocation. See final-native-admission.md and native-runner-audit.md. |

Focused subsets overlap these complete runs; their counts are not added. Source-only Rust test graph and production schema/import repairs are complete: independent parsers accepted 26 files, checked 285 closed literals and reported zero bounded contract findings. See Rustunitrepair.md, rust-companion-repair.md and the final rust-production-audit.md review. Rust compilation has not run on the current source.

## Repaired Source Areas

The deployment router, bounded E2E sessions, retained proctor artifacts, finite image archive, runtime-only pinned Docker recipe and immutable image tagging now share the current Course contracts. Release callbacks preserve original mode, source ownership, progress, cancellation, store authority and artifact lifetime. The Course/Quiz distinction remains explicit: the course uses `semio.course/v1`; its child quizzes retain `semio.quiz/v1` and current topic-relative paths.

Current public workspace exports, controlled asynchronous ownership revalidation, teaching lock records and developer launch commands have been repaired. Course React presence, navigation, roster, locale and session fixtures now pass the complete current diagnostic suite. Reports in this ticket document neutral fixtures, third-party test oracles, rejected assumptions and the limits of each proof.

The complete source TypeScript diagnostic now passes after restoring the actual manifest public wrapper and retaining precise Bun FFI inference through a typed Windows security-library opener. The latter preserves the native declaration, authority checks, privilege policy, invocation order and cleanup. Current Rust production schema aggregation, sibling imports, façade namespace and companion field construction have been repaired; their bounded parser/contract audit is green, while native Rust compilation remains unexecuted.

## Remaining Release Prerequisite and Gates

The authored compiler contract requires a genuine Debian 12 Linux x64 invocation with effective CAP_SYS_ADMIN, writable loop-control, at least seven visible writable unused loop devices, ext4 loop mounts, at least 72 GiB free at the checkout with sufficient physical backing, required system tools, and working Docker/Compose. The current host does not meet that contract. Read-only repository runner discovery returned zero registered runners; the required `ARCHITECTURE_COURSE_RUNNER` variable endpoint returned 404. Existing WSL and devcontainer configurations do not establish an admitted alternative. See native-runner-audit.md for exact observations and visibility limits.

On an admitted host, run the existing registered canonical dependency synchronization and typecheck, native proctor validation, release frontend build, image build/check, Compose and offline bundle checks, and release browser E2E. The current workflow must select the actual registered compiler runner. The existing generated UI-axes source is present; no generation of replacement types is required for that repaired export. No raw Cargo path, substitute binary, fake compiler grant, nominal virtual free-space claim or relaxed admission policy is used to manufacture these results.

No current release image, registry publication or public deployment was produced. The deployment goal and ticket cannot be marked complete until these current-source gates succeed.

## Final Review Boundary

The latest complete diagnostic processes were owned and observed by the coordinator. Earlier focused runs and the 598/599-test totals are historical subsets or earlier source states, not additional passing test counts. Two requested High-agent turns hit model capacity during the final coexistence repair; the coordinator completed the guarded parity edit and both final executions while the Low-agent audit continued. No alternative model was substituted. See final-evidence-audit.md for the independent bounded evidence review, and runbook-final.md for the current operator instructions. No Git mutation, worktree or AGENTS edit was used to reconcile the shared checkout.

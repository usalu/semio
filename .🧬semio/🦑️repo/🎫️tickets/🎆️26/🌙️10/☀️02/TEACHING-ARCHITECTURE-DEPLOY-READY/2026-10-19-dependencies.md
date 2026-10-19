# Dependency Bootstrap Reader Boundary

## Finding and Change

The build CLI opened a dependencies Store reader in `runOwnedBuildInvocationV1` before calling `runOwnedBuildInLeasesV1`. The latter already calls `ensureBuildEnvironmentV1` before opening its task dependencies reader. When the parser receipt was absent or stale, bootstrap requested the real queued dependencies maintenance writer while the redundant outer reader was live. The existing cache guard correctly refused this nested writer.

Removed that redundant outer dependencies reader. The original leases and diagnostics reader still enclose the invocation, the diagnostics environment is restored in `finally`, and `runOwnedBuildInLeasesV1` retains the task dependencies reader after bootstrap. No cache safety checks, ownership issuance, or physical cache data changed.

## Regression and Validation

Added a closed language-neutral `DependencyBootstrap` schema and six command/install vectors to the existing build phase corpus. The regression executes the actual parsed build entry boundary through explicit observation ports. It checks diagnostics custody, absence of the premature dependencies reader, queued writer completion, subsequent task reader custody, and lease close. AJV validates the vectors and SQLite independently checks writer count and reader/writer event ordering. The test also checks the actual inner bootstrap call precedes the task reader. Debug output confirms every vector executes.

Test-first results:

- Before production change: focused regression failed because dispatch observed both diagnostics and dependencies readers.
- After production change: focused regression passed: 1 test, 94 assertions, 0 failures, six runtime debug traces.
- `bun repo --help`: passed, including repository CLI completion.
- A broader direct phase test attempt passed the new regression but could not exercise the existing issuance-dependent tests: they require the genuine parent Script process envelope supplied by managed test execution. This is an invocation limitation, not a passing claim for those tests.

Focused direct Bun was used only for repairing startup before managed ingress could run. The parent authorized this recovery validation. The course release worker was notified to retry managed course publication for full real-cache runtime validation. Managed retry outcome is pending that worker.

Files changed:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎟️phase/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎟️phase/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎟️phase/🧬️schema/🔣️.json`

Temporary test logs are under this ticket’s `🗑️generated` directory; they must be removed at final ticket cleanup after their outcomes are preserved here. Concurrent dependency lock work was observed and left untouched.

## Read-Only Queue Audit at 2026-10-19 11:06 UTC

The managed retries were waiting for a genuine different-process dependencies writer. No remaining same-process reader-to-writer deadlock was observed.

- Exclusive owner: PID `43281`, parent `43279`, command `bun .../🏗️build/📜️script.ts repo dependencies lock --all`, from unrelated shell PID `43268`. It remained alive after about three minutes, with observed CPU between 71.6% and 101.1%.
- Exact held resource: `native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d`. Its SHA-256 is `aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b`.
- Exact held database: `.cache/b/l/m/data/aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b.sqlite`. `lsof` showed only PID 43281 holding this database (FD 11). A read-only SQLite connection returned `database is locked`, consistent with the actual held exclusive transaction.
- Exclusive queue ticket: `00mvf5bgs1-0000xe9-ZLtwgsNAps_Q2utgCu7Ggw`, arrived `11:03:08.209 UTC`.
- The seven shared cohort tickets all arrived later: course core test PID 41732 at 11:03:15.980, course typecheck PID 41421 at 11:03:16.704, publication PID 41244 at 11:03:19.966, course React test PID 41609 at 11:03:20.242, proctor test PID 41422 at 11:03:21.926, course E2E PID 41423 at 11:03:22.199, and architecture test PID 41643 at 11:03:22.321. Their queue files agree with those PIDs and shared modes.
- A one-second OS sample of the external writer showed active main-thread execution and filesystem `openat` calls rather than a sleeping lock acquisition. Earlier `lsof` also showed the writer inside its staged dependency workspace. A sample of publication showed active SQLite acquisition work while waiting. The sample cannot prove successful completion of the writer, but establishes current external ownership and ongoing execution.

Safe next step: preserve the live queue and let the unrelated writer finish or its original owner cancel it. Existing managed readers will then acquire through their shared cohort. Do not delete the queue, detach the mounted image, forge receipts, or terminate the unrelated writer. No code change is warranted by this audit. Snapshot logs and OS samples remain temporary generated output under the ticket.

The publication worker subsequently reported retry 3 exited with `ENFILE` (`file table overflow`) at lease directory observation (`leases/🟦️.ts:152`), after dependency progress and without the original nested-reader refusal. This confirms a distinct observed failure. Its underlying host/file-handle cause is outside the queue snapshot conclusions above and was handed back to the parent for investigation.

## Read-Only ENFILE Follow-Up

No descriptor leak or unbounded source-custody fanout was confirmed. Post-failure host observations were `kern.num_files=8320`, `kern.maxfiles=245760`, and `kern.maxfilesperproc=122880`; the launchd inherited soft descriptor limit was 256 with unlimited hard limit. A six-sample monitor during the remaining active architecture E2E preparation and unrelated lock command observed global counts 6909, 6908, 6853, 6868, 6889, 6880. Numeric process descriptors remained constant: E2E PID 41423 had 19; unrelated lock PID 45510 had 13. These post-failure measurements cannot establish the peak at the original failure or attribute it to either set of commands.

Source inspection found bounded, sequential filesystem ownership reads. Bun ownership opens each manifest with `openSync` and closes it in `finally`, and opens/closes each directory cursor around its synchronous roster read. Readonly rechecks admit at most 32 pending operations. The shared native descriptor registry independently admits at most 32 owners before opening and retains unresolved close failures; dependency staging walks input files with awaited loops. The observed ENFILE originated in Bun `realpathSync` directory observation (`lstat`) in lease path resolution, rather than a repository descriptor-capacity refusal.

Safe action remains serialization of managed preparation, as the parent is already doing, while preserving other owners and OS limits. Treat the prior ENFILE as an observed transient resource failure with unproven cause; do not claim global exhaustion was measured. No speculative cache/source rewrite or limit mutation is warranted by the available evidence.

## Kernel Count Attribution Follow-Up

The parent subsequently sampled `kern.num_files` rising from 99552 to 129852 while E2E PID 41423 still had 19 numbered descriptors, then falling to 25646 during the parent’s all-process descriptor scan. That scan’s largest numbered-FD owner had 502, with no high-count process identified. My following ten half-second kernel samples were 26149, 26104, 26094, 25740, 25739, 25848, 25767, 26795, 23595, 25078. This supports a real fluctuating system counter but does not identify a process or capture the failure peak. Separate direct Bun/Bazel test processes and a recursive grep from another agent were also running; no causal attribution follows from their presence.

Apple’s primary XNU sources explain why this cannot be reduced to a sum of numbered descriptors: `kern.num_files` exposes `nfiles`; that counter tracks allocated `fileglob` objects, and Mach fileports can retain references outside an ordinary numbered-FD table. See [kernel sysctl mapping](https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/kern/kern_sysctl.c) and [fileglob allocation, free, and fileport ownership](https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/kern/kern_descrip.c). This is a mechanism that can explain a count discrepancy, not evidence that fileports caused this incident.

A system-header-guided, read-only `PROC_PIDLISTFILEPORTS` query via system libproc found no visible fileports; 244 process queries were denied with `EPERM`, and one process had exited. Thus hidden/system-owned references remain unattributed. Cache volumes remain listed as ordinary mounted APFS images, without a mount failure observed in this audit. No process, descriptor leak, or mount bug has been proven responsible; retain serialization and preserve unrelated work/limits.

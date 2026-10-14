# Read-Only Build Audit

Reviewed the active graph, selection, caller, Bazel adapter, bootstrap and lease implementation on 2026-10-14. Source files were not edited. Findings were communicated to the responsible agents while they were actively editing; recheck the final implementation before closing the ticket.

## Confirmed Configuration Loss

In graph/🟦️.ts, neutralBuildTargetV1 normalizes every configuration property into execution. A runtime Bun probe constructed a command target with base outputs [base] and release outputs [release], then resolved release. The resolved target retained outputs [base] and placed outputs [release] and cache false inside execution. Bazel reads top-level outputs/cache, so configuration-specific output restoration and cache policy are incorrect. The core agent was notified.

## Nested Bazel Invocation Deadlock

runBazelTasksV1 acquires exclusive resource bazel-workspace before publishing and holds it through the complete Bazel build and continuous launch. executeTask forwards SEMIO_BUILD_TOOL=bazel to owner commands. Any owner that calls the repository build entry point recursively waits for the same resource while its parent holds it and waits for the owner. The lease implementation opens an independent SQLite connection and starts BEGIN EXCLUSIVE; it provides no inherited or reentrant ownership. The Bazel agent was notified. This is a code-path finding, not an end-to-end reproduction.

## Continuous Dependency Snapshot Mismatch

STARLARK_RULES assumes every dependency DefaultInfo first file is a snapshot directory. repo_launch instead publishes a launcher executable. A finite task depending on a continuous task passes that executable as an action dependency to restoreBazelSnapshotV1, which reads snapshot.json underneath it and fails. Continuous dependencies also need lifecycle semantics, since building a launcher does not start the service before its consumer. The Bazel agent was notified. This is a code-path finding, not an end-to-end reproduction.

## Validation

The configuration probe ran successfully and printed the incorrect resolved object. A second planner probe confirmed an explicit missing project selector silently produces an empty dependency list; Nx parity was not established, so this is not classified as a defect.

Ran bun test against graph tests and Bazel adapter tests together. Bun 1.3.14 crashed with segmentation fault after 17.9 seconds, exit 133, before emitting test results. No tests are claimed passing. No generated output files were written by the audit.

## Follow-Up Verification

The core agent corrected configuration normalization. Reran exact file paths with a leading ./, which avoids Bun interpreting long paths as test filters and scanning the entire repository. Graph tests: 8 pass, 0 fail, 43 assertions. Bazel planner tests: 2 pass, 1 skipped integration test, 0 fail, 11 assertions. Earlier crashes came from filter-form invocation, not an observed test failure.

## Explicit Task Argument Loss

Confirmed with a runtime planner probe: request tasks a:build,b:build and args [--flag], where a depends on b through ^build. The planner first creates b as a dependency with empty args, then deduplicates its explicit selection without applying selected args. Output was [DEBUG] selected b args []. Sent to the Bazel agent for regression coverage and correction.

## Affected Working-Tree Parity

Current selection code only runs git diff --name-only base [head]. It does not enumerate untracked files and ignores --untracked/--uncommitted switches. This is a code-path observation requiring final parity verification against the supported public contract. Sent to the core agent.

## Unknown Execution Flags

Bazel task selection silently ignores options outside its small recognized set. In particular repo run project:target --graph is forwarded to Nx as graph visualization but selects and executes an actual Bazel action. Public --graph handling or explicit refusal is needed to prevent a purported inspection command executing work. Standalone repo graph already has a dedicated read path. This is a source-path finding, not an executed action.

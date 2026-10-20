# Wave 1020b brief (applies on top of brief-1019.md + brief-executor.md)

User goal (10-20 13:00): everything works end to end, fully optimized, ready for remote cache, remote execution, worker pools; nx+bun and Bazel behind the tool-neutral build interface (`bun repo --tool=nx|bazel`, neutral operations catalogue `B/⚙️operations/`, Bazel adapter `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`).

## Phase rule
- Until the coordinator sends "PAUSE LIFTED": a dependency sync and then `bun repo show projects` run; ANY write in the repo outside C/ aborts them. Stage everything as COPIES under `C/<prefix>-tools/staged/` with an anchored `apply.sh` (content-hash check per target, refuse on drift, `--dry-run`). Verify in overlay copies under `C/🗑️generated/<prefix>-*` (copies, never symlinks).
- After "PAUSE LIFTED": apply your staged set yourself (re-check drift), then verify live.
- Keep load low: at most 1 heavy process (test runner, tsc, bazel) at a time; `bun test ./<explicit file>` only; no repo-wide grep -r (use scoped `command grep -rn` on one subtree, never across `.cache`, `node_modules`, `.🧬semio`). Kill your own long-running commands; a single Bash call < 10 min.
- Do NOT run `bun repo dependencies sync|lock`, `bun repo show projects`, cargo, or product activations — coordinator serializes them.
- Greenfield, schema-first (JSON schema + data + language-agnostic fixture test + third-party cross-check), no compat layers, concise code, emoji-first docstrings, no comments inside definitions, EN+DE help texts for CLI surfaces, progress + cancellation for expensive operations.
- No external runtime deps; third-party libraries only for tests/cross-checks or behind our interface.
- Report `C/result-<prefix>.md` (changes, exact commands + outcomes, "not run" where not run, open items). Final message ≤ 8 lines. Do not close the ticket. No sub-agents.

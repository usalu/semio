# Wave 1021a brief (on top of brief-wave-1020e.md and its predecessors)

User (10-21 ~00:50): multi-agent, single worktree, shared cache, incremental compilation with blazingly fast hot caches, completely polyglot, no IDE, NO direct dependencies on tools such as bun, bazel, nx, cargo, … — tools are always used behind an interface such as the `repo` binary, which can swap external dependencies behind the internal interface.

## Rules
- Same as 1020e: stage under `C/<prefix>-tools/staged/` (anchored apply, drift refusal, `--dry-run` default), overlays under `/private/tmp/<prefix>/`, `nice -n 10`, ≤ 1 heavy process, no git modifying commands, no sub-agents, foreground only, never kill processes you did not start.
- LIVE FREEZE while chain17 (show projects → setup nx → setup bazel) runs: no live edits under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` or `🧰️framework/🔨️modules/🏃️process/` (any edit there restarts it). Other trees: live edits allowed only when your result says READY and the coordinator sends GO.
- Report `C/result-<prefix>.md` with a READY line; final message ≤ 8 lines; do not close the ticket.

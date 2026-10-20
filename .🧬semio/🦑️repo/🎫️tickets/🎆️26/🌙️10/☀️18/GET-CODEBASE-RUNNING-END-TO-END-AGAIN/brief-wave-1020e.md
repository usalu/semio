# Wave 1020e brief (on top of brief-wave-1020d.md and its predecessors)

User (10-20 ~19:45): the repo must be optimized for incremental development. Many agents must work in the SAME folder on different crates/packages at the same time. Caches must be shared and incremental.

## Observed blockers today (evidence in coordination.md 17:47 → 19:45)
- `repository-graph-facts` EXCLUSIVE held 2.5 h by one `repo exec --dry-run`; 3 other repo commands queued behind it. graph-speed (applied 19:00) serves a CURRENT seal to shared readers, but any source edit stales the seal → exclusive rebuild → everyone waits.
- `native-fixed-image:.cache/b/d` (dependency image) EXCLUSIVE held > 1 h by a course test that triggered a cold dependency install; `show projects` waits.
- Bazel: one output base per workspace → concurrent bazel invocations in one folder serialize on the server lock.
- Cargo: raw cargo refused (ingress gate: `bun repo cargo --store=shared … -- <cmd>`); shared build dir with fine-grain locking had wasm deadlocks; private per-agent target dirs filled 926 GB.
- mem-gov capacity gate (live) is FIFO and mis-measured darwin memory (fix in flight by mem-gov).

## Rules
Same as 1020d (stage-only under `C/<prefix>-tools/staged/`, anchored apply with drift refusal, `--dry-run` default; no `bun repo`/bazel/cargo over the real repo, no repo-wide find/grep, overlays under `/private/tmp/<prefix>/`, ≤ 1 heavy process, ≤ 2 GB; no git modifying commands; no sub-agents; foreground only; never kill processes you did not start; never touch `.cache/b/*/image.json` or journals). Exception: you may apply live yourself once your set is READY and the coordinator sends "GO". chain15 (show → setup nx → setup bazel) is running: do not apply graph/inference/storage-lease code until the coordinator says chain15 is green.
Report `C/result-<prefix>.md` with a READY line; final message ≤ 8 lines; do not close the ticket.
Every design is cross-platform (macOS, Linux, Windows, devcontainer), schema-first, tool-neutral behind `bun repo`, with progress + cancellation, EN/DE texts.

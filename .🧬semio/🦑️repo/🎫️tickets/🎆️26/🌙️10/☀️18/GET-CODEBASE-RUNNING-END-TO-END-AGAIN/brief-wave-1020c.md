# Wave 1020c brief (on top of brief-1019.md + brief-wave-1020b.md)

User (10-20 16:45): use as many agents as possible; goal = the whole monorepo runs end to end efficiently on Bazel with incremental builds, Bazel-native caching, remote cache/exec ready, worker pools; nx stays the second implementation behind the neutral interface.

## Phase rule (STRICT — the machine kernel-panicked at 16:03 under load)
- Until the coordinator sends you "GO": `bun repo show projects` runs on a quiet tree. Write NOTHING in the repo outside C/. Stage as copies under `C/<prefix>-tools/staged/` with an anchored apply (content-hash check, refuse on drift, `--dry-run` default, all-or-nothing).
- Until "GO": NO heavy processes. Allowed: reading code, small scripts, `bun test ./<one file>` in an overlay COPY under `/private/tmp/<prefix>/` (short paths; ≤ 1 process at a time; ≤ 2 GB RSS), tsc on a handful of files. Forbidden: any `bun repo …` command, bazel/cargo builds over the real repo, `find`/`grep -r` over the whole repo, `.cache` or `/private/tmp` (scope searches to one subtree), docker builds.
- After "GO": apply your set yourself (re-check drift), verify live, measure.
- Never kill processes you did not start. Never edit `.cache/b/*/image.json` or journals by hand.
- Report `C/result-<prefix>.md`; final message ≤ 8 lines; no sub-agents; do not close the ticket.

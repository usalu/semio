"""📝 Records the lint-repo verdict and its root cause (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
anchor = "\n## Verdicts"
add = ("FOR coordinator: LINT AGGREGATES BROKEN (root cause, no batch): `workspace:lint-repo` refuses in 32 s with `Unknown project in the closure: @semio-tech/repo-*` (GENERATION_UNKNOWN_OWNER): `sliceClosureV1` in `📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs:182` "
       "expands an object `dependsOn.projects` only for the exact string `\"*\"` or literal names, so Nx project patterns are taken literally — `@semio-tech/repo-*` (lint-repo) and `[\"*\",\"!workspace\"]` (workspace:lint, the `*` inside an array and the negation) both refuse. "
       "Fix: expand each entry as a name glob over the known owners, subtract `!pattern` entries, then admit (law: lint-repo and lint slices equal an Nx `run-many` project list / a Python fnmatch oracle). The file carries UNSTAGED live edits right now, so I did not stage it.\n")
t = t.replace(anchor, add + anchor, 1)
t = t.rstrip("\n") + "\n| rn-lintrepo | workspace:lint-repo | 01:33 | 32 s | 1,593 MB | 1 | RED in the slice: dependsOn project pattern `@semio-tech/repo-*` unexpanded → LINT AGGREGATES line |\n"
open(p, "w", encoding="utf8").write(t)

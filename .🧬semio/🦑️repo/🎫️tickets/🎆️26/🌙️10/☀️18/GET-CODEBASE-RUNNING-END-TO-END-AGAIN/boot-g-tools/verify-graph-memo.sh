#!/bin/sh
# Laws of the graph memo (graph-free single-file tests, run AFTER apply): policy + key, the model written in Python, the soundness law against the engine (22 adversarial mutations, model journal), the lifecycle with a fake journal, the cached exclusion rule == the journal's, and (with the compiled watcher) the same mutations against a REAL file watcher. Needs journal-fix of verify2-g in the live tree for the real service to stay alive (SEMIO_REPO_CHANGES=off disables the memo and the journal).
# Real-run proof owed after the gate: `SEMIO_REPO_PROFILE=1 bun repo run <product>:build` three times in a quiet tree: run 1 = cold child (profile mark `graph.memo` = `miss:none`, `graph.memo-seal` = `light`), run 2 = child again, `graph.memo-seal` = `sealed` (the child wrote its relevance), run 3 = `graph.memo` = `hit`, no `build:graph-plan` child, line `graph: N projects served from the memo ...`.
R=$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)
cd "$R" || exit 1
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
status=0
binary=$(ls "$R"/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
[ -n "$binary" ] && export SEMIO_REPO_WATCH_BINARY="$binary" || echo "no compiled watcher: the REAL journal law is skipped"
for file in "$B/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts" "$B/🧭️invocation/🧪️tests/🧊️isolated-plan/🟦️.ts" "$B/🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts" "$B/🧭️invocation/🧪️tests/🧊️taxonomy/🟦️.ts"; do
  out=$(bun test --timeout 300000 "./$file" 2>&1); echo "$out" | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
  echo "$out" | grep -qE "^ *0 fail" || status=1
done
exit $status

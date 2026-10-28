#!/bin/sh
# verify.sh [root]: the laws of the obs-g batch `fleet`, single-file runs, graph-free (run after `python3 -I obs-g-tools/stage.py apply`).
# Known reds that are NOT this batch (same red without it): bazel storage 'tools setup bazel' text oracle (BUILD_GRAPH_PLAN_OUT_V1 guard), suites 'witness' unrouted, operations 'real CLI prints the own help' when the root contains symlinked entries.
cd "${1:-/Users/ueli/Documents/semio}" || exit 1
F="🧰️framework"; P="$F/🛍️products/🦑️repo"; B="$P/🔨️modules/📚️library/🏗️build"; M="$P/🔨️modules/🔌️mcp"
for t in \
 "$B/🚢️fleet-status/🧪️tests/🟦️.ts" \
 "$B/🚢️fleet-status/🧪️tests/🛡️guard/🟦️.ts" \
 "$B/⚙️operations/🧪️tests/🟦️.ts" \
 "$B/⚙️operations/🧪️tests/🛗️requires/🟦️.ts" \
 "$B/🚪️door/🧪️tests/🟦️.ts" \
 "$B/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts" \
 "$B/🗂️suites/🧪️tests/🟦️.ts" \
 "$M/🧪️tests/🏗️build-surface/🟦️.ts"; do
  echo "== $t"; nice -n 10 bun test --timeout 120000 "./$t" 2>&1 | tail -8
done
echo "== door smoke"; SEMIO_REPO_IMPLEMENTATION=live SEMIO_LANGUAGE=en bun "$B/🚪️door/📜️script.ts" repo fleet status --limit=5 | head -12

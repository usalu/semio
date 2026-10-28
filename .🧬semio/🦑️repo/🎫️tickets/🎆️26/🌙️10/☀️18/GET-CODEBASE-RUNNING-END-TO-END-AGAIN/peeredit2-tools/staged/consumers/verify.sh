#!/bin/sh
# 🧩 Verifies the `consumers` batch at <root> (providers + tools live): every suite that reads command closures or projected inputs may fail only with failures that fail identically on a mirror of live MINUS providers/tools (names in allowed.txt, measured under the runner envelope 16:00); prints NEW failures. usage: sh verify.sh <root> [envelope wire]   (run under the gate runner, or inside a mirror via red-g mrun.sh)
set -u
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"; cd "$ROOT" || exit 2
L="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; I="$B/🕸️graph/📥️inference"
bad=0
for s in "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🟦️.ts" "$I/🧪️tests/🎓️course-inputs/🟦️.ts" "$B/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts" "$I/🧪️tests/🧷️closure-facts/🟦️.ts" "$I/🧪️tests/🔗️workspace-closure/🟦️.ts" "$I/🧪️tests/🖨️print-documents/🟦️.ts" "$B/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts" "$B/🧪️tests/🕸️graph/🛂️admission/🟦️.ts" "$L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts" "$I/🧪️tests/💾️cache/🟦️.ts" "$I/🧪️tests/📐️declared-inputs/🟦️.ts" "$B/🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts" "$L/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts" "$L/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts" "$B/📤️arguments/🧪️tests/🟦️.ts" "$B/📥️inputs/🧪️tests/🟦️.ts" "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts" "$I/🧪️tests/🔌️providers/🟦️.ts"; do
  out=$(bun test "$s" 2>&1)
  echo "$out" | grep -E "^ +[0-9]+ (pass|fail)$" | tr -s ' ' | tr '\n' ' '; echo " $(basename "$(dirname "$s")")"
  new=$(echo "$out" | grep -E "^\(fail\)" | sed -E 's/^\(fail\) //; s/ \[[0-9.]+m?s\]$//' | grep -vxFf "$HERE/allowed.txt")
  [ -n "$new" ] && { echo "NEW FAILURE: $new"; bad=1; }
done
[ $bad = 0 ] && echo "consumers VERIFIED" || echo "consumers NOT VERIFIED"
exit $bad

#!/bin/sh
# 🧪 Runs the suites the coordinator's live verify flagged inside a mirror under the runner envelope (red-g mrun.sh chain mode) and writes each sorted (fail) set to <out>/<label>.<n>.fails. usage: sh chainsuites.sh <mirror-name> <label> <out-dir>
NAME="$1"; LABEL="$2"; OUT="$3"; mkdir -p "$OUT"
C="$(cd "$(dirname "$0")/.." && pwd)"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; I="$B/🕸️graph/📥️inference"
n=0
for s in "$I/🧪️tests/🧷️closure-facts/🟦️.ts" "$I/🧪️tests/🔗️workspace-closure/🟦️.ts" "$I/🧪️tests/🖨️print-documents/🟦️.ts" "$B/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts" "$B/🧪️tests/🕸️graph/🛂️admission/🟦️.ts" "$L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts" "$I/🧪️tests/🎓️course-inputs/🟦️.ts" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🟦️.ts"; do
  n=$((n+1)); sh "$C/red-g-tools/mrun.sh" "$NAME" chain "$s" > "$OUT/$LABEL.$n.log" 2>&1
  grep -E "^\(fail\)" "$OUT/$LABEL.$n.log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$OUT/$LABEL.$n.fails"
  echo "$n $(head -1 "$OUT/$LABEL.$n.log" | cut -c1-60) $(basename "$(dirname "$s")")"
done

#!/bin/sh
# 🧪 Runs the neighbour suites of walk-views in mirror <name> and writes the sorted (fail) set of each to <out>/<label>.<n>.fails
# usage: sh suites.sh <mirror-name> <label> <out-dir>
C="$(cd "$(dirname "$0")/.." && pwd)"; NAME="$1"; LABEL="$2"; OUT="$3"; mkdir -p "$OUT"
L="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
n=0
for s in "$L/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts" "$L/🕸️graph/📥️inference/🧪️tests/🔗️late-closures/🟦️.ts" "$L/🕸️graph/📥️inference/🧪️tests/🚧️withheld-rows/🟦️.ts" "$L/🕸️graph/🧪️tests/🧳️carried-closures/🟦️.ts" "$L/📥️inputs/🧪️tests/🟦️.ts" "$L/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🟦️.ts" "$L/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🪁️program-reads/🟦️.ts" "$L/📖️texts/🧪️tests/🟦️.ts" "$L/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "$L/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts" "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts"; do
  n=$((n+1))
  SEMIO_MIRROR_HOME=/Users/ueli/.gem python3 -I "$C/mirror-tools/mirror.py" run "$NAME" -- bun test "$s" > "$OUT/$LABEL.$n.log" 2>&1
  grep -E "^\(fail\)" "$OUT/$LABEL.$n.log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$OUT/$LABEL.$n.fails"
  echo "$n $(grep -cE '^\(pass\)' "$OUT/$LABEL.$n.log") pass $(wc -l < "$OUT/$LABEL.$n.fails" | tr -d ' ') fail $(grep -o 'guard [a-z]*' "$OUT/$LABEL.$n.log" | head -1) $(basename "$(dirname "$s")")"
done

#!/bin/sh
# 🧪 Runs the neighbour suites of the provider-closure batch inside a root and writes, per suite, the sorted (fail) set to <out>/<label>.<n>.fails and a summary line; compare failure SETS between base and batch.
# usage (inside a mirror): sh suites.sh <root> <label> <out-dir> [n...]
ROOT="$1"; LABEL="$2"; OUT="$3"; shift 3; mkdir -p "$OUT"; cd "$ROOT" || exit 2
L="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; I="$B/🕸️graph/📥️inference"
n=0
for s in "$I/🧪️tests/🔌️providers/🟦️.ts" "$I/🧪️tests/✂️derived-walks/🟦️.ts" "$I/🧪️tests/🔗️late-closures/🟦️.ts" "$I/🧪️tests/🚧️withheld-rows/🟦️.ts" "$I/🧪️tests/🧷️closure-facts/🟦️.ts" "$I/🧪️tests/🗜️interned-closures/🟦️.ts" "$I/🧪️tests/🔗️workspace-closure/🟦️.ts" "$I/🧪️tests/🦀️compiled-closure/🟦️.ts" "$I/🧪️tests/🪆️owner-child-closure/🟦️.ts" "$I/🧪️tests/🎓️course-inputs/🟦️.ts" "$I/🧪️tests/🖨️print-documents/🟦️.ts" "$I/🧪️tests/🧭️walk-edits/🟦️.ts" "$I/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🧪️tests/🪁️program-reads/🟦️.ts" "$I/🧭️script-inputs/👓️walk-views/🧪️tests/🟦️.ts" "$B/🕸️graph/🧪️tests/🧳️carried-closures/🟦️.ts" "$B/📥️inputs/🧪️tests/🟦️.ts" "$B/📖️texts/🧪️tests/🟦️.ts" "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "$B/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts" "$B/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts" "$B/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts" "$B/🧪️tests/🕸️graph/🛂️admission/🟦️.ts" "$L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/🧪️tests/📥️closure/🟦️.ts" "$B/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts" "$I/🧪️tests/🚶️repository-walks/🟦️.ts" "$I/🧪️tests/🙈️presentation-resources/🟦️.ts" "$I/🧪️tests/🧮️covered-inputs/🟦️.ts" "$L/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts" "$L/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts" "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts" "$I/♻️revision/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🟦️.ts"; do
  n=$((n+1))
  if [ $# -gt 0 ]; then case " $* " in *" $n "*) ;; *) continue;; esac; fi
  [ -f "$s" ] || { echo "$n MISSING $s"; continue; }
  bun test "$s" > "$OUT/$LABEL.$n.log" 2>&1
  grep -E "^\(fail\)" "$OUT/$LABEL.$n.log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$OUT/$LABEL.$n.fails"
  echo "$n $(grep -E '^ +[0-9]+ pass' "$OUT/$LABEL.$n.log" | tr -d ' ') $(wc -l < "$OUT/$LABEL.$n.fails" | tr -d ' ') fail $(basename "$(dirname "$s")")"
done

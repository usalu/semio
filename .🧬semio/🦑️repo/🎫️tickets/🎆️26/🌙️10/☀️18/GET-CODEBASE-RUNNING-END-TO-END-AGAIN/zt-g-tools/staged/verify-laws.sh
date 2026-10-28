#!/bin/sh
# zt-g: run the laws of the batch one file at a time (after `python3 stage.py apply`); results in 🗑️generated/zt-g/laws-after/
cd "$(dirname "$0")/../../../../../../../../.." || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
out="$C/🗑️generated/zt-g/laws-after"; mkdir -p "$out"
n=0
for f in \
  "$B/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts" \
  "$B/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts" \
  "$B/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts" \
  "$B/🚪️door/🧪️tests/🚦️startup/🟦️.ts" \
  "$B/🚪️door/🧪️tests/🟦️.ts" \
  "$B/🏛️sealed/🧭️mode/🧪️tests/🟦️.ts" \
  "$B/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts" \
  "$B/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts"
do
  n=$((n+1))
  nice -n 10 bun test "./$f" > "$out/$n.txt" 2>&1
  echo "$? $(grep -E '^ *[0-9]+ (pass|fail)' "$out/$n.txt" | tr '\n' ' ') $f"
done

#!/bin/sh
# 🧭 [cmdid] runs the law files that load the captured command module or run a captured kernel, bare, on <root>; prints "<pass>/<fail> <file>" plus failing names (compare the failure SETS of base vs batch). usage: sh neighbours.sh <root>
R="${1:?root}"; B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for f in "$B/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🪆️captured/🟦️.ts" "$B/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/✏️edit/🟦️.ts" "$B/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🕰️verified/🟦️.ts" "$B/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts" "$B/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts" "$B/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts" "$B/🏛️sealed/🧪️tests/👥️processes/🟦️.ts"; do
 [ -f "$R/$f" ] || { echo "absent $f"; continue; }
 out=$(cd "$R" && env -u SEMIO_SCRIPT_PROCESS_INVOCATION bun test --timeout 170000 "./$f" 2>&1 </dev/null)
 echo "$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ pass$' | tail -1 | tr -dc 0-9)/$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ fail$' | tail -1 | tr -dc 0-9) $f"
 printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]$//' | cut -c1-200
done

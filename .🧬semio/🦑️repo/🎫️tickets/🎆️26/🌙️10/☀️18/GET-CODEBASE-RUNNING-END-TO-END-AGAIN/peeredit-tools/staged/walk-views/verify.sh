#!/bin/sh
# 👓 Verifies the walk-views batch applied at <root>: the new law must be fully green, and every neighbour suite may fail only with its failures that are pre-existing on the base (measured 10:50 on mirror pe-walk, identical base vs batch).
# usage: sh verify.sh <root>     (run inside a mirror: python3 -I C/mirror-tools/mirror.py run <name> -- sh <abs>/verify.sh <mirror-root>)
set -u
ROOT="$1"; cd "$ROOT" || exit 2
L="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
LAW="$L/🕸️graph/📥️inference/🧭️script-inputs/👓️walk-views/🧪️tests/🟦️.ts"
ALLOWED='a TypeScript program reused by a project is named by one derived fact too|Cargo manifest inference projects the workspace sentinel before strict Source inventory admission'
bad=0
out=$(bun test "$LAW" 2>&1); echo "$out" | grep -E "^ +[0-9]+ (pass|fail)$" | tr -s ' ' | tr '\n' ' '; echo " law"
echo "$out" | grep -qE "^ +0 fail$" || { echo "LAW RED"; echo "$out" | grep -E "^\(fail\)|^error" | head -5; bad=1; }
for s in "$L/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts" "$L/🕸️graph/📥️inference/🧪️tests/🔗️late-closures/🟦️.ts" "$L/🕸️graph/📥️inference/🧪️tests/🚧️withheld-rows/🟦️.ts" "$L/🕸️graph/🧪️tests/🧳️carried-closures/🟦️.ts" "$L/📥️inputs/🧪️tests/🟦️.ts" "$L/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🟦️.ts" "$L/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🪁️program-reads/🟦️.ts" "$L/📖️texts/🧪️tests/🟦️.ts" "$L/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "$L/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts" "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts"; do
  out=$(bun test "$s" 2>&1)
  echo "$out" | grep -E "^ +[0-9]+ (pass|fail)$" | tr -s ' ' | tr '\n' ' '; echo " $(basename "$(dirname "$s")")"
  new=$(echo "$out" | grep -E "^\(fail\)" | grep -vE "$ALLOWED")
  [ -n "$new" ] && { echo "NEW FAILURE: $new"; bad=1; }
done
[ $bad = 0 ] && echo "walk-views VERIFIED" || echo "walk-views NOT VERIFIED"
exit $bad

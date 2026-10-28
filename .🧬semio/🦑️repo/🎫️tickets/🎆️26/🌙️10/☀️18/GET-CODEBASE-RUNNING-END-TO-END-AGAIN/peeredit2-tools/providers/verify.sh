#!/bin/sh
# 🔌 Verifies the `providers` batch applied at <root>: the new law must be fully green, and every neighbour suite may fail only with its failures that are pre-existing on the base (measured on mirror p2-prov, base vs batch; the selected-scope suite and the Nx engine/admission suites need the official runner envelope and fail the same on the base outside it).
# usage: sh verify.sh <root>     (inside a mirror: python3 -I C/mirror-tools/mirror.py run <name> -- sh <abs>/verify.sh <mirror-root>)
set -u
ROOT="$1"; cd "$ROOT" || exit 2
L="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; I="$B/🕸️graph/📥️inference"
LAW="$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🟦️.ts"
ALLOWED='^\(fail\) (\(unnamed\)|Cargo manifest inference projects the workspace sentinel before strict Source inventory admission|a TypeScript program reused by a project is named by one derived fact too, and asking it again follows the files the compiler reads|closure digest moves exactly when an imported module changes|command closure follows workspace package specifiers and agrees with esbuild and Bun.s bundler|literal module inputs a target declares contribute their own import closure|literal new URL references to existing files join the closure with the imports of the modules they name|neutral selected scope .*|keyed code imports the lane scheduling modules only through their ports|every real command hashes the designation and the closure of exactly the designated repository provider modules whose ports its closure defines) '
bad=0
out=$(bun test "$LAW" 2>&1); echo "$out" | grep -E "^ +[0-9]+ (pass|fail)$" | tr -s ' ' | tr '\n' ' '; echo " law provider-closures"
echo "$out" | grep -qE "^ +0 fail$" || { echo "LAW RED"; echo "$out" | grep -E "^\(fail\)|^error" | head -5; bad=1; }
for s in "$I/♻️revision/🧪️tests/🟦️.ts" "$I/🧪️tests/🔌️providers/🟦️.ts" "$I/🧪️tests/✂️derived-walks/🟦️.ts" "$I/🧪️tests/🔗️late-closures/🟦️.ts" "$I/🧪️tests/🚧️withheld-rows/🟦️.ts" "$I/🧪️tests/🧷️closure-facts/🟦️.ts" "$I/🧪️tests/🗜️interned-closures/🟦️.ts" "$I/🧪️tests/🔗️workspace-closure/🟦️.ts" "$I/🧪️tests/🦀️compiled-closure/🟦️.ts" "$I/🧪️tests/🪆️owner-child-closure/🟦️.ts" "$I/🧪️tests/🎓️course-inputs/🟦️.ts" "$I/🧪️tests/🖨️print-documents/🟦️.ts" "$I/🧪️tests/🧭️walk-edits/🟦️.ts" "$I/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🧪️tests/🪁️program-reads/🟦️.ts" "$I/🧭️script-inputs/👓️walk-views/🧪️tests/🟦️.ts" "$B/🕸️graph/🧪️tests/🧳️carried-closures/🟦️.ts" "$B/📥️inputs/🧪️tests/🟦️.ts" "$B/📖️texts/🧪️tests/🟦️.ts" "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "$B/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts" "$B/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts" "$B/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts" "$B/🧪️tests/🕸️graph/🛂️admission/🟦️.ts" "$L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/🧪️tests/📥️closure/🟦️.ts" "$B/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts" "$I/🧪️tests/🚶️repository-walks/🟦️.ts" "$I/🧪️tests/🙈️presentation-resources/🟦️.ts" "$I/🧪️tests/🧮️covered-inputs/🟦️.ts" "$L/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts" "$L/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts" "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts"; do
  out=$(bun test "$s" 2>&1)
  echo "$out" | grep -E "^ +[0-9]+ (pass|fail)$" | tr -s ' ' | tr '\n' ' '; echo " $(basename "$(dirname "$s")")"
  new=$(echo "$out" | grep -E "^\(fail\)" | grep -vE "$ALLOWED")
  [ -n "$new" ] && { echo "NEW FAILURE: $new"; bad=1; }
done
[ $bad = 0 ] && echo "providers VERIFIED" || echo "providers NOT VERIFIED"
exit $bad

#!/bin/sh
# usage: sh regress.sh MIRROR OUTFILE   runs the existing suites touching the patched files, one process at a time, and writes "suite<TAB>test<TAB>result" rows
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
M=$1; OUT=$2; : > "$OUT"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D="$L/⚡️caching/🚀️bootstrap/📦️dependencies"
mkdir -p /private/tmp/si/art
ENVELOPE=$(cd "$HERE/../../../../../../../.." && bun "$HERE/mint.ts")
cd "$M"
for suite in "$D/💾️storage/🧪️tests/🟦️.ts" "$D/💾️storage/🧪️tests/👓️reading/🟦️.ts" "$D/💾️storage/🧪️tests/📤️diagnostics/🟦️.ts" "$D/💾️storage/🧪️tests/🚧️fence/🟦️.ts" "$D/💾️storage/🧪️tests/📏️paths/🟦️.ts" "$D/💾️storage/🧪️tests/🔭️providers/🟦️.ts" "$L/🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts" "$D/🧾️generation/🧪️tests/🟦️.ts" "$D/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts"; do
  SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/si/art perl -e 'alarm 540; exec @ARGV' -- bun test --timeout 240000 "./$suite" 2>&1 | sed 's/ \[[0-9.]*ms\]//' | command grep -E '^(\((pass|fail)\)| *[0-9]+ (pass|fail)$)' | sed "s|^|$(basename $(dirname $suite))\t|" >> "$OUT"
done
sort "$OUT" | uniq -c | awk '{print}' | wc -l

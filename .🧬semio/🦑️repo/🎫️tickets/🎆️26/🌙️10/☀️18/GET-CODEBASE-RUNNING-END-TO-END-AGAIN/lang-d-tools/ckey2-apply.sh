#!/bin/sh
# usage: sh C/lang-d-tools/ckey2-apply.sh   (from the repo root; run it when nobody edits the Bazel executor/scopes/closure/script.ts)
# 1 catalog rows, 2 code + consumers, 3 law findings, 4 re-cut of the stale rows, 5 law again, 6 typecheck, 7 catalog laws.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/lang-d-tools"; O="$C/🗑️generated/lang-d"; mkdir -p "$O"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
python3 "$T/ckey2-apply-code.py" --check || exit 1
python3 "$T/ckey2-apply-catalog.py" || exit 1
python3 "$T/ckey2-apply-code.py" --apply || exit 1
nice -n 10 bun "$T/language-findings.ts" "$O/apply-1.json"
python3 "$T/recut.py" "$O/apply-1.json"
nice -n 10 bun "$T/language-findings.ts" "$O/apply-2.json"
sh "$T/tsc.sh" "$LIB/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts" "$LIB/🔧️tools/🏗️bazel/🎬️scopes/🟦️.ts" "$LIB/📥️source/🔏️closure/🟦️.ts" "$LIB/🔧️tools/🏗️bazel/📜️script.ts" "$LIB/📖️texts/🟦️.ts"
SEMIO_TEST_ARTIFACT_DIR="$PWD/$O/art" nice -n 10 bun test "./$LIB/📖️texts/🧪️tests/🟦️.ts" "./$LIB/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts"

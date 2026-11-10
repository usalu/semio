#!/usr/bin/env bash
# usage: rowstore-suites.sh <label>   runs every affected suite file one by one, one summary line per file
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/INCREMENTAL-MULTI-AGENT-BUILD-GRAPH-SERVICE"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$T/🗑️generated/rowstore/artifacts"
out="$T/🗑️generated/rowstore/suites-$1.txt";: > "$out"
{ find "$B/👁️observation" -path "*tests*" -name "🟦️.ts"; find "$B/🕸️graph/📥️inference" -path "*tests*" -name "🟦️.ts"; find "$B/🕸️graph/🕰️generations" -path "*tests*" \( -path "*cutoff*" -o -path "*load*" -o -path "*doubt*" -o -path "*claims*" -o -path "*row-keys*" -o -path "*fact-codec*" -o -path "*fact-index*" -o -path "*shared-strings*" \) -name "🟦️.ts"; find "$B/🕸️graph/🧮️derived-nodes" -path "*tests*" -name "🟦️.ts"; } | sort -u | while read -r f; do
  r=$(repo tools exec --light bun -- test "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr -s ' ' | tr '\n' ' ')
  echo "$f :: $r" >> "$out"
done
echo done >> "$out"

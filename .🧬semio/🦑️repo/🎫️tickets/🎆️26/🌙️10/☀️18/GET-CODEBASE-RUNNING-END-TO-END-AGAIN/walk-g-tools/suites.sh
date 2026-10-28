#!/bin/zsh
# [walk-g] runs the test files of the given directories one by one (graph-free, niced) and prints "<pass> <fail> <skip> <file>". usage: suites.sh <batch dir or -> <dir>...   ('-' = tree as it is; a batch dir applies its edits in memory through staged/preload_generic.ts)
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
batch=$1; shift
pre=(); if [ "$batch" != "-" ]; then export BATCH_DIRS="$batch"; pre=(--preload "$PWD/$C/walk-g-tools/staged/preload_generic.ts"); fi
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/tmp/walk-g-art}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
for dir in "$@"; do
  find "$dir" -name '🟦️.ts' -path '*🧪️tests*' -not -path '*/node_modules/*' | sort | while read -r file; do
    out=$(cd "$(dirname "$file")" && nice bun test "${pre[@]}" --timeout 120000 ./🟦️.ts 2>&1)
    p=$(echo "$out" | grep -E "^ *[0-9]+ pass$" | tr -dc 0-9); f=$(echo "$out" | grep -E "^ *[0-9]+ fail$" | tr -dc 0-9); s=$(echo "$out" | grep -E "^ *[0-9]+ skip$" | tr -dc 0-9)
    echo "${p:-?} ${f:-?} ${s:-0} $file"
  done
done

#!/bin/zsh
# usage: buntest.sh <label> <repo-relative test file>...  runs bun test (junit) with the scratch node_modules and writes C/🗑️generated/jsi-bun-<label>.{xml,log} and jsi-bunset-<label>.txt ("status<TAB>suite<TAB>name")
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated"; mkdir -p "$G/jsi-artifact"
label="$1"; shift
args=(); for a in "$@"; do args+=("./$a"); done
SEMIO_TEST_ARTIFACT_DIR="$PWD/$G/jsi-artifact/$label" NODE_PATH=/Users/ueli/.cache/jsi/nm/node_modules nice -n 10 perl -e 'alarm 900; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/jsi/nm/tsconfig.override.json test --timeout 120000 --reporter=junit --reporter-outfile="$G/jsi-bun-$label.xml" "${args[@]}" > "$G/jsi-bun-$label.log" 2>&1
echo "exit=$?" >> "$G/jsi-bun-$label.log"
python3 "$C/jsi-tools/junitset.py" "$G/jsi-bun-$label.xml" "$G/jsi-bunset-$label.txt"

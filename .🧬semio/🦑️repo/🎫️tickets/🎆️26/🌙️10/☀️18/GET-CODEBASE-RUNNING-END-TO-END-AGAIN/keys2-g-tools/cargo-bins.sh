#!/bin/sh
# cargo-bins.sh <tag> <list-key> : builds the test executables of the crates in crate-lists.json[<list-key>] ONCE through the repo (`bun repo cargo`, shared store, prototype profile), JSON messages to <tag>.jsonl in the generated folder.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/keys2-g"; T="$C/keys2-g-tools"
TAG="$1"; KEY="$2"
ARGS=$(python3 -c "
import json,sys
print(' '.join('-p '+n for n in json.load(open('$T/crate-lists.json'))['$KEY']))")
START=$(date +%s)
# shellcheck disable=SC2086
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml --no-run --message-format=json $ARGS > "$G/$TAG.jsonl" 2> "$G/$TAG.err"
echo "$? $(( $(date +%s) - START ))" > "$G/$TAG.done"

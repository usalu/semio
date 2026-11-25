#!/bin/sh
# usage: real-closure-tests.sh <names-file|-> <crate>...
cd "$(git rev-parse --show-toplevel)" || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
N="$1"; shift; [ "$N" = "-" ] && N=""
python3 "$T/real-closure-jobs.py" "$@" > "$T/🗑️generated/tests-jobs.$$.json" || exit 1
DUMP=1 node --no-warnings "$T/real-closure-probe.mjs" "$T/🗑️generated/tests-jobs.$$.json" | python3 "$T/real-closure-tests.py" $N
rm -f "$T/🗑️generated/tests-jobs.$$.json"

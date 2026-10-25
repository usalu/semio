#!/bin/sh
# usage: exec-test.sh <log-name> <abs test file...>  (runs the files through the owned repo exec route)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
name="$1"; shift
s=$(date +%s)
SEMIO_LANGUAGE=en SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/$C/🗑️generated/knot-d/artifacts-$name" nice -n 10 bun repo exec --projects=@semio-tech/repo-lib -- bun test "$@" > "$C/🗑️generated/knot-d/$name.log" 2>&1
rc=$?
echo "$name rc=$rc $(( $(date +%s)-s ))s" >> "$C/🗑️generated/knot-d/verdicts.txt"

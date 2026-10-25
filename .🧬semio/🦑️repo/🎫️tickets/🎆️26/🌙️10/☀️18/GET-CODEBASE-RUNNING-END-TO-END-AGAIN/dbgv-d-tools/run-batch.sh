#!/bin/sh
# usage: run-batch.sh <name> <project> <listfile>   (runs the listed test files through the real repo exec envelope)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name="$1"; proj="$2"; list="$3"
out="$C/🗑️generated/dbgv-d/$name"; mkdir -p "$out/art"
files=""
while IFS= read -r f; do [ -n "$f" ] && files="$files /Users/ueli/Documents/semio/$f"; done < "$list"
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/$out/art"
unset SEMIO_SCRIPT_STALL_MS
nice -n 10 bun repo --tool=semio exec --projects="$proj" -- repo tools exec bun -- test --timeout 900000 $files > "$out/run.log" 2>&1
echo $? > "$out/rc"

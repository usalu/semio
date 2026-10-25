#!/bin/sh
# usage: official.sh <target>... : official semio-tool targets of @semio-tech/framework-process, sequential; logs + rc under generated/authrev-d
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/authrev-d"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
for t in "$@"; do
 date +%T > "$out/o-$t.start"
 nice -n 10 bun repo --tool=semio run "@semio-tech/framework-process:$t" --exclude-dependencies > "$out/o-$t.log" 2>&1
 echo $? > "$out/o-$t.rc"; date +%T > "$out/o-$t.end"
done

#!/bin/sh
# usage: official.sh <project:target>... : official semio-tool targets, sequential; logs + rc under generated/bound-d
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/bound-d"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
for t in "$@"; do
 n=$(printf '%s' "$t" | tr '/@:' '___')
 date +%T > "$out/o-$n.start"
 nice -n 10 bun repo --tool=semio run "$t" --exclude-dependencies > "$out/o-$n.log" 2>&1
 echo $? > "$out/o-$n.rc"; date +%T > "$out/o-$n.end"
done
echo done > "$out/official.done"

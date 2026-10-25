#!/bin/sh
# usage: one.sh <name> <tool> <project> <target>  -> runs one official target, log + rc under C/🗑️generated/proc-d/<name>.{log,rc}
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/proc-d"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
name="$1"; tool="$2"; proj="$3"; target="$4"
date +%T > "$out/$name.start"
nice -n 10 bun repo --tool="$tool" run "$proj:$target" --exclude-dependencies > "$out/$name.log" 2>&1
echo $? > "$out/$name.rc"; date +%T > "$out/$name.end"

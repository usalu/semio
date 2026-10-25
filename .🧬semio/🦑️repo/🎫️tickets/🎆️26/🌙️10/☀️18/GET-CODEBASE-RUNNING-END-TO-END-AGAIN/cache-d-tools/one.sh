#!/bin/sh
# usage: one.sh <name> <tool> <target>  -> runs one official `repo:<target>` target, log + rc under C/🗑️generated/cache-d/<name>.{log,rc}
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/cache-d"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
name="$1"; tool="$2"; target="$3"
date +%T > "$out/$name.start"
nice -n 10 bun repo --tool="$tool" run "repo:$target" --exclude-dependencies > "$out/$name.log" 2>&1
echo $? > "$out/$name.rc"; date +%T > "$out/$name.end"

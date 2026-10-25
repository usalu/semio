#!/bin/sh
# Ticket input script (dl-c): one real describe-rs native action with a deliberately SHORT idle window; it must outlive many windows by progress alone.
#   proof.sh <name> <idle-ms>
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/dl-c"
name=$1; idle=$2
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$G/art" SEMIO_PROGRESS_DEADLINE_IDLE_MS="$idle" SEMIO_PROGRESS_DEADLINE_TRACE=1
unset SEMIO_SCRIPT_STALL_MS
date +%T > "$G/$name.started"
nice -n 10 bun repo --tool=bazel --workers=off run @semio-tech/os-plugin-describe-rs:build:prototype --skip-cache > "$G/$name.run.log" 2>&1
echo $? > "$G/$name.rc"
date +%T > "$G/$name.ended"

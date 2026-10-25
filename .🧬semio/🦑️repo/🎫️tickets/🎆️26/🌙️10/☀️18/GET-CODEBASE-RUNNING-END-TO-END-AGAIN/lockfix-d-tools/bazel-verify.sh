#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/lockfix-d"
export SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
date +%T > "$G/bazel.start"
nice -n 10 bun repo --tool=bazel run workspace:verify-dependencies-locks > "$G/bazel.log" 2>&1
echo "rc=$? $(date +%T)" > "$G/bazel.rc"

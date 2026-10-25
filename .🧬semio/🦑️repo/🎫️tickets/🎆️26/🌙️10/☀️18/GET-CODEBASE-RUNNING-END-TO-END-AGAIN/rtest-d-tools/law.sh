#!/bin/sh
# usage: law.sh <run-id>   official Bazel run of the test-domain artifact-root-source law suite
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/rtest-d"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
nice -n 10 bun repo --tool=bazel run @semio-tech/repo-test-domain:test-artifact-root-source > "$O/$1.log" 2>&1
echo $? > "$O/$1.rc"

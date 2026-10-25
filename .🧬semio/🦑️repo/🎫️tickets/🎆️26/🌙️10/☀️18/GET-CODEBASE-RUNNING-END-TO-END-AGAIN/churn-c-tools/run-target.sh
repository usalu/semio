#!/bin/sh
# usage: run-target.sh <name> <repo args...>; rc in 🗑️generated/churn-c/<name>.rc
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name=$1; shift
mkdir -p "$C/🗑️generated/churn-c/art"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$C/🗑️generated/churn-c/art" SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo "$@"
echo "rc=$?" > "$C/🗑️generated/churn-c/$name.rc"

#!/bin/sh
# usage: run-tests.sh <name> <file...>   (files relative to the repo root); runs them under a repo envelope, rc in 🗑️generated/churn-c/<name>.rc
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name=$1; shift
mkdir -p "$C/🗑️generated/churn-c/art"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$C/🗑️generated/churn-c/art" SEMIO_SCRIPT_STALL_MS=14400000
files=""
for f in "$@"; do files="$files $PWD/$f"; done
nice -n 10 bun repo exec --projects=@semio-tech/filesystem-observation -- bun test $files
echo "rc=$?" > "$C/🗑️generated/churn-c/$name.rc"

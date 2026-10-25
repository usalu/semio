#!/bin/sh
# usage: run-exec.sh <abs test file>... : explicit bun test files inside the repo exec envelope (nice 10), short artifact dir
cd /Users/ueli/Documents/semio || exit 1
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/.slot-d-art/files"
export SLOT_D_OUT="$C/🗑️generated/slot-d"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/framework-process -- bun test "$@"
echo "rc=$?"

#!/bin/sh
# usage: test.sh <suite> [log-name]   (framework-process contract suite through the repo exec envelope; nice 10)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR="${CAP_C_ARTIFACTS:-/Users/ueli/.cap-c-art}/$1"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
export SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/framework-process -- repo ./📜️script.ts test "$1"
echo "rc=$?"

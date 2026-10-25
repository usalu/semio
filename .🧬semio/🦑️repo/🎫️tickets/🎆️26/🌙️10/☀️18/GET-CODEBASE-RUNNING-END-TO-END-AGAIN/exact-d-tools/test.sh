#!/bin/sh
# usage: test.sh <suite> : framework-process contract suite through the repo exec envelope (nice 10)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR="${EXACT_D_ARTIFACTS:-/Users/ueli/.exact-d-art}/$1"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
unset SEMIO_SCRIPT_STALL_MS
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/framework-process -- repo ./📜️script.ts test "$1"
echo "rc=$?"

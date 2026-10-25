#!/bin/sh
# usage: test-files.sh <abs test file>... : explicit bun test files inside the repo exec envelope (nice 10)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR="${CAP_C_ARTIFACTS:-/Users/ueli/.cap-c-art}/files"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
export SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/framework-process -- bun test "$@"
echo "rc=$?"

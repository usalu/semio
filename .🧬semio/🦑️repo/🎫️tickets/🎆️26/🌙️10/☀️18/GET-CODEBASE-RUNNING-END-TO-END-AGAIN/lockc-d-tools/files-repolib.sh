#!/bin/sh
# usage: files-repolib.sh <abs test file>... : explicit bun test files inside the repo exec envelope of @semio-tech/repo-lib (nice 10)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR="${LOCKC_D_ARTIFACTS:-/Users/ueli/.lockc-d-art}/repolib"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
unset SEMIO_SCRIPT_STALL_MS
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/repo-lib -- bun test "$@"
echo "rc=$?"

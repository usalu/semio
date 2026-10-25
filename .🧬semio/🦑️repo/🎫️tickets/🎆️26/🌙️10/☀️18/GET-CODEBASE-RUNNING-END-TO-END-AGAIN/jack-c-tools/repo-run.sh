#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
exec nice -n 10 bun repo --tool=bazel run "$@"

#!/bin/sh
# usage: devrun.sh <log> ; reproduces the human dev route `repo run @teaching/architecture-course:dev` on Bazel (continuous task); the caller stops it (own pid only)
cd /Users/ueli/Documents/semio || exit 1
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
exec nice -n 10 bun repo --tool=bazel run @teaching/architecture-course:dev > "$1" 2>&1

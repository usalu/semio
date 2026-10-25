#!/bin/sh
# usage: serve-hold.sh : official goal-gate path, `serve-hold --serve http://127.0.0.1:6070/` (no --variant: the owner defaults to s).
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export SEMIO_SCRIPT_STALL_MS=14400000
exec nice -n 10 bun repo --tool="${SEMIO_TOOL:-bazel}" run @semio-tech/framework-os-dev:serve-hold -- --serve http://127.0.0.1:6070/

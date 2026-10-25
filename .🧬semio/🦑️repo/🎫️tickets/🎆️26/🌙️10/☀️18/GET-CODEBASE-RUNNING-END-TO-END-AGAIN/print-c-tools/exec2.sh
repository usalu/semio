#!/bin/sh
# usage: exec2.sh <project> <command words...>  (runs `repo <command words>` in the project's cwd via the semio tool, no Bazel slot, no dependsOn)
cd /Users/ueli/Documents/semio || exit 1
p=$1; shift
export NODE_OPTIONS= SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false SEMIO_SCRIPT_STALL_MS=14400000 SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 bun repo --tool=semio exec --projects="$p" -- repo "$@"

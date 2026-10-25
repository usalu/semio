#!/bin/sh
# usage: exec.sh <project> <script args...>  (runs `repo ./📜️script.ts <args>` in the project's cwd via the semio tool, no Bazel slot)
cd /Users/ueli/Documents/semio || exit 1
p=$1; shift
export NODE_OPTIONS= SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false SEMIO_SCRIPT_STALL_MS=14400000 SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 bun repo --tool=semio exec --projects="$p" -- repo ./📜️script.ts "$@"

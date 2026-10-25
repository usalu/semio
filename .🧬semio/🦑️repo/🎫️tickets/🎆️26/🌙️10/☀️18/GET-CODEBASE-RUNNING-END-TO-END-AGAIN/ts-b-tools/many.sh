#!/bin/sh
# usage: many.sh <tag> <targets> <projects-comma-list>   (bazel run-many, parallel 1)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel run-many --parallel=1 --targets="$2" --projects="$3" > "$C/🗑️generated/ts-b/$1.log" 2>&1
echo $? > "$C/🗑️generated/ts-b/$1.rc"

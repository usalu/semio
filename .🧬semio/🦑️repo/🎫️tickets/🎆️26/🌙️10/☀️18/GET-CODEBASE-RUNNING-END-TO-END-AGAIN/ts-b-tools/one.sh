#!/bin/sh
# usage: one.sh <project> <target>   (bazel lane)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag=$(echo "$1-$2" | tr '/@:' '___')
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel run "$1:$2" > "$C/🗑️generated/ts-b/$tag.log" 2>&1
echo $? > "$C/🗑️generated/ts-b/$tag.rc"

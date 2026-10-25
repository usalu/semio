#!/bin/sh
# usage: one.sh <project> <target>   (bazel lane, headless via detach.py)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag=$(echo "$1-$2" | tr '/@:' '___')
export SEMIO_SCRIPT_STALL_MS=14400000 SEMIO_BUILD_GRAPH_POLICY=current
date +%s > "$C/🗑️generated/ts-c/$tag.start"
nice -n 10 bun repo --tool=bazel run "$1:$2" > "$C/🗑️generated/ts-c/$tag.log" 2>&1
echo $? > "$C/🗑️generated/ts-c/$tag.rc"

#!/bin/sh
# usage: one.sh <nx|bazel> <project>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag=$(echo "$2" | tr '/@' '__')-$1
if [ "$1" = bazel ]; then T="--tool=bazel"; else T=""; fi
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo $T run "$2:typecheck" > "$C/🗑️generated/ts-verify/$tag.log" 2>&1
echo $? > "$C/🗑️generated/ts-verify/$tag.rc"

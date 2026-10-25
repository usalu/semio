#!/bin/sh
# usage: one.sh <tag> <project:target> [-- extra repo args...]   (bazel)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; spec="$2"; shift 2
export SEMIO_BUILD_GRAPH_POLICY="${GRAPH_POLICY:-current}" SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
export PLAYWRIGHT_BROWSERS_PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
O="$C/🗑️generated/layout-d"
date +%s > "$O/$tag.start"
nice -n 10 bun repo --tool=bazel run "$spec" "$@" > "$O/$tag.log" 2>&1
echo $? > "$O/$tag.rc"

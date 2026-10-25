#!/bin/sh
# usage: many.sh <tag> <tool> <targets> <projects-comma-list> [extra repo args...]
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; tool="$2"; targets="$3"; projects="$4"; shift 4
export SEMIO_BUILD_GRAPH_POLICY="${GRAPH_POLICY:-current}" SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
export PLAYWRIGHT_BROWSERS_PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
O="$C/🗑️generated/ctest-d"
date +%s > "$O/$tag.start"
nice -n 10 bun repo --tool="$tool" run-many --parallel=1 --targets="$targets" --projects="$projects" "$@" > "$O/$tag.log" 2>&1
echo $? > "$O/$tag.rc"

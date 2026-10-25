#!/bin/sh
# usage: many.sh <tag> <targets> <projects-comma-list> [extra repo args...]   (bazel run-many, parallel 1, headless via detach.py)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; targets="$2"; projects="$3"; shift 3
export SEMIO_BUILD_GRAPH_POLICY=current
unset SEMIO_SCRIPT_STALL_MS
date +%s > "$C/🗑️generated/ts-c/$tag.start"
nice -n 10 bun repo --tool=bazel run-many --parallel=1 --targets="$targets" --projects="$projects" "$@" > "$C/🗑️generated/ts-c/$tag.log" 2>&1
echo $? > "$C/🗑️generated/ts-c/$tag.rc"

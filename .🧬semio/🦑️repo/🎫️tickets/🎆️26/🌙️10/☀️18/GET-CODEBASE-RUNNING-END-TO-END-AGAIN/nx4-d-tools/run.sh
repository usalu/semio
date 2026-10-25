#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nx4-d"
target="$1"; name="$2"; shift 2
echo "start=$(date +%H:%M:%S) pid=$$" >> "$G/$name.log"
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo run "$target" "$@" >> "$G/$name.log" 2>&1
echo "rc=$? end=$(date +%H:%M:%S)" >> "$G/$name.log"

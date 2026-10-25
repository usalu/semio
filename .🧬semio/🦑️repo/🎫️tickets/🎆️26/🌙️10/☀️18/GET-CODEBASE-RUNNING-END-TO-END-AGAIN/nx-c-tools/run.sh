#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nx-c"
target="$1"; name="$2"; shift 2
export SEMIO_BUILD_GRAPH_POLICY=current NX_VERBOSE_LOGGING=true
echo "start=$(date +%H:%M:%S) pid=$$" >> "$G/$name.log"
nice -n 10 bun repo --tool=nx run "$target" "$@" >> "$G/$name.log" 2>&1
echo "rc=$? end=$(date +%H:%M:%S)" >> "$G/$name.log"

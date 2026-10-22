#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nx-daemon"
target="$1"; name="$2"; shift 2
export SEMIO_BUILD_GRAPH_POLICY=current NX_VERBOSE_LOGGING=true
nice -n 10 bun repo run "$target" "$@" > "$G/$name.log" 2>&1
echo "rc=$?" >> "$G/$name.log"

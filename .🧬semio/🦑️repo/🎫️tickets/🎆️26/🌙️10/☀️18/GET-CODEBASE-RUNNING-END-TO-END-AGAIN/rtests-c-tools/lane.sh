#!/bin/sh
# usage: lane.sh <name> <tool: nx|bazel> <project:target>  -> out/err/rc in 🗑️generated/rtests-c/lanes
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/🗑️generated/rtests-c/lanes"; mkdir -p "$S"
export NX_FORCE_REUSE_CACHED_GRAPH=true
nice -n 10 bun repo --tool="$2" run "$3" > "$S/$1.out" 2> "$S/$1.err"
echo "rc=$?" >> "$S/$1.err"

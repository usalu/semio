#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
echo "pid=$$ $(date)" > "$C/🗑️generated/default-d/$n.pid"
nice -n 10 bun repo "$@" > "$C/🗑️generated/default-d/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/default-d/$n.raw"

#!/bin/sh
# usage: run.sh <logname> <repo args...>   (runs `bun repo <args>` at nice 10, writes rc into <log>.rc)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/ext-d"
export SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
name="$1"; shift
echo "start $(date +%T)" > "$G/$name.rc"
nice -n 10 bun repo "$@" > "$G/$name.log" 2>&1
echo "rc=$? $(date +%T)" >> "$G/$name.rc"

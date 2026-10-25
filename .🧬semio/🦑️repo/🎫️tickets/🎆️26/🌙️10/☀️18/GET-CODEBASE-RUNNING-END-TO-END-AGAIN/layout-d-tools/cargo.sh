#!/bin/sh
# usage: cargo.sh <tag> <cargo args...>   (graph-free repo cargo, shared store, prototype)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; shift
export SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
O="$C/🗑️generated/layout-d"
date +%s > "$O/$tag.start"
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$O/$tag.log" 2>&1
echo $? > "$O/$tag.rc"

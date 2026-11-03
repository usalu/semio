#!/bin/bash
# usage: run.sh <n> <vitest paths relative to renderer-react package root...>
cd /Users/ueli/Documents/semio || exit 1
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=reacthost-$1; shift; O="$C/🗑️generated/1102/$N"; rm -f "$O.rc"
"$C/1102-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en SEMIO_TEST_LEVEL=long repo run @semio-tech/framework-renderer-react:test -- "$@" --run > "$O.txt" 2>&1
echo $? > "$O.rc"

#!/bin/bash
# usage: run.sh <n> <project:target> [args...]
cd /Users/ueli/Documents/semio || exit 1
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=reacthost-$1; T=$2; shift 2; O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en SEMIO_TEST_LEVEL=long NX_DAEMON=false repo run "$T" -- "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"

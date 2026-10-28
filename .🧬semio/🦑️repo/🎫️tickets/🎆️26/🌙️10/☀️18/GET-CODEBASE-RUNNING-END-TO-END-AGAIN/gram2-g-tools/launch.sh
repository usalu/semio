#!/bin/sh
# [gram2-g] usage: launch.sh <label> [VAR=value ...]  detached run.sh; prints the wrapper output
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
label=$1
mkdir -p "$C/🗑️generated/gram2-g"
env -u SEMIO_SCRIPT_STALL_MS python3 "$C/chain-tools/detach.py" "$C/🗑️generated/gram2-g/$label.wrap.log" sh "$C/gram2-g-tools/run.sh" "$@"

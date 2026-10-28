#!/bin/sh
# usage: launch.sh <label> [VAR=value ...]  detached (headless) run.sh; prints the pid
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
label=$1
mkdir -p "$C/🗑️generated/gram-g"
env -u SEMIO_SCRIPT_STALL_MS python3 "$C/chain-tools/detach.py" "$C/🗑️generated/gram-g/$label.wrap.log" sh "$C/gram-g-tools/run.sh" "$@"

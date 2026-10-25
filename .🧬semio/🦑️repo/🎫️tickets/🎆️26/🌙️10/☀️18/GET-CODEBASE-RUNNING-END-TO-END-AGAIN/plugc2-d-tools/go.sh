#!/bin/sh
# usage: go.sh <tag> <cargo args...>  -> detached run, log C/🗑️generated/plugc2-d/<tag>.log
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag=$1; shift
env -u SEMIO_SCRIPT_STALL_MS python3 "$C/chain-tools/detach.py" "$C/🗑️generated/plugc2-d/$tag.drv" sh "$C/plugc2-d-tools/run.sh" "$C/🗑️generated/plugc2-d/$tag.log" "$@"

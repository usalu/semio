#!/bin/sh
# usage: go.sh <tag> <waitseconds> <cargo args...>  detached cargo via cargo.sh, bounded foreground wait, prints tail
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; wait="$2"; shift 2
O="$C/🗑️generated/cui-d"
rm -f "$O/$tag.rc" "$O/$tag.log"
pid=$(env -u SEMIO_SCRIPT_STALL_MS python3 "$C/chain-tools/detach.py" "$O/$tag.pid.log" sh "$C/cui-d-tools/cargo.sh" "$tag" "$@")
echo "$pid" > "$O/$tag.pid"
echo "pid $pid"
i=0
while [ $i -lt $((wait/5)) ]; do [ -f "$O/$tag.rc" ] && break; sleep 5; i=$((i+1)); done
[ -f "$O/$tag.rc" ] && echo "rc $(cat $O/$tag.rc)" || echo "still running"

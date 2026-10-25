#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name="$1"; list="$2"; out="$C/🗑️generated/dbgv-d/$name"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS
export ART="/Users/ueli/Documents/semio/$C/🗑️generated/dbgv-d/a"
nice -n 10 bun repo --tool=semio exec --projects=workspace -- sh "/Users/ueli/Documents/semio/$C/dbgv-d-tools/loop.sh" "/Users/ueli/Documents/semio/$out/files" "/Users/ueli/Documents/semio/$list" > "$out/run.log" 2>&1
echo $? > "$out/rc"

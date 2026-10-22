#!/bin/sh
# usage: probe-slice.sh <label> <offset> <limit> [extra probe args]; foreground, one process, nice 10
cd /Users/ueli/Documents/semio || exit 1
L="$1"; O="$2"; N="$3"; shift 3
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/audit-fix-tools/staged"
nice -n 10 bun "$D/probe.ts" --roots "$(cat /private/tmp/audit-fix/roots.txt)" --offset "$O" --limit "$N" --out "/private/tmp/audit-fix/out/$L-$O.json" "$@" 2>&1 | grep -v '^\[script\]' | tail -2

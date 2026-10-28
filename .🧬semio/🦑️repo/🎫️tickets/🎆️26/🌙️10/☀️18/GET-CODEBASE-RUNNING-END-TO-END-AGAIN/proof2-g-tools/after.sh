#!/bin/sh
# after.sh <pid> <outdir> <label> <cmd...>: waits for a peer's run (pid) to end, then runs the command measured
cd "$(dirname "$0")/../../../../../../../.." || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
P="$1"; O="$2"; L="$3"; shift 3
while kill -0 "$P" 2>/dev/null; do sleep 5; done
exec python3 "$C/proof2-g-tools/measure.py" "$O" "$L" "$@"

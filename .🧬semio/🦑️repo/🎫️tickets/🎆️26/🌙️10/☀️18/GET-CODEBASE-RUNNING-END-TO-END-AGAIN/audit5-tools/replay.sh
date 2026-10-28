#!/bin/sh
# 🔁 [audit5] Graph-free inference replay in mirror a5 (adapted gram3 harness): sh replay.sh <label> [VAR=value ...]; rows dump + projects + log in 🗑️generated/audit5/<label>*, scratch store in $STORE_BASE/<label>.
here=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$here"); label=$1; shift
M=$(python3 -c "import os;print(os.path.realpath(os.path.expandvars('\$TMPDIR/sm/a5')))")
G="$C/🗑️generated/audit5"; mkdir -p "$G"; BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
STORE="${STORE_BASE:?STORE_BASE required}/$label"; rm -rf "$G/$label.rows"
python3 -I "$C/mirror-tools/mirror.py" run a5 --seal-allow ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/audit5" -- env -u SEMIO_SCRIPT_STALL_MS GRAM_ROOT="$M" FILES_LIST="$G/files.txt" STORE="$STORE" OUT="$G/$label.json" ROWS_DUMP="$G/$label.rows" PROJECT_OUT="$G/$label.projects.json" PROBES=0 "$@" nice -n 10 "$BUN" "$here/full.mjs" > "$G/$label.log" 2>&1
echo "rc $?"; grep -E "DEBUG\] (files|PROJECT_OUT|result|WALL|ABORT)" "$G/$label.log" | cut -c1-300; tail -c 600 "$G/$label.log"

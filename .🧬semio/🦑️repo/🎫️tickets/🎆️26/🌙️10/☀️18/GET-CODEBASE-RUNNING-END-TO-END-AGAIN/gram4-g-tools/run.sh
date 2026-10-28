#!/bin/sh
# [gram4-g] usage: run.sh <label> [VAR=value ...]  foreground niced harness (gram4-g-tools/full.mjs) on the live checkout (read only: the library resolves its own workspace root, so a mirror library cannot be pointed at live data); scratch store /private/tmp/gram4-g/<label>, log + json + fp samples in 🗑️generated/gram4-g/. BATCHES=<dir>,<dir> applies staged edits.json batches in memory (walk-g preload) on top of the live library. Own pids only.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; shift
ulimit -n 8192
OUTD="/Users/ueli/Documents/semio/$C/🗑️generated/gram4-g"; mkdir -p "$OUTD" /private/tmp/gram4-g
G="/Users/ueli/Documents/semio/$C"
PRE=""; B=""; FLAGS=""
for a in "$@"; do case "$a" in BATCHES=*) B="${a#BATCHES=}";; BUNFLAGS=*) FLAGS="${a#BUNFLAGS=}";; esac; done
[ -n "$B" ] && PRE="--preload $G/walk-g-tools/staged/preload_generic.ts"
env -u SEMIO_SCRIPT_STALL_MS ${B:+BATCH_DIRS="$B"} STORE=/private/tmp/gram4-g/$label OUT="$OUTD/$label.json" OWNER_KINDS=1 "$@" nice -n 10 $BUN $PRE $FLAGS "$G/gram4-g-tools/full.mjs" > "$OUTD/$label.log" 2>&1 &
pid=$!
echo "$pid $(ps -o lstart= -p $pid)" > "$OUTD/$label.pid"
sh "$G/gram4-g-tools/fp.sh" $pid "$OUTD/$label.fp" 5 &
wait $pid; echo "rc $?"
rg -o '\[DEBUG\] OWNER .*' "$OUTD/$label.log" | cut -c1-260 | head -20
tail -c 700 "$OUTD/$label.log" | cut -c1-500
python3 -I "$G/gram4-g-tools/fpmax.py" "$OUTD/$label.fp"

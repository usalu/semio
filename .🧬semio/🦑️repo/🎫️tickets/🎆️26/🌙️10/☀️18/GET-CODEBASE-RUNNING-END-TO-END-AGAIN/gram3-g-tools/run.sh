#!/bin/sh
# [gram3-g] usage: run.sh <label> [VAR=value ...]  foreground niced harness (gram3-g-tools/full.mjs), scratch store /private/tmp/gram3-g/<label>, log + json + fp samples in 🗑️generated/gram3-g/. BATCHES=<dir>,<dir> applies staged batches in memory (staged/preload_generic.ts of walk-g). BUNFLAGS="--cpu-prof ..." extra bun flags. Not a library file.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; shift
ulimit -n ${GRAM_NOFILE:-8192}
mkdir -p "$C/🗑️generated/gram3-g" /private/tmp/gram3-g
G="$PWD/$C"
PRE=""; B=""
for a in "$@"; do case "$a" in BATCHES=*) B="${a#BATCHES=}";; esac; done
[ -n "$B" ] && PRE="--preload $G/walk-g-tools/staged/preload_generic.ts"
for a in "$@"; do case "$a" in COUNT=1) PRE="--preload $G/gram3-g-tools/count/preload.ts"; export COUNT_OUT="$G/🗑️generated/gram3-g/$label.count";; esac; done
for a in "$@"; do case "$a" in BUNFLAGS=*) FLAGS="${a#BUNFLAGS=}";; esac; done
env -u SEMIO_SCRIPT_STALL_MS ${B:+BATCH_DIRS="$B"} STORE=/private/tmp/gram3-g/$label OUT="$C/🗑️generated/gram3-g/$label.json" OWNER_KINDS=1 "$@" nice -n 10 $BUN $PRE $FLAGS $C/gram3-g-tools/full.mjs > "$C/🗑️generated/gram3-g/$label.log" 2>&1 &
pid=$!
echo "$pid $(ps -o lstart= -p $pid)" > "$C/🗑️generated/gram3-g/$label.pid"
sh "$C/gram3-g-tools/fp.sh" $pid "$C/🗑️generated/gram3-g/$label.fp" 5 &
wait $pid; echo "rc $?"
rg -o '\[DEBUG\] OWNER .*' "$C/🗑️generated/gram3-g/$label.log" | cut -c1-300 | head -20
tail -c 900 "$C/🗑️generated/gram3-g/$label.log" | cut -c1-500

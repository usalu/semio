#!/bin/sh
# [walk-g] usage: run.sh <label> [VAR=value ...]  foreground niced harness (walk-g-tools/full.mjs = full.mjs of gram2-g with OWNER_KINDS), scratch store /private/tmp/walk-g/<label>, log + json in 🗑️generated/walk-g/. STAGED=1 applies the batch `staged/walks` in memory through staged/preload_generic.ts; a footprint sampler (fp.sh) writes <label>.fp beside the log.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; shift
ulimit -n ${GRAM_NOFILE:-8192}
mkdir -p "$C/🗑️generated/walk-g" /private/tmp/walk-g
PRE=""; case " $* " in *" STAGED=1 "*) PRE="--preload $PWD/$C/walk-g-tools/staged/preload_generic.ts"; export BATCH_DIRS="$PWD/$C/walk-g-tools/staged/walks";; esac
env -u SEMIO_SCRIPT_STALL_MS STORE=/private/tmp/walk-g/$label OUT="$C/🗑️generated/walk-g/$label.json" OWNER_KINDS=1 "$@" nice -n 10 $BUN $PRE $C/walk-g-tools/full.mjs > "$C/🗑️generated/walk-g/$label.log" 2>&1 &
pid=$!
sh "$C/walk-g-tools/fp.sh" $pid "$C/🗑️generated/walk-g/$label.fp" 5 &
wait $pid; echo "rc $?"
rg -o '\[DEBUG\] OWNER .*' "$C/🗑️generated/walk-g/$label.log" | cut -c1-400 | head -20
tail -c 800 "$C/🗑️generated/walk-g/$label.log" | cut -c1-500

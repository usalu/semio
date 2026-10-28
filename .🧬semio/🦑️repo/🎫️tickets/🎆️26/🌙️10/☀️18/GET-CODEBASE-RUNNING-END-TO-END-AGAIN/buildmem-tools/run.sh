#!/bin/sh
# [buildmem] usage: run.sh <label> [VAR=value ...]  foreground niced graph-free harness (gram4-g-tools/full.mjs) with buildmem preloads; scratch store /private/tmp/buildmem/<label>; log/json/fp in 🗑️generated/buildmem/. PRELOADS=<file>,<file> extra preloads. Not a library file.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=$(ls -d .cache/b/t/m/bun/*/bun | head -1)
label=$1; shift
ulimit -n 8192
G="$PWD/$C"; O="$G/🗑️generated/buildmem"; mkdir -p "$O" /private/tmp/buildmem
PRE=""
for a in "$@"; do case "$a" in PRELOADS=*) for p in $(echo "${a#PRELOADS=}" | tr ',' ' '); do PRE="$PRE --preload $p"; done;; esac; done
env -u SEMIO_SCRIPT_STALL_MS STORE=/private/tmp/buildmem/$label OUT="$O/$label.json" "$@" nice -n 10 $BUN $PRE "$C/${HARNESS:-gram4-g-tools/full.mjs}" > "$O/$label.log" 2>&1 &
pid=$!
echo "$pid" > "$O/$label.pid"
sh "$C/gram3-g-tools/fp.sh" $pid "$O/$label.fp" 5 &
fpid=$!
wait $pid; rc=$?; kill $fpid 2>/dev/null; echo "rc $rc"
tail -c 1200 "$O/$label.log" | cut -c1-400

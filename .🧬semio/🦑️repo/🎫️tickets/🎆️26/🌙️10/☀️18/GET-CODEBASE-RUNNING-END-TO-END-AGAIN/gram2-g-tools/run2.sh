#!/bin/sh
# [gram2-g] usage: run2.sh <label> [VAR=value ...]  like run.sh plus PRELOAD=<file> (bun --preload) and STORE_FROM=<label> (copy a warm store first); a NEW file because a running run.sh must never be edited.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; shift
ulimit -n ${GRAM_NOFILE:-8192}
mkdir -p "$C/🗑️generated/gram2-g" /private/tmp/gram2-g/art
PRE=""
for a in "$@"; do case $a in
  STORE_FROM=*) src=/private/tmp/gram2-g/${a#STORE_FROM=}; rm -rf /private/tmp/gram2-g/$label; cp -R "$src" /private/tmp/gram2-g/$label;;
  PRELOAD=*) PRE="--preload ${a#PRELOAD=}";;
esac; done
RUN="test $PRE --timeout 7200000 ./$C/gram2-g-tools/part.test.ts"
case " $* " in *" PART=1 "*) ;; *) RUN="$PRE $C/gram2-g-tools/full.mjs";; esac
WIRE=""; case " $* " in *" PART=1 "*) WIRE="$($BUN $C/chain-g-tools/wire.ts)";; esac
env -u SEMIO_SCRIPT_STALL_MS ${WIRE:+SEMIO_SCRIPT_PROCESS_INVOCATION="$WIRE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/gram2-g/art} STORE=/private/tmp/gram2-g/$label OUT="$C/🗑️generated/gram2-g/$label.json" "$@" nice -n 10 $BUN $RUN > "$C/🗑️generated/gram2-g/$label.log" 2>&1
echo "rc $?"; tail -c 1500 "$C/🗑️generated/gram2-g/$label.log" | cut -c1-600

#!/bin/sh
# usage: run.sh <label> [VAR=value ...]  runs full.mjs headless-ish in the foreground, niced, log in 🗑️generated/gram-g/<label>.log, json in <label>.json; prints the tail
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; shift
ulimit -n ${GRAM_NOFILE:-8192}
case " $* " in *" PART=1 "*) RUN="test --timeout 7200000 ./$C/gram-g-tools/part.test.ts";; *) RUN="$C/gram-g-tools/full.mjs";; esac
WIRE=""; case " $* " in *" PART=1 "*) mkdir -p /private/tmp/gram-g/art; WIRE="$($BUN $C/chain-g-tools/wire.ts)";; esac
env -u SEMIO_SCRIPT_STALL_MS ${WIRE:+SEMIO_SCRIPT_PROCESS_INVOCATION="$WIRE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/gram-g/art} STORE=/private/tmp/gram-g/$label OUT="$C/🗑️generated/gram-g/$label.json" "$@" nice -n 10 $BUN $RUN > "$C/🗑️generated/gram-g/$label.log" 2>&1
echo "rc $?"; tail -c 1500 "$C/🗑️generated/gram-g/$label.log" | cut -c1-600

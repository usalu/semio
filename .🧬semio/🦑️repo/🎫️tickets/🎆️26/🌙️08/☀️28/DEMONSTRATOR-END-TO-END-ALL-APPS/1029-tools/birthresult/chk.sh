#!/bin/bash
# usage: chk.sh <N> <manifest-relative-to-repo-root> <wasm|native> <cargo check args...>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; M=$2; K=$3; shift 3
rm -f "$C/🗑️generated/1029/birthresult-chk$N.rc"
T=""; [ "$K" = wasm ] && T="--target wasm32-wasip2"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$M" $T --keep-going "$@" > "$C/🗑️generated/1029/birthresult-chk$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/birthresult-chk$N.rc"

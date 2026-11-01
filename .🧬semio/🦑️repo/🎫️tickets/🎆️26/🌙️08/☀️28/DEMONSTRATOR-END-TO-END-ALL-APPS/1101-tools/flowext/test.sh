#!/bin/bash
# usage: test.sh <label> <manifest> <cargo test args...>   (native)
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1101/flowtest-$1"; M=$2; shift 2
cd "$ROOT" || exit 1
rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/$M" --no-fail-fast "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"

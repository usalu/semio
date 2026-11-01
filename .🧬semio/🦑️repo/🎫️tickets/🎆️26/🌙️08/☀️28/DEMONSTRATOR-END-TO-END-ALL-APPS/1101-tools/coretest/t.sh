#!/bin/bash
# usage: MAN=<manifest> t.sh <n> <cargo test args...>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
O="$C/🗑️generated/1101/coretest-$N"; rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "${MAN:-Cargo.toml}" --no-fail-fast "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"

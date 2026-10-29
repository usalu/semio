#!/bin/bash
# usage: chk.sh N wasm|native <artifact-dir-name> [extra cargo args...]
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; MODE=$2; WS=$3; shift 3
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/stdiofix-$N.rc"
T=(); [ "$MODE" = wasm ] && T=(--target wasm32-wasip2)
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$WS/Cargo.toml" "${T[@]}" --workspace --keep-going "$@" > "$C/🗑️generated/1029/stdiofix-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/stdiofix-$N.rc"

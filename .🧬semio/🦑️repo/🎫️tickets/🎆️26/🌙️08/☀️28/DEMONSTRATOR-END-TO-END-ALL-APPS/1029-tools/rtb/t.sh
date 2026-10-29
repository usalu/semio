#!/bin/bash
# usage: t.sh <name> <workspace-dir relative to repo root> <cargo test args...>
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; WS=$2; shift 2
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/rtb-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/$WS/Cargo.toml" "$@" --lib --tests --no-fail-fast > "$C/🗑️generated/1029/rtb-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/rtb-$N.rc"

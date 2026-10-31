#!/bin/bash
# usage: nr.sh <name> <workspace-dir> <cargo test args...>  (compile only)
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; WS=$2; shift 2
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1030/hubplug-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --no-run --manifest-path "$ROOT/$WS/Cargo.toml" "$@" > "$C/🗑️generated/1030/hubplug-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/hubplug-$N.rc"

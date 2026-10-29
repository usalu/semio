#!/bin/bash
# usage: c.sh <name> <workspace-dir rel to repo root> <extra cargo check args...>  (native, --lib --tests, keep-going)
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; WS=$2; shift 2
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/hostinit-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$WS/Cargo.toml" "$@" --lib --tests --keep-going > "$C/🗑️generated/1029/hostinit-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/hostinit-$N.rc"

#!/bin/bash
# usage: t.sh <name> <workspace-dir rel to repo root> <package> [test filter...]  (native lib tests of one package)
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; WS=$2; PKG=$3; shift 3
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/hostinit-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/$WS/Cargo.toml" -p "$PKG" --lib --no-fail-fast -- "$@" > "$C/🗑️generated/1029/hostinit-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/hostinit-$N.rc"

#!/bin/bash
# usage: chk.sh NAME WORKSPACE_DIR(relative to repo root, "." for root) crate...
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; WS=$2; shift 2
P=(); for c in "$@"; do P+=(-p "$c"); done
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1030/cleanup-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$WS/Cargo.toml" "${P[@]}" --tests --keep-going > "$C/🗑️generated/1030/cleanup-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/cleanup-$N.rc"

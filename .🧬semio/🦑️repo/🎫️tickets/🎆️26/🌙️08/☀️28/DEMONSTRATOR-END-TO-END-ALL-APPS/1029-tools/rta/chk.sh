#!/bin/bash
# usage: chk.sh <name> <manifest-dir> <cargo check args...>
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd "$ROOT" || exit 1
name="$1"; dir="$2"; shift 2
rm -f "$C/🗑️generated/1029/rta-$name.rc"
CARGO_INCREMENTAL=0 SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$dir/Cargo.toml" "$@" > "$C/🗑️generated/1029/rta-$name.txt" 2>&1
echo $? > "$C/🗑️generated/1029/rta-$name.rc"

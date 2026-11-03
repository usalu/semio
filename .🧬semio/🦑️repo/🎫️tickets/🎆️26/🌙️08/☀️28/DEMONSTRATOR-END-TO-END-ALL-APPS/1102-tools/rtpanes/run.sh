#!/bin/bash
# usage: run.sh <name> <artifact workspace dir rel to repo root> <cargo test args...>   (one native cargo test, rc marker)
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; WS=$2; shift 2
cd "$ROOT" || exit 1
mkdir -p "$C/🗑️generated/1102"
rm -f "$C/🗑️generated/1102/rtpanes-$N.rc"
SEMIO_LANGUAGE=en "$C/1102-tools/slot.sh" "rtpanes-$N" repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/$WS/Cargo.toml" --no-fail-fast "$@" > "$C/🗑️generated/1102/rtpanes-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1102/rtpanes-$N.rc"

#!/bin/bash
# usage: build.sh <tag> <cargo test --no-run args...>   (framework workspace)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
O="$C/🗑️generated/1101/pixtest-$N"; rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml --no-run "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"
/usr/bin/grep -o 'Executable.*(\(.*\))$' "$O.txt" | sed -E 's/.*\((.*)\)$/\1/' | while read -r p; do echo "$PWD/$p"; done > "$O.bins"

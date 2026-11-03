#!/bin/bash
# usage: test3d.sh <label> <3d|2d> <crate-suffix>... — native lib tests of procedural crates
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; D=$2; shift 2; O="$C/🗑️generated/1102/gen-$N"; rm -f "$O.rc"
P=(); for c in "$@"; do P+=(-p "semio-s-artifact-procedural-$c"); done
G="🧊️generation3d"; [ "$D" = 2d ] && G="🌀️generation2d"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/$G/Cargo.toml" --no-fail-fast --lib "${P[@]}" > "$O.txt" 2>&1
echo $? > "$O.rc"

#!/bin/bash
# usage: test3d.sh <N> <crate-suffix>... — native lib tests of generation3d crates (the source gate admits few crates per run)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift; O="$C/🗑️generated/1101/gen-test$N"; rm -f "$O.rc"
P=""
for c in "$@"; do P="$P -p semio-s-artifact-procedural-$c"; done
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml" --no-fail-fast --lib $P > "$O.txt" 2>&1
echo $? > "$O.rc"

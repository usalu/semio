#!/bin/bash
# usage: testpkg.sh <label> <3d|2d> <package> <cargo test args...> — native cargo test of one package
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; D=$2; P=$3; shift 3; O="$C/🗑️generated/1102/gen-$N"; rm -f "$O.rc"
G="🧊️generation3d"; [ "$D" = 2d ] && G="🌀️generation2d"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/$G/Cargo.toml" --no-fail-fast -p "$P" "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"

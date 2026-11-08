#!/bin/bash
# usage: t.sh <n> <crate> [cargo test args...] — tst.sh label valuetests-<n> on the framework manifest
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; K=$2; shift 2
"$C/1105b-tools/runtime/tst.sh" "valuetests-$N" "${MANIFEST:-🧰️framework/Cargo.toml}" native -p "$K" "$@"

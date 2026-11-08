#!/bin/bash
# usage: c.sh <n> <crate>... — cargo check --tests --keep-going on the framework manifest for all crates
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
A=(); for k in "$@"; do A+=(-p "$k"); done
"$C/1105b-tools/chk.sh" "testdebt-g-$N" "${MANIFEST:-🧰️framework/Cargo.toml}" native --tests "${A[@]}"

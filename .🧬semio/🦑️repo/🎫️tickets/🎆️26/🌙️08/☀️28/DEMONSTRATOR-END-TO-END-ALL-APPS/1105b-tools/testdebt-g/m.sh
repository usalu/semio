#!/bin/bash
# usage: m.sh <n> <crate>... — one repo cargo test --lib --tests over all crates (compile errors of every crate in one log)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
A=(); for k in "$@"; do A+=(-p "$k"); done
"$C/1105b-tools/runtime/tst.sh" "testdebt-g-$N" "${MANIFEST:-🧰️framework/Cargo.toml}" native "${A[@]}" --lib --tests -- --test-threads=4

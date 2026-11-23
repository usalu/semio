#!/bin/zsh
# usage: residue-run.sh <label> <crate-dir-relative-to-modules> <cargo test args...>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules"
L=$1; D=$2; shift 2
cd "$M/$D/📦️packages/🦀️rust" || exit 1
zsh "$T/stale-cargo.sh" "$L" test --manifest-path "$PWD/Cargo.toml" "$@"
F="$T/🗑️generated/stale-$L.txt"
grep -n "^error" -A12 "$F" | cut -c1-500 | head -60
grep -n "^test \|panicked\|left:\|right:\|\[DEBUG\]" -A2 "$F" | cut -c1-900 | head -120

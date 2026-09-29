#!/bin/zsh
# 🧪️ Builds the unified lib test binaries of the listed crates in the BASE clone (same package set as the overlay proof, so feature
# unification matches) and runs each overlay red there alone (`--exact`): a red that is also red on base is pre-existing.
# base-reds.sh <reds-file: "crate-name test-name" per line> <crate…>
reds="$1"; shift
B=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-bbuild/debug/build
args=(); for c in "$@"; do args+=(-p "$c"); done
cargo test --offline --no-run --message-format short --lib $args 2>&1 | /usr/bin/grep -E "^error" -A3 | head -20
while read -r crate name; do
  bin=$(ls -t $B/$crate/*/out/${crate//-/_}-* 2>/dev/null | /usr/bin/grep -v '\.d$' | head -1)
  if [[ -z "$bin" ]]; then echo "NOBIN $crate"; continue; fi
  if "$bin" --exact "$name" --test-threads 1 >/dev/null 2>&1; then echo "BASE-OK $crate $name"; else echo "BASE-RED $crate $name"; fi
done < "$reds"

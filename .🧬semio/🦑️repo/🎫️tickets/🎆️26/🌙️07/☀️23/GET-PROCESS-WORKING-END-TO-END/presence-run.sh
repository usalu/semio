#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/📦️packages/🦀️rust" || exit 1
zsh "$T/stale-cargo.sh" "$1" test --manifest-path "$PWD/Cargo.toml" --lib
F="$T/🗑️generated/stale-$1.txt"
grep -n "^error" -A12 "$F" | cut -c1-500 | head -60
grep -n "^test \|panicked\|left:\|right:" -A2 "$F" | cut -c1-900 | head -50

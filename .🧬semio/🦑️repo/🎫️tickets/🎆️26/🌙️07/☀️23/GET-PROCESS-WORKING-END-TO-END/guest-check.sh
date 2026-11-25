#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
# Type-checks the guest crates this ticket edits (host target, metadata only) in the shared native build dir.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📦️packages/🦀️rust" || exit 1
cargo check -p semio-plugin-instance-docs -p semio-plugin-instance-ladders -p semio-plugin-time-travel-turns -p semio-plugin-instance-render -p semio-plugin-instance-app -p semio-plugin-instance-construction > $T/🗑️generated/guest-check.txt 2>&1; echo "exit $?" >> $T/🗑️generated/guest-check.txt

#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/📦️packages/🦀️rust" || exit 1
cargo test -p semio-plugin-document-media --lib -- stage_laws progress_laws > $T/🗑️generated/progress-law.txt 2>&1; echo "exit $?" >> $T/🗑️generated/progress-law.txt

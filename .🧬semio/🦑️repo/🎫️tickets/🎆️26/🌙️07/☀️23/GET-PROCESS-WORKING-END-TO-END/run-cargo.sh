#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-Vo4lE6/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/run
export PATH="$HOME/.bun/bin:$PATH"
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" || exit 1
L=$1; shift
cargo "$@" > "$T/🗑️generated/run-$L.txt" 2>&1; echo exit $?

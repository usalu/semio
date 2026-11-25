#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; G=$T/🗑️generated
export PATH="$HOME/.bun/bin:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" || exit 1
L=$1; shift
cargo "$@" > $G/replay-$L.txt 2>&1; echo exit $?

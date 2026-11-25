#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; G=$T/🗑️generated
export PATH="$HOME/.bun/bin:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"
D=$1; L=$2; shift 2
cd "$D" || exit 1
cargo "$@" > $G/carve-$L.txt 2>&1; echo exit $?

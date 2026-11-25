#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native/build-retire
mkdir -p $N
export PATH="$HOME/.bun/bin:$PATH"
cd "$1" || exit 1
L=$2; shift 2
cargo "$@" > $T/🗑️generated/retire-$L.txt 2>&1; echo exit $? >> $T/🗑️generated/retire-$L.txt

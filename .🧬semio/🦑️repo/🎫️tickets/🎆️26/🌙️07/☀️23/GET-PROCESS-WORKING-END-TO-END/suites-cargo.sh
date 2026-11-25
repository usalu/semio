#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native/build-ext
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process"
D=$1; L=$2; shift 2
cd "$P/$D" || exit 1
cargo "$@" > $T/🗑️generated/suites-$L.txt 2>&1; echo exit $? >> $T/🗑️generated/suites-$L.txt

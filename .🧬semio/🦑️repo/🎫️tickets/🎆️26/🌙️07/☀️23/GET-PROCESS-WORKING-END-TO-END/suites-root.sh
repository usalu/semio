#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native/build-ext
cd "${SUITES_CWD:-/Users/ueli/Documents/semio}" || exit 1
L=$1; shift
cargo "$@" > $T/🗑️generated/suites-$L.txt 2>&1; echo exit $? >> $T/🗑️generated/suites-$L.txt

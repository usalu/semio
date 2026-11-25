#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
# usage: guest-tests.sh <label> <package> [test filter...]: runs one guest test target in the shared native build dir (host target, dev profile).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
L=$1; P=$2; shift 2
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📦️packages/🦀️rust" || exit 1
cargo test -p $P -- "$@" > $T/🗑️generated/guest-test-$L.txt 2>&1; echo "exit $?" >> $T/🗑️generated/guest-test-$L.txt

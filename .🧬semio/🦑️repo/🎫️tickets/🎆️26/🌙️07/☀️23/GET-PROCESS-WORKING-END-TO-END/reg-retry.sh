#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
L=$1; shift
for i in 1 2 3 4 5 6 7 8; do
  $T/reg-cargo.sh $L "$@"
  grep -q "^error: could not compile\|^error\[" $T/🗑️generated/reg/$L.txt || break
  sleep 180
done
touch $T/🗑️generated/reg/$L.done

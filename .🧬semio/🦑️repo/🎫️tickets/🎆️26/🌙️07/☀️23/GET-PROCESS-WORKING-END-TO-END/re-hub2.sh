#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
H="/Users/ueli/Documents/semio/🌎️hub"
for crate in energy raster; do
  for i in 1 2 3 4 5 6; do
    $T/re-cargo.sh "$H" hubf-$crate test -p semio-hub-$crate-component --no-fail-fast
    grep -q "^error: could not compile\|^error\[" $N/re-hubf-$crate.txt || break
    sleep 120
  done
done
touch $N/re-hubf.done

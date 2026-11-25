#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
until [ -e $T/🗑️generated/reg/host6.done ]; do sleep 10; done
$T/reg-cargo.sh compr2 test -p semio-hub-raster-component --no-fail-fast
$T/reg-cargo.sh compe2 test -p semio-hub-energy-component --no-fail-fast
touch $T/🗑️generated/reg/comp2.done

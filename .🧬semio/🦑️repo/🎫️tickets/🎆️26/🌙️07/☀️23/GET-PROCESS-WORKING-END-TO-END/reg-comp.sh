#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
$T/reg-retry.sh compp test -p semio-hub-process-component --no-fail-fast
$T/reg-retry.sh compr test -p semio-hub-raster-component --no-fail-fast
$T/reg-retry.sh compe test -p semio-hub-energy-component --no-fail-fast
touch $T/🗑️generated/reg/comp-all.done

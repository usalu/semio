#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
until [[ -e $N/re2-queue.done ]]; do sleep 5; done
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"
$T/re2-cargo.sh "$R" rhost2 build --tests --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-mutations-pixels-tests
$T/re2-cargo.sh "$R" rmodel3 test -p semio-s-artifact-raster-raster-model --lib --no-fail-fast
touch $N/re2-queue2.done

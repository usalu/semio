#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
until [[ -e $N/re2-queue4.done ]]; do sleep 5; done
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"
E="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"
$T/re2-cargo.sh "$R" rmodel4 test -p semio-s-artifact-raster-raster-model --lib --no-fail-fast
$T/re2-cargo.sh "$R" rhost3 build --tests --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-mutations-pixels-tests
$T/re2-cargo.sh "$E" eview3 test -p semio-s-artifact-energy-model-viewer --lib --no-fail-fast
touch $N/re2-queue5.done

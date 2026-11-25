#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"
E="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"
$T/re2-cargo.sh "$E" eview4 test -p semio-s-artifact-energy-model-viewer --lib --no-fail-fast
$T/re2-cargo.sh "$E" efix3 build --tests -p semio-s-artifact-energy-model-mutation-fixtures
$T/re2-cargo.sh "$R" rmodel6 test -p semio-s-artifact-raster-raster-model --lib --no-fail-fast
$T/re2-cargo.sh "$R" rhost5 build --tests --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-mutations-pixels-tests
touch $N/re2-queue8.done

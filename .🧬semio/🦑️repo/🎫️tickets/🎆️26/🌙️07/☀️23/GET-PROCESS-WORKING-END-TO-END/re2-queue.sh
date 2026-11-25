#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"
E="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"
$T/re2-cargo.sh "$R" rmodel2 test -p semio-s-artifact-raster-raster-model --lib --no-fail-fast
$T/re2-cargo.sh "$R" rhost build --tests --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-mutations-pixels-tests
$T/re2-cargo.sh "$E" eview test -p semio-s-artifact-energy-model-viewer --lib --no-fail-fast
$T/re2-cargo.sh "$E" efix build --tests -p semio-s-artifact-energy-model-mutation-fixtures
touch "$T/🗑️generated/native/re2-queue.done"

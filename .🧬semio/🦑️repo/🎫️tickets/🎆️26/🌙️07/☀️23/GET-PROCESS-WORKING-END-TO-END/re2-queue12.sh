#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
until [[ -e $N/re2-queue11.done ]]; do sleep 5; done
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"
E="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"
$T/re2-cargo.sh "$R" rmodel8 test -p semio-s-artifact-raster-raster-model --lib --no-run
$T/re2-cargo.sh "$E" eview6 test -p semio-s-artifact-energy-model-viewer --lib --no-fail-fast
$T/re2-cargo.sh "$E" efix5 test -p semio-s-artifact-energy-model-mutation-fixtures --lib --test construction_vectors --test model_vectors --no-fail-fast
$T/re2-cargo.sh "$R" rhost6 build --tests --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-mutations-pixels-tests
touch $N/re2-queue12.done

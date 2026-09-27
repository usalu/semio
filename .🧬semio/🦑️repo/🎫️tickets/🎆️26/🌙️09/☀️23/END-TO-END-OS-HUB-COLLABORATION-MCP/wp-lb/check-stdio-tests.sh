#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
zsh .tmp-ticket/wp-lb/check-native.sh check-stdio-tests-1 --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-json -p semio-s-artifact-stdio-semio -p semio-s-artifact-stdio-deflate --lib --tests

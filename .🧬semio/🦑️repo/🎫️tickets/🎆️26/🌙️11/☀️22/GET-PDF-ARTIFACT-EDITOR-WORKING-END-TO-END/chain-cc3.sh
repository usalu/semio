#!/bin/sh
cd "$(dirname "$0")"
export NATIVE_CARGO_DIR=/Users/ueli/Documents/semio/🧰️framework
sh run-s4.sh cc-companion test -p semio-framework-value-tests --lib -- --test-threads=1 paged
sh run-s4.sh cc-pack2 test -p semio-framework-pack-record-encode-cursor --lib -- --test-threads=1

#!/bin/sh
# usage: run-s2-regression-libs.sh  (the library half of run-multi-regression.sh, one gated, watched cargo run; log in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
G=🗑️generated
P=semio-s-artifact-stdio-pdf
export WATCHDOG_TEST_SECONDS=1800
python3 watchdog-cargo.py sh ./native-cargo.sh test -p $P -p $P-io-export -p $P-mut-family-5 -p $P-mut-family-8 -p $P-mut-family-11 -p $P-mut-family-12 -p $P-mut-family-13 -p $P-mut-family-14 -p $P-mut-family-15 -p $P-page-editor-tests -p $P-page-editor-authoring-tests -p $P-native-save -p $P-testkit --lib > $G/s2-regression-libs.log 2>&1
echo "exit $?" > $G/s2-regression-libs.done

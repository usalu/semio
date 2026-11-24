#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=1500
cd "$(dirname "$0")"
P=semio-s-artifact-stdio-pdf
(time python3 watchdog-cargo.py sh ./native-cargo.sh test -p $P-testkit --test original_mutation_archive -- --test-threads=1) > 🗑️generated/round2-archive.log 2>&1
(time python3 watchdog-cargo.py sh ./native-cargo.sh test -p $P -p $P-io-export -p $P-mut-family-11 -p $P-mut-family-12 -p $P-mut-family-13 -p $P-mut-family-14 -p $P-mut-family-15 -p $P-mutation-round-trip-tests -p $P-page-editor-tests -p $P-page-editor-authoring-tests -p $P-presence-lane -p semio-s-artifact-stdio-pdf14-canvas -p $P-native-save --lib) > 🗑️generated/round2-regression.log 2>&1

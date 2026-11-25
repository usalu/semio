#!/bin/sh
# usage: run-media-in-save.sh <log name> [test filters...]  (one gated, watched run of the PDF editor unit tests; log and .done in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-900}
cd "$(dirname "$0")"
G=🗑️generated
name=$1; shift
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- "$@" --test-threads=1 --nocapture > $G/$name.log 2>&1
echo "exit $?" > $G/$name.done

#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=900
cd "$(dirname "$0")"
G=🗑️generated
while [ ! -f $G/final3.done ]; do sleep 10; done
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- an_out_of_range_page_edit --test-threads=1 --nocapture > $G/range.log 2>&1
echo done > $G/range.done

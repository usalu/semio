#!/bin/sh
# usage: run-s2.sh <log name>  (one gated, watched cargo test run over the slice-2 library crates; log in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-1200}
cd "$(dirname "$0")"
G=🗑️generated
P=semio-s-artifact-stdio-pdf
python3 watchdog-cargo.py sh ./native-cargo.sh test -p $P-testkit -p $P-mut-family-8 --lib -- controlled_real_world document_contracts page_content_contracts content_program codec set_text fault_notices --test-threads=1 > $G/$1.log 2>&1
echo "exit $?" > $G/$1.done

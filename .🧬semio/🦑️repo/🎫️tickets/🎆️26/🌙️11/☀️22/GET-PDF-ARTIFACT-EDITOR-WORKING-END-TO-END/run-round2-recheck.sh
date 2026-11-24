#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
P=semio-s-artifact-stdio-pdf
(time python3 watchdog-cargo.py sh ./native-cargo.sh test -p $P-testkit --lib --test original_mutation_archive -- exact_float_words a_details analyzer_to_builder original_archive --test-threads=1 --nocapture) > 🗑️generated/round2-recheck.log 2>&1

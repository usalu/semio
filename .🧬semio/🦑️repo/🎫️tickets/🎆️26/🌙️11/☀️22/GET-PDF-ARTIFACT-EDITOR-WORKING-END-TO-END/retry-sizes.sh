#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
i=0
while [ $i -lt 6 ]; do
  sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests -- pdf14::tests::debug_sizes14 --nocapture > 🗑️generated/sizes.log 2>&1
  strings 🗑️generated/sizes.log | grep -q "DEBUG\] sizes" && exit 0
  i=$((i+1)); sleep 180
done

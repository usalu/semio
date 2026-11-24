#!/bin/sh
# usage: run-retry.sh <name> <test filters...>; reruns while the shared tree does not compile for reasons outside this ticket
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-600}
cd "$(dirname "$0")"
G=🗑️generated
name=$1; shift
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
  python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- "$@" --test-threads=1 --nocapture > $G/$name.log 2>&1
  if grep -aq "^error\[E\|^error: could not compile" $G/$name.log && ! grep -aq "^running" $G/$name.log; then sleep 150; else break; fi
done
echo done > $G/$name.done

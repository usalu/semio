#!/bin/sh
# usage: run-s13.sh <log name> <cargo test args..>  (slice 13: one gated, watched cargo run, retried while a foreign crate does not compile and stopped at once when a PDF or natural-file crate does not; log and exit code in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-2400}
cd "$(dirname "$0")"
G=🗑️generated
mkdir -p $G/real-world
export SEMIO_PDF_REAL_WORLD_DIR="$PWD/$G/real-world" SEMIO_PDF_FONTS_DIR="$PWD/$G/real-world"
name=$1; shift
rm -f $G/$name.done
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
  python3 watchdog-cargo.py sh ./native-cargo.sh "$@" > $G/$name.log 2>&1
  status=$?
  grep -a -q "could not compile" $G/$name.log || break
  grep -a "could not compile" $G/$name.log | grep -a -q "semio-s-artifact-stdio-pdf\|semio-plugin-decl-natural-file\|natural-file" && break
  echo "foreign compile error, retry $attempt: $(grep -a 'could not compile' $G/$name.log | head -1)" >> $G/$name.retries
  sleep 240
done
echo "exit $status" > $G/$name.done

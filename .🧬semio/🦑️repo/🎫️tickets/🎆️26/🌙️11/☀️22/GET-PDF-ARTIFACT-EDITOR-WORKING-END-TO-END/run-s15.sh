#!/bin/sh
# usage: run-s15.sh <log name> <cargo test args..>  (slice 15: one gated, watched cargo test run; log and exit code in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-2400}
cd "$(dirname "$0")"
G=🗑️generated
mkdir -p $G/real-world
export SEMIO_PDF_REAL_WORLD_DIR="$PWD/$G/real-world" SEMIO_PDF_FONTS_DIR="$PWD/$G/real-world"
name=$1; shift
rm -f $G/$name.done
python3 watchdog-cargo.py sh ./native-cargo.sh "$@" > $G/$name.log 2>&1
echo "exit $?" > $G/$name.done

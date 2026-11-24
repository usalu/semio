#!/bin/sh
# usage: run-pdf17-tests.sh OUT cargo-args...
out=$1; shift
export PATH="$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH" RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
sh native-cargo.sh "$@" > "🗑️generated/$out" 2>&1
echo "done $?" >> "🗑️generated/$out"

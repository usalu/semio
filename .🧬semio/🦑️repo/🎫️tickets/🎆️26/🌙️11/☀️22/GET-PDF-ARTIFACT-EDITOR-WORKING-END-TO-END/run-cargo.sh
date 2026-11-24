#!/bin/sh
# usage: run-cargo.sh LOG cargo-args...
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
log=$1; shift
(time sh ./native-cargo.sh "$@") > "$log" 2>&1

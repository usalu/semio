#!/bin/sh
# usage: run-s14-prep.sh <log name> <test filters..>  (slice 14: retained-graph preparation laws of mut-family-11 under the importer accounting, one gated cargo run; log in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
G=🗑️generated
name=$1; shift
rm -f $G/$name.done
sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-mut-family-11 --lib -- "$@" --test-threads=1 --nocapture > $G/$name.log 2>&1
echo "exit $?" > $G/$name.done

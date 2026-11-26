#!/bin/sh
# usage: run-s15-check.sh <log name> <cargo check args..>  (slice 15: one gated cargo check over the PDF workspace; log and exit code in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
G=🗑️generated
name=$1; shift
rm -f $G/$name.done
sh ./native-cargo.sh check "$@" --message-format short > $G/$name.log 2>&1
echo "exit $?" > $G/$name.done

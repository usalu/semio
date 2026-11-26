#!/bin/sh
# usage: run-s14-codec.sh <log name>  (slice 14: gated lib tests of the Deflate codec, the retained Pack cursor and the replication facade under the speed tier; log in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
export NATIVE_CARGO_DIR="$root/🧰️framework"
G=🗑️generated
name=$1
rm -f $G/$name.done
sh ./native-cargo.sh test -p semio-framework-deflate -p semio-framework-pack-record-encode-cursor -p semio-framework-replication --lib --no-fail-fast -- --test-threads=2 > $G/$name.log 2>&1
echo "exit $?" > $G/$name.done

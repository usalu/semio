#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
G=🗑️generated
free=$(df -g /System/Volumes/Data | awk 'NR==2 {print $4}')
echo "free GB: $free" > $G/wasm-final2.log
[ "$free" -ge 12 ] || { echo "disk below 12 GB, not building" >> $G/wasm-final2.log; touch $G/wasm-final2.done; exit 1; }
sh mount-products.sh >> $G/wasm-final2.log 2>&1
sh rebuild-pdf-wasm.sh >> $G/wasm-final2.log 2>&1
echo "exit $?" >> $G/wasm-final2.log
touch $G/wasm-final2.done

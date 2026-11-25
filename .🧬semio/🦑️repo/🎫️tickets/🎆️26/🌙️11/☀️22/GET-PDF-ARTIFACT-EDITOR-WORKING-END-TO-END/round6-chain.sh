#!/bin/sh
# usage: round6-chain.sh <tag>  (the committed_descriptor run that writes the described pair, its settle into the committed owner, then the measured wasm build with stage; markers in 🗑️generated/round6-chain.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
G="$here/🗑️generated"
tag=${1:?tag}
bun=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
echo "descriptor $(date +%T)" >> "$G/round6-chain.log"
sh "$here/committed-descriptor-with-retries.sh" "${tag}d" || { echo "descriptor failed $(date +%T)" >> "$G/round6-chain.log"; exit 1; }
described=$(ls -td "$G"/native-target/lane-*/*/tmp/stdio-pdf-described 2>/dev/null | head -1)
[ -n "$described" ] || { echo "no described pair" >> "$G/round6-chain.log"; exit 1; }
(cd "$root" && "$bun" "$here/settle-pdf-descriptor.ts" "$described") >> "$G/round6-chain.log" 2>&1
echo "settled $(date +%T)" >> "$G/round6-chain.log"
sh "$here/build-with-retries.sh" "${tag}w" >> "$G/round6-chain.log" 2>&1
echo "wasm done $(date +%T)" >> "$G/round6-chain.log"

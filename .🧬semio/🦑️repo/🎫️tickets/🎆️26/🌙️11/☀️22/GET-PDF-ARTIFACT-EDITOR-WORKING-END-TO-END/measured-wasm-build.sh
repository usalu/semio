#!/bin/sh
# usage: measured-wasm-build.sh <tag>  (one gated rebuild-pdf-wasm.sh with the 20 s rustc memory sampler; 🗑️generated/wasm-<tag>.{out,mem.log,peaks.txt,done})
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
G="$here/🗑️generated"
tag=${1:?tag}
stop="$G/wasm-$tag.stop"
rm -f "$stop" "$G/wasm-$tag.done"
sh "$here/sample-rustc-memory.sh" "$G/wasm-$tag.mem.log" "$stop" &
sampler=$!
status=0
sh "$here/rebuild-pdf-wasm.sh" > "$G/wasm-$tag.out" 2>&1 || status=$?
touch "$stop"
wait "$sampler" 2>/dev/null || true
python3 "$here/peak-rustc-memory.py" "$G/wasm-$tag.mem.log" > "$G/wasm-$tag.peaks.txt"
echo "exit $status" > "$G/wasm-$tag.done"

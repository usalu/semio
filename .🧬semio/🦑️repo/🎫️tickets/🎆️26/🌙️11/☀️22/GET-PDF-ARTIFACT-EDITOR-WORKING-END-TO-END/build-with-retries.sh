#!/bin/sh
# usage: build-with-retries.sh <tag>  (up to three measured wasm builds <tag>1..3, each after at most a 2-minute quiet wait, 5 minutes apart after a failure; stops at the first green build)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
G="$here/🗑️generated"
tag=${1:?tag}
for attempt in 1 2 3; do
  timeout 120 sh "$here/await-quiet-tree.sh" 2 || true
  sh "$here/measured-wasm-build.sh" "$tag$attempt"
  grep -q "exit 0" "$G/wasm-$tag$attempt.done" && { echo "green $tag$attempt"; exit 0; }
  grep -a "^error" -A3 "$G/wasm-$tag$attempt.out" | grep -a -- "-->" | head -3
  [ "$attempt" = 3 ] || sleep 300
done
exit 1

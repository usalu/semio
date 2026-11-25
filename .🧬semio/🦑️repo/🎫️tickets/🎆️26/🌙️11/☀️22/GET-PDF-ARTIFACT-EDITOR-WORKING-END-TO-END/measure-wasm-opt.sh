#!/bin/sh
# usage: measure-wasm-opt.sh <tag> <input.core.wasm> [passes…]  (runs the repository binaryen `wasm-opt` with the given passes, default `-Oz`, on one core module; samples its footprint
# every 2 s and stops it above 6 GB; writes 🗑️generated/wasmopt/<tag>.{wasm,log,size.json,done})
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}; input=${2:?input}; shift 2
[ $# -gt 0 ] || set -- -Oz
out="$here/🗑️generated/wasmopt"
mkdir -p "$out"
opt=$(ls "$root"/.cache/b/t/m/binaryen/*/*/bin/wasm-opt | head -1)
start=$(date +%s)
"$opt" --all-features "$@" "$input" -o "$out/$tag.wasm" > "$out/$tag.log" 2>&1 &
pid=$!
peak=0
while kill -0 "$pid" 2>/dev/null; do
  mb=$(top -l 1 -pid "$pid" -stats pid,mem 2>/dev/null | awk -v p="$pid" '$1==p { v=$2; n=v+0; if (v ~ /G/) n*=1024; else if (v ~ /K/) n/=1024; printf "%d", n }')
  [ -n "$mb" ] && [ "$mb" -gt "$peak" ] && peak=$mb
  if [ -n "$mb" ] && [ "$mb" -gt 6144 ]; then echo "stopped at ${mb}M" >> "$out/$tag.log"; kill -9 "$pid"; fi
  sleep 2
done
wait "$pid"; status=$?
end=$(date +%s)
bun=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
[ -s "$out/$tag.wasm" ] && "$bun" "$here/wasm-code-size.mjs" "$out/$tag.wasm" > "$out/$tag.size.json"
echo "exit $status seconds $((end - start)) peakMB $peak passes $*" > "$out/$tag.done"

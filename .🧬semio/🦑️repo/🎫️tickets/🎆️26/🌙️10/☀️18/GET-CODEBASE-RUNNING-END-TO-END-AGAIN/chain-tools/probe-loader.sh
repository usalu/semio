#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
A="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs"
last=""
while true; do
  cur="$(stat -f '%m' "$A" "$B" | tr '\n' ' ')"
  if [ "$cur" != "$last" ]; then
    last="$cur"; sleep 120
    out=$(SEMIO_BUILD_GRAPH_POLICY=current bun repo --tool=bazel show projects --json 2>&1 | tail -c 4000)
    if print -r -- "$out" | grep -a -q "not found in module"; then print -r -- "$(date +%H:%M) still broken"; else print -r -- "$(date +%H:%M) LOADER OK"; print -r -- "$out" | grep -a -v '^\[script\]' | tail -3; exit 0; fi
  fi
  sleep 60
done

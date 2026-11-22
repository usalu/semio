#!/bin/sh
# 👀 Pollarded London plane author: the product's 2D drawings of the candidate document at the scales named as arguments, reduced to 1700 px.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-london-plane-pollarded/london-plane-pollarded"
for scale in "$@"; do
  out=$("$BUN" r3-2d.ts --file "$G/🔣️.json" --scale "$scale" 2>&1 | tail -1)
  [ -f "$out" ] && sips -Z 1700 "$out" --out "$G/d$scale.png" >/dev/null 2>&1 && rm -f "$out" && echo "$G/d$scale.png"
done

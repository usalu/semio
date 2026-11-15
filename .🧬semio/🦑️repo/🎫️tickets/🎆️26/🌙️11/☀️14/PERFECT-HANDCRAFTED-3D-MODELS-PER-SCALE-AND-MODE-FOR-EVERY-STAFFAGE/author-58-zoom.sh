#!/bin/bash
# usage: author-58-zoom.sh slug scale  -> crops the elevation panel of the render at 3x
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-render.ts vehicles/$1 --scale $2 --width 4800 >/dev/null 2>&1
F=🗑️generated/volumes-tools/render/vehicles/$1.s$2.png
sips -c ${4:-990} ${3:-1600} --cropOffset ${5:-0} ${6:-0} $F --out 🗑️generated/author-58/zoom.png >/dev/null

#!/bin/bash
source /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/boot1113/env.sh
set -e
rm -rf "$W"; cp -R "$S/base" "$W"
cp "$S/patch/files/script.ts" "$W/🔁️replica/📜️script.ts"
for p in "$S"/patch/p[0-9]*.py; do echo "== $(basename $p)"; python3 "$p" "$W"; done
echo applied

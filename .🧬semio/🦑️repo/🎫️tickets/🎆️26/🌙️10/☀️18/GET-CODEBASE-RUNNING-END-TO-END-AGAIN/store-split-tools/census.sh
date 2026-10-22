#!/bin/sh
# usage: census.sh <dir>  -> per crate closure size
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
find "$1" -name Cargo.toml -path '*📦️packages/🦀️rust*' | while read m; do
  d=$(dirname "$m")
  lib=$(/usr/bin/grep -A3 '^\[lib\]' "$m" | /usr/bin/grep '^path' | sed 's/.*"\(.*\)".*/\1/')
  [ -z "$lib" ] && continue
  (cd "$d" && p=$(cd "$(dirname "$lib")" && pwd)/$(basename "$lib"); cd /Users/ueli/Documents/semio; p=${p#/Users/ueli/Documents/semio/}; bun "$C/store-split-tools/closure.ts" "$p" 2>&1 | head -1)
done

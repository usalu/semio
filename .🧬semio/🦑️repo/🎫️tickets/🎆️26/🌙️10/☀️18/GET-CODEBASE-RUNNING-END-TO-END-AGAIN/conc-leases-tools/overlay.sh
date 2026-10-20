#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
ROOT=/Users/ueli/Documents/semio
OV=${1:-/private/tmp/conc-leases/ov}
F="🧰️framework"; M="$F/🔨️modules"
L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"
rm -rf "$OV"; mkdir -p "$OV"
mirror() { # mirror <relative dir> <real entry to keep out>
  mkdir -p "$OV/$1"
  for e in "$ROOT/$1"/* "$ROOT/$1"/.[!.]*; do [ -e "$e" ] || continue; n=$(basename "$e"); [ "$n" = "$2" ] || ln -s "$e" "$OV/$1/$n"; done
}
mirror "$F" "🔨️modules"; rm -f "$OV/$F/🛍️products"
mirror "$M" "🏃️process"
cp -R "$ROOT/$M/🏃️process" "$OV/$M/🏃️process"
mirror "$F/🛍️products" "🦑️repo"
mirror "$F/🛍️products/🦑️repo" "🔨️modules"
mirror "$F/🛍️products/🦑️repo/🔨️modules" "📚️library"
mirror "$L" "🏗️build"
mkdir -p "$OV/$L/🏗️build"
for e in "$ROOT/$L/🏗️build"/*; do n=$(basename "$e"); case "$n" in "⚙️operations"|"📜️script.ts") ;; *) ln -s "$e" "$OV/$L/🏗️build/$n";; esac; done
cp -R "$ROOT/$L/🏗️build/⚙️operations" "$OV/$L/🏗️build/⚙️operations"
cp "$ROOT/$L/🏗️build/📜️script.ts" "$OV/$L/🏗️build/📜️script.ts"
ln -s "$ROOT/node_modules" "$OV/node_modules"
cp "$ROOT/package.json" "$ROOT/tsconfig.json" "$OV/" 2>/dev/null || true
cp -R "$T/staged/." "$OV/" 
mkdir -p "$OV/.cache/b/c" "$OV/gen"
echo "$OV"

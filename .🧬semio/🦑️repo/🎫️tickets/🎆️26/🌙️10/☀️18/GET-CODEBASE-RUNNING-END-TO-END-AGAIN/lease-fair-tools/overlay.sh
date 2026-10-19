#!/bin/sh
set -e
ROOT=/Users/ueli/Documents/semio
OV=${1:-/private/tmp/claude-501/lease-fair-ov}
F="🧰️framework"; M="$F/🔨️modules"
rm -rf "$OV"; mkdir -p "$OV/$M"
for e in "$ROOT/$F"/*; do n=$(basename "$e"); [ "$n" = "🔨️modules" ] || ln -s "$e" "$OV/$F/$n"; done
for e in "$ROOT/$M"/*; do n=$(basename "$e"); [ "$n" = "🏃️process" ] || ln -s "$e" "$OV/$M/$n"; done
cp -R "$ROOT/$M/🏃️process" "$OV/$M/🏃️process"
ln -s "$ROOT/node_modules" "$OV/node_modules"
echo "$OV"

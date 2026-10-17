#!/bin/zsh
# usage: census.sh <tag> <names comma>
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
ONLY="${2:-semio-hub-artifact-creation,semio-hub-stdio,semio-hub-directory-laws,semio-hub-directory-backend-laws,semio-s-composition-laws,semio-s-artifact-procedural-2d-io-sqlite,semio-s-artifact-procedural-3d-io-sqlite}" nice -n 10 bun "$F/tc4-tools/tc4-cgc7.ts" > "$F/🗑️generated/tc4-census-$1.log" 2>"$F/🗑️generated/tc4-census-$1.err"
cp "$F/🗑️generated/tc4-cgc7-crate-census.csv" "$F/🗑️generated/tc4-census-$1.csv"
echo done $?

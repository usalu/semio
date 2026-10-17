#!/bin/zsh
# usage: census.sh <tag> [names comma]
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
ONLY="${2:-semio-framework-expression,semio-framework-time-travel,semio-framework-plugin-child-group-wire-laws,semio-framework-plugin-publication-mounted,semio-framework-3d-brep-boolean,semio-framework-3d-mesh-tests,semio-framework-3d-mesh-document,semio-framework-os-kernel,semio-framework-number,teaching-proctor,semio-framework-os-shell}" nice -n 10 bun "$F/tc3-tools/tc3-cgc7.ts" > "$F/🗑️generated/tc3-census-$1.log" 2>"$F/🗑️generated/tc3-census-$1.err"
cp "$F/🗑️generated/tc3-cgc7-crate-census.csv" "$F/🗑️generated/tc3-census-$1.csv"
echo done $?

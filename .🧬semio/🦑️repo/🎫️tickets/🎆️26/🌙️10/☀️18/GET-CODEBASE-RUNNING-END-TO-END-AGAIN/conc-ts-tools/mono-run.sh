#!/bin/sh
# usage: mono-run.sh <tsconfig> <log> [cwd]   one tsc, 3 GB heap, prints wall + max RSS
TSC=/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc
cd "${3:-/Users/ueli/Documents/semio}" || exit 1
/usr/bin/time -l nice -n 10 node --max-old-space-size=3072 "$TSC" -p "$1" --pretty false --extendedDiagnostics > "$2" 2>&1
echo "exit $?" >> "$2"

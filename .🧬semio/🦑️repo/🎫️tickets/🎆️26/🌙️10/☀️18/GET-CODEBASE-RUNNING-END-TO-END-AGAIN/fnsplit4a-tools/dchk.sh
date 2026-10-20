#!/bin/zsh
# usage: dchk.sh <rootFileRel> <snapshotFile> [nthDescribe] : verify-expand + tsdiff + shape + fnlen for a dsplit result (snapshot is an absolute path)
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio; setopt nullglob
D="${1%/*}"
node $C/fnsplit2-tools/verify-expand.cjs "$1" "$2" "${3:-0}" 2>&1 | tail -3
if [[ "$1" == *.tsx ]]; then export TSDIFF_REACT=1; fi
if [[ -n "$4" ]]; then node $T/tsdiff.cjs "$1" "$2" --spec "$4" 2>&1; else node $T/tsdiff.cjs "$1" "$2" "$D"/*/🟦️.ts "$D"/*/🟦️.tsx 2>&1; fi | tail -4 | cut -c1-300
$C/fnsplit2-tools/shape.sh "$D" 2>&1 | grep -v "^RGI\|error:\|^$\|Bun v\|ok$" | tail -8 | cut -c1-200
node $C/fnsplit3-tools/fnlen.cjs "$1" "$D"/*/🟦️.ts "$D"/*/🟦️.tsx 2>&1 | sort -n | tail -2

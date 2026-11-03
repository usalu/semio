#!/bin/bash
# usage: go.sh <asset-id> <author-script-suffix>   -> author, then verify (ranges with RANGES=1)
cd /c/git/semio || exit 1
export PYTHONUTF8=1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
OUT="$T/🗑️generated/M1"
mkdir -p "$OUT"
id="$1"
script="$2"
if [ -n "$script" ]; then python "$T/🧰️M1/$script" || exit 1; fi
repo tools exec bun -- "$T/🧰️M1/verify.ts" "$id" > "$OUT/$id-verify.txt" 2>&1
echo "done $id" >> "$OUT/$id-verify.txt"

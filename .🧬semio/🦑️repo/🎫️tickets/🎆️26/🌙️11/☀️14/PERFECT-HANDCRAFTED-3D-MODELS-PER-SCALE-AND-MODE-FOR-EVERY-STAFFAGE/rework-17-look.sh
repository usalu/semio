#!/bin/sh
# 👀 Author 17: render sheets and keep a reduced copy of each (1300 px) under 🗑️generated/rework-17. Usage: rework-17-look.sh <slug> "<r3-sheet args>" ["<r3-sheet args>" ...]; SIZE=2400 keeps the full size.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; shift
G="🗑️generated/rework-17"; mkdir -p "$G"
for a in "$@"; do
  f=$($BUN r3-sheet.ts "appliances/$slug" $a 2>/dev/null | tail -1)
  o="$G/$(basename "$f")"
  sips -Z ${SIZE:-1300} "$f" --out "$o" >/dev/null 2>&1
  rm -f "$f"
  echo "$o"
done

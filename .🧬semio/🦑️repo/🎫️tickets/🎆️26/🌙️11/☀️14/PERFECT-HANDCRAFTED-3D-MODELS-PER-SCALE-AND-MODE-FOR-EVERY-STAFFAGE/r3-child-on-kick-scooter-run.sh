#!/bin/sh
# 🔁 Rebuilds the candidate document of humans/child-on-kick-scooter (roots, volumes, 2D) and validates it.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-child-on-kick-scooter"
python3 r3-child-on-kick-scooter-root.py > "$G/root.log"
python3 r3-child-on-kick-scooter-build.py
python3 r3-child-on-kick-scooter-draw2d.py
"$BUN" r3-validate.ts --file "$G/🔣️.json" > "$G/val.txt" || true
tail -1 "$G/val.txt"

#!/bin/sh
# 📸 Renders r3-sheet for humans/jogger once per argument string: rework-jogger-shots.sh "--scale 50 --pose stretch-stand" "--scale 50 --list structure" ...
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T" || exit 1
for a in "$@"; do
  "$BUN" r3-sheet.ts humans/jogger $a 2>&1 | tail -1 | sed 's|.*/||'
done

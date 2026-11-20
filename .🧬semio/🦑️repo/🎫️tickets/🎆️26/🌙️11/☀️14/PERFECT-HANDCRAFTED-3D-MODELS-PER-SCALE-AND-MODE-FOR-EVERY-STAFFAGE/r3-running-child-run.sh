#!/bin/sh
# 🔁 Rebuilds the candidate of humans/running-child: roots, volumes, 2D geometry (when the drawing script exists), then validates it.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
[ -f "🗑️generated/r3-running-child/🔣️.json" ] || python3 r3-running-child-roots.py --candidate >/dev/null
"$BUN" r3-running-child-figure.ts || exit 1
python3 r3-running-child-apply.py --candidate >/dev/null || exit 1
if [ -f r3-running-child-draw2d.ts ] && [ "$1" != "--no2d" ]; then
  "$BUN" r3-running-child-draw2d.ts || exit 1
  python3 r3-running-child-apply.py --candidate >/dev/null || exit 1
fi
"$BUN" r3-validate.ts --file "🗑️generated/r3-running-child/🔣️.json" | cut -c1-400 | tail -${2:-12}

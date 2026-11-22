#!/bin/sh
# Render the 2D drawings and the shaded 3D sheets of one PW-02 tree with the round-3 tools (one tool process at a time).
# usage: look.sh <id> <scale...>   (env LIST=body|thermal|structure, POSE=<id>, VIEWS=light-line,light-solid)
set -u
ID="$1"; shift
HERE="$(cd "$(dirname "$0")" && pwd)"
P="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
C="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$ID/🔣️.json"
cd "$P"
for S in "$@"; do
  if [ -n "${POSE:-}" ]; then
    repo tools exec bun -- r3-2d.ts --file "$C" --scale "$S" --pose "$POSE" 2>&1 | grep -v "^\[capacity\]\|still working\|waits for" | tail -2
  else
    repo tools exec bun -- r3-2d.ts --file "$C" --scale "$S" 2>&1 | grep -v "^\[capacity\]\|still working\|waits for" | tail -2
  fi
  repo tools exec bun -- r3-sheet.ts --file "$C" --scale "$S" --list "${LIST:-body}" 2>&1 | grep -v "^\[capacity\]\|still working\|waits for" | tail -2
done

#!/bin/bash
# 🌳 Round 5 london plane: model -> 2D redraw -> candidate asset document (roots + geometry + anchors + volumes) -> validation -> 2D sheets of the candidate. `--write` then writes the catalog file.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C=$(ls -d ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/london-plane-pollarded)/🔣️.json
G="🗑️generated/r5-london-plane-pollarded"
A="vegetation/london-plane-pollarded"
AREAS=(); [ -f r3-london-plane-pollarded-areas.json ] && AREAS=(--areas r3-london-plane-pollarded-areas.json)
$BUN r5-london-plane-pollarded-model.ts || exit 1
$BUN r5-london-plane-pollarded-draw.ts || exit 1
python3 r3-london-plane-pollarded-apply.py "$C" "$G/🔣️.json" --geometry "$G/geometry.json" --anchors "$G/anchors.json" "${AREAS[@]}" || exit 1
python3 volumes-insert.py "volumes/$A.json" "$G/🔣️.json" | tail -1
$BUN r3-validate.ts --file "$G/🔣️.json" > "$G/validate.txt"; OK=$?
tail -12 "$G/validate.txt" | cut -c1-300
if [ "$1" = "--write" ] && [ $OK -eq 0 ]; then
  python3 r3-london-plane-pollarded-apply.py "$C" "$C" --geometry "$G/geometry.json" --anchors "$G/anchors.json" "${AREAS[@]}" && python3 volumes-insert.py "volumes/$A.json" "$C" | tail -1
fi

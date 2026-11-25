#!/bin/zsh
# 🖼️ Product sheets of catalog files inside a grant that is already held: catalog-sheets.zsh <label> <pose> <id>… writes $GEN/sheets/c.<id>.s<scale>.<pose>.<label>.png for the four authored scales with the r3-2d tool of the 3D ticket (side, front, plan; line and solid).
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
label="$1"; pose="$2"; shift 2
for a in "$@"; do work="$GEN/sheets/work-$label/$a"; mkdir -p "$work"; cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$work/🔣️.json"
  for s in 50 100 200 500; do png="$(cd "$T14" && timeout 600 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
    [[ -f "$png" ]] && mv "$png" "$GEN/sheets/c.$a.s$s.$pose.$label.png" && echo "c.$a.s$s.$pose.$label.png" || echo "no sheet: $png"; done
  rm -rf "$GEN/sheets/work-$label"; done

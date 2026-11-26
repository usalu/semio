#!/bin/zsh
# 👀️ Draws product sheets of one asset document inside the grant of the worker: `sheet.sh <tag> <file.json> <id> <scale>:<pose>…` renders side, front and plan (line and solid row) with `r3-2d.ts` of the 3D ticket into `$GEN/sheets/<id>.s<scale>.<pose>.<tag>.png`; with ZOOM=1 also the three line panels alone at 1300 pixels (`../MV-L1/zoom.ts`) as `…<tag>.<view>.png`.
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; file="$2"; a="$3"; shift 3; out="$GEN/sheets"; work="$GEN/sheets/work-$tag-$$/$a"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"; cd "$T14"
for item in "$@"; do
  s="${item%%:*}"; pose="${item#*:}"
  png="$(timeout 900 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
  if [[ -n "$ZOOM" ]]; then for v in elevation front plan; do DENSITY=${DENSITY:-500} timeout 900 "$B" "$U/🧰️matrix/MV-L1/zoom.ts" "$work/🔣️.json" "$s" "$pose" "$v" "$out/$a.s$s.$pose.$tag.$v.png" 1300 | tail -1; done; fi
done
rm -rf "$GEN/sheets/work-$tag-$$"

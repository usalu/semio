#!/bin/zsh
# 🖼️ Product sheets (side, front, plan) of asset documents at every authored scale in the default pose and, for the first and last scale, in the poses of POSES, into `$GEN/sheets/<id>.s<scale>.<pose>.<tag>.png`; start it through the memory queue (800 MB): sheets.sh <tag>:<file.json>…   (the tag names the state: before, fit, nest, after)
BATCH=MA source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
mkdir -p "$GEN/sheets"
for pair in "$@"; do
  tag="${pair%%:*}"; src="${pair#*:}"
  a="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['id'])" "$src")"
  scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$src"))
  rest="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$src")"
  work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"; cp "$src" "$work/🔣️.json"
  for pose in $rest ${=POSES}; do
    for s in $scales; do
      [[ "$pose" != "$rest" && "$s" != "$scales[1]" && "$s" != "$scales[-1]" ]] && continue
      png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
      [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$a.s$s.$pose.$tag.png" && echo "$GEN/sheets/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
    done
  done
  rm -rf "$GEN/sheets/work-$tag"
done

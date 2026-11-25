#!/bin/zsh
# 🖼️ Renders several product 2D sheets (side, front, plan; line and solid view) of one asset under ONE grant of the memory queue (2500 MB), each given as <tag>:<pose|default>:<scale> with tag `before` (the backup), `try` (the trial copy) or `after` (the catalog file), into $GEN/sheets/<id>.s<scale>.<pose>.<tag>.png: sheets.sh <id> before:default:50 try:default:50 …   (inside the grant the script calls itself with INSIDE=1)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
if [[ -z "$INSIDE" ]]; then INSIDE=1 exec "$W/q.sh" ${SHEETS_MB:-800} ${SHEETS_TIMEOUT:-7200} "$T14" "${0:A}" "$@"; fi
cd "$T14"; a="$1"; shift; out="$GEN/sheets"; mkdir -p "$out"
for spec in "$@"; do
  tag="${spec%%:*}"; rest="${spec#*:}"; pose="${rest%%:*}"; s="${rest#*:}"
  work="$out/work-$tag/$a"; mkdir -p "$work"
  if [[ "$tag" == before ]]; then cp "$GEN/backup/$a.json" "$work/🔣️.json"; elif [[ "$tag" == try ]]; then cp "$GEN/try/$a.json" "$work/🔣️.json"; else cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$work/🔣️.json"; fi
  [[ "$pose" == default ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")"
  png="$("$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
  rm -rf "$out/work-$tag"
done

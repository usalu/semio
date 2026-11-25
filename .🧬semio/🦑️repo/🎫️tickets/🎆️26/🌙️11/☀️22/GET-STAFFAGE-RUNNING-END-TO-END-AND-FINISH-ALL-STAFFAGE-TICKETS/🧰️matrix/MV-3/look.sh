#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an asset with the r3-2d tool of the 3D ticket into $GEN/sheets, from the catalog file (TAG=after, default), from the backup `$GEN/backup/<id>.json` (TAG=before) or from the trial copy `$GEN/try/<id>.json` (TAG=try), in the default pose or POSE=<id>: look.sh <id> <scale>…   (copy of ../MR/look.sh with the trial source; the render runs through the memory queue `q.sh`, 2500 MB)
BATCH=MV-3 source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
a="$1"; shift; tag="${TAG:-after}"
out="$GEN/sheets"; work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"
case "$tag" in before) cp "$GEN/backup/$a.json" "$work/🔣️.json";; try) cp "$GEN/try/$a.json" "$work/🔣️.json";; *) cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$work/🔣️.json";; esac
pose="${POSE:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")}"
for s in "$@"; do
  png="$(q 2500 "$T14" 600 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
done
rm -rf "$GEN/sheets/work-$tag"

#!/bin/zsh
# 🔍️ The product sheet of a trial copy beside the backup: renders $GEN/try/<name>.json (TAG=after) and $GEN/backup/<id>.json (TAG=before, once) at a scale in the default pose or POSE=<id> into $GEN/sheets as <name>.s<scale>.<pose>.{before,after}.png and writes the diff image (`sheetdiff.py`): look-file.sh <id> <name> <scale>…
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
a="$1"; n="$2"; shift 2; out="$GEN/sheets"; mkdir -p "$out"
pose="${POSE:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$GEN/backup/$a.json")}"
for s in "$@"; do
  for tag in before after; do
    src="$GEN/backup/$a.json"; [[ $tag == after ]] && src="$GEN/try/$n.json"
    work="$out/work-$n-$tag/$a"; mkdir -p "$work"; cp "$src" "$work/🔣️.json"
    png="$("$W/q.sh" 1000 300 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
    [[ -f "$png" ]] && mv "$png" "$out/$n.s$s.$pose.$tag.png" || echo "no sheet: $png"
    rm -rf "$out/work-$n-$tag"
  done
done
"$R/.venv/bin/python3" -W ignore "$W/sheetdiff.py" "$out" "$n"

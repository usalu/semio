#!/bin/zsh
# 🔬️ Which ops move the silhouette guard of a scale: bisect.sh <id> <scale>   applies, per shape id that the ops of the scale name, only the ops of that id (and of the other scales) to the backup and prints the guard line of the scale for each, after the line of the backup and of all ops; a view that leaves its limit with one id alone names the op to replace.
source "${0:A:h}/env.sh"
a="$1"; sc="$2"; d="$GEN/bisect/$a-$sc"; rm -rf "$d"; mkdir -p "$d"
python3 - "$W/ops/$a.json" "$sc" "$d" <<'PY'
import json, sys
ops = json.load(open(sys.argv[1])); sc = sys.argv[2]; d = sys.argv[3]
ids = []
for x in ops:
    if x.get("scale") == sc and x["id"] not in ids: ids.append(x["id"])
json.dump(ops, open(f"{d}/all.ops", "w"))
for i in ids: json.dump([x for x in ops if x.get("scale") != sc or x["id"] == i], open(f"{d}/only-{i}.ops", "w"))
PY
files=("$GEN/backup/$a.json")
for o in "$d"/*.ops; do "$W/cap.sh" 1000 300 "$T" "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$o" --out "${o%.ops}.json" >/dev/null; files+=("${o%.ops}.json"); done
("$W/cap.sh" 2500 3600 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" $files --scales "$sc" 2>&1 | grep bbox | cut -c1-230) | paste -d' ' <(printf '%s\n' ${files:t:r}) - | sed "s/$a 1:$sc bbox [^ ]* [^ ]* [^ ]* //"
rm -rf "$d"

#!/bin/zsh
# 🧫️ Which ops cost the silhouette: fits a subset of `ops/<id>.json` (the ops of one scale whose id matches a regex) onto the backup and prints the silhouette guard of that scale: subset.sh <id> <scale> <id-regex> [<op-regex>]
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
a="$1"; t="$GEN/try/$a.subset.json"; o="$GEN/try/$a.subset.ops.json"; cp "$GEN/backup/$a.json" "$t"
python3 -c "
import json,re,sys
ops=[x for x in json.load(open(sys.argv[1])) if x.get('scale')==sys.argv[3] and re.search(sys.argv[4],x['id']) and re.search(sys.argv[5],x['op']+'/'+x.get('view',''))]
json.dump(ops,open(sys.argv[2],'w')); print(len(ops),'ops')" "$W/ops/$a.json" "$o" "$2" "$3" "${4:-.}"
for i in 1 2 3; do ("$W/q.sh" 1000 600 "$T" "$B" "$W/fit2.ts" "$t" --ops "$o" --out "$t") | grep -E "REFUSED" | cut -c1-200; done
("$W/q.sh" 2500 900 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$t" --scales "$2") | grep -E "bbox" | cut -c1-220

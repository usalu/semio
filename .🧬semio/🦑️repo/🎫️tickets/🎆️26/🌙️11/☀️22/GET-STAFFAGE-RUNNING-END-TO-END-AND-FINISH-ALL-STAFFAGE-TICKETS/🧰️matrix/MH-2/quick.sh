#!/bin/zsh
# ⚡️ A quick look at one variant: fits `ops/<id>.json` (or OPS=<file>) onto the backup into $GEN/try/<id>.json, prints the shapes and anchors of the bones matching <bone-regex> in one subject (`who.ts`) and the silhouette guard of the scales given: quick.sh <id> <scales,…> "<subject id>" <bone-regex>
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
a="$1"; t="$GEN/try/$a.json"; cp "$GEN/backup/$a.json" "$t"
[[ -f "$W/anchors/$a.json" ]] && python3 "$W/split-anchors.py" "$t" "$W/anchors/$a.json" --write > /dev/null
("$W/q.sh" 1000 600 "$T" "$B" "$W/fit2.ts" "$t" --ops "${OPS:-$W/ops/$a.json}" --out "$t") | grep -E "REFUSED|ops," | cut -c1-300
for s in ${(s:,:)2}; do ("$W/q.sh" 1000 600 "$T" "$B" "$W/who.ts" "$t" "$s" "$3" ${4:+"$4"}) | sed "s/^/1:$s /"; done
[[ -n "$NOMEASURE" ]] || ("$W/q.sh" 2500 900 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$t" --scales "$2") | grep -E "bbox|silhouette"

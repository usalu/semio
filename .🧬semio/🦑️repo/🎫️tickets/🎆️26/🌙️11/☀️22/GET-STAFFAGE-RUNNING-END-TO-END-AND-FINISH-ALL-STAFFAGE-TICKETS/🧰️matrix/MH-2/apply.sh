#!/bin/zsh
# 🖊️ Writes the repair of an asset into the catalog, the last step of its chain: first splits the lumped contact anchors named in `anchors/<id>.json` if the asset has one (`split-anchors.py`, D45), then runs `fit2.ts` with `ops/<id>.json` on the catalog file until a run changes nothing (at most five runs; `fit2.ts` refuses to write when the file changed on disk while it ran), then, where `plans/<id>.json` exists, builds the solids it names from the volume (`vol.ts`, every entry with its `make`; it runs after the ops and replaces what they made of those shapes, so a second pass of the whole script must again end unchanged: the ops whose shapes a plan entry rebuilds are not in `ops/<id>.json`). Idempotent: on a repaired file every step reports that nothing changed. apply.sh <id>
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
a="$1"; mkdir -p "$GEN/apply"
[[ -f "$W/anchors/$a.json" ]] && python3 "$W/split-anchors.py" "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$W/anchors/$a.json" --write
for i in 1 2 3 4 5; do
  ("$W/q.sh" 1000 600 "$T" "$B" "$W/fit2.ts" "$a" --ops "$W/ops/$a.json" --write) > "$GEN/apply/$a.$i.log" 2>&1
  echo "run $i: $(tail -2 "$GEN/apply/$a.$i.log" | tr '\n' ' ' | cut -c1-300)"
  grep -q " 0 changed something" "$GEN/apply/$a.$i.log" && break
done
if [[ -f "$W/plans/$a.json" ]]; then
  ("$W/q.sh" 1000 900 "$T" "$B" "$W/vol.ts" "$a" --plan "$W/plans/$a.json" --write) > "$GEN/apply/$a.vol.log" 2>&1
  echo "vol: $(tail -2 "$GEN/apply/$a.vol.log" | tr '\n' ' ' | cut -c1-300)"
fi

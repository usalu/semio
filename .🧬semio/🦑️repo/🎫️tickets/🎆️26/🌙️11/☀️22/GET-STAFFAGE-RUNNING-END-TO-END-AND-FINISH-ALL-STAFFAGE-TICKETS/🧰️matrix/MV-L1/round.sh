#!/bin/zsh
# 🔁️ One search round of batch MV-L1 inside ONE reservation of the memory queue (the queue of the machine is long, so everything a look at the drawings needs runs behind one request), nothing in the catalog is written: for every plant `body.ts` (search, ops saved to `ops/<id>.json`, result in `$GEN/try/<id>.json`), the anchor search of batch MV-1 on that copy (`ops/<id>-anchors.json`), the gate over every subject, the guard, validator and sweep outside the test runner (`../MR/measure.ts`: backup, then copy) and the measurement of the schema's solids (`solids.ts`); with LOFT=<id> the loft of that plant as evidence (`loft.ts` into `$GEN/try/<id>-loft.json`, then `measure.ts` on it); then the large panels of ZOOMS=<list> and the before and after sheets of PAIRS=<list> (`pairs.sh` format, drawn from the copies, the side elevation cut off with CROP=1). round.sh <megabytes> <id>…
source "${0:A:h}/env.sh"
mb="$1"; shift; mkdir -p "$GEN/try" "$GEN/log" "$GEN/sheets" "$W/ops"; export MEMLOG="$GEN/log/mem-round.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c '
for a in "$@"; do
  "$W/q.sh" 0 "$T" 3000 "$B" "$W/body.ts" "$a" --save "$W/ops/$a.json" --out "$GEN/try/$a.json" ${=BODY} > "$GEN/log/$a-body.log" 2>&1; echo "$a body exit $? $(tail -3 "$GEN/log/$a-body.log" | cut -c1-160 | tr "\n" ";")"
  "$W/q.sh" 0 "$T" 3000 "$B" "$W1/anchors.ts" "$GEN/try/$a.json" --save "$W/ops/$a-anchors.json" --out "$GEN/try/$a.json" ${=ANCHORS} > "$GEN/log/$a-anchors.log" 2>&1; echo "$a anchors exit $? $(tail -2 "$GEN/log/$a-anchors.log" | head -1 | cut -c1-160)"
  "$W/q.sh" 0 "$T" 1800 "$B" "$M/gate.ts" "$GEN/try/$a.json" --soft > "$GEN/log/$a-gate.log" 2>&1; echo "$a gate exit $? $(tail -1 "$GEN/log/$a-gate.log")"
  "$W/q.sh" 0 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json" > "$GEN/log/$a-measure.log" 2>&1; echo "$a measure exit $? $(grep -E "silhouette:|validate:|sweep:" "$GEN/log/$a-measure.log" | cut -c1-120 | tr "\n" ";")"
  "$W/q.sh" 0 "$T" 600 "$B" "$W/solids.ts" "$GEN/backup/$a.json" > "$GEN/log/$a-solids.log" 2>&1; echo "$a solids exit $? $(tail -1 "$GEN/log/$a-solids.log" | cut -c1-200)"
done
if [[ -n "$LOFT" ]]; then
  "$W/q.sh" 0 "$T" 3000 "$B" "$W/loft.ts" "$GEN/backup/$LOFT.json" --out "$GEN/try/$LOFT-loft.json" > "$GEN/log/$LOFT-loft.log" 2>&1; echo "$LOFT loft exit $? $(tail -2 "$GEN/log/$LOFT-loft.log" | cut -c1-160 | tr "\n" ";")"
  "$W/q.sh" 0 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/try/$LOFT-loft.json" > "$GEN/log/$LOFT-loft-measure.log" 2>&1; echo "$LOFT loft measure exit $? $(grep -E "silhouette:|validate:|sweep:" "$GEN/log/$LOFT-loft-measure.log" | cut -c1-200 | tr "\n" ";")"
fi
[[ -n "$ZOOMS" ]] && while read -r a tag pose s view height; do
  [[ -z "$a" ]] && continue
  case "$tag" in before) f="$GEN/backup/$a.json";; try) f="$GEN/try/$a.json";; *) f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)";; esac
  "$W/q.sh" 0 "$T14" 600 "$B" "$W/zoom.ts" "$f" "$s" "$pose" "$view" "$GEN/sheets/z.$a.s$s.$pose.$view.$tag.png" ${height:-1500} 2>&1 | tail -1
done < "$ZOOMS"
[[ -n "$PAIRS" ]] && while read -r a pose scales; do
  [[ -z "$a" ]] && continue
  mkdir -p "$GEN/sheets/work-before/$a" "$GEN/sheets/work-try/$a"; cp "$GEN/backup/$a.json" "$GEN/sheets/work-before/$a/🔣️.json"; cp "$GEN/try/$a.json" "$GEN/sheets/work-try/$a/🔣️.json"
  [[ "$pose" == "-" ]] && pose="$(jq -r .asset.presentation.defaultPose "$GEN/backup/$a.json")"
  for s in ${=scales}; do
    for tag in before try; do
      png="$("$W/q.sh" 0 "$T14" 900 "$B" r3-2d.ts --file "$GEN/sheets/work-$tag/$a/🔣️.json" --scale "$s" --pose "$pose" --views light-line 2>&1 | tail -1)"
      [[ -f "$png" ]] && mv "$png" "$GEN/sheets/work-$tag/$a.png" || echo "no sheet: $png"
    done
    out="$GEN/sheets/$a.s$s.$pose.pair.png"
    "$W/q.sh" 0 "$T14" 300 "$B" "$W/pair.ts" "$GEN/sheets/work-before/$a.png" "$GEN/sheets/work-try/$a.png" "$out" > /dev/null 2>&1
    [[ -n "$CROP" && -f "$out" ]] && sips -c "$(sips -g pixelHeight "$out" | tail -1 | tr -dc 0-9)" 1600 --cropOffset 0 800 "$out" > /dev/null 2>&1
    echo "$out"
  done
  rm -rf "$GEN/sheets/work-before" "$GEN/sheets/work-try"
done < "$PAIRS"' round "$@"

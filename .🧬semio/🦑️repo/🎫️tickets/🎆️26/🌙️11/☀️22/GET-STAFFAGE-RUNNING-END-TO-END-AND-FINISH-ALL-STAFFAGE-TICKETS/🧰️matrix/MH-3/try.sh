#!/bin/zsh
# 🧪️ The dry run of one asset: applies the earlier step of the asset named in chain.tsv (if any) and ops/<id>.json to the catalog document into $GEN/try/<id>.json (fit.ts of this folder, run again on its own result until a run changes nothing, at most five times), then runs the gate on the copy (GATE_FLAGS, default "--poses"; "" for every pose, clip instant and transition) and, unless NOMEASURE is set, the silhouette guard, validateAsset, the three-views sweep and the print suitability (suit.ts): try.sh <id>
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
B="$here/qbun"
a="$1"; mkdir -p "$GEN/try" "$GEN/gate" "$GEN/measure"
src="$a"; pre="$(awk -F'\t' -v a="$a" '$1 == a { print $2 }' "$here/chain.tsv")"
if [[ -n "$pre" ]]; then (cd "$T" && QMEM=1000 QTIME=600 "$B" "$here/$pre" "$a" --out "$GEN/try/$a.json") > "$GEN/try/$a.pre.log" 2>&1; echo "$pre exit $? $(head -1 "$GEN/try/$a.pre.log")"; src="$GEN/try/$a.json"; fi
(cd "$T" && QMEM=1000 QTIME=600 "$B" "$here/fit.ts" "$src" --ops "$here/ops/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.fit.log" 2>&1
echo "fit exit $? $(tail -1 "$GEN/try/$a.fit.log")"; grep -E "REFUSED|mismatch (side )?[0-9.]+ mm" "$GEN/try/$a.fit.log" | awk '{ for (i = 1; i <= NF; i++) if ($i == "mismatch" || $i == "side" || $i == "front") { v = $(i + 1) + 0; if (v > 20) { print; break } } } /REFUSED/ { print }' | cut -c1-330
for n in 2 3 4 5; do
  (cd "$T" && QMEM=1000 QTIME=600 "$B" "$here/fit.ts" "$GEN/try/$a.json" --ops "$here/ops/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.fit2.log" 2>&1
  echo "fit run $n: $(tail -1 "$GEN/try/$a.fit2.log")"
  grep -q " 0 changed something" "$GEN/try/$a.fit2.log" && break
done
(cd "$T" && QMEM=2500 QTIME=${GATE_TIMEOUT:-1700} "$B" "$M/gate.ts" "$GEN/try/$a.json" --twins ${=GATE_FLAGS---poses}) > "$GEN/gate/$a.try.log" 2>&1
echo "gate exit $? $(tail -1 "$GEN/gate/$a.try.log")"; print -r -- "${GATE_FLAGS---poses}" > "$GEN/gate/$a.flags"; grep -A12 "HARD" "$GEN/gate/$a.try.log" | grep -v "^$a 1:[0-9]* \(twin\|reported\|subjects\)" | cut -c1-260 | head -${LINES_MAX:-60}
if [[ -z "$NOMEASURE" ]]; then
  (cd "$S/📦️packages/🟦️typescript" && QMEM=2500 QTIME=900 "$B" "$M/measure.ts" "$GEN/try/$a.json") > "$GEN/measure/$a.try.log" 2>&1
  python3 "$here/silcmp.py" "$GEN/measure/$a.before.log" "$GEN/measure/$a.try.log"
  (cd "$S/📦️packages/🟦️typescript" && QMEM=1000 QTIME=600 "$B" "$here/suit.ts" "$GEN/try/$a.json") 2>&1 | grep -E "UNSUITABLE|^suitability:" | cut -c1-300
fi

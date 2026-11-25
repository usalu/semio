#!/bin/zsh
# 🧪️ The dry run of try.sh for several assets inside ONE grant (start it with qsh): per asset the earlier step of chain.tsv, ops/<id>.json with fit.ts into $GEN/try/<id>.json until a run changes nothing, then ops2/<id>.json (if any) with volume.ts until a run changes nothing and fit.ts once more (it must change nothing), lofts/<id>.json (if any) with ../MX-2/loft.ts and fit.ts once more, the gate on the copy (GATE_FLAGS, default "--poses"; "" for every subject), the silhouette guard, validateAsset, the sweep and the print suitability; every process one after the other, output per asset in $GEN/try/<id>.out and a summary line on stdout: QMEM=2500 QTIME=<s> GATE_FLAGS="" qsh in-try.sh <id>[:poses|:all]…   (the suffix sets the gate of one asset)
[[ -n "$MH3_GRANT" ]] || { echo "in-try.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
mkdir -p "$GEN/try" "$GEN/gate" "$GEN/measure"
for spec in "$@"; do
  a="${spec%%:*}"; flags="${GATE_FLAGS---poses}"; [[ "$spec" == *:poses ]] && flags="--poses"; [[ "$spec" == *:all ]] && flags=""
  {
    src="$a"; pre="$(awk -F'\t' -v a="$a" '$1 == a { print $2 }' "$here/chain.tsv")"
    if [[ -n "$pre" ]]; then (cd "$T" && "$B" "$here/$pre" "$a" --out "$GEN/try/$a.json") > "$GEN/try/$a.pre.log" 2>&1; echo "$pre exit $? $(head -1 "$GEN/try/$a.pre.log")"; src="$GEN/try/$a.json"; fi
    (cd "$T" && "$B" "$here/fit.ts" "$src" --ops "$here/ops/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.fit.log" 2>&1
    echo "fit exit $? $(tail -1 "$GEN/try/$a.fit.log")"; grep -E "REFUSED|mismatch (side )?[0-9.]+ mm" "$GEN/try/$a.fit.log" | awk '{ for (i = 1; i <= NF; i++) if ($i == "mismatch" || $i == "side" || $i == "front") { v = $(i + 1) + 0; if (v > 20) { print; break } } } /REFUSED/ { print }' | cut -c1-330
    for n in 2 3 4 5; do
      (cd "$T" && "$B" "$here/fit.ts" "$GEN/try/$a.json" --ops "$here/ops/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.fit2.log" 2>&1
      echo "fit run $n: $(tail -1 "$GEN/try/$a.fit2.log")"
      grep -q " 0 changed something" "$GEN/try/$a.fit2.log" && break
    done
    if [[ -f "$here/ops2/$a.json" ]]; then
      for n in 1 2 3; do
        (cd "$T" && "$B" "$here/volume.ts" "$GEN/try/$a.json" --ops "$here/ops2/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.vol.log" 2>&1
        echo "volume run $n: exit $? $(tail -1 "$GEN/try/$a.vol.log")"; [[ $n -eq 1 ]] && grep -v "unchanged" "$GEN/try/$a.vol.log" | cut -c1-330
        grep -q " 0 changed something" "$GEN/try/$a.vol.log" && break
      done
      (cd "$T" && "$B" "$here/fit.ts" "$GEN/try/$a.json" --ops "$here/ops/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.fit3.log" 2>&1
      echo "fit after volume: $(tail -1 "$GEN/try/$a.fit3.log")"
    fi
    if [[ -f "$here/lofts/$a.json" ]]; then
      for n in 1 2 3; do
        (cd "$S/📦️packages/🟦️typescript" && "$B" "$here/../MX-2/loft.ts" "$GEN/try/$a.json" --ops "$here/lofts/$a.json" --out "$GEN/try/$a.loft.json") > "$GEN/try/$a.loft.log" 2>&1
        echo "loft run $n: exit $? $(tail -1 "$GEN/try/$a.loft.log" | cut -c1-200)"
        cmp -s "$GEN/try/$a.json" "$GEN/try/$a.loft.json" && break; mv "$GEN/try/$a.loft.json" "$GEN/try/$a.json"
      done
      (cd "$T" && "$B" "$here/fit.ts" "$GEN/try/$a.json" --ops "$here/ops/$a.json" --out "$GEN/try/$a.json") > "$GEN/try/$a.fit4.log" 2>&1
      echo "fit after loft: $(tail -1 "$GEN/try/$a.fit4.log")"
    fi
    start=$(date +%s)
    (cd "$T" && timeout ${GATE_TIMEOUT:-3000} "$B" "$M/gate.ts" "$GEN/try/$a.json" --twins ${=flags}) > "$GEN/gate/$a.try.log" 2>&1
    echo "gate exit $? $(tail -1 "$GEN/gate/$a.try.log") ($(( $(date +%s) - start )) s; $(grep " subjects " "$GEN/gate/$a.try.log" | sed -E 's/.* 1:([0-9]+) subjects ([0-9]+).*/1:\1 \2/' | tr '\n' ' ')subjects)"; print -r -- "$flags" > "$GEN/gate/$a.flags"; grep -A12 "HARD" "$GEN/gate/$a.try.log" | grep -v "^$a 1:[0-9]* \(twin\|reported\|subjects\)" | cut -c1-260 | head -${LINES_MAX:-60}
    if [[ -z "$NOMEASURE" ]]; then
      (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/try/$a.json") > "$GEN/measure/$a.try.log" 2>&1
      python3 "$here/silcmp.py" "$GEN/measure/$a.before.log" "$GEN/measure/$a.try.log"
      (cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$here/suit.ts" "$GEN/try/$a.json") 2>&1 | grep -E "UNSUITABLE|^suitability:" | cut -c1-300
    fi
  } > "$GEN/try/$a.out" 2>&1
  echo "$a: $(grep -E '^gate exit' "$GEN/try/$a.out") | $(grep -E 'silhouette:|validate:|sweep:|^suitability:' "$GEN/try/$a.out" | tr '\n' ' ')"
done

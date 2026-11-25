#!/bin/zsh
# 🧷️ Takeover chain D of batch MV-3 for `weeping-willow`, meant for one grant (`grant.sh 1500 <timeout> chain-d.sh <step>…`): `strands` lays the strands on a copy of the catalog file (`strands.ts`, flags in STRANDS), `look` draws its sheets (LOOKS=`<scale>:<pose> …`), `anchors` searches the anchors on that copy, `gate` and `suit` measure it, `measure` runs the guard of the other tests per scale, `timing` times `evaluate`. Result lines go to `$GEN/log/chain-d.txt`.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
q() { local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; }
o="$GEN/log/chain-d.txt"; a=weeping-willow; f="$GEN/try/$a.json"; echo "chain D $* $(date +%F\ %H:%M)" >> "$o"
BONES=curt-w1,curt-w2,curt-w3,curt-w4,curt-e1,curt-e2,curt-e3,curt-e4
for s in "$@"; do case "$s" in
  strands|tress) rm -f "$f"; q 1000 "$T" 1800 "$B" "$W/strands.ts" ${SOURCE:-$a} --bones $BONES --out "$f" ${=STRANDS} > "$GEN/log/$a-strands.log" 2>&1; echo "strands exit $? $(tail -2 "$GEN/log/$a-strands.log" | tr "\n" ";" | cut -c1-300)" >> "$o" ;;
  look) for item in ${=LOOKS:-50:storm 500:storm}; do TAG=try POSE="${item#*:}" "$W/look.sh" $a "${item%%:*}" >> "$o" 2>&1; done ;;
  anchors) q 1000 "$T" 3000 "$B" "$W/anchors.ts" "$f" --save "$W/ops/$a.json" --out "$f" > "$GEN/log/$a-anchors.log" 2>&1; echo "anchors exit $? $(tail -2 "$GEN/log/$a-anchors.log" | head -1 | cut -c1-200)" >> "$o" ;;
  gate) q 1000 "$T" 3000 "$B" "$M/gate.ts" "$f" > "$GEN/log/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$a-gate.log")" >> "$o" ;;
  suit) q 1000 "$T" 1200 "$B" "$W/suit.ts" "$f" > "$GEN/log/$a-suit.log" 2>&1; echo "suit exit $? $(tail -1 "$GEN/log/$a-suit.log")" >> "$o" ;;
  timing) q 1000 "$T" 1500 "$B" "$W/timing.ts" "$f" > "$GEN/log/timing-$a-try.log" 2>&1; echo "timing exit $? $(tr '\n' ';' < "$GEN/log/timing-$a-try.log" | cut -c1-900)" >> "$o" ;;
  measure) for n in 50 100 200 500; do q 1450 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$f" --scales $n > "$GEN/log/$a-measure-try-$n.log" 2>&1; { echo "== $a measure 1:$n exit $?"; grep -E "bbox|silhouette:|validate:|sweep:|!" "$GEN/log/$a-measure-try-$n.log" | cut -c1-300; } >> "$o"; done ;;
esac; done
echo "chain D done $(date +%H:%M)" >> "$o"
[[ -z "$NOWORKER" ]] && zsh "$W/worker.sh" ${WORKER_MB:-900}

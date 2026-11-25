#!/bin/zsh
# 🧷️ Takeover chain B of batch MV-3 for `weeping-willow`, meant for one grant (`grant.sh 1450 <timeout> chain-b.sh`; measured peaks: `carry.ts` 1083 MB, anchor search 1102 MB): the trial run (`trial.sh`: depth plates of at most twelve points below the curtains, anchor search, gate, print suitability), the time `evaluate` needs on the trial copy, the difference of the rest drawing (`refdiff.ts`, plan 1:200) and the guard of the other tests on the trial copy, one process per scale so that no step needs more than the grant (`M/measure.ts --scales <n>`; the file as found measured OK at every scale on 2026-11-25 10:50). Result lines go to `$GEN/log/chain-b.txt`.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
q() { local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; }
o="$GEN/log/chain-b.txt"; a=weeping-willow; echo "chain B $(date +%F\ %H:%M)" >> "$o"
zsh "$W/trial.sh" $a >> "$o" 2>&1
q 1000 "$T" 1500 "$B" "$W/timing.ts" "$GEN/try/$a.json" > "$GEN/log/timing-$a-try.log" 2>&1; echo "timing exit $? $(tr '\n' ';' < "$GEN/log/timing-$a-try.log" | cut -c1-900)" >> "$o"
q 1500 "$T" 1500 "$B" "$W/refdiff.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json" plan 200 > "$GEN/log/refdiff-$a.log" 2>&1; echo "refdiff exit $? $(grep -v capacity "$GEN/log/refdiff-$a.log" | tr '\n' ';' | cut -c1-600)" >> "$o"
for s in 50 100 200 500; do q 1450 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/try/$a.json" --scales $s > "$GEN/log/$a-measure-try-$s.log" 2>&1; { echo "== $a measure 1:$s exit $?"; grep -E "bbox|silhouette:|validate:|sweep:|!" "$GEN/log/$a-measure-try-$s.log" | cut -c1-300; } >> "$o"; done
echo "chain B done $(date +%H:%M)" >> "$o"

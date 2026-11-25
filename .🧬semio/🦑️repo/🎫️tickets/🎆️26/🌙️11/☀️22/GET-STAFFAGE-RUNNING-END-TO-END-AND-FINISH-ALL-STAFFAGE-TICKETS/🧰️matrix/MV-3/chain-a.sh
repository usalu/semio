#!/bin/zsh
# 🧷️ Takeover chain A of batch MV-3, meant for one grant each: `grant.sh 1200 <timeout> chain-a.sh small` times `evaluate` for `weeping-willow` (catalog file and trial copy with prisms), then runs the official scan of today on the catalog file of every plant but the lime, one process per plant; `grant.sh 3000 <timeout> chain-a.sh lime` runs the guard of the other tests on the trial copy of `small-leaved-lime` (`M/measure.ts`, backup then trial), its sheets before and trial and its scan (the 23 MB tree: 2652 MB measured for its chain steps). Result lines go to `$GEN/log/chain-a-<part>.txt`, peaks to `$GEN/log/rss.txt`.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; }
o="$GEN/log/chain-a-$1.txt"; echo "chain A $1 $(date +%F\ %H:%M)" >> "$o"
scan() { q 1000 "$T" 3000 "$B" 🔎️scan.ts --language en --report "$GEN/log/scan-now-$1.json" "$1" > "$GEN/log/scan-now-$1.log" 2>&1; echo "$1 $(date +%H:%M) scan exit $? $(tail -1 "$GEN/log/scan-now-$1.log" | cut -c1-200)" >> "$o"; }
if [[ "$1" == small ]]; then
  for f in weeping-willow "$GEN/try/weeping-willow.json"; do n="timing-${${f:t}%.json}-${${f:h:t}}"; q 1000 "$T" 1500 "$B" "$W/timing.ts" "$f" > "$GEN/log/$n.log" 2>&1; echo "timing ${f:t} exit $? $(date +%H:%M): $(tr '\n' ';' < "$GEN/log/$n.log" | cut -c1-900)" >> "$o"; done
  for a in rosemary-bush rowan saucer-magnolia scots-pine stone-pine sunflower sycamore-maple tulip-tree wisteria-pergola weeping-willow; do scan "$a"; done
else
  a=small-leaved-lime
  q 2500 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json" > "$GEN/log/$a-measure-try.log" 2>&1; { echo "== $a measure exit $? $(date +%H:%M)"; grep -E "silhouette:|validate:|sweep:|!" "$GEN/log/$a-measure-try.log" | cut -c1-300; } >> "$o"
  MV3_GRANT=1 zsh "$W/look-many.sh" before,try $a:50:default $a:100:default $a:50:gale >> "$o" 2>&1
  scan "$a"
fi
echo "chain A $1 done $(date +%H:%M)" >> "$o"

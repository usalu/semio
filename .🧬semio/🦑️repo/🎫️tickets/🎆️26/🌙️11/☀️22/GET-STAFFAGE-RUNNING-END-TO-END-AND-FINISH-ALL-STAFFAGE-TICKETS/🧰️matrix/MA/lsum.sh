#!/bin/zsh
# 🧾️ The short state of loft trial copies from their logs, nothing is run: lsum.sh <id>…   per asset the hard total of the gate with one line per scale, code and bone (count, worst mm, subject), the views of the silhouette guard that fail, the validation issues and the sweep findings of the copy (second block of the measure log), and the parts the build left or refused.
BATCH=MA source "${0:A:h}/../MR/env.sh"
g="${LDIR:-$GEN/loft}"
for a in "$@"; do
  echo "== $a: gate $(tail -1 "$g/$a.gate.log" 2>/dev/null) ($(stat -f '%Sm' -t '%H:%M' "$g/$a.gate.log" 2>/dev/null)), copy $(stat -f '%Sm' -t '%H:%M' "$g/$a.json")"
  awk '/ HARD /{h=$2" "$4; next} /^    /{ if (h!="" && $1 !~ /\//) print "  "h" "$0 } /^[a-z]/{ if ($0 !~ / HARD /) h="" }' "$g/$a.gate.log" 2>/dev/null | tr -s ' ' | cut -c1-150 | head -${TOP:-14}
  awk -v a="$a" 'n>=1; $0 ~ "^"a" three-views sweep:"{n++}' "$g/$a.measure.log" 2>/dev/null | grep -E "!|silhouette:|validate:|sweep:|^   " | cut -c1-${CUT:-210} | head -${MOST:-12}
  [[ -f "$g/$a.prints.log" && "$g/$a.prints.log" -nt "$g/$a.build1.log" ]] && head -4 "$g/$a.prints.log" | cut -c1-200
  grep -E "LEFT|REFUSED" "$g/$a.build1.log" 2>/dev/null | cut -c1-120 | head -6
done

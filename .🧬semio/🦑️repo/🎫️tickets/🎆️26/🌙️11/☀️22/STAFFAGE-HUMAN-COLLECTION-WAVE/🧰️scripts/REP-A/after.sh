#!/bin/zsh
# 📏️ The final measurement of people with the finisher's script (HW-03/measure.ts: silhouette guard and checkVolumes of every rule), one log per person in 🗑️generated/REP-A/after-<slug>.log. after.sh <slug>…
source "${0:A:h}/env.sh"
for a in "$@"; do (cd "$T/🧰️scripts/HW-03" && $B measure.ts $a) > "$GEN/after-$a.log" 2>&1; tail -1 "$GEN/after-$a.log"; done

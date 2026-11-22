#!/bin/zsh
# 🧵️ One pass of the local silhouette repair of people: the ops below <stage> are applied to the generator output (🗑️generated/REP-A/base/<slug>.json), fit.ts writes ops/<slug>.<stage>-auto.json, then every stage is applied and the catalog file written.
# fit.sh <stage> <scales|-> <views|-> <slug>…
source "${0:A:h}/env.sh"
stage=$1; scales=$2; views=$3; shift 3
mkdir -p "$GEN/work"
for a in "$@"; do
  python3 "$W/apply.py" $a "$GEN/base/$a.json" "$GEN/work/$a.json" --upto $stage > /dev/null || { echo "$a APPLY FAILED"; continue; }
  (cd "$W" && $B fit.ts "$GEN/work/$a.json" "$W/ops/$a.$stage-auto.json" "$scales" "$views" ${MAXOPS:-14}) > "$GEN/[DEBUG]-fit-$a-$stage.log" 2>&1
  [[ "$(cat "$W/ops/$a.$stage-auto.json" 2>/dev/null)" == "[]" ]] && rm "$W/ops/$a.$stage-auto.json"
  python3 "$W/apply.py" $a "$GEN/base/$a.json" "$CAT/$a/🔣️.json"
  grep -E " (ok|FAIL) \(" "$GEN/[DEBUG]-fit-$a-$stage.log" | grep FAIL; tail -1 "$GEN/[DEBUG]-fit-$a-$stage.log"
done

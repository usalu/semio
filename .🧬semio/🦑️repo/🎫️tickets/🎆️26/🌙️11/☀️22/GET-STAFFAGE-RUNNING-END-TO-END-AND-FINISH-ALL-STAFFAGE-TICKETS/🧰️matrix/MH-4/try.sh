#!/bin/zsh
# 🧪️ The dry run of the ops of one asset on a copy: try.sh <id> [full]   applies $W/ops/<id>.json to the backup into $GEN/try/<id>.json, prints the refusals and mismatches, what moved at rest (rest.ts), the gate over the worst subjects of the baseline log and of earlier tries (or every subject with `full`), and the silhouette guard before and after (measure.ts). Logs in $GEN/try.
source "${0:A:h}/env.sh"
source "$W/lock.sh"
a="$1"; d="$GEN/try"; mkdir -p "$d"
(cd "$T" && "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$W/ops/$a.json" --out "$d/$a.json") > "$d/$a.fit.log" 2>&1
grep -E "REFUSED|ops, " "$d/$a.fit.log" | cut -c1-300
(cd "$T" && "$B" "$W/rest.ts" "$GEN/backup/$a.json" "$d/$a.json" --limit ${LIMIT:-3}) > "$d/$a.rest.log" 2>&1; tail -1 "$d/$a.rest.log"; grep -c "painting order" "$d/$a.rest.log"
if [[ "$2" == full ]]; then
  (cd "$T" && timeout 2400 "$B" "$W/gate.ts" "$d/$a.json" --twins --soft) > "$d/$a.gate-full.log" 2>&1; tail -1 "$d/$a.gate-full.log"
  "$W/hard.sh" "$d/$a.gate-full.log" | grep -v " twin \|^    .*/" | cut -c1-260
else
  only="$(python3 "$W/worst.py" "$GEN/gate0/$a.log" "$d"/$a.gate-full.log(N))"
  (cd "$T" && timeout 1800 "$B" "$W/gate.ts" "$d/$a.json" --twins --soft --only "$only") > "$d/$a.gate.log" 2>&1; tail -1 "$d/$a.gate.log"
  "$W/hard.sh" "$d/$a.gate.log" | grep -v " twin \|^    .*/" | cut -c1-260
fi
[[ -n "$NOMEASURE" ]] || (cd "$S/📦️packages/🟦️typescript" && "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$d/$a.json") 2>&1 | cut -c1-260

#!/bin/zsh
# 🧪️ The dry run of the ops of one asset on a copy: try.sh <id> [full]   applies $W/ops/<id>.json to the backup into $GEN/try/<id>.json, prints the refusals and mismatches, what moved at rest (rest.ts), the gate over the worst subjects of the baseline log and of earlier tries (or every subject with `full`), the silhouette guard before and after (measure.ts) and the authors' scan of the copy (scan-file.ts: its summary line and the codes the gate does not measure). Every run goes through the memory queue (cap.sh). Logs in $GEN/try.
source "${0:A:h}/env.sh"
source "$W/lock.sh"
a="$1"; d="$GEN/try"; mkdir -p "$d"
"$W/cap.sh" 800 300 "$T" "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$W/ops/$a.json" --out "$d/$a.json" > "$d/$a.fit.log" 2>&1
grep -E "REFUSED|ops, " "$d/$a.fit.log" | cut -c1-300
"$W/cap.sh" 800 600 "$T" "$B" "$W/rest.ts" "$GEN/backup/$a.json" "$d/$a.json" --limit ${LIMIT:-3} > "$d/$a.rest.log" 2>&1; tail -1 "$d/$a.rest.log"; grep -c "painting order" "$d/$a.rest.log"
if [[ "$2" == full ]]; then
  "$W/cap.sh" 800 3600 "$T" "$B" "$W/gate.ts" "$d/$a.json" --twins --soft > "$d/$a.gate-full.log" 2>&1; tail -1 "$d/$a.gate-full.log"
  "$W/hard.sh" "$d/$a.gate-full.log" | grep -v " twin \|^    .*/" | cut -c1-260
else
  only="$(python3 "$W/worst.py" "$GEN/gate0/$a.log" "$d"/$a.gate-full.log(N))"
  "$W/cap.sh" 800 1800 "$T" "$B" "$W/gate.ts" "$d/$a.json" --twins --soft --only "$only" > "$d/$a.gate.log" 2>&1; tail -1 "$d/$a.gate.log"
  "$W/hard.sh" "$d/$a.gate.log" | grep -v " twin \|^    .*/" | cut -c1-260
fi
[[ -n "$NOMEASURE" ]] || "$W/cap.sh" 1000 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$d/$a.json" 2>&1 | cut -c1-260
[[ -n "$NOSCAN" ]] || "$W/cap.sh" 800 900 "$T" "$B" "$W/scan-file.ts" "$d/$a.json" 2>&1 | tee "$d/$a.scan.log" | grep -v "sync-bone\|anchor-landmark\|   sync-" | cut -c1-260

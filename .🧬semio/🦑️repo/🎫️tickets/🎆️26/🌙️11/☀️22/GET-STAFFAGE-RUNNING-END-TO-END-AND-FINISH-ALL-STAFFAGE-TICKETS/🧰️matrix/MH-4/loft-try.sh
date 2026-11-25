#!/bin/zsh
# 🧶️ The loft trial of one asset on a copy (second takeover): loft-try.sh <id> [full]   applies $W/lofts/<id>.json (`../MX-2/loft.ts`) to $GEN/base/<id>.json (the proven catalog file before the lofts), then $W/after/<id>.json (fit.ts) when it exists, into $GEN/try/<id>.json; prints refusals, the silhouette guard, validate and sweep (measure.ts, base and copy), the gate over the worst subjects of the last gate logs (or every subject with `full`) and the authors' scan of the copy. PART=light leaves out measure.ts (the only step above 800 MB), PART=heavy runs it alone on the copy as it is. Run it inside a grant of g.sh.
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/try"; mkdir -p "$d" "$GEN/base"
[[ -f "$GEN/base/$a.json" ]] || cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/base/$a.json"
P="$S/📦️packages/🟦️typescript"
if [[ "$PART" != heavy ]]; then
"$W/cap.sh" 800 600 "$P" "$B" "${LOFT:-$W/lofta.ts}" "$GEN/base/$a.json" --ops "$W/lofts/$a.json" --out "$d/$a.json" > "$d/$a.loft.log" 2>&1; code=$?
grep -E "REFUSED|rror" "$d/$a.loft.log" | cut -c1-250; tail -1 "$d/$a.loft.log" | cut -c1-200
[[ $code == 0 || -f "$d/$a.json" ]] || exit 1
if [[ -f "$W/after/$a.json" ]]; then
  "$W/cap.sh" 800 300 "$T" "$B" "$W/fit.ts" "$d/$a.json" --ops "$W/after/$a.json" --out "$d/$a.after.json" > "$d/$a.fit.log" 2>&1; grep -E "REFUSED|ops, |rror" "$d/$a.fit.log" | cut -c1-250
  mv "$d/$a.after.json" "$d/$a.json"
fi
fi
[[ "$PART" == light ]] || { "$W/cap.sh" 1000 1800 "$P" "$B" "$M/measure.ts" "$d/$a.json" > "$d/$a.measure.log" 2>&1; cut -c1-260 "$d/$a.measure.log"; }
[[ "$PART" == heavy ]] && exit 0
if [[ "$2" == full ]]; then
  "$W/cap.sh" 800 3600 "$T" "$B" "$W/gate.ts" "$d/$a.json" --twins > "$d/$a.gate-full.log" 2>&1; tail -1 "$d/$a.gate-full.log"
  "$W/hard.sh" "$d/$a.gate-full.log" | grep -v " twin \|^    .*/" | cut -c1-260
else
  only="$(python3 "$W/worst.py" "$GEN/proof/$a-gate.log" "$d"/$a.gate.log(N))"
  "$W/cap.sh" 800 1800 "$T" "$B" "$W/gate.ts" "$d/$a.json" --twins --only "$only" > "$d/$a.gate.log" 2>&1; tail -1 "$d/$a.gate.log"
  "$W/hard.sh" "$d/$a.gate.log" | grep -v " twin \|^    .*/" | cut -c1-260
fi
"$W/cap.sh" 800 900 "$T" "$B" "$W/scan-file.ts" "$d/$a.json" > "$d/$a.scan.log" 2>&1; grep -v "reported\|^    \|^      " "$d/$a.scan.log" | cut -c1-200; tail -1 "$d/$a.scan.log" | cut -c1-200

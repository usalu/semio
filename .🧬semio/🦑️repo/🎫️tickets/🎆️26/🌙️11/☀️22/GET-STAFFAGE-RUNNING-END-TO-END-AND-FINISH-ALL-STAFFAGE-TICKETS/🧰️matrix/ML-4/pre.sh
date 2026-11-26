#!/bin/zsh
# 🧫️ The trial copies of the fill-rule repair against their backups before anything is written to the catalog, in one queue grant: the overlap oracle of `🕳️fill-rule` on both, then per asset `MR/measure.ts` (silhouette guard, `validateAsset`, three-views sweep) on backup and trial with their diff, the fixer's gate and the print check on the trial: pre.sh <id>…
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=1600 -- timeout 7200 "$me" --inside "$@"; fi
shift; d="$GEN/pre"; mkdir -p "$d"; MC="$U/🧰️matrix/MC"
(cd "$P" && timeout 600 "$B" "$W/overlaps.ts" $(for a in "$@"; do echo "$GEN/backup/$a.json"; done)) 2>&1 | tail -20 | sed 's/^/before: /'
(cd "$P" && timeout 600 "$B" "$W/overlaps.ts" $(for a in "$@"; do echo "$GEN/trial/$a.json"; done)) 2>&1 | tail -5 | sed 's/^/trial: /'
for a in "$@"; do
  echo "== $a $(date +%H:%M)"
  (cd "$P" && /usr/bin/time -l timeout 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json") > "$d/$a-before.log" 2>&1
  (cd "$P" && timeout 2400 "$B" "$M/measure.ts" "$GEN/trial/$a.json") > "$d/$a-after.log" 2>&1
  diff <(grep -vE "real|maximum|  [0-9]+  " "$d/$a-before.log" | sed "s#$GEN/backup/##") <(sed "s#$GEN/trial/##" "$d/$a-after.log") > "$d/$a-diff.log" && echo "measure: same as before: $(grep -E 'silhouette:|validate:|sweep:' "$d/$a-after.log" | tr '\n' ';' | cut -c1-400)" || { echo "measure DIFFERS"; head -30 "$d/$a-diff.log" | cut -c1-300; }
  grep -E "maximum resident" "$d/$a-before.log"
  (cd "$T" && timeout 3600 "$B" "$M/gate.ts" "$GEN/trial/$a.json" --twins) > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log" | cut -c1-200)"
  (cd "$T" && timeout 900 "$B" "$MC/prints.ts" "$GEN/trial/$a.json") 2>&1 | grep -v '^\[' | cut -c1-300
done
echo "pre ended $(date +%H:%M)"

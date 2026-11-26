# 📏️ The state of catalog files now (sourced by q.sh): measure.sh <id>…   gate over every subject with twins, authors' scan, into $GEN/proof/<id>-gate.log and -scan.log; keeps the file as $GEN/base/<id>.json when no base exists.
for a in "$@"; do
  [[ -f "$GEN/base/$a.json" ]] || cp "$C/$a/🔣️.json" "$GEN/base/$a.json"
  cap 3600 "$T" "$B" "$H/gate.ts" "$a" --twins > "$GEN/proof/$a-gate.log" 2>&1; echo "$a gate exit $? $(tail -1 "$GEN/proof/$a-gate.log") ($(date +%H:%M))"
  cap 1800 "$T" "$B" "$T/🔎️scan.ts" --language en "$a" > "$GEN/proof/$a-scan.log" 2>&1; echo "$a scan exit $? $(tail -1 "$GEN/proof/$a-scan.log" | cut -c1-200)"
done

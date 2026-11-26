# 🩺️ The proofs of one asset on its catalog file (sourced by q.sh, grant 1000 MB): prove.sh <id>   gate over every subject with twins, authors' scan, silhouette guard with validate and sweep (`../MR/measure.ts`), prints at every band and its ends (`../MC/prints.ts`), check CLI, then the matrix of the asset to its end with the cells by status (`../MR/cells.py`). Logs in $GEN/proof, lines in $GEN/proof/<id>.prove.out; the six suites run under their own grant (suites.sh).
a="$1"; d="$GEN/proof"
{
  echo "== prove $a $(date +%H:%M) md5 $(md5 -q "$C/$a/🔣️.json")"
  cap 3600 "$T" "$B" "$H/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log") ($(date +%H:%M))"
  cap 1800 "$T" "$B" "$T/🔎️scan.ts" --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
  cap 1800 "$P" "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1; cut -c1-260 "$d/$a-measure.log"
  cap 600 "$T" "$B" "$U/🧰️matrix/MC/prints.ts" "$a" 2>&1 | cut -c1-300
  cap 1800 "$S" "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check exit $?: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
  rm -rf "$GEN/matrix-$a"
  cap 10800 "$T" "$B" "$T/📊️matrix.ts" --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log" | cut -c1-200) ($(date +%H:%M))"
  python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300
  echo "== proved $a $(date +%H:%M) md5 $(md5 -q "$C/$a/🔣️.json")"
} > "$d/$a.prove.out" 2>&1
cat "$d/$a.prove.out"

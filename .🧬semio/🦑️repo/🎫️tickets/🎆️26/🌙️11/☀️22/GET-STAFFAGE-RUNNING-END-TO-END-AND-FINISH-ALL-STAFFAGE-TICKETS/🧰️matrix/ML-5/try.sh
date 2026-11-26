# 🧪️ One trial of an asset on a copy (sourced by q.sh; the chain of MH-4 continued): try.sh <id> <variant> [steps]   applies $W/trials/<id>.<variant>.json (variant `landed`: $W/lofts/<id>.json) with `$H/lofta.ts` to $GEN/base/<id>.json, then $W/trials/<id>.<variant>.after.json (else $W/after/<id>.json) with `$H/fit.ts` and $W/trials/<id>.<variant>.say.json (else $W/say/<id>.json) with `say.ts`, into $GEN/try/<id>.<variant>.json; steps (default `loft prints guard gate scan`): prints (`../MC/prints.ts`), guard (`../MR/measure.ts`: silhouette, validate, sweep), gate (every subject, twins), scan (authors' scan on the copy). Lines in $GEN/try/<id>.<variant>.out.
a="$1"; v="$2"; steps="${3:-loft prints guard gate scan}"; d="$GEN/try"; f="$d/$a.$v.json"
ops="$W/trials/$a.$v.json"; [[ "$v" == landed ]] && ops="$W/lofts/$a.json"
after="$W/trials/$a.$v.after.json"; [[ -f "$after" ]] || after="$W/after/$a.json"
[[ -f "$GEN/base/$a.json" ]] || cp "$C/$a/🔣️.json" "$GEN/base/$a.json"
{
  echo "== $a $v $(date +%H:%M)"
  if [[ "$steps" == *loft* ]]; then
    cap 600 "$P" "$B" "$H/lofta.ts" "$GEN/base/$a.json" --ops "$ops" --out "$f" > "$d/$a.$v.loft.log" 2>&1
    grep -E "REFUSED|rror" "$d/$a.$v.loft.log" | cut -c1-250; tail -1 "$d/$a.$v.loft.log" | cut -c1-200
    [[ -f "$f" ]] || cp "$GEN/base/$a.json" "$f"
    if [[ -f "$after" ]]; then
      cap 300 "$T" "$B" "$H/fit.ts" "$f" --ops "$after" --out "$f.after" > "$d/$a.$v.fit.log" 2>&1; grep -E "REFUSED|ops, |rror" "$d/$a.$v.fit.log" | cut -c1-250
      [[ -f "$f.after" ]] && mv "$f.after" "$f"
    fi
    say="$W/trials/$a.$v.say.json"; [[ -f "$say" ]] || say="$W/say/$a.json"
    [[ -f "$say" ]] && cap 300 "$T" "$B" "$W/say.ts" "$f" --ops "$say" --out "$f" | head -1
  fi
  [[ "$steps" == *prints* ]] && cap 600 "$T" "$B" "$U/🧰️matrix/MC/prints.ts" "$f" 2>&1 | cut -c1-300
  [[ "$steps" == *guard* ]] && { cap 1800 "$P" "$B" "$M/measure.ts" "$f" ${SCALES:+--scales=$SCALES} > "$d/$a.$v.measure.log" 2>&1; cut -c1-260 "$d/$a.$v.measure.log"; }
  if [[ "$steps" == *gate* ]]; then
    cap 3600 "$T" "$B" "$H/gate.ts" "$f" --twins > "$d/$a.$v.gate.log" 2>&1; tail -1 "$d/$a.$v.gate.log"
    "$H/hard.sh" "$d/$a.$v.gate.log" | grep -v " twin \|^    .*/" | cut -c1-260
  fi
  [[ "$steps" == *scan* ]] && { cap 900 "$T" "$B" "$H/scan-file.ts" "$f" > "$d/$a.$v.scan.log" 2>&1; grep -v "reported\|^    \|^      " "$d/$a.$v.scan.log" | cut -c1-200; }
  echo "== end $(date +%H:%M)"
} > "$d/$a.$v.out" 2>&1

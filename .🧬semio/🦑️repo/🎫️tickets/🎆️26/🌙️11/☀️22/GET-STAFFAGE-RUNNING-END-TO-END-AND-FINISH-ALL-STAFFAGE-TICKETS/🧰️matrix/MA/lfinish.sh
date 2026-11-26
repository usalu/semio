#!/bin/zsh
# 🏁️ Writes and proves the loft repair of assets in ONE grant of the memory queue (1000 MB): lfinish.sh <id>…   The catalog file must be the backup, the hybrid copy the predecessor wrote or the loft copy (else the asset is skipped); it is set to the backup, `loft.ts --ops loft/<id>.json --write` runs twice (the second run changes nothing), then `../MX-2/loft.ts --ops loft/<id>.lofts.json --write` twice when that file exists, the file must equal the proven trial copy `$GEN/loft/<id>.json`; then scan, gate, measure, check CLI and the matrix of the asset (MATRIX=0 leaves it out). An asset whose measure or check is not green is put back to the file it had before. Logs `$GEN/proof/<id>-*.log`, block `$GEN/proof/<id>-finish.txt`.
BATCH=MA source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d" "$GEN/was"
for a in "$@"; do
  file="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  {
    echo "== $a (loft) $(date '+%H:%M')"
    if ! { cmp -s "$GEN/backup/$a.json" "$file" || cmp -s "$GEN/nest/$a.json" "$file" || cmp -s "$GEN/loft/$a.json" "$file" || cmp -s "$GEN/keep/$a.v1.json" "$file" || cmp -s "$GEN/keep/$a.v2.json" "$file" || cmp -s "$GEN/written/$a.json" "$file"; }; then echo "catalog file is not a known state: skipped"; continue; fi
    cp "$file" "$GEN/was/$a.json"
    cat "$GEN/backup/$a.json" > "$file"
    (cd "$T" && "$B" "$W/loft.ts" "$a" --ops "$W/loft/$a.json" --write > "$d/$a-apply1.log" 2>&1; "$B" "$W/loft.ts" "$a" --ops "$W/loft/$a.json" --write > "$d/$a-apply2.log" 2>&1)
    [[ -f "$W/loft/$a.lofts.json" ]] && { (cd "$S/📦️packages/🟦️typescript" && "$B" "$M/../MX-2/loft.ts" "$a" --ops "$W/loft/$a.lofts.json" --write > "$d/$a-body1.log" 2>&1; "$B" "$M/../MX-2/loft.ts" "$a" --ops "$W/loft/$a.lofts.json" --write > "$d/$a-body2.log" 2>&1); echo "body: $(tail -2 "$d/$a-body1.log" | tr '\n' ' ') | second run: $(tail -2 "$d/$a-body2.log" | tr '\n' ' ')"; }
    echo "apply: $(tail -2 "$d/$a-apply1.log" | tr '\n' ' ') | second run: $(tail -2 "$d/$a-apply2.log" | tr '\n' ' ')"
    if ! cmp -s "$GEN/loft/$a.json" "$file"; then cat "$GEN/was/$a.json" > "$file"; echo "PUT BACK: the written file is not the proven trial copy"; continue; fi
    mkdir -p "$GEN/written"; cp "$file" "$GEN/written/$a.json"
    (cd "$T" && timeout 1800 "$B" 🔎️scan.ts --language en --quiet "$a" > "$d/$a-scan.log" 2>&1); echo "scan exit $?: $(grep -m1 "^$a " "$d/$a-scan.log" | cut -c1-200)"
    "$W/stage.sh" gate "$a"; "$W/stage.sh" measure "$a"; "$W/stage.sh" check "$a"
    if grep -qE "failing checks|sweep: [1-9]|validate: [1-9]" <(grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" | tail -3) || ! grep -q '^✓' "$d/$a-check.log"; then
      cat "$GEN/was/$a.json" > "$file"; echo "PUT BACK: measure or check is not green"; continue
    fi
    (cd "$T" && timeout 900 "$B" "$U/🧰️matrix/MC/prints.ts" "$a") 2>&1 | grep -v '^\[' | cut -c1-200
    [[ "${MATRIX:-1}" == 1 ]] && "$W/nstage.sh" matrix "$a"
    echo "== $a end $(date '+%H:%M')"
  } > "$d/$a-finish.txt" 2>&1
  echo "$a: $(grep -E 'PUT BACK|skipped|scan exit|gate exit|check exit|matrix exit' "$d/$a-finish.txt" | cut -c1-170 | tr '\n' '|')"
done

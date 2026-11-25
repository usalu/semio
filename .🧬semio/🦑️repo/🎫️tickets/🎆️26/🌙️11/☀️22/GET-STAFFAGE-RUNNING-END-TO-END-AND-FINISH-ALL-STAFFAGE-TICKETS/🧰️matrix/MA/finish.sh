#!/bin/zsh
# 🏁️ Writes and proves assets in ONE grant of the memory queue (`repo capacity run --memory=1200 -- timeout <s> finish.sh <chain>:<id>…`, chain `fit` = `ops/<id>.json` with `fit.ts`, chain `nest` = `nest/<id>.json` with `nest.ts`, chain `hybrid` = first `fit`, then `nest` on the shapes `fit` left flat, the chain `guard.sh` proved): the backup must still equal the catalog file (else the asset is skipped), then per asset the apply stage of its chain (second run changes nothing, catalog equals the proven trial copy), the official scan, the gate, measure (silhouette guard, validateAsset, three-views sweep), the check CLI and the matrix of the asset; an asset whose measure or check is worse than the backup's is put back to the backup at once. Logs in `$GEN/proof`, one block per asset in `$GEN/proof/<id>-finish.txt`.
BATCH=MA source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"
for pair in "$@"; do
  chain="${pair%%:*}"; a="${pair#*:}"; file="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  {
    echo "== $a ($chain) $(date '+%H:%M')"
    if ! cmp -s "$GEN/backup/$a.json" "$file"; then echo "catalog file differs from the backup: skipped"; continue; fi
    trial="$GEN/nest/$a.json"
    case "$chain" in
      fit) "$W/stage.sh" apply "$a"; trial="$GEN/try/$a.json" ;;
      nest) "$W/nstage.sh" apply "$a" ;;
      hybrid) "$W/stage.sh" apply "$a"; "$W/nstage.sh" apply "$a" ;;
    esac
    if ! cmp -s "$trial" "$file"; then cp "$GEN/backup/$a.json" "$file"; echo "PUT BACK: the written file is not the proven trial copy"; continue; fi
    (cd "$T" && timeout 1800 "$B" 🔎️scan.ts --language en --quiet "$a" > "$d/$a-scan.log" 2>&1); echo "scan exit $?: $(grep -m1 "^$a " "$d/$a-scan.log" | cut -c1-200)"
    "$W/stage.sh" gate "$a"
    "$W/stage.sh" measure "$a"
    "$W/stage.sh" check "$a"
    if grep -qE "failing checks|sweep: [1-9]|validate: [1-9]" <(grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" | tail -3) || ! grep -q '^✓' "$d/$a-check.log"; then
      cp "$GEN/backup/$a.json" "$file"; echo "PUT BACK: measure or check is not green"; continue
    fi
    "$W/nstage.sh" matrix "$a"
    echo "== $a end $(date '+%H:%M')"
  } > "$d/$a-finish.txt" 2>&1
  echo "$a: $(grep -E 'PUT BACK|skipped|scan exit|gate exit|check exit|matrix exit' "$d/$a-finish.txt" | cut -c1-170 | tr '\n' '|')"
done

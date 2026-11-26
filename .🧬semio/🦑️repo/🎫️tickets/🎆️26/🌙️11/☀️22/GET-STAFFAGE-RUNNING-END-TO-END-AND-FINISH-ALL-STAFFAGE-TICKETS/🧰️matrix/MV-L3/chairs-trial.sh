#!/bin/zsh
# 🪑️ The 1:500 cure of `used-bentwood-dining-chairs` (lane MU, `--with-500`) on a copy with its light proof, inside one grant: the catalog file is backed up, `mu-ops.py` writes the ops here, `../MU/trial.ts --spec` applies the chain of MU with them to the catalog document in memory and the result becomes `$GEN/try/<id>.json`; then `light.sh`. Nothing of lane MU and nothing in the catalog is written.
source "${0:A:h}/env.sh"
a=used-bentwood-dining-chairs; f="$(ls "$C"/*/"$a"/🔣️.json)"; mkdir -p "$GEN/backup" "$GEN/try" "$GEN/log"
[[ -f "$GEN/backup/$a.json" ]] || cp "$f" "$GEN/backup/$a.json"
python3 "$W/mu-ops.py" 2>&1 | tail -2
(cd "$S/📦️packages/🟦️typescript" && GEN="$GEN" timeout 2400 "$B" "$U/🧰️matrix/MU/trial.ts" "$a" --spec "$W/ops/$a.json" --no-measure > "$GEN/log/$a-trial.log" 2>&1); echo "trial exit $?"; cut -c1-300 "$GEN/log/$a-trial.log" | tail -25
mv "$GEN/try/$a.$a.json" "$GEN/try/$a.json" && zsh "$W/light.sh" "$a" && cat "$GEN/light/$a.txt"

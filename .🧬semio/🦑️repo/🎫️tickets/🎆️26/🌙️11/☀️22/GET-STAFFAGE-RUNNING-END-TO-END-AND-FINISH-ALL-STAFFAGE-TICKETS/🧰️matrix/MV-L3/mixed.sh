#!/bin/zsh
# 🧬️ The mixed trial of one tree inside one grant: `mixed.sh <id> [pick items of pick.py, default the line of `flags/<id>.lofts`]` keeps the part of the full loft search `$GEN/loft-all/<id>.json` named by the items as `loft/<id>.json`, applies it to the backup as `$GEN/base/<id>.json` (`../MV-L1/loft.ts --ops`), lays bodies in the place of the plates that are left (`trial.sh`, on that base), and runs the light proof of the result `$GEN/try/<id>.json`.
source "${0:A:h}/env.sh"
a="$1"; shift; (( $# )) || set -- ${=$(cat "$W/flags/$a.lofts")}; mkdir -p "$GEN/base"
python3 "$W/pick.py" "$GEN/loft-all/$a.json" "$W/loft/$a.json" "$@"
(cd "$T" && timeout 900 "$B" "$U/🧰️matrix/MV-L1/loft.ts" "$GEN/backup/$a.json" --ops "$W/loft/$a.json" --out "$GEN/base/$a.json" 2>&1 | tail -2 | cut -c1-200)
zsh "$W/trial.sh" "$a"; zsh "$W/light.sh" "$a"; cat "$GEN/light/$a.txt"

#!/bin/zsh
# 🖨️ Print suitability at the key and both ends of every band (`../MC/prints.ts`) of the backups of lane CL-H, for the comparison with the written files: prints-before.sh <id>…
source "${0:A:h}/env.sh"
cd "$T"; for a in "$@"; do timeout 900 "$B" "$U/🧰️matrix/MC/prints.ts" "$GEN/backup/$a.json" > "$GEN/before/$a-prints.log" 2>&1; echo "$a exit $?"; done

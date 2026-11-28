#!/bin/zsh
# 🧭️ The silhouette guard, `validateAsset` and the three-views sweep of catalog assets as they are (`../MR/measure.ts`), the baseline a trial copy is compared with: base.sh <id>… writes `$GEN/base/<id>.txt`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/base"
for a in "$@"; do (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" "$M/measure.ts" "$a") > "$GEN/base/$a.log" 2>&1; grep -E "silhouette:|validate:|sweep:|bbox" "$GEN/base/$a.log" | cut -c1-300 > "$GEN/base/$a.txt"; tail -3 "$GEN/base/$a.txt"; done

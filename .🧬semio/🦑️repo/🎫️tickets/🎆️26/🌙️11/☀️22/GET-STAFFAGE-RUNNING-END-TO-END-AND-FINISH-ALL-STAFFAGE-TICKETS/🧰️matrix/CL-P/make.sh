#!/bin/zsh
# 🏭️ The trial copy of one asset of lane CL-P: `fix.ts` with `ops/<id>.json` on the catalog file into `$GEN/try/<id>.json`; the digest of the catalog file it was made from goes to `$GEN/try/<id>.base`, then the geometry findings of `🧊️model-catalog` and the weld report of the solids named `$2` on the catalog file and on the copy (`geo.ts`). make.sh <id> <substring of the solid ids>
source "${0:A:h}/env.sh"
a="$1"; f="$(ls "$C"/*/"$a"/🔣️.json)"; mkdir -p "$GEN/try"
shasum -a 256 < "$f" | cut -c1-16 > "$GEN/try/$a.base"
(cd "$T" && "$B" "$W/fix.ts" "$a" --ops "$W/ops/$a.json" --out "$GEN/try/$a.json")
cd "$S" && echo "BEFORE" && "$B" "$W/geo.ts" --report "$2" "$f"; echo "AFTER"; "$B" "$W/geo.ts" --report "$2" "$GEN/try/$a.json"

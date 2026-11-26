#!/bin/zsh
# 🏭️ The trial copy of one asset of lane CL-H: `fix.ts` with `ops/<id>.json` on the catalog file into `$GEN/try/<id>.json`; the digest of the catalog file it was made from goes to `$GEN/try/<id>.base`. make.sh <id>
source "${0:A:h}/env.sh"
a="$1"; f="$(ls "$C"/*/"$a"/🔣️.json)"; mkdir -p "$GEN/try" "$GEN/backup"
[[ -f "$GEN/backup/$a.json" ]] || cp "$f" "$GEN/backup/$a.json"
shasum -a 256 < "$f" | cut -c1-16 > "$GEN/try/$a.base"
cd "$T" && "$B" "$W/fix.ts" "$a" --ops "$W/ops/$a.json" --out "$GEN/try/$a.json"

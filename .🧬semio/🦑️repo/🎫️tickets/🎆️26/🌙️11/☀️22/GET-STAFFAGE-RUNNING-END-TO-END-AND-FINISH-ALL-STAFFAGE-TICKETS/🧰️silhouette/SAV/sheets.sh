#!/bin/zsh
# 👀️ Before and after product drawings of an asset of batch SAV at the given scales (upper row the backup of the state before this fixer, lower row the candidate or, with `-`, the catalog file): sheets.sh <id> <cand.json|-> <scale>…  -> 🗑️generated/silhouette/SAV/look/<id>.<scale>.png
source "${0:A:h}/env.sh"
a="$1"; f="$2"; shift 2
[[ "$f" == "-" ]] && f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
mkdir -p "$GEN/look"
cd "$S/📦️packages/🟦️typescript"
for s in "$@"; do timeout 300 "$B" "$W/sheet.ts" "$GEN/backup/$a.json" "$f" "$s" "$GEN/look/$a.$s.png" ${VIEW:-light-line}; done

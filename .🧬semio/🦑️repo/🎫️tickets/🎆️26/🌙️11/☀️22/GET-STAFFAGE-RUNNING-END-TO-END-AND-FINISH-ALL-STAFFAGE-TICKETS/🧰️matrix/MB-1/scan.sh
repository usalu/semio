#!/bin/zsh
# 🛰️ The cheap gate of the batch: the catalog scan of the diagnosis over the given assets into 🗑️generated/matrix/MB-1/scan/<tag>.jsonl and one line per finding kind: scan.sh <tag> <id>…
source "${0:A:h}/env.sh"
tag="$1"; shift; mkdir -p "$GEN/scan"; out="$GEN/scan/$tag.jsonl"; rm -f "$out"
(cd "$T" && timeout 1500 "$B" "$U/🧰️matrix/catalog-scan.ts" "$S/🧪️tests" "$out" "${(j:,:)@}" > /dev/null 2>&1)
python3 "$W/scan-lines.py" "$out"

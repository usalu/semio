#!/usr/bin/env bash
# 🛰️ Scans assets with the catalog scan of the diagnosis into 🗑️generated/matrix/MB-2/scan-<tag>.jsonl (fresh) and prints the findings: scan.sh <tag> <id,id,…>
source "$(dirname "$0")/env.sh"
mkdir -p "$G"; rm -f "$G/scan-$1.jsonl"
(cd "$T" && timeout 1200 "$BUN" "$U/🧰️matrix/catalog-scan.ts" "$S/🧪️tests" "$G/scan-$1.jsonl" "$2" 2> "$G/scan-$1.err") || echo "scan exit $?"
python3 "$M/show.py" "$G/scan-$1.jsonl"

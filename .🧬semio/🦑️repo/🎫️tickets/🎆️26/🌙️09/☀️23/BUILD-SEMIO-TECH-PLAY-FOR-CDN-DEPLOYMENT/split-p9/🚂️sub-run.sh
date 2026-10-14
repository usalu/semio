#!/bin/bash
# 🚂️ One second-level cut: rehost a family crate into its sub-owner crates, then manifests and consumer dependencies.
# usage: 🚂️sub-run.sh <config.json> [plan|apply]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
CFG="$1"; MODE="${2:-apply}"; N="$(basename "$CFG" .json)"
cd "$HERE"
python3 🏗️rehost.py "$CFG" "$MODE" > "🗑️generated/rehost-$N.log" 2>&1 || { tail -5 "🗑️generated/rehost-$N.log"; exit 1; }
grep -E '^PROBLEM|^## (unhosted|test files moved)|files change' "🗑️generated/rehost-$N.log" | cut -c1-200
python3 🔁️plan-cycles.py "🗑️generated/rehost-$N.log" | head -1 | cut -c1-300
[ "$MODE" = apply ] || exit 0
python3 🧾️family-manifests.py "$CFG" apply > "🗑️generated/manifests-$N.log" 2>&1
python3 🧾️consumer-deps.py "$CFG" apply > "🗑️generated/consumers-$N.log" 2>&1
echo "applied $N"

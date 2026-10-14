#!/bin/bash
# 🚂️ One norm artifact slice: config, rehost, family manifests, consumer dependencies and the script router (P9 N1–N8).
# usage: 🚂️norm-run.sh <artifact-dir-name> [plan|apply]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ART="$1"; MODE="${2:-apply}"
N="$(printf '%s' "$ART" | sed 's/^[^a-z]*//')"
cd "$HERE"
python3 🧾️norm-config.py "$ART" > /dev/null
python3 🏗️rehost.py "🧾️configs/norm-$N.json" "$MODE" > "🗑️generated/rehost-$N.log" 2>&1 || { tail -5 "🗑️generated/rehost-$N.log"; exit 1; }
grep -E '^## (unresolved|unhosted|test files moved)|files change' "🗑️generated/rehost-$N.log" | cut -c1-200
[ "$MODE" = apply ] || exit 0
python3 🧾️family-manifests.py "🧾️configs/norm-$N.json" apply > "🗑️generated/manifests-$N.log" 2>&1
python3 🧾️consumer-deps.py "🧾️configs/norm-$N.json" apply > "🗑️generated/consumers-$N.log" 2>&1
python3 📜️family-script.py "🧾️configs/norm-$N.json" > /dev/null
echo "applied $ART"

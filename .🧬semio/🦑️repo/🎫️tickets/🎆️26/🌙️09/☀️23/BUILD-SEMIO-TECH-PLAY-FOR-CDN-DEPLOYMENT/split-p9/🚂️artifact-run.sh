#!/bin/bash
# 🚂️ One artifact slice outside norm: config, rehost, family manifests, consumer dependencies and the script router.
# usage: 🚂️artifact-run.sh <artifact dir (repo-relative)> [plan|apply]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
A="$1"; MODE="${2:-apply}"
cd "$HERE"
CFG="$(python3 🧾️artifact-config.py "$A")"
N="$(basename "$CFG" .json)"
python3 🏗️rehost.py "$CFG" "$MODE" > "🗑️generated/rehost-$N.log" 2>&1 || { tail -5 "🗑️generated/rehost-$N.log"; exit 1; }
grep -E '^## (unresolved|unhosted|test files moved)|files change' "🗑️generated/rehost-$N.log" | cut -c1-200
python3 🔁️plan-cycles.py "🗑️generated/rehost-$N.log" | cut -c1-300
[ "$MODE" = apply ] || exit 0
python3 🧾️family-manifests.py "$CFG" apply > "🗑️generated/manifests-$N.log" 2>&1
python3 🧾️consumer-deps.py "$CFG" apply > "🗑️generated/consumers-$N.log" 2>&1
python3 📜️family-script.py "$CFG" > /dev/null
echo "applied $A"

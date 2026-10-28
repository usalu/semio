#!/bin/sh
# 🔁 [pinpub] rebuilds both batches from the CURRENT live files (after any landing): copies the live bytes to a scratch root, replays the anchored edits, restages with base = live; usage: sh rebuild.sh [pinned|nxsparse|all]
C="$(cd "$(dirname "$0")/.." && pwd)"; T="$C/pinpub-tools"; L="$(cd "$C" && while [ ! -d 🧰️framework ]; do cd ..; done; pwd)"; S="$C/🗑️generated/pinpub/rebuild"; which="${1:-all}"
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; G="$P/🏗️build/🧪️tests"; N="$P/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph"
rm -rf "$S"; set -e
copy(){ mkdir -p "$S/$(dirname "$1")"; cp "$L/$1" "$S/$1"; }
if [ "$which" = all ] || [ "$which" = pinned ]; then
 copy "$G/🕸️graph/📌️pinned-publication/🟦️.ts"; copy "$G/🕸️graph/🧰️support/🟦️.ts"; copy "$G/🧫️fixtures/🕸️graph/🔣️.json"
 python3 -I "$T/edit_pinned.py" "$S"; python3 -I "$T/edit_support.py" "$S"; python3 -I "$T/edit_fixture.py" "$S"
 python3 -I "$T/mkbatch.py" pinned "$S" "$G/🕸️graph/📌️pinned-publication/🟦️.ts=live" "$G/🕸️graph/🧰️support/🟦️.ts=live" "$G/🧫️fixtures/🕸️graph/🔣️.json=live"
fi
if [ "$which" = all ] || [ "$which" = nxsparse ]; then
 copy "$N/🟦️.ts"; python3 -I "$T/edit_nx.py" "$S"; mkdir -p "$S/$N/🧪️tests/🪶️sparse"; for f in 🟦️.ts 🔣️.json 🐍️.py; do cp "$T/staged/nxsparse/files/$(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1]))[sys.argv[2]]['file'])" "$T/staged/nxsparse/manifest.json" "$N/🧪️tests/🪶️sparse/$f")" "$S/$N/🧪️tests/🪶️sparse/$f"; done
 python3 -I "$T/mkbatch.py" nxsparse "$S" "$N/🟦️.ts=live" "$N/🧪️tests/🪶️sparse/🟦️.ts=none" "$N/🧪️tests/🪶️sparse/🔣️.json=none" "$N/🧪️tests/🪶️sparse/🐍️.py=none"
fi
rm -rf "$S"

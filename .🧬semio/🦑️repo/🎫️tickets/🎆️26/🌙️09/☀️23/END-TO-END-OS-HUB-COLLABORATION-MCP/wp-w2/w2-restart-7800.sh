#!/bin/zsh
# 🔁️ W2: restart canonical hub 7800 on a published catalog with a copied binary. Stops the current hold (hub exits on pipe close),
# makes a fresh private data root with a real copy of the catalog's current generation, provisions the two human credentials,
# boots the hold detached and waits for ADMIN issued.
# The hub data root lives under `.🧬semio/🌐hub/` (the canonical hub data home, never swept); binary copy and hold state under generated/.
# usage: zsh w2-restart-7800.sh <catalog data root> <source os-hub binary> <data root name> <old hold pid|->
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2
cd "$W/generated"
mkdir -p bin
SRC="$1"; BIN_SRC="$2"; NAME="$3"; OLD="$4"
test -f "$SRC/trusted-catalog/current.json"
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
ROOT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/$NAME"
test ! -e "$ROOT"
mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
cp -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
diff -r "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/$GEN"
DATA=$(cd "$ROOT" && pwd -P)
BIN="$W/generated/bin/os-hub-$NAME"
rm -f "$BIN"; cp "$BIN_SRC" "$BIN"; codesign -s - -f "$BIN"
echo "source-sha256 $(shasum -a 256 "$BIN_SRC" | cut -c1-64) signed-sha256 $(shasum -a 256 "$BIN" | cut -c1-64) generation $GEN data $DATA"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$E" --display-name "$N"
done
OLDHUB=$(sed -n 's/^hub=//p' "$W/generated/state-7800/pids.txt" 2>/dev/null || true)
if [ "$OLD" != "-" ]; then kill "$OLD" 2>/dev/null || true; fi
if [ -n "$OLDHUB" ]; then for i in $(seq 1 30); do ps -p "$OLDHUB" >/dev/null || break; sleep 1; done; ps -p "$OLDHUB" >/dev/null && kill "$OLDHUB"; for i in $(seq 1 10); do ps -p "$OLDHUB" >/dev/null || break; sleep 1; done; ps -p "$OLDHUB" >/dev/null && { echo "old hub $OLDHUB still alive"; exit 1; }; fi
lsof -nP -iTCP:7800 -sTCP:LISTEN && { echo "port 7800 still bound"; exit 1; }
STATE="$W/generated/state-7800"; [ -d "$STATE" ] && mv "$STATE" "$W/generated/state-7800-$(date +%H%M%S)"
mkdir -p "$STATE"
python3 "$W/w2-detach.py" "$STATE/hold.txt" bun "$W/w2-hub-hold.ts" 7800 "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt")"

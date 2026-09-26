#!/bin/zsh
# 🛰️ G11 session 13 (C11's recipe): boots G11's own hub on <port> (8030–8039) on a fresh durable data root with a clone of a
# catalog's current generation, provisions user1/user2/user3 with the hub's own `credential set`, and detaches the hold (setsid).
# usage: zsh g11-hub.sh <port> <catalog root> <source os-hub binary> <name>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
PORT="$1"; SRC="$2"; BIN_SRC="$3"; NAME="s13-g11-$4"
STATE="$H/$NAME-state"
test -f "$SRC/trusted-catalog/current.json"
lsof -nP -iTCP:"$PORT" -sTCP:LISTEN && { echo "port $PORT bound"; exit 1; }
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
ROOT="$H/$NAME"
test ! -e "$ROOT"
mkdir -p "$ROOT/trusted-catalog/generations" "$H/s13-g11-bin"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
cp -c -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
DATA=$(cd "$ROOT" && pwd -P)
BIN="$H/s13-g11-bin/os-hub-$4"
rm -f "$BIN"; cp "$BIN_SRC" "$BIN"; codesign -s - -f "$BIN"
echo "source-sha256 $(shasum -a 256 "$BIN_SRC" | cut -c1-64) generation $GEN data $DATA"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2" "user3@semio.dev|User Three|gm1-local-dev-pass-3"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$E" --display-name "$N"
done
mkdir -p "$STATE"
python3 "$W/g11-detach.py" "$STATE/hold.txt" bun "$W/g11-hub-hold.ts" "$PORT" "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt")"

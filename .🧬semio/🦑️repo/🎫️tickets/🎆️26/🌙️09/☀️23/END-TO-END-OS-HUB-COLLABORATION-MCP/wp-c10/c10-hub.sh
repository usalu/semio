#!/bin/zsh
# 🛰️ C10 session 12: boots C10's own long-lived hub on <port> (8021–8029) on a fresh durable data root with a clone of a
# catalog's current generation, provisions user1/user2 (7800's passwords) with the hub's own `credential set`, and detaches
# the hold (setsid). Optional env: OS_HUB_MERGE_POLICY (normal|vigilant).
# usage: zsh c10-hub.sh <port> <catalog root> <source os-hub binary> <name>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-c10
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
PORT="$1"; SRC="$2"; BIN_SRC="$3"; NAME="s12-c10-$4"
STATE="$H/$NAME-state"
test -f "$SRC/trusted-catalog/current.json"
lsof -nP -iTCP:"$PORT" -sTCP:LISTEN && { echo "port $PORT bound"; exit 1; }
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
ROOT="$H/$NAME"
test ! -e "$ROOT"
mkdir -p "$ROOT/trusted-catalog/generations" "$H/s12-c10-bin"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
cp -c -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
DATA=$(cd "$ROOT" && pwd -P)
BIN="$H/s12-c10-bin/os-hub-$4"
rm -f "$BIN"; cp "$BIN_SRC" "$BIN"; codesign -s - -f "$BIN"
echo "source-sha256 $(shasum -a 256 "$BIN_SRC" | cut -c1-64) generation $GEN data $DATA merge-policy ${OS_HUB_MERGE_POLICY:-normal}"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$E" --display-name "$N"
done
mkdir -p "$STATE"
python3 "$W/c10-detach.py" "$STATE/hold.txt" bun "$W/c10-hub-hold.ts" "$PORT" "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt")"

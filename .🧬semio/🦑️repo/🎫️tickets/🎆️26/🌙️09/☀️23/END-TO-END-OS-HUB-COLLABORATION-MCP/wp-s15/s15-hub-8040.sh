#!/bin/zsh
# 🛰️ S15 session 12: boots S15's own hub on 8040 on a fresh durable data root with a real copy of a catalog's current generation,
# provisions user1/user2 with the hub's own `credential set`, and detaches S15's hold (setsid) so no turn teardown reaches it.
# usage: zsh s15-hub-8040.sh <catalog data root> <source os-hub binary> <data root name>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-s15
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
SRC="$1"; BIN_SRC="$2"; NAME="$3"
STATE="$H/$NAME-state"
test -f "$SRC/trusted-catalog/current.json"
lsof -nP -iTCP:8040 -sTCP:LISTEN && { echo "port 8040 bound"; exit 1; }
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
ROOT="$H/$NAME"
test ! -e "$ROOT"
mkdir -p "$ROOT/trusted-catalog/generations" "$H/$NAME-bin"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
cp -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
diff -r "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/$GEN"
DATA=$(cd "$ROOT" && pwd -P)
BIN="$H/$NAME-bin/os-hub"
rm -f "$BIN"; cp "$BIN_SRC" "$BIN"; codesign -s - -f "$BIN"
echo "source-sha256 $(shasum -a 256 "$BIN_SRC" | cut -c1-64) generation $GEN data $DATA"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$E" --display-name "$N"
done
mkdir -p "$STATE"
python3 "$W/s15-detach.py" "$STATE/hold.txt" bun "$W/s15-hub-hold.ts" 8040 "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt")"

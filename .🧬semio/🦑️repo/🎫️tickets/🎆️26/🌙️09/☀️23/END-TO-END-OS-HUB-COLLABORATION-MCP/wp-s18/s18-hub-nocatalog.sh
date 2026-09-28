#!/bin/zsh
# 🛰️ S18 (copy of S16's recipe): a catalog-less loopback hub (sign-in, directory, preference lane, presence — no artifact creation) on <port> with a
# fresh durable data root and a signed COPY of a current-tree os-hub, user1/user2 provisioned by the hub's own `credential set`,
# the hold detached in its own session. usage: zsh s16-hub-nocatalog.sh <port> <source os-hub binary> <name>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-s16
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
PORT="$1"; BIN_SRC="$2"; NAME="s14-s18-$3"
STATE="$H/$NAME-state"; ROOT="$H/$NAME"
lsof -nP -iTCP:"$PORT" -sTCP:LISTEN && { echo "port $PORT bound"; exit 1; }
test ! -e "$ROOT"
mkdir -p "$ROOT" "$H/s14-s18-bin" "$STATE"; chmod 700 "$ROOT"
DATA=$(cd "$ROOT" && pwd -P)
BIN="$H/s14-s18-bin/os-hub-$3"
rm -f "$BIN"; cp "$BIN_SRC" "$BIN"; codesign -s - -f "$BIN"
echo "source-sha256 $(shasum -a 256 "$BIN_SRC" | cut -c1-64) data $DATA"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$E" --display-name "$N"
done
python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/w2-detach.py "$STATE/hold.txt" bun "$W/s16-hub-hold-nocatalog.ts" "$PORT" "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt")"

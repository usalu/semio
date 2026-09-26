#!/bin/zsh
# ♻️ W2 (coordinator R2): bring hub 7800 back after a process loss on its EXISTING data root, catalog and binary copy — nothing is
# copied, provisioned or re-derived (the catalog generation and the users are durable in the data root). Starts the supervised hold.
# usage: zsh w2-hub-resume.sh <data root name under .🧬semio/🌐hub>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
NAME="$1"; ROOT="$H/$NAME"; BIN="$H/s12-w2-bin/os-hub-$NAME"; STATE="$H/s12-w2-state-7800"
test -f "$ROOT/trusted-catalog/current.json"; test -x "$BIN"
OLDHOLD=$(sed -n 's/^hold=//p' "$STATE/pids.txt" 2>/dev/null || true)
[ -n "$OLDHOLD" ] && ps -p "$OLDHOLD" >/dev/null && { echo "hold $OLDHOLD is alive; nothing to resume"; exit 1; }
lsof -nP -iTCP:7800 -sTCP:LISTEN && { echo "port 7800 still bound"; exit 1; }
[ -d "$STATE" ] && mv "$STATE" "$STATE-$(date +%H%M%S)"
mkdir -p "$STATE"
DATA=$(cd "$ROOT" && pwd -P)
python3 "$W/w2-detach.py" "$STATE/hold.txt" bun "$W/w2-hub-hold.ts" 7800 "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt") data $DATA bin $BIN"

#!/bin/zsh
# ♻️ W4 (W3's w3-hub-resume.sh): bring hub 7800 back after a whole-process loss on its EXISTING data root, catalog and binary directory —
# nothing is copied, provisioned or re-derived. Admits `s14-w4-hub-7800-*` (binary under s14-w4-bin) and `s13-w3-hub-7800-*` (s13-w3-bin)
# roots only (built after the rebuild's envelope wire); the hold is w4-hub-hold.ts (rolling capture).
# usage: zsh w4-hub-resume.sh <s14-w4-hub-7800-…|s13-w3-hub-7800-… data root name under .🧬semio/🌐hub>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w3
W4=/Users/ueli/Documents/semio/.tmp-ticket/wp-w4
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
NAME="$1"; ROOT="$H/$NAME"; STATE="$H/s13-w3-state-7800"
case "$NAME" in s14-w4-hub-7800-*) BIN="$H/s14-w4-bin/$NAME/os-hub" ;; s13-w3-hub-7800-*) BIN="$H/s13-w3-bin/$NAME/os-hub" ;; *) echo "refused: $NAME is not a W3/W4 7800 root"; exit 1 ;; esac
test -f "$ROOT/trusted-catalog/current.json"; test -x "$BIN"
OLDHOLD=$(sed -n 's/^hold=//p' "$STATE/pids.txt" 2>/dev/null || true)
[ -n "$OLDHOLD" ] && ps -p "$OLDHOLD" >/dev/null && { echo "hold $OLDHOLD is alive; nothing to resume"; exit 1; }
lsof -nP -iTCP:7800 -sTCP:LISTEN && { echo "port 7800 still bound"; exit 1; }
[ -d "$STATE" ] && mv "$STATE" "$STATE-$(date +%H%M%S)"
mkdir -p "$STATE"
DATA=$(cd "$ROOT" && pwd -P)
python3 "$W/w3-detach.py" "$STATE/hold.txt" bun "$W4/w4-hub-hold.ts" 7800 "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt") data $DATA bin $BIN"

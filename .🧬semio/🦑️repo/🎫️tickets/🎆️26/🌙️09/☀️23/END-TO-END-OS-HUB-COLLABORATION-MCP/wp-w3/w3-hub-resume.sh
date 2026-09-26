#!/bin/zsh
# ♻️ W3: bring hub 7800 back after a whole-process loss (desktop restart) on its EXISTING session-13 data root, catalog and binary
# directory — nothing is copied, provisioned or re-derived. Only `s13-w3-hub-7800-*` roots (built after the rebuild's envelope wire)
# are admitted: a pre-rebuild root's WAL cannot replay (preamble rule 23).
# usage: zsh w3-hub-resume.sh <s13-w3-hub-7800-… data root name under .🧬semio/🌐hub>
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w3
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
NAME="$1"; ROOT="$H/$NAME"; BIN="$H/s13-w3-bin/$NAME/os-hub"; STATE="$H/s13-w3-state-7800"
case "$NAME" in s13-w3-hub-7800-*) ;; *) echo "refused: $NAME is not a session-13 W3 root"; exit 1 ;; esac
test -f "$ROOT/trusted-catalog/current.json"; test -x "$BIN"
OLDHOLD=$(sed -n 's/^hold=//p' "$STATE/pids.txt" 2>/dev/null || true)
[ -n "$OLDHOLD" ] && ps -p "$OLDHOLD" >/dev/null && { echo "hold $OLDHOLD is alive; nothing to resume"; exit 1; }
lsof -nP -iTCP:7800 -sTCP:LISTEN && { echo "port 7800 still bound"; exit 1; }
[ -d "$STATE" ] && mv "$STATE" "$STATE-$(date +%H%M%S)"
mkdir -p "$STATE"
DATA=$(cd "$ROOT" && pwd -P)
python3 "$W/w3-detach.py" "$STATE/hold.txt" bun "$W/w3-hub-hold.ts" 7800 "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt") data $DATA bin $BIN"

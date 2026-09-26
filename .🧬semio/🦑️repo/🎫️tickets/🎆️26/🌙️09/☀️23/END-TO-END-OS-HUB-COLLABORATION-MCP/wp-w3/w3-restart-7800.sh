#!/bin/zsh
# 🔁️ W3: move canonical hub 7800 onto a published catalog. Fresh private data root (never an existing one: WALs from before the
# rebuild's envelope wire cannot replay), a real copy of the catalog's current generation, a binary directory holding `os-hub` +
# its `os-hub.sources.json` (so `os-hub-ts:hub-freshness` can verify it), two human credentials, the old hold stopped (its hub
# killed if it is still loading), the supervised hold started detached.
# usage: zsh w3-restart-7800.sh <catalog data root> <staged os-hub dir (os-hub + os-hub.sources.json)> <data root name> <old hold pid|->
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w3
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
STATE="$H/s13-w3-state-7800"
SRC="$1"; STAGED="$2"; NAME="$3"; OLD="$4"
OLDSTATE="${OLDSTATE:-$H/s12-w2-state-7800}"
test -f "$SRC/trusted-catalog/current.json"
test -x "$STAGED/os-hub" && test -f "$STAGED/os-hub.sources.json"
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
ROOT="$H/$NAME"
test ! -e "$ROOT"
mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
cp -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
diff -r "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/$GEN"
[ -d "$SRC/guest-codec-verifications" ] && cp -Rp "$SRC/guest-codec-verifications" "$ROOT/"
DATA=$(cd "$ROOT" && pwd -P)
BINDIR="$H/s13-w3-bin/$NAME"
mkdir -p "$BINDIR"; rm -f "$BINDIR/os-hub"
cp "$STAGED/os-hub" "$BINDIR/os-hub"; cp -p "$STAGED/os-hub.sources.json" "$BINDIR/os-hub.sources.json"
BIN="$BINDIR/os-hub"
echo "binary-sha256 $(shasum -a 256 "$BIN" | cut -c1-64) generation $GEN data $DATA"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$E" --display-name "$N"
done
OLDHUB=$(sed -n 's/^hub=//p' "$OLDSTATE/pids.txt" 2>/dev/null || true)
if [ "$OLD" != "-" ]; then touch "$OLDSTATE/stop"; for i in $(seq 1 20); do ps -p "$OLD" >/dev/null || break; sleep 1; done; ps -p "$OLD" >/dev/null && kill "$OLD"; fi
if [ -n "$OLDHUB" ]; then for i in $(seq 1 30); do ps -p "$OLDHUB" >/dev/null || break; sleep 1; done; ps -p "$OLDHUB" >/dev/null && kill "$OLDHUB"; for i in $(seq 1 10); do ps -p "$OLDHUB" >/dev/null || break; sleep 1; done; ps -p "$OLDHUB" >/dev/null && { echo "old hub $OLDHUB still alive"; exit 1; }; fi
lsof -nP -iTCP:7800 -sTCP:LISTEN && { echo "port 7800 still bound"; exit 1; }
[ -d "$STATE" ] && mv "$STATE" "$STATE-$(date +%H%M%S)"
mkdir -p "$STATE"
python3 "$W/w3-detach.py" "$STATE/hold.txt" bun "$W/w3-hub-hold.ts" 7800 "$DATA" "$BIN" "$STATE" > "$STATE/detach.txt"
echo "hold pid $(cat "$STATE/detach.txt")"

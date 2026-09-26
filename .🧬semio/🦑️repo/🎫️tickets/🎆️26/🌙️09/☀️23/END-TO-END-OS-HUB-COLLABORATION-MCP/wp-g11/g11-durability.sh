#!/bin/zsh
# 🤝️ G11: hub-edit-durability gate on its own hub (port 8031) from a credentialed copy of catalog B2 (W2's recipe:
# current generation + two local test users via `os-hub credential set`), run by G11's current-tree os-hub.
# Root, binary and run copies live under `.🧬semio/🌐hub/s13-g11-*`. usage: zsh g11-durability.sh <binaryName in s13-g11-bin>
set -eu
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
SRC="$H/w2-catalog-b2"; BIN="$H/s13-g11-bin/${1:?usage: g11-durability.sh <binaryName>}"
ROOT="$H/s13-g11-durability-src-$1"; RUN="$H/s13-g11-durability-run"
mkdir -p "$RUN"
if [ ! -f "$ROOT/trusted-catalog/current.json" ]; then
  GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
  mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
  cp -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
  cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
  for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
    E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
    printf '%s' "$P" | OS_HUB_DATA="$ROOT" "$BIN" credential set --email "$E" --display-name "$N"
  done
  echo "prepared generation $GEN in $ROOT"
fi
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"
export OS_MCP_HUB_BINARY="$BIN" OS_MCP_HUB_DATA_DIR="$ROOT" OS_MCP_HUB_PORT=8031 SEMIO_TEST_ARTIFACT_DIR="$RUN"
date; S=$(date +%s); nice -n 10 bun ./📜️script.ts hub-edit-durability-check; echo "RC=$? secs=$(( $(date +%s)-S ))"; date

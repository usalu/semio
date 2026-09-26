#!/bin/zsh
# 🌱️ G11: a fresh hub data root from catalog B2 (current generation copied, 0700) with the two local test users
# provisioned by the given os-hub binary. usage: zsh g11-hub-prepare.sh <rootName> <binaryName in s13-g11-bin>
set -eu
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
SRC="$H/w2-catalog-b2"; BIN="$H/s13-g11-bin/${2:?usage: g11-hub-prepare.sh <rootName> <binaryName>}"; ROOT="$H/${1:?}"
[ -e "$ROOT" ] && { echo "exists: $ROOT"; exit 1; }
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
cp -Rp "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
  printf '%s' "$P" | OS_HUB_DATA="$ROOT" "$BIN" credential set --email "$E" --display-name "$N"
done
echo "prepared generation $GEN in $ROOT"

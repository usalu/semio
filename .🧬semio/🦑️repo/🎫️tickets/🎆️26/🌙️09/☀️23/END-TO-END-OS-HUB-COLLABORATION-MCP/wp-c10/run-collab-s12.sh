#!/bin/zsh
# 🤝️ C10 session 12: collab-e2e (`verify collab`) against hub 7800 (`external`) or an own hub on 8020 (`own`) that the
# harness boots, restarts (STEP 9/10) and stops itself. Durable output under .🧬semio/🌐hub/s12-c10-logs/<tag>.
# usage: zsh run-collab-s12.sh <tag> external
#        zsh run-collab-s12.sh <tag> own <catalog root holding trusted-catalog/> <os-hub binary to copy>
set -u
TAG="$1"; MODE="$2"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s12-c10-logs/$TAG"
mkdir -p "$OUT"
export NX_DAEMON=false S_COLLAB_OUT="$OUT" S_COLLAB_SKIP_PREBUILD=1 S_COLLAB_SKIP_ACTIVATE=1 S_COLLAB_USER1_PORT=6521 S_COLLAB_USER2_PORT=6522 S_COLLAB_LANE="${S_COLLAB_LANE:-dev}"
export COLLAB_E2E_HUB_BOOT_BUDGET_MS=900000 COLLAB_E2E_DEV_BOOT_BUDGET_MS=600000
if [[ "$MODE" == "external" ]]; then
  export S_COLLAB_HUB_URL=http://127.0.0.1:7800 S_COLLAB_USER1_PASSWORD=gm1-local-dev-pass-1 S_COLLAB_USER2_PASSWORD=gm1-local-dev-pass-2
  export S_COLLAB_HUB_ADMIN_CAPABILITY_FILE="${S_COLLAB_HUB_ADMIN_CAPABILITY_FILE:-$H/s12-w2-state-7800/admin-capability.json}"
  touch "${S_COLLAB_HUB_ADMIN_CAPABILITY_FILE%admin-capability.json}admin-request"
else
  CAT="$3"; BIN_SRC="$4"
  lsof -nP -iTCP:8020 -sTCP:LISTEN && { echo "port 8020 bound"; exit 1; }
  GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$CAT/trusted-catalog/current.json")
  DATA="$H/s12-c10-hub-8020-$TAG"
  test ! -e "$DATA" || { echo "data root exists: $DATA"; exit 1; }
  mkdir -p "$DATA/trusted-catalog/generations" "$H/s12-c10-bin"
  chmod 700 "$DATA" "$DATA/trusted-catalog" "$DATA/trusted-catalog/generations"
  cp -c -Rp "$CAT/trusted-catalog/generations/$GEN" "$DATA/trusted-catalog/generations/"
  cp -p "$CAT/trusted-catalog/current.json" "$DATA/trusted-catalog/current.json"
  BIN="$H/s12-c10-bin/os-hub-$TAG"
  rm -f "$BIN"; cp "$BIN_SRC" "$BIN"; codesign -s - -f "$BIN"
  echo "catalog $GEN data $DATA binary $BIN source-sha256 $(shasum -a 256 "$BIN_SRC" | cut -c1-64)"
  export S_COLLAB_HUB_PORT=8020 S_COLLAB_HUB_DATA="$DATA" S_COLLAB_HUB_BINARY="$BIN"
fi
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
echo "START $(date '+%F %T') tag=$TAG mode=$MODE pid=$$ lane=$S_COLLAB_LANE"
bun ./📜️script.ts verify collab
echo "EXIT rc=$? $(date '+%F %T')"

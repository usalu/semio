#!/bin/zsh
# 🤝️ C11: collab-e2e (`verify collab`) against an already-running hub (external): two serves 6521/6522 started and stopped by
# the harness. Durable output under .🧬semio/🌐hub/s13-c11-logs/<tag>/.
# usage: zsh run-collab.sh <tag> <hubUrl> <adminCapabilityFile> [pw1] [pw2]
set -u
TAG="$1"; HUB="$2"; ADMIN="$3"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s13-c11-logs/$TAG"
mkdir -p "$OUT"
export NX_DAEMON=false S_COLLAB_OUT="$OUT" S_COLLAB_SKIP_PREBUILD=1 S_COLLAB_SKIP_ACTIVATE=1 S_COLLAB_USER1_PORT=6521 S_COLLAB_USER2_PORT=6522 S_COLLAB_LANE="${S_COLLAB_LANE:-dev}"
export COLLAB_E2E_HUB_BOOT_BUDGET_MS=900000 COLLAB_E2E_DEV_BOOT_BUDGET_MS=600000
export S_COLLAB_HUB_URL="$HUB" S_COLLAB_USER1_PASSWORD="${4:-gm1-local-dev-pass-1}" S_COLLAB_USER2_PASSWORD="${5:-gm1-local-dev-pass-2}" S_COLLAB_HUB_ADMIN_CAPABILITY_FILE="$ADMIN"
touch "${ADMIN%admin-capability.json}admin-request"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
echo "START $(date '+%F %T') tag=$TAG hub=$HUB pid=$$ lane=$S_COLLAB_LANE"
nice -n 10 bun ./📜️script.ts verify collab
echo "EXIT rc=$? $(date '+%F %T')"

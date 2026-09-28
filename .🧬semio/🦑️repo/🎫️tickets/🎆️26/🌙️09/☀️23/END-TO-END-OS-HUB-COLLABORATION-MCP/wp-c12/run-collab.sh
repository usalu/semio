#!/bin/zsh
# 🤝️ C12: collab-e2e (`verify collab --hub <url> --locale en|de`, goal-gate record `react-collaboration-e2e-<locale>`) against an
# already-running hub; the harness starts and stops its two serves (S_COLLAB_USER1_PORT/S_COLLAB_USER2_PORT, default 6521/6522).
# Credentials come from env only (`source env.sh`), never argv. Durable output under .🧬semio/🌐hub/s14-c12-logs/<tag>/.
# usage: zsh run-collab.sh <tag> <hubUrl> <adminCapabilityFile> [en|de]
set -u
TAG="$1"; HUB="$2"; ADMIN="$3"; LOCALE="${4:-en}"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s14-c12-logs/$TAG"
mkdir -p "$OUT"
source /Users/ueli/Documents/semio/.tmp-ticket/wp-c12/env.sh
export NX_DAEMON=false S_COLLAB_OUT="$OUT" S_COLLAB_SKIP_PREBUILD=1 S_COLLAB_SKIP_ACTIVATE=1 S_COLLAB_USER1_PORT="${S_COLLAB_USER1_PORT:-6521}" S_COLLAB_USER2_PORT="${S_COLLAB_USER2_PORT:-6522}" S_COLLAB_LANE="${S_COLLAB_LANE:-dev}"
export COLLAB_E2E_HUB_BOOT_BUDGET_MS=900000 COLLAB_E2E_DEV_BOOT_BUDGET_MS=600000
export S_COLLAB_USER1_PASSWORD="$C12_USER1_PASSWORD" S_COLLAB_USER2_PASSWORD="$C12_USER2_PASSWORD" S_COLLAB_HUB_ADMIN_CAPABILITY_FILE="$ADMIN"
touch "${ADMIN%admin-capability.json}admin-request"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
echo "START $(date '+%F %T') tag=$TAG hub=$HUB locale=$LOCALE pid=$$ lane=$S_COLLAB_LANE"
nice -n 10 bun ./📜️script.ts verify collab --hub "$HUB" --locale "$LOCALE"
echo "EXIT rc=$? $(date '+%F %T')"

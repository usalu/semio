#!/bin/zsh
# Usage: zsh run-collab.sh <tag> [external]  — collab-e2e against hub 7800 (external) or an own replica hub on 8020.
TAG="$1"; MODE="$2"
C10=/Users/ueli/Documents/semio/.tmp-ticket/wp-c10
OUT="$C10/generated/$TAG"
mkdir -p "$OUT"
export NX_DAEMON=false S_COLLAB_OUT="$OUT" S_COLLAB_SKIP_PREBUILD=1 S_COLLAB_SKIP_ACTIVATE=1 S_COLLAB_USER1_PORT=6521 S_COLLAB_USER2_PORT=6522 S_COLLAB_LANE="${S_COLLAB_LANE:-release}"
if [[ "$MODE" == "external" ]]; then
  export S_COLLAB_HUB_URL=http://127.0.0.1:7800 S_COLLAB_USER1_PASSWORD=gm1-local-dev-pass-1 S_COLLAB_USER2_PASSWORD=gm1-local-dev-pass-2
  export S_COLLAB_HUB_ADMIN_CAPABILITY_FILE=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2/generated/state-7800/admin-capability.json
  touch /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/generated/state-7800/admin-request
else
  DATA="$C10/generated/data/hub-8020-$TAG"
  rm -rf "$DATA"; mkdir -p "$DATA"; chmod 700 "$DATA"; cp -c -R "$C10/generated/data/catalog-a-seed/trusted-catalog" "$DATA/"
  export S_COLLAB_HUB_PORT=8020 S_COLLAB_HUB_DATA="$DATA" S_COLLAB_HUB_BINARY="$C10/generated/data/bin/os-hub"
fi
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
echo "START $(date +%s) tag=$TAG mode=${MODE:-own} pid=$$"
bun ./📜️script.ts verify collab
echo "EXIT rc=$? $(date +%s)"

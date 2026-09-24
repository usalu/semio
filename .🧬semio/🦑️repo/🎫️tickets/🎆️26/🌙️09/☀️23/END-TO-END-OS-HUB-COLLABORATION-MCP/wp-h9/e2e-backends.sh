#!/bin/zsh
# Two-client e2e (open, relay, presence, graceful restart, crash restart) on each named backend, one after the other,
# with this tree's all-driver os-hub and a copy of catalog B; data roots under .🧬semio/🌐hub/s11-h9-e2e.
# usage: e2e-backends.sh <capture-dir> <backend> [<backend>…]
OUT=$1; shift
ROOT=/Users/ueli/Documents/semio
cd "$ROOT/🌎️hub/📦️packages/🟦️typescript"
port=8011
for backend in "$@"; do
  echo "=== $backend start $(date +%T) port=$port"
  HUB_TWO_CLIENT_PORT=$port OS_HUB_BINARY="$ROOT/.🧬semio/🌐hub/s11-h9-bin/os-hub" OS_HUB_TRUSTED_CATALOG_SOURCE="$ROOT/.🧬semio/🌐hub/s11-h9-catalog-b/trusted-catalog" \
    HUB_E2E_DATA_PARENT="$ROOT/.🧬semio/🌐hub/s11-h9-e2e" HUB_E2E_RECEIPT="$OUT/two-client-$backend-receipt.json" \
    bun ./📜️script.ts two-client-e2e $backend > "$OUT/two-client-$backend.txt" 2>&1
  echo "=== $backend exit $? $(date +%T)"
  port=$((port + 1))
done
echo "=== all done $(date +%T)"

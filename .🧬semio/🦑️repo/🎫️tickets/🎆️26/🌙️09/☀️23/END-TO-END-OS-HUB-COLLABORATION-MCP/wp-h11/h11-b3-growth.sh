#!/bin/zsh
# 📈️ H11 (B3 wave): the document-growth e2e (24 documents grown together, SIGTERM restart, every document reopens and
# accepts more) on the current-tree os-hub + a copy of catalog B3, hub on 8012. usage: h11-b3-growth.sh <backend> <label>
cd "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
export OS_HUB_BINARY="$H/s13-h11-bin/os-hub-b3" OS_HUB_TRUSTED_CATALOG_SOURCE="$H/s13-w3-catalog-b3/trusted-catalog" HUB_E2E_DATA_PARENT="$H/s13-h11-e2e" HUB_TWO_CLIENT_PORT=${H11_GROWTH_PORT:-8012} HUB_E2E_RECEIPT="$H/s13-h11-logs/$2-receipt.json"
echo "=== start $(date +%T) backend=$1"
nice -n 10 bun ./📜️script.ts document-growth-e2e $1
echo "=== exit $? $(date +%T)"

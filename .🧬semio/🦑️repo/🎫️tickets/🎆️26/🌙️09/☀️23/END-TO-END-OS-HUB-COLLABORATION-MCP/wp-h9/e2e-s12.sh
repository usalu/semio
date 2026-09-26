#!/bin/zsh
# Real-binary hub e2e scenarios on each named backend, one after the other, with the H9 session-12 all-driver
# os-hub and a copy of catalog B; data roots under .🧬semio/🌐hub/s12-h9-e2e, captures under .🧬semio/🌐hub/s12-h9-logs.
# usage: e2e-s12.sh <scenario: two-client|document-growth> <tag> <backend>…
SCENARIO=$1; TAG=$2; shift 2
ROOT=/Users/ueli/Documents/semio
OUT="$ROOT/.🧬semio/🌐hub/s12-h9-logs"
mkdir -p "$OUT" "$ROOT/.🧬semio/🌐hub/s12-h9-e2e"
cd "$ROOT/🌎️hub/📦️packages/🟦️typescript"
port=${H9_PORT:-8011}
for backend in "$@"; do
  echo "=== $SCENARIO $backend start $(date +%T) port=$port"
  SEMIO_TEST_BUDGET_MS=${H9_BUDGET_MS:-7200000} HUB_TWO_CLIENT_PORT=$port OS_HUB_BINARY="$ROOT/.🧬semio/🌐hub/s12-h9-bin/os-hub" OS_HUB_TRUSTED_CATALOG_SOURCE="${H9_CATALOG:-$ROOT/.🧬semio/🌐hub/s11-h9-catalog-b/trusted-catalog}" \
    HUB_E2E_DATA_PARENT="$ROOT/.🧬semio/🌐hub/s12-h9-e2e" HUB_E2E_RECEIPT="$OUT/$SCENARIO-$TAG-$backend-receipt.json" \
    nice -n 15 bun ./📜️script.ts $SCENARIO-e2e $backend > "$OUT/$SCENARIO-$TAG-$backend.txt" 2>&1
  echo "=== $SCENARIO $backend exit $? $(date +%T)"
  port=$((port + 1))
done
echo "=== all done $(date +%T)"

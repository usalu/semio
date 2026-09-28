#!/bin/zsh
# 🗄️ H13 item 5: one os-hub-ts backend e2e (`two-client-e2e` | `document-growth-e2e`) on a backend (sqlite | postgres |
# neo4j) with the current-tree all-driver os-hub (hold 1's private-target build, copied + signed) and a copy of the p24 catalog (H13_CATALOG_SRC overrides),
# hub port 8012, durable data + receipt under .🧬semio/🌐hub/s14-h13-*. No cargo runs here.
# usage: h13-backend-e2e.sh <two-client-e2e|document-growth-e2e> <backend> <label>
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
cd "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false OS_HUB_BINARY="${H13_HUB_BINARY:-$H/s14-h13-bin/os-hub-all-drivers}" OS_HUB_TRUSTED_CATALOG_SOURCE="${H13_CATALOG_SRC:-$H/s14-w4-catalog-p24}/trusted-catalog" HUB_E2E_DATA_PARENT="$H/s14-h13-e2e" HUB_TWO_CLIENT_PORT=${H13_E2E_PORT:-8012} HUB_E2E_RECEIPT="$H/s14-h13-logs/$3-receipt.json"
mkdir -p "$HUB_E2E_DATA_PARENT"
LOG="$H/s14-h13-logs/$3.txt"
echo "=== start $(date +%T) $1 $2 binary=$OS_HUB_BINARY" | tee "$LOG"
nice -n 10 bun ./📜️script.ts "$1" "$2" >> "$LOG" 2>&1
rc=$?
echo "=== exit $rc $(date +%T)" | tee -a "$LOG"
/usr/bin/grep -E '✓|✗|×|Tests |FAIL|passed|failed|\[os-hub-ts\]' "$LOG" | cut -c1-260 | tail -25
exit $rc

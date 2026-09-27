#!/bin/zsh
# 💾️ H11 (B3 wave): V1's permanent backup/restore drill on the current-tree os-hub with a copy of catalog B3 (fresh roots, free
# loopback port, never 7800). usage: h11-b3-drill.sh <label>
cd "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript"
export OS_HUB_BINARY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-bin/os-hub-b3"
echo "=== start $(date +%T)"
nice -n 10 bun ./📜️script.ts backup-restore-drill --catalog-root "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-catalog-b3" --kind note --edits 20 --rounds 1
echo "=== exit $? $(date +%T)"

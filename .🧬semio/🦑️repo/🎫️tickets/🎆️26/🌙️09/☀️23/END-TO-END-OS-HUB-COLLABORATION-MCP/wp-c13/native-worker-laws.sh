#!/bin/zsh
# 🧪️ C13: the store worker's in-source laws + the pairing-rule law on the live tree, through the native lane (rule 21a). Superseded by native-host-laws.sh.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "👷️worker" "surface-opens-kind"
echo "EXIT rc=$? $(date '+%F %T')"

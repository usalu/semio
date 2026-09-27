#!/bin/zsh
# 🤝️ WG11: runs the staged permanent harness with the local test humans from the private env file (never printed).
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg11-harness-run.sh <harness args…>
set -u
source "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-wg11-state/humans.env"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-wg11 || exit 1
echo "[wg11-harness] start $(date '+%F %T') $*"
nice -n 5 bun harness/wg11-verb.ts "$@"
echo "[wg11-harness] rc=$? $(date '+%F %T')"

#!/bin/zsh
# 🤝️ WG9 run wrapper: runs wg9-browser-collab.mjs against hub 7800 (B3) with the given serve/space/document and tag.
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-collab-run.sh <tag> <shellUrl> <space> <document> [outage 0|1]
set -u
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-wg9 || exit 1
export WG7_HUB=http://127.0.0.1:7800 SEMIO_PROBE_URL="$2" WG7_SPACE="$3" WG7_DOCUMENT="$4" WG7_OUTAGE="${5:-0}" WG7_MODE="${6:-edits}"
echo "[wg9-run] $1 start $(date '+%F %T')"
nice -n 5 bun wg9-browser-collab.mjs "$1"
echo "[wg9-run] $1 rc=$? $(date '+%F %T')"

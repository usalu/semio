#!/bin/zsh
# 🤝️ WG11 run wrapper (from WG9's): runs wg11-browser-collab.mjs (two wasm32 wgpu browser shells) against an explicit hub.
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg11-collab-run.sh <tag> <hub> <shellUrl> <space> <document> [outage 0|1] [mode edits|cursors]
set -u
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-wg11 || exit 1
export WG7_HUB="$2" SEMIO_PROBE_URL="$3" WG7_SPACE="$4" WG7_DOCUMENT="$5" WG7_OUTAGE="${6:-0}" WG7_MODE="${7:-edits}"
echo "[wg11-run] $1 start $(date '+%F %T')"
nice -n 5 bun wg11-browser-collab.mjs "$1"
echo "[wg11-run] $1 rc=$? $(date '+%F %T')"

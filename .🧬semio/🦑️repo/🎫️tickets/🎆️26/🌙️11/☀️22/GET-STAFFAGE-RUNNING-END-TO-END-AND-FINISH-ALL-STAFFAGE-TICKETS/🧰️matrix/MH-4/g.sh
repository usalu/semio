#!/bin/zsh
# 🎟️ One script of the takeover under ONE 1000 MB grant of the memory queue (gates, scans, fits, the silhouette measure: each below 200 MB but the silhouette measure of two documents, 737 MB, see $GEN/rss.log): [GRANT=<MB>] g.sh <script.zsh> [timeout-s]   the script is sourced with the paths of env.sh; cap.sh calls of at most 1000 MB inside it run directly under the grant.
source "${0:A:h}/env.sh"
(cd "$R" && repo capacity run --memory=${GRANT:-1000} -- timeout "${2:-10000}" /bin/zsh -c 'export MH4_GRANT=$3 BATCH=MH-4; source "$1/env.sh"; source "$2"' g "$W" "${1:A}" "${GRANT:-1000}") >> "$GEN/capacity.log" 2>&1

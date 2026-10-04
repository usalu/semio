#!/usr/bin/env bash
# 🧍️ A second dev server of the quiz site that does not watch the sources (other sessions' edits never reload a page
# mid-check), beside the shared one: `bash steady_site.sh [port]` serves on 6065 and proxies to the proctor on 8791.
# Restart it to see an edit. It runs the site's own script with the shell's bun, because the preview launcher's older
# bun ends Vite's WebSocket proxy when a presence socket closes mid-handshake.
set -euo pipefail
cd "$(dirname "$0")/../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript"
TEACHING_ARCHITECTURE_QUIZ_PORT="${1:-6065}" PROCTOR_PORT=8791 TEACHING_ARCHITECTURE_QUIZ_WATCH=off exec bun ./📜️script.ts dev-site

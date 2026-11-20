#!/bin/sh
# Runs Playwright projects against the stack kept by `KEEP=1 node e2efix-stack.ts dev all` (ports 6070 and 8801).
cd "$(dirname "$0")/../../../../../../.." || exit 1
PLAYWRIGHT_BROWSERS_PATH="$HOME/Library/Caches/ms-playwright" PLAYWRIGHT_BASE_URL=http://127.0.0.1:6070 TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR=http://127.0.0.1:8801 TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL="${CONTROL:-http://127.0.0.1:1}" exec "$HOME/.local/share/semio/node/v24.15.0/bin/node" .cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/cli.js test --config "🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts" --output "${OUT:-/tmp/e2efix-pw}" "$@"

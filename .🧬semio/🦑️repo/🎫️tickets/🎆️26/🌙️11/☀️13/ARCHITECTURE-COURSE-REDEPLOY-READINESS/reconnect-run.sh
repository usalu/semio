#!/bin/sh
# 🔁️ Runs the reconnect-storm spec against the static stack (node reconnect-stack.mjs must run). Env: RECONNECT_LEARNERS RECONNECT_CYCLES RECONNECT_SEED
cd "$(dirname "$0")/../../../../../../.." || exit 1
export PATH=$HOME/.local/share/semio/node/v24.15.0/bin:$PATH
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/🗑️generated/reconnect"
. "$G/stack.env"
export PLAYWRIGHT_BROWSERS_PATH=$HOME/Library/Caches/ms-playwright RECONNECT_REPORT="${RECONNECT_REPORT:-$G/report.json}"
exec node "$PWD/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/cli.js" test --config "🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts" --project reconnect --no-deps --workers 1 --output "$G/pw"

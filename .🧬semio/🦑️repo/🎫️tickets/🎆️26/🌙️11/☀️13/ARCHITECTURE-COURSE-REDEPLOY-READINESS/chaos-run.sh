#!/bin/sh
# 🌪️ Reruns the chaos crowd spec against the native stack (node chaos-stack.mjs must run). Env: CHAOS_BROWSERS CHAOS_PROTOCOL CHAOS_QUIZZES CHAOS_KILLS CHAOS_INTENSITY
cd "$(dirname "$0")/../../../../../../.." || exit 1
export PATH=$HOME/.local/share/semio/node/v24.15.0/bin:$PATH
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/${CHAOS_WORK:-🗑️generated/chaos}"
. "$G/stack.env"
export PLAYWRIGHT_BROWSERS_PATH=$HOME/Library/Caches/ms-playwright CHAOS_REPORT="$G/report.json"
exec node "$PWD/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/cli.js" test --config "🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts" --project chaos --no-deps --workers 1 --output "$G/pw"

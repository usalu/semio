#!/usr/bin/env bash
# 🚦️ Ticket tool of work package P: runs specs of the architecture quiz site against a private stack of `wp_p_stack.sh`
# the way the end-to-end gate does — the desktop specs beside the pet spec, the phone spec beside them, a given number
# of workers — without the gate itself (which builds the proctor with cargo and takes the gate's own ports).
# Usage (from the repository root, with the stack up):
#   bash ".../wp_p_gate.sh" <dev|rehearsal> <label> [workers=2] [repeat=1] [specs=all desktop specs and the pet spec]
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
topology="${1:-dev}"
label="${2:-run}"
workers="${3:-2}"
repeat="${4:-1}"
specs="${5:-🪪️first-visit,🥞️layered-home,🎯️quiz-runs,🏆️live-leaderboard,🗣️both-languages,🐕️pet-walk}"
if [ "$topology" = "rehearsal" ]; then site=6194; proctor=8924; else site=6193; proctor=8923; fi
out="$ticket/🗑️generated/wp-p/gate-$topology-$label"
mkdir -p "$out"
cd "$ticket/../../../../../../.." || exit 1
WP_J_SPECS="$specs" PLAYWRIGHT_BASE_URL="http://127.0.0.1:$site" TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY="$topology" TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR="http://127.0.0.1:$proctor" \
  node node_modules/playwright/cli.js test --config "$ticket/wp_j_playwright.config.ts" --output "$out/results" --workers "$workers" --repeat-each "$repeat" > "$out/playwright.txt" 2>&1
code=$?
grep -E "^\s+[0-9]+ (passed|failed|flaky|skipped|did not run)" "$out/playwright.txt"
echo "exit=$code ($out/playwright.txt)"
exit $code

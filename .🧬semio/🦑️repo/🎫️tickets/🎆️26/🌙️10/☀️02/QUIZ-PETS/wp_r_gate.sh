#!/usr/bin/env bash
# 🚦️ Ticket tool of work package R: runs specs of the architecture quiz site against a private stack of
# `wp_r_stack.sh` with the configuration of `wp_j_playwright.config.ts` (the pet spec in its desktop project, the phone
# spec beside it) — never the end-to-end gate itself, which builds the proctor with cargo and takes the gate's ports.
# Usage (from the repository root, with the stack up):
#   bash ".../wp_r_gate.sh" <dev|rehearsal> <label> [workers=4] [repeat=1] [specs=🐕️pet-walk] [grep]
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
topology="${1:-dev}"
label="${2:-run}"
workers="${3:-4}"
repeat="${4:-1}"
specs="${5:-🐕️pet-walk}"
only="${6:-}"
if [ "$topology" = "rehearsal" ]; then site=6198; proctor=8928; else site=6197; proctor=8927; fi
out="$ticket/🗑️generated/wp-r/gate-$topology-$label"
mkdir -p "$out"
cd "$ticket/../../../../../../.." || exit 1
if [ -n "$only" ]; then
  WP_J_SPECS="$specs" PLAYWRIGHT_BASE_URL="http://127.0.0.1:$site" TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY="$topology" TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR="http://127.0.0.1:$proctor" \
    node node_modules/playwright/cli.js test --config "$ticket/wp_j_playwright.config.ts" --output "$out/results" --workers "$workers" --repeat-each "$repeat" --project desktop --grep "$only" > "$out/playwright.txt" 2>&1
else
  WP_J_SPECS="$specs" PLAYWRIGHT_BASE_URL="http://127.0.0.1:$site" TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY="$topology" TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR="http://127.0.0.1:$proctor" \
    node node_modules/playwright/cli.js test --config "$ticket/wp_j_playwright.config.ts" --output "$out/results" --workers "$workers" --repeat-each "$repeat" > "$out/playwright.txt" 2>&1
fi
code=$?
grep -E "^\s+[0-9]+ (passed|failed|flaky|skipped|did not run)" "$out/playwright.txt"
echo "exit=$code ($out/playwright.txt)"
exit $code

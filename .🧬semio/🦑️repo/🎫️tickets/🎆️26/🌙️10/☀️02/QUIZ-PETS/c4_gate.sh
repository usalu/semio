#!/usr/bin/env bash
# 🚦️ Ticket tool of work package C4: runs the `pets` project of the site's own end-to-end configuration
# (`🎭️e2e/🎚️config/🟦️.ts`, spec `🐕️pet-walk`) alone — `--no-deps`, so none of the projects it waits for in the gate runs —
# against the private stack of `c4_stack.sh` (site 6243, proctor 8943), never the gate itself. Besides the list it
# writes Playwright's JSON report (with the spec's annotations) to `report.json` of the run's folder.
# Usage (from the repository root, with the stack up):
#   bash ".../c4_gate.sh" <dev|rehearsal> <label> [workers=4] [grep]
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
topology="${1:-dev}"
label="${2:-run}"
workers="${3:-4}"
only="${4:-}"
out="$ticket/🗑️generated/c4/gate-$topology-$label"
mkdir -p "$out"
cd "$ticket/../../../../../../.." || exit 1
config="🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🎚️config/🟦️.ts"
start=$(date +%s)
filter=()
[ -n "$only" ] && filter=(--grep "$only")
PLAYWRIGHT_JSON_OUTPUT_NAME="$out/report.json" PLAYWRIGHT_BASE_URL="http://127.0.0.1:6243" TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY="$topology" TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR="http://127.0.0.1:8943" \
  node node_modules/playwright/cli.js test --config "$config" --project pets --no-deps --workers "$workers" --output "$out/results" --reporter=list,json "${filter[@]}" > "$out/playwright.txt" 2>&1
code=$?
grep -E "^\s+[0-9]+ (passed|failed|flaky|skipped|did not run)" "$out/playwright.txt"
echo "exit=$code seconds=$(( $(date +%s) - start )) ($out/playwright.txt)"
exit $code

#!/bin/sh
set -eu
cd /home/deck/semio
export SEMIO_LANGUAGE=en
export PLAYWRIGHT_BASE_URL=http://127.0.0.1:6061
export TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev
export PLAYWRIGHT_BROWSERS_PATH=/home/deck/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright
OUT=/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/verify-e2e-fixes
mkdir -p "$OUT/desktop" "$OUT/pets"
echo "DESKTOP_START $(date -Is)"
set +e
repo tools exec node -- node_modules/playwright/cli.js test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts --project=desktop --grep "balanced pools rotate bim-concepts and preserve individual answers|software mockups render and grade in de" --output "$OUT/desktop" --reporter=list
echo "DESKTOP_EXIT $?"
echo "PETS_START $(date -Is)"
repo tools exec node -- node_modules/playwright/cli.js test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts --project=pets --grep "no two pets ever overlap during a lively minute" --output "$OUT/pets" --reporter=list
echo "PETS_EXIT $?"
echo "DONE $(date -Is)"

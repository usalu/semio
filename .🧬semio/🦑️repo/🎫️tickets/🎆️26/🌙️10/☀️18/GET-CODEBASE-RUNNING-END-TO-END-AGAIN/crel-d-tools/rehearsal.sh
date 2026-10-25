#!/bin/sh
# Ticket input script: the rehearsal topology of the course end-to-end gate through the driver, run again (new run id) while it died on a peer's live
# dependency sync ("Playwright Test did not expect test() to be called here": two dependency generations under one run) or a build that read a half-written file.
#   rehearsal.sh <run id prefix> [topology]      topology: rehearsal (default) | both
here=$(cd "$(dirname "$0")" && pwd)
ticket=$(cd "$here/.." && pwd)
generated="$ticket/$(printf '\360\237\227\221\357\270\217generated')"
prefix=$1; topology=${2:-rehearsal}
attempt=0
while [ "$attempt" -lt 4 ]; do
  attempt=$((attempt + 1))
  id="$prefix$attempt"
  sh "$ticket/e2e-orch-tools/run.sh" course --phase e2e --course-topology "$topology" --go --skip-preflight --run-id "$id"
  code=$?
  log="$generated/e2e-course-${id#e2e-}-course-test-e2e.log"
  [ "$code" = 0 ] && exit 0
  if grep -q -e "did not expect test() to be called here" -e "Cannot find module" -e "build:graph-plan failed" -e "Stale loaded Source revision" -e "Original bounded Bun input file required" -e "Current shared Source-owner generation changed" -e "Source-owner custody changed" -e "outdated implementation" "$log" 2>/dev/null; then
    echo "[rehearsal.sh] attempt $attempt died on peer churn; again"
    sleep 60
    continue
  fi
  exit "$code"
done
exit 75

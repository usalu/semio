#!/bin/sh
# Ticket input script: the course e2e gate through the driver, run again (new run id) while it ended before a single spec ran (a peer's live edit, an infrastructure fault).
#   retry.sh <run id prefix> <topology: dev|rehearsal|both> [orch args...]     e.g. retry.sh e2e-1025f-d dev
here=$(cd "$(dirname "$0")" && pwd)
ticket=$(cd "$here/.." && pwd)
generated="$ticket/$(printf '\360\237\227\221\357\270\217generated')"
prefix=$1; topology=$2; shift 2
attempt=0
while [ "$attempt" -lt 8 ]; do
  attempt=$((attempt + 1))
  id="$prefix$attempt"
  sh "$ticket/e2e-orch-tools/run.sh" course --phase e2e --course-topology "$topology" --go --skip-preflight --run-id "$id" "$@"
  code=$?
  log="$generated/e2e-course-${id#e2e-}-course-test-e2e.log"
  [ "$code" = 0 ] && exit 0
  if ! grep -q -E '\] (dev|rehearsal): specs (passed|failed)' "$log" 2>/dev/null; then
    echo "[retry.sh] attempt $attempt ($id) ended before the specs finished; again in 60 s"
    sleep 60
    continue
  fi
  exit "$code"
done
exit 75

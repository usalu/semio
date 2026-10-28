#!/bin/sh
# Ticket input script: the course e2e gate through the driver, run again (new run id) while it ended before a single wave of specs finished (a peer's live edit, an infrastructure fault).
#   retry.sh <run id prefix> <topology: dev|rehearsal|both> [orch args...]     e.g. retry.sh e2e-1025f-d dev
# A wave that finished (the gate printed `[e2e] <stack>: <shard> passed|failed (<code>)`, see specs-finished.pattern) is a verdict and ends the loop, a green one with 0 and a red one with its exit code.
# RETRY_PAUSE (seconds, default 60) is the wait before a new attempt; the run log of an attempt is `e2e-course-<run id>-course-test-e2e.log` (the driver's stamp is `<run id>-course`).
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
  log="$generated/e2e-course-${id}-course-test-e2e.log"
  [ "$code" = 0 ] && exit 0
  if ! grep -a -q -E -f "$here/specs-finished.pattern" "$log" 2>/dev/null; then
    echo "[retry.sh] attempt $attempt ($id) ended before the specs finished; again in ${RETRY_PAUSE:-60} s"
    sleep "${RETRY_PAUSE:-60}"
    continue
  fi
  exit "$code"
done
exit 75

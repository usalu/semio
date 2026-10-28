#!/bin/bash
# Ticket input script: a focused course e2e run that is restarted when the run ends before a wave of specs finished (churn faults of live peer edits, owned by churn-d).
#   focus.sh <stamp> <topology|-> [playwright args...]     e.g. focus.sh d3 dev --grep "with 200% text"
# A finished wave (see ../e2e-f-tools/specs-finished.pattern) is a verdict: the loop ends with the exit code of the run.
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ticket="$(cd "$here/.." && pwd)"
stamp="$1"; topology="$2"; shift 2
gen="$ticket/$(printf '\xf0\x9f\x97\x91\xef\xb8\x8fgenerated')"
attempt=0
while [ "$attempt" -lt 6 ]; do
  attempt=$((attempt + 1))
  args=(course --tool=bazel)
  if [ "$topology" = "-" ]; then args+=(--release); else args+=("--topology=$topology"); [ "$topology" != dev ] && args+=(--release); fi
  [ $# -gt 0 ] && args+=(-- "$@")
  E2E_STAMP="$stamp-$attempt" nice -n 10 bash "$ticket/e2e-tools/run-e2e.sh" "${args[@]}"
  code=$?
  log="$gen/e2e-course-$stamp-$attempt-test-e2e.log"
  [ "$code" = 0 ] && exit 0
  if ! /usr/bin/grep -a -q -E -f "$ticket/e2e-f-tools/specs-finished.pattern" "$log" 2>/dev/null; then echo "[focus] attempt $attempt ended before the specs finished (a peer's live edit or a build fault); retrying"; sleep "${RETRY_PAUSE:-20}"; continue; fi
  exit "$code"
done
exit 75

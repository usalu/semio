#!/usr/bin/env bash
# Builds the listed plugins as browser components with at most $JOBS running at once, retrying each failure once. Usage: JOBS=2 native-queue.sh <plugin-id>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="${QUEUE_GENERATED:-$T/🗑️generated}"; JOBS="${JOBS:-2}"; DRIVER="${QUEUE_DRIVER:-$T/native-lanes.sh}"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
M="${QUEUE_MODULES:-$ROOT/.cache/b/p/m/browser/release/modules}"
mkdir -p "$G"
build() {
  local id="$1" attempt
  rm -f "$G/confirmed/$id.json"
  for attempt in 1 2; do
    [ -f "$G/browser-$id.log" ] && mv "$G/browser-$id.log" "$G/browser-$id.log.prev"
    FORCE=1 "$DRIVER" step "browser-$id" "" repo ./📜️script.ts components describe "$id" --facet browser; local rc=$?
    if [ "$rc" -eq 0 ] || { [ "$rc" -eq 75 ] && awk '/^\[native-build\] [0-9]+ unit\(s\),/ { accepted = /, 1 component\(s\) staged,/ && /, 0 failure\(s\) in .*; report / } END { exit !accepted }' "$G/browser-$id.log"; }; then
      if python3 "$T/release-receipts.py" record "$M" "$G/confirmed" "$id"; then
        echo "[queue ok] $id"
        return 0
      fi
    fi
    echo "[queue retry] $id attempt $attempt failed"
  done
  echo "[queue FAIL] $id"
  return 1
}
pids=()
failed=0
for id in "$@"; do
  while [ "$(jobs -pr | wc -l)" -ge "$JOBS" ]; do sleep 1; done
  build "$id" &
  pids+=("$!")
done
for pid in ${pids[@]+"${pids[@]}"}; do wait "$pid" || failed=1; done
echo "[queue done]"
exit "$failed"

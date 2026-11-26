#!/usr/bin/env bash
# Usage: JOBS=2 native-queue.sh [--pairs] <plugin-id>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="${QUEUE_GENERATED:-$T/🗑️generated}"; JOBS="${JOBS:-2}"; DRIVER="${QUEUE_DRIVER:-$T/native-lanes.sh}"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
M="${QUEUE_MODULES:-$ROOT/.cache/b/p/m/browser/release/modules}"
mkdir -p "$G"
staged() {
  awk -v count="$2" '/^\[native-build\] [0-9]+ unit\(s\),/ { accepted = $0 ~ (", " count " component\\(s\\) staged,") && /, 0 failure\(s\) in .*; report / } END { exit !accepted }' "$1"
}
build() {
  local id="$1" attempt
  rm -f "$G/confirmed/$id.json"
  for attempt in 1 2; do
    [ -f "$G/browser-$id.log" ] && mv "$G/browser-$id.log" "$G/browser-$id.log.prev"
    FORCE=1 "$DRIVER" step "browser-$id" "" repo ./📜️script.ts components describe "$id" --facet browser; local rc=$?
    if { [ "$rc" -eq 0 ] || [ "$rc" -eq 75 ]; } && [ -f "$G/browser-$id.log" ] && staged "$G/browser-$id.log" 1; then
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
build_pair() {
  local left="$1" right="$2" name="browser-pair-$1+$2" rc pending status=0
  rm -f "$G/confirmed/$left.json" "$G/confirmed/$right.json"
  [ ! -f "$G/$name.log" ] || mv "$G/$name.log" "$G/$name.log.prev"
  FORCE=1 "$DRIVER" step "$name" "" repo ./📜️script.ts components describe "$left" "$right" --facet browser
  rc=$?
  if { [ "$rc" -eq 0 ] || [ "$rc" -eq 75 ]; } && staged "$G/$name.log" 2 &&
     pending="$(mktemp -d "$G/pair.XXXXXX")"; then
    if python3 "$T/release-receipts.py" record "$M" "$pending" "$left" &&
       python3 "$T/release-receipts.py" record "$M" "$pending" "$right" &&
       mkdir -p "$G/confirmed" &&
       mv "$pending/$left.json" "$pending/$right.json" "$G/confirmed/"; then
      rmdir "$pending"
      echo "[queue ok] $left"
      echo "[queue ok] $right"
      return 0
    fi
    rm -rf "$pending"
    rm -f "$G/confirmed/$left.json" "$G/confirmed/$right.json"
  fi
  echo "[queue fallback] $left $right"
  build "$left" || status=1
  build "$right" || status=1
  return "$status"
}
if [ "${1:-}" = --pairs ]; then
  shift
  if [ "$JOBS" != 2 ] || [ "$#" -lt 2 ]; then
    echo 'pair mode requires JOBS=2 and at least two module IDs' >&2
    exit 2
  fi
  seen=" "
  for id in "$@"; do
    if [[ ! "$id" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ || "$seen" == *" $id "* ]]; then
      echo "invalid or repeated module ID: $id" >&2
      exit 2
    fi
    seen="$seen$id "
  done
  failed=0
  while [ "$#" -ge 2 ]; do
    build_pair "$1" "$2" || failed=1
    shift 2
  done
  if [ "$#" -eq 1 ]; then build "$1" || failed=1; fi
  echo "[queue done]"
  exit "$failed"
fi
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

#!/usr/bin/env bash
# Requeues each module until it stages, one at a time, after an earlier queue log settled. Usage: queue-until.sh <queue-log-to-await|-> <rounds> <plugin-id>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; await="$1"; rounds="$2"; shift 2
[ "$await" = - ] || until grep -q 'queue done' "$G/$await"; do sleep 3; done
for id in "$@"; do
  if [ "$await" != - ] && grep -qx "\[queue ok\] $id" "$G/$await"; then echo "[until ok] $id"; continue; fi
  for ((round = 1; round <= rounds; round++)); do
    JOBS=1 bash "$T/native-queue.sh" "$id" > "$G/queue-until-$id.log" 2>&1 && { echo "[until ok] $id"; break; }
    echo "[until round $round failed] $id: $(grep -hoE 'RepoError: .{0,220}|^error(\[|:).{0,160}' "$G/browser-$id.log" | tail -1)"
  done
done
echo "[until done]"

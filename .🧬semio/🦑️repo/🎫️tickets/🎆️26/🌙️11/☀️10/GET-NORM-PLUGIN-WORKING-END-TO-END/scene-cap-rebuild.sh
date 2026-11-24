#!/usr/bin/env bash
# 🏗️ Rebuilds the norm play component, retrying while the build reports exit 75.
T="$(cd "$(dirname "$0")" && pwd)"
cd "$(cd "$T/../../../../../../.." && pwd)" || exit 1
for attempt in $(seq 1 8); do
  repo "$T/native-exec.ts" "$T" ./build-play-components.ts build norm > "$T/🗑️generated/rebuild-norm-scene.log" 2>&1
  code=$?
  [ "$code" -ne 75 ] && break
done
echo "[DEBUG] rebuild exit $code" >> "$T/🗑️generated/rebuild-norm-scene.log"

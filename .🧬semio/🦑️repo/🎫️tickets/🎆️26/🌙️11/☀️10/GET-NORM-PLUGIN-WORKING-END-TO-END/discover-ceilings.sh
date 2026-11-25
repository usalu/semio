#!/usr/bin/env bash
# 🔭️ Builds each Standards component once and lists every compile unit over the library budget, so the carve list is measured and not guessed.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; ROOT="$(cd "$T/../../../../../../.." && pwd)"; cd "$ROOT"
status=0
for component in norm-eurocode norm-eurocode-materials norm norm-energy; do
  "$T/source-gate.sh"
  repo "$T/native-exec.ts" "$T" ./build-play-components.ts build "$component" > "$G/discover-$component.log" 2>&1
  code=$?
  if [ "$status" -eq 0 ]; then status=$code; fi
  echo "$(date +%H:%M) $component exit $code over: $(grep -aoE 'unit [a-z0-9_]+(@[a-z0-9_]+)?\|[^ ]+ peaked at [0-9]+ MB' "$G/discover-$component.log" | sed -E 's/unit ([a-z0-9_]+).*peaked at ([0-9]+) MB/\1=\2/' | sort -u | tr '\n' ' ') failed: $(grep -aoE 'could not compile `[^`]+`' "$G/discover-$component.log" | sort -u | tr '\n' ' ')" >> "$G/discover-summary.txt"
done
echo done >> "$G/discover-summary.txt"
exit "$status"

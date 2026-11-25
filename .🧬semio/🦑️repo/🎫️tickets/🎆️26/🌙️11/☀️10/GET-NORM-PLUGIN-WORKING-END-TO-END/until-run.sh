#!/usr/bin/env bash
# 🦀️ Re-runs run-editor-history.sh until the log holds a test result (compile cap retries), at most 4 attempts.
T="$(cd "$(dirname "$0")" && pwd)"
for n in 1 2 3 4; do
  "$T/run-editor-history.sh" "$@"
  grep -q "^test result" "$T/🗑️generated/$4" && break
  grep -q "^error\[" "$T/🗑️generated/$4" && break
done
echo finished > "$T/🗑️generated/$4.done"

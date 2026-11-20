#!/usr/bin/env bash
# Builds the release site and runs acceptance against the exact Vite output captured by that build.
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
MEM=4096 bash "$T/native-wasm.sh" site
OUT="$(python3 "$T/captured-site.py" "$ROOT" "$T/🗑️generated/native-site.log")"
echo "SITE $OUT"
if [[ "${VERIFY_QUEUE_RECEIPTS:-0}" == 1 ]]; then
  python3 "$T/release-receipts.py" verify "$ROOT/.cache/b/p/m/browser/release/modules" "$T/🗑️generated/confirmed" --site "$OUT"
fi
"$T/verify-static.sh" "$OUT" 8765

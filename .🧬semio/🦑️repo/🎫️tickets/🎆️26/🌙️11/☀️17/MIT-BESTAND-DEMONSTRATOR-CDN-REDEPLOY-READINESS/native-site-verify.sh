#!/usr/bin/env bash
# Builds the release site and runs acceptance against the exact Vite output captured by that build.
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
accepted="$T/🗑️generated/static-accepted.sha256"
before="$T/🗑️generated/static-before.sha256"
if [[ "${VERIFY_QUEUE_RECEIPTS:-0}" == 1 ]]; then rm -f "$accepted"; fi
capture=$(mktemp "$T/🗑️generated/native-site-verify.XXXXXX")
NATIVE_SITE_RUN_LOG="$capture" MEM=4096 bash "$T/native-wasm.sh" site
OUT="$(python3 "$T/captured-site.py" "$ROOT" "$capture")"
echo "SITE $OUT"
if [[ "${VERIFY_QUEUE_RECEIPTS:-0}" == 1 ]]; then
  python3 "$T/site-manifest.py" "$OUT" "$ROOT" "$before"
  python3 "$T/release-receipts.py" verify "$ROOT/.cache/b/p/m/browser/release/modules" "$T/🗑️generated/confirmed" --site "$OUT"
fi
"$T/verify-static.sh" "$OUT" 8765
if [[ "${VERIFY_QUEUE_RECEIPTS:-0}" == 1 ]]; then
  after="$T/🗑️generated/static-after.sha256"
  python3 "$T/site-manifest.py" "$OUT" "$ROOT" "$after"
  cmp "$before" "$after"
  mv "$after" "$accepted"
fi

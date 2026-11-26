#!/usr/bin/env bash
# Builds the release site and runs acceptance against the exact Vite output captured by that build.
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
generated="$T/🗑️generated"
mkdir -p "$generated"
lock="$generated/native-site-build.lock"
if ! mkdir "$lock" 2>/dev/null; then echo "another native site build owns $lock" >&2; exit 1; fi
trap 'rmdir "$lock"' EXIT
repo tools exec --light python -- -I "$T/release-receipts.py" status "$ROOT/.cache/b/p/m/browser/release/modules" "$generated/confirmed" >&2
capture=$(mktemp "$T/🗑️generated/native-site-verify.XXXXXX")
NATIVE_SITE_RUN_LOG="$capture" MEM="${MEM:-3000}" bash "$T/native-wasm.sh" site
OUT="$(repo tools exec --light python -- -I "$T/captured-site.py" "$ROOT" "$capture")"
echo "SITE $OUT"
repo tools exec --light node -- "$T/verify-flow-browser.mjs" --site "$OUT" "$ROOT/.cache/b/p/m/browser/release/flow"
support=$(mktemp "$generated/renderer-current.XXXXXX")
repo tools exec --light bun -- "$T/renderer-diagnostic-assets.ts" --inventory > "$support"
repo tools exec --light python -- -IB "$T/verify-renderer-http.py" "$OUT" "$support"
repo tools exec --light python -- -I "$T/accept-site.py" "$OUT"
repo tools exec --light python -- -I "$T/package-site.py" "$OUT" "$ROOT/.cache/deploy" "$generated/static-accepted.sha256"

#!/bin/sh
# ✅ laws of batch `builda` (lane isolate): sh verify.sh <root>; stage check, parse scan, then every staged test file runs (plain bun test, the official envelope when it asks for one) and may fail only tests of its recorded failure set (expected.json, measured on mirror is1, equal to the base set), must not hit the live-store fence, and (once the law module is present at <root>) carries no test-store-isolation finding. Expect `VERIFY OK builda`.
exec python3 -I "$(cd "$(dirname "$0")/../.." && pwd)/verify_batch.py" "$(cd "$(dirname "$0")" && pwd)" "${1:?root}"

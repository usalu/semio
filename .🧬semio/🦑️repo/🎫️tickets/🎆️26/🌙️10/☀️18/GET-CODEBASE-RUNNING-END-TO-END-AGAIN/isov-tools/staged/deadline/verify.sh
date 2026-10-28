#!/bin/sh
# ✅ laws of batch `deadline` (lane isov): sh verify.sh <root>; stage check, parse scan, every staged test file runs (plain bun test, the official envelope when it asks for one) and fails only tests of its recorded set (expected.json), no live-store fence refusal, test-store-isolation law 0 findings. Expect `VERIFY OK deadline`.
exec python3 -I "$(cd "$(dirname "$0")/../.." && pwd)/verify_batch.py" "$(cd "$(dirname "$0")" && pwd)" "${1:?root}"

#!/bin/sh
# 🪞 law of batch `productparse` (lane dashnoop): sh verify.sh <root> [before]
#   after  (default): the runner patterns find every summary line of every recorded run log, singular included (law 2/0)
#   before : product.py restored to base, the law kept: RED (the singular `1 process` lines are missed); the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gate-g-tools"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
if [ "$MODE" = before ]; then
  cp "$HERE/base/f000" "$ROOT/$G/product.py"
  if python3 -I "$ROOT/$G/tests/test_product_patterns.py" >/dev/null 2>&1; then echo "FAIL  law is green on base"; FAILED=1; else echo "OK    law RED on base"; fi
  cp "$HERE/files/f000" "$ROOT/$G/product.py"
  python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK productparse (before)" || echo "VERIFY FAIL productparse (before)"; exit $FAILED
fi
python3 -I "$ROOT/$G/tests/test_product_patterns.py" 2>&1 | tail -4 || FAILED=1
python3 -I "$ROOT/$G/tests/test_product_patterns.py" >/dev/null 2>&1 && echo "OK    law green" || { echo "FAIL  law"; FAILED=1; }
[ $FAILED = 0 ] && echo "VERIFY OK productparse" || echo "VERIFY FAIL productparse"; exit $FAILED

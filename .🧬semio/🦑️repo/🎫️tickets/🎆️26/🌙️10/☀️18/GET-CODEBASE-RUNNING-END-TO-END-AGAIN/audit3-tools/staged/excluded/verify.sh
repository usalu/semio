#!/bin/sh
# 🕳 Laws of batch `excluded` (lane audit3). usage: [RED_ON_BASE=1] sh verify.sh <MIRROR root>   (needs `git` and `bun`; the laws write only below $TMPDIR)
set -eu
ROOT="${1:?mirror root required}"; HERE="$(cd "$(dirname "$0")" && pwd)"; cd "$ROOT"
B="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; LAW="$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🕳️excluded-names/🟦️.ts"; SCRIPT="$B/🔧️tools/🏗️bazel/📜️script.ts"
python3 -I "$HERE/stage.py" apply --root "$ROOT"
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  cp "$SCRIPT" "$SCRIPT.audit3-keep"; cp "$HERE/base/00.ts" "$SCRIPT"
  if bun test "$LAW"; then mv "$SCRIPT.audit3-keep" "$SCRIPT"; echo "RED_ON_BASE: the law passed on the base script (expected a failure)"; exit 1; fi
  mv "$SCRIPT.audit3-keep" "$SCRIPT"; echo "RED_ON_BASE: law fails on the base script as expected"
fi
bun test "$LAW" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts" "$B/🔧️tools/🏗️bazel/🌌️universe/🧪️tests/🟦️.ts" "$B/🔭️witness/🧪️tests/🟦️.ts" "$B/🔭️witness/🧪️tests/🖥️session/🟦️.ts"
echo "excluded VERIFIED"

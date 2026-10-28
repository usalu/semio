#!/bin/sh
# 🔒️ Laws of batch `tmpdirep` after apply: sh verify.sh <mirror-root>   (a MIRROR root with the batch applied, never the live checkout)
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); XPLAT5=$(cd "$HERE/../../../xplat5-tools" && pwd); . "$XPLAT5/verify-common.sh"
S='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service'
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
law "endpoint directory: TMPDIR only, Python reading of os.TempDir" bun test "./$S/🧪️tests/🟦️.ts" -t "without XDG_RUNTIME_DIR"
law "service core (whole file)" bun test "./$S/🧪️tests/🟦️.ts"
cp "$S/🔑️session/🟨️.mjs" "$ROOT/.new.mjs"; cp "$HERE/base/00.mjs" "$S/🔑️session/🟨️.mjs"
out=$(bun test "./$S/🧪️tests/🟦️.ts" -t "without XDG_RUNTIME_DIR" 2>&1); code=$?; cp "$ROOT/.new.mjs" "$S/🔑️session/🟨️.mjs"; rm -f "$ROOT/.new.mjs"
if [ $code -eq 0 ]; then echo "RED   negative control DID NOT BITE with the base session module"; FAILED=1; else echo "GREEN negative control: red with the base session module"; fi
exit $FAILED

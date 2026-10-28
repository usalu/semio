#!/bin/sh
# 🪟️ Laws of batch `native` after apply: sh verify.sh <MIRROR root>   (never the live checkout: the negative controls rewrite files). Needs bun and python3; process laws run without the macOS seal (sandbox-exec changes process liveness). Nothing here runs on Windows: the kernel is a model that validates every call; the Windows-only law compares the kernel chain with the CIM chain on the first Windows host.
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../../.." && pwd); XPLAT5="$C/xplat5-tools"; . "$XPLAT5/verify-common.sh"
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" >/dev/null || { echo "batch not applied in $ROOT"; exit 2; }
P="🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions"; NATIVE="./$P/🧭️ancestry/🪟️native/🧪️tests/🟦️.ts"; CORE="./$P/🧪️tests/🟦️.ts"
law "native listing laws (layout + FILETIME oracle, corpus equivalence, call model, degradations, session resolver)" bun test "$NATIVE"
law "agent-session laws keep their vectors (schema, ancestry readers, resolver)" bun test "$CORE"
law "parse check of every new or changed TS (Bun transpile + node --check)" sh -c "python3 -I -c \"import json,sys;print('\n'.join(r for r in json.load(open(sys.argv[1],encoding='utf8')) if r.endswith('.ts')))\" '$HERE/manifest.json' >'$C/🗑️generated/winps/scan-list.txt' && bun '$C/gate-g-tools/scan.ts' '$C/🗑️generated/winps/scan-list.txt' '$C/🗑️generated/winps/scan' | tail -1 | grep -q 'bad 0$'"
for id in parent leak floor recycled command size; do
 python3 -I "$XPLAT5/swap.py" "$HERE" "$ROOT" new; python3 -I "$HERE/mutate.py" "$ROOT" "$id" || { echo "RED   mutation $id could not be applied"; FAILED=1; continue; }
 out=$(bun test "$NATIVE" 2>&1); code=$?; python3 -I "$XPLAT5/swap.py" "$HERE" "$ROOT" new
 if [ $code -eq 0 ]; then echo "RED   negative control '$id' DID NOT BITE"; FAILED=1; else echo "GREEN negative control '$id': red ($(printf '%s\n' "$out" | grep -cE '^\(fail\)') failing tests)"; fi
done
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v '^applied' ; exit $FAILED

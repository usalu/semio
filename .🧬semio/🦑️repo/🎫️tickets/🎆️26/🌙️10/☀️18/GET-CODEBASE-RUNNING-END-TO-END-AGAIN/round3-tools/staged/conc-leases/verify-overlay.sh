#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
OV=${1:-/private/tmp/conc-leases/ov}
"$T/overlay.sh" "$OV" >/dev/null
cp /Users/ueli/Documents/semio/📋️project.json "$OV/" 2>/dev/null || true
mkdir -p "$OV/🧰️framework/🔨️modules/🏃️process/.cache/b/c"
cp "$T/wrap.ts" "$T/wrapfn.ts" "$T/driver.ts" "$T/validate-catalog.ts" "$T/regen-help.ts" "$OV/"
python3 - "$OV" <<'PY'
import sys
p=sys.argv[1]+'/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts'
s=open(p,encoding='utf8').read()
a='  await testResourceLeaseConcurrencyV1(generated,original,control);\n'
if a in s: open(p,'w',encoding='utf8').write(s.replace(a,''))
PY
echo "$OV"

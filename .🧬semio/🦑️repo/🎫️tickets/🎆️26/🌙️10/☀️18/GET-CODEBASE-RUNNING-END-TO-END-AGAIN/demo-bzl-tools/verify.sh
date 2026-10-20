#!/usr/bin/env bash
# usage: verify.sh [overlay-root]   static verification of the staged set: apply dry run, independent python check, law + ajv, print layout cross-check (TypeScript projection vs the python derivation).
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"; ov="${1:-/private/tmp/dbz/ov}"
python3 "$here/apply.py" --root "$ov" --accept-drift | tail -7 || exit 1
python3 -I "$here/verify-independent.py" "$ov" | command grep -E "FAIL|^ok" | awk '{print $1}' | sort | uniq -c
perl -e 'alarm 120; exec @ARGV' -- bun "$here/law-check.ts" "$ov" | command grep -v '\[\]$'
ts="$(perl -e 'alarm 60; exec @ARGV' -- bun "$here/layout.ts" | sort)"
py="$(python3 -I - <<'PY'
import json
ns=json.load(open("/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/📦️artifacts/🧬️schema/🔣️.json"))["definitions"]
for i in ("forschungsbericht","kompaktbericht","zwischenbericht"): print("{workspaceRoot}/.cache/b/p/m/%s/{options.env.SEMIO_BUILD_MODE}/documents/report/%s"%(ns["Protocol"]["const"]["namespace"],i))
PY
)"
[ "$ts" = "$py" ] && echo "layout cross-check: equal" || { echo "layout cross-check: DIFFER"; exit 1; }

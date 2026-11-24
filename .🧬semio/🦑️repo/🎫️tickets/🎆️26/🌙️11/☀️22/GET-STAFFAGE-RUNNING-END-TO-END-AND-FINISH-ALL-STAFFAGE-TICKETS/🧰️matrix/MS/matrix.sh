#!/bin/zsh
# 📊️ The confirmation of repaired assets, one asset after the other in the foreground: `T/📊️matrix.ts --language en --out 🗑️generated/matrix/MS/matrix/<id> --jobs 1 --assets <id>` and one line per asset with the cells by status and the failing cells by check, appended to 🗑️generated/matrix/MS/proof/matrix.txt: matrix.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  out="$GEN/matrix/$a"; mkdir -p "$out"
  (cd "$T" && timeout ${TMO:-7000} "$B" "📊️matrix.ts" --language en --out "$out" --jobs 1 ${=FRESH---fresh} --assets "$a") > "$out.log" 2>&1
  echo "$a | matrix: exit $? $(python3 -c '
import json,sys,collections
cells=json.load(open(sys.argv[1]))
cells=cells["cells"] if isinstance(cells,dict) else cells
status=collections.Counter(c["status"] for c in cells)
checks=collections.Counter()
for c in cells:
    if c["status"]=="fail":
        for name in sorted(c.get("failures") or {"?": 0}): checks[name]+=1
print(len(cells),"cells",dict(status),"failing cells by check",dict(checks))
' "$out/matrix.json" 2>&1 | tail -1)" | tee -a "$GEN/proof/matrix.txt"
done

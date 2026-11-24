#!/bin/sh
# 🆎 Alternates the English journey with the progress back-off on and patched off in the test browser.
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"; G="$T/🗑️generated"
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
for n in $(seq 1 "$2"); do for v in on off; do
  VARIANT=$v timeout 400 repo tools exec --light bun -- "$T/journey-ab.mjs" --serve http://127.0.0.1:6064/ --locale en --output "$G/journey-$1-$v$n" > "$G/ab-$1-$v$n.log" 2>&1
  python3 - "$G/journey-$1-$v$n/report.json" $v $n <<'P'
import json,sys
try: r=json.load(open(sys.argv[1])); print(sys.argv[2:],len(r["steps"]),r.get("failure","ok")[:140].replace("\n"," "),r["errors"][:1],r.get("unpatched",""))
except Exception as e: print(sys.argv[2:],"no report",e)
P
  uptime | sed 's/.*averages/load/'
done; done

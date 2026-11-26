#!/bin/sh
# 🏃️ Runs the history/import-export journey and prints the failing verdicts.
# Usage: sh run-history.sh [en|de ...]   (run from the workspace root)
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
BUN=$(ls -d ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
JOURNEY=history "$BUN" "$T/journey-history.ts" "$@" 2>&1 | tail -120
python3 - "$T" <<'PY'
import json,sys
for v in json.load(open(sys.argv[1]+"/🗑️generated/journey/history.json")):
    if not v["pass"]: print("FAIL", v["step"]); print("   ", json.dumps(v["evidence"],ensure_ascii=False)[:900])
PY

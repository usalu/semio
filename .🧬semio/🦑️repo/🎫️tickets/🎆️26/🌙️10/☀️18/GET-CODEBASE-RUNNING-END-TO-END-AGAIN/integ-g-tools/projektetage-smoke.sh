#!/bin/sh
# usage: projektetage-smoke.sh <cycle-id> ; headless dev smoke of the projektetage deck on 6050 (print-f's pres-d-tools/dev.sh + smoke.mjs), then stops the server it started (own process group only)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/integ-g"; id="$1"
unset SEMIO_SCRIPT_STALL_MS; export SEMIO_LANGUAGE=en
exe=$(ls -d "$HOME"/Library/Caches/ms-playwright/chromium-1234/chrome-mac*/"Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing" 2>/dev/null | head -1)
python3 - "$C/pres-d-tools/dev.sh" "$G/$id.projektetage-dev.log" "$G/$id.projektetage.pid" <<'PY'
import os, subprocess, sys
p = subprocess.Popen(["sh", sys.argv[1]], stdout=open(sys.argv[2], "wb"), stderr=subprocess.STDOUT, start_new_session=True)
open(sys.argv[3], "w").write(str(p.pid))
PY
pid=$(cat "$G/$id.projektetage.pid")
up=0
for i in $(seq 1 180); do
  if curl -fs -o /dev/null http://127.0.0.1:6050/; then up=1; break; fi
  kill -0 "$pid" 2>/dev/null || break
  sleep 5
done
rc=1
if [ $up = 1 ]; then SEMIO_PROBE_EXE="$exe" bun "$C/pres-d-tools/smoke.mjs" 6050 "$G/$id.projektetage" > "$G/$id.projektetage.verdict" 2>&1; grep -q '"pass":true' "$G/$id.projektetage.verdict" && rc=0; fi
kill -TERM -- "-$pid" 2>/dev/null; sleep 5; kill -KILL -- "-$pid" 2>/dev/null
echo "projektetage-dev up=$up rc=$rc $(tail -c 300 "$G/$id.projektetage.verdict" 2>/dev/null)"
exit $rc

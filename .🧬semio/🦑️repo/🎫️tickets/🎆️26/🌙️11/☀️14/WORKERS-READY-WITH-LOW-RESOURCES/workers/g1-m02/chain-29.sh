#!/bin/zsh
cd /Users/ueli/Documents/semio
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m02"
while kill -0 67422 2>/dev/null; do python3 -c "import time; time.sleep(20)"; done
for i in 1 2 3 4 5 6 7 8 9 10; do
  (cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica && repo ./📜️script.ts status --lang en) > $D/round-29.replica-status-$i.txt 2>&1
  grep -aqi "diverged" $D/round-29.replica-status-$i.txt || break
  python3 -c "import time; time.sleep(180)"
done
python3 -u $D/run-round-29.py 2>&1 | grep --line-buffered -E "Z (TRY|STOP|STALL|STORE|ROUND|NEWER|CONSENT|ZERO|three|ACCESS|same)" | cut -c1-420

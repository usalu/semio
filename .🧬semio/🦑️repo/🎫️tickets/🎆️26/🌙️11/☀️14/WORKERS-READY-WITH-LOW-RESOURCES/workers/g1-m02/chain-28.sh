#!/bin/zsh
cd /Users/ueli/Documents/semio
while kill -0 451 2>/dev/null; do python3 -c "import time; time.sleep(20)"; done
python3 -u .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m02/run-round-28.py 2>&1 | grep --line-buffered -E "Z (TRY|STOP|STALL|STORE|ROUND|NEWER|CONSENT|ZERO|three|ACCESS|same)" | cut -c1-420

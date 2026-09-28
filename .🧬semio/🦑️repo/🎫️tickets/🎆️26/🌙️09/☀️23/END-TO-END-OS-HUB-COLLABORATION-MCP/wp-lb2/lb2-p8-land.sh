#!/bin/zsh
# 🪪️ LB2 p8 landing (host TS, rule 20): apply → renderer-react typecheck + Interpreter vitest (native lane) → one Home boot on
# LB2's own serve 6630 (local-only, HMR off; stopped right after) → revert on any red. Capture: wp-lb2/generated/<name>.txt
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
R="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
{
echo "START $(date '+%H:%M:%S') apply"
python3 .tmp-ticket/wp-lb2/lb2-p8-row-scoped-dom-ids.py --write || exit 3
NX_DAEMON=false zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c "cd '$R' && nice -n 15 bun ./📜️script.ts typecheck; echo TYPECHECK rc=\$?; nice -n 15 bun ./📜️script.ts test long '🗣️Interpreter/🟦️.tsx'; echo VITEST rc=\$?" 2>&1 | tee .tmp-ticket/wp-lb2/generated/$1-ts.txt | /usr/bin/grep -E 'rc=|error TS|Tests |FAIL'
if /usr/bin/grep -qE 'TYPECHECK rc=[1-9]|VITEST rc=[1-9]' .tmp-ticket/wp-lb2/generated/$1-ts.txt; then python3 .tmp-ticket/wp-lb2/lb2-p8-row-scoped-dom-ids.py --revert; echo "P8 REVERTED (ts)"; exit 4; fi
if lsof -nP -iTCP:6630 -sTCP:LISTEN >/dev/null 2>&1; then echo "port 6630 busy"; exit 5; fi
pid=$(python3 .tmp-ticket/wp-w2/w2-detach.py "/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/generated/$1-serve.txt" zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-s18/s18-serve-local.sh 6630)
echo "SERVE pid=$pid $(date '+%H:%M:%S')"
for i in {1..120}; do curl -s -o /dev/null -m 2 http://127.0.0.1:6630/ && break; sleep 5; done
echo "SERVE up after ~$((i*5)) s $(date '+%H:%M:%S')"
bun .tmp-ticket/wp-lb2/lb2-boot-probe.mjs http://127.0.0.1:6630/ en-US
kill -TERM -- "-$pid" 2>/dev/null; sleep 5; ps -axo pid,pgid,command | awk -v g="$pid" '$2==g' | head -3; lsof -nP -iTCP:6630 -sTCP:LISTEN | tail -1
echo "END $(date '+%H:%M:%S')"
} > "$capture" 2>&1

#!/bin/bash
# runs in the container: one replica, seal, sabotage, restart, inspect
M=$(cd /mir && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree)
S="$M/🔁️replica/📜️script.ts"
export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@e.test GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@e.test GIT_CONFIG_GLOBAL=/dev/null
mkdir -p /lw /store && cd /lw && git init -q -b main && printf '.cache/\n🧰️framework/\n' > .gitignore && echo a > a.txt && git add -A && git commit -qm i
bun /mir/$S seed --group g1 --origin o1 --worktree /lw --store /store --lang en | tail -1
cp -a /mir/🧰️framework /lw/
start(){ (cd /lw && setsid bun /lw/$S run --worktree /lw --store /store --poll 100 --scan 5 --lang en >> /d.log 2>&1 &) ; }
killall_(){ for p in $(ps -eo pid,args | grep -E "bun .*(script|stage).ts" | grep -v grep | awk '{print $1}'); do kill -9 $p; done; sleep 1; }
start; for i in $(seq 1 60); do ls /lw/.cache/f/code/*/sealed.json >/dev/null 2>&1 && break; sleep 1; done; ls /lw/.cache/f/code
ps -eo pid,args | grep -E "(script|stage).ts" | grep -v grep | cut -c1-160
killall_
f=$(ls -d /lw/$M/*tiers)/🟦️.ts; sed -i 's/ adopt(mtime:/ adoptStale(mtime:/' $f; grep -c adoptStale $f
: > /d.log; start; sleep 25
echo "--- log"; grep -v '"event":"\(progress\|applied\|published\)"' /d.log | cut -c1-300 | head -20
echo "--- beat"; cat /lw/.cache/f/boot/beat.json; echo; echo "--- attestation"; cat /store/v1/attest/*o1* 2>/dev/null | head -c 600; ls /store/v1 | head
echo "--- ps"; ps -eo pid,args | grep -E "(script|stage).ts" | grep -v grep | cut -c1-200
echo "--- history"; cat /lw/.cache/f/code/history.json

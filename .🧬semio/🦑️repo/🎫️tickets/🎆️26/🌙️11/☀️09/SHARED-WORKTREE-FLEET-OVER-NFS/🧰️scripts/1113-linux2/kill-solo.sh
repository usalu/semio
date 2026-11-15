#!/bin/bash
M=$(cd /mir && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree)
S="$M/🔁️replica/📜️script.ts"
export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@e.test GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@e.test GIT_CONFIG_GLOBAL=/dev/null
mkdir -p /lw /store && cd /lw && git init -q -b main && printf '.cache/\n🧰️framework/\n' > .gitignore && echo a > a.txt && git add -A && git commit -qm i
bun /mir/$S seed --group g1 --origin o1 --worktree /lw --store /store --lang en | tail -1
cp -a /mir/🧰️framework /lw/
docker_exec_d(){ (cd /lw && setsid bash -c "exec bun /lw/$S run --worktree /lw --store /store --poll 100 --scan 5 --lang en >> /d.log 2>&1" &); }
docker_exec_d; sleep 12
ps -eo pid,args | cut -c1-140
echo ---
bash -c 'for p in $(ps -eo pid,args | grep -E "^ *[0-9]+ (/usr/local/bin/)?bun .*(script|stage)\.ts" | grep -v -e grep -e "bash -c" | awk "{print \$1}"); do echo kill $p; kill -9 $p; done'
sleep 1; ps -eo pid,args | cut -c1-140

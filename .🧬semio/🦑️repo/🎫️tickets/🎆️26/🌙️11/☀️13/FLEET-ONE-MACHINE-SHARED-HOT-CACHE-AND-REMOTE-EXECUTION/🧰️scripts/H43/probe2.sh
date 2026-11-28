#!/bin/bash
# H43: run the narrow inputs command for at most $2 s; at each offset sample the youngest bun process below an --eval parent (the slice worker of this read)
S=$1; LIMIT=${2:-150}; mkdir -p "$S"; cd /Users/ueli/Documents/semio || exit 1
START=$(date +%s)
perl -e 'alarm shift; exec @ARGV' "$LIMIT" env SEMIO_REPO_PROFILE=1 nice -n 10 repo inputs repo-graph-maintenance:test >"$S/run.out" 2>"$S/run.err" &
CP=$!
( while kill -0 $CP 2>/dev/null; do echo "$(( $(date +%s) - START )) $(ps -axo pid,etime,command | /usr/bin/grep -a 'hdiutil\|diskutil\|diskimages\|hdiejectd' | /usr/bin/grep -av grep | awk '{print $1":"$3}' | tr '\n' ' ')" >> "$S/disk.txt"; sleep 1; done ) &
for T in 22 36 50; do
  while [ $(( $(date +%s) - START )) -lt $T ]; do sleep 1; done
  ps -axo pid,ppid,etime,rss,pcpu,command | cut -c1-120 > "$S/all-$T.txt"
  W=$(ps -axo pid,etime,rss,command | awk -v t=$T '$2 ~ /^00:[0-9][0-9]$/ && $3>300000 && /cache\/b\/t\/m\/bun/ {split($2,a,":"); e=a[1]*60+a[2]; d=t-6-e; if(d<0)d=-d; print d, $1}' | sort -n | head -1 | awk '{print $2}')
  echo "worker $W" > "$S/w-$T.txt"
  [ -n "$W" ] && lsof -p "$W" 2>/dev/null | awk '{print $4,$5,$9,$10}' | /usr/bin/grep -av "/usr/lib\|/System\|/dev/\|cwd\|txt" > "$S/lsof-$T.txt"; (cd /Users/ueli/Documents/semio && nice -n 10 repo leases status > "$S/leases-$T.txt" 2>&1)
done
wait $CP

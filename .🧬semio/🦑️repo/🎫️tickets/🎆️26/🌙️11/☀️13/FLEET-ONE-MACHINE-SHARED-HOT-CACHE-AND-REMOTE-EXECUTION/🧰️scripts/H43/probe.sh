#!/bin/bash
# H43: run `repo inputs repo-graph-maintenance:test` for at most $2 s, snapshotting processes, leases and a native sample of the newest slice worker at the given offsets
S=$1; LIMIT=${2:-170}; mkdir -p "$S"; cd /Users/ueli/Documents/semio || exit 1
START=$(date +%s)
perl -e 'alarm shift; exec @ARGV' "$LIMIT" env SEMIO_REPO_PROFILE=1 nice -n 10 repo inputs repo-graph-maintenance:test >"$S/run.out" 2>"$S/run.err" &
CP=$!
for T in 40 70; do
  while [ $(( $(date +%s) - START )) -lt $T ]; do sleep 1; done
  ps -axo pid,ppid,lstart,etime,rss,pcpu,command | /usr/bin/grep -a "bun --eval" | /usr/bin/grep -av grep | cut -c1-110 > "$S/ps-$T.txt"
  ps -axo pid,ppid,etime,rss,pcpu,command | cut -c1-150 > "$S/all-$T.txt"
  W=$(ps -axo pid,etime,command | /usr/bin/grep -a "bun --eval" | /usr/bin/grep -av grep | awk '{n=split($2,a,/[-:]/); s=0; for(i=1;i<=n;i++) s=s*60+a[i]; print s, $1}' | sort -n | head -1 | awk '{print $2}')
  echo "newest bun --eval: $W at $T" >> "$S/ps-$T.txt"
  [ -n "$W" ] && sample "$W" 3 -file "$S/sample-$T.txt" >/dev/null 2>&1
  lsof -p "$W" -Fn 2>/dev/null | /usr/bin/grep -a "^n/" | /usr/bin/grep -av "/usr/lib\|/System\|/dev/" | cut -c1-200 > "$S/lsof-$T.txt"
done
wait $CP

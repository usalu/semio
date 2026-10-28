#!/bin/sh
# sample-child.sh <outdir> -- <command...>: starts the command, finds the first `--eval` executor child of it, runs `sample` on it for ~4 s (macOS), waits.
out=$1; shift; shift; mkdir -p "$out"
SEMIO_REPO_PROFILE=1 "$@" > "$out/run.txt" 2>&1 &
top=$!
i=0; kid=
while [ $i -lt 400 ]; do
  kid=$(ps -axo pid=,ppid=,command= | awk -v p=$top '$2!=""{ } {print}' | grep -- "--eval" | grep -v grep | awk '{print $1}' | while read k; do pp=$(ps -o ppid= -p $k | tr -d ' '); anc=$pp; for n in 1 2 3 4; do [ "$anc" = "$top" ] && echo $k && break; anc=$(ps -o ppid= -p $anc 2>/dev/null | tr -d ' '); [ -z "$anc" ] && break; done; done | head -1)
  [ -n "$kid" ] && break
  i=$((i+1)); sleep 0.05
done
echo "child=$kid after $i polls" > "$out/info.txt"
[ -n "$kid" ] && sample $kid 5 1 -file "$out/sample.txt" >/dev/null 2>&1
wait $top

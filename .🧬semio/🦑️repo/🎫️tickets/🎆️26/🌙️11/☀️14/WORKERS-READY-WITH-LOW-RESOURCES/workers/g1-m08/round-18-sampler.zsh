#!/bin/zsh
# P12 sampler, with OUT supplied under workers/g1-m08 and newest LOG refreshed.
setopt NULL_GLOB
OUT="$1"; : > "$OUT"
LOG=$(ls -t /var/folders/*/*/T/semio-repo/$(id -u)/host-*.log 2>/dev/null | head -1)
echo "service log: $LOG" >> "$OUT"
echo "closure entries before: $(find "$PWD"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' ')" >> "$OUT"
SAMPLED=0
while [ ! -e "$OUT.stop" ]; do
 ts=$(date -u +%FT%TZ)
 LOG=$(ls -t /var/folders/*/*/T/semio-repo/$(id -u)/host-*.log 2>/dev/null | head -1)
 row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
 pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
 if [ -n "$pid" ]; then
  fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
  last=$(tail -n 600 "$LOG" 2>/dev/null | grep -E 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
  echo "$ts pid=$pid rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$OUT"
  if [ "$rssmb" -gt 4096 ] && [ "$SAMPLED" = 0 ]; then SAMPLED=1; sample "$pid" 5 -file "$OUT.sample-${ts//:/}.txt" >/dev/null 2>&1 & fi
 else
  echo "$ts no slice worker" >> "$OUT"
 fi
 sleep 5
done
wait
echo "closure entries after: $(find "$PWD"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' ')" >> "$OUT"


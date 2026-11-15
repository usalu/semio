#!/bin/bash
# 🔬 Sampler of ROUND-NOTE-P12 section 1: every 5 s the slice worker's resident size and footprint beside the newest progress line of the newest graph service log, one 5 s `sample` above 4 GB; ends when <out>.stop exists. Reads only. Usage: p12-sampler.sh <out>
OUT="$1"; ROOT="$HOME/Documents/semio"; : > "$OUT"; rm -f "$OUT.stop"
newest() { ls -t "$TMPDIR"/semio-repo/"$(id -u)"/host-*.log 2>/dev/null | head -1; }
entries() { find "$ROOT"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' '; }
echo "service log: $(newest)" >> "$OUT"; echo "closure entries before: $(entries)" >> "$OUT"
SAMPLED=0
while [ ! -e "$OUT.stop" ]; do
  ts=$(date -u +%FT%TZ); LOG="$(newest)"
  row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
  pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
  if [ -n "$pid" ]; then
    fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
    last=$(tail -n 600 "$LOG" 2>/dev/null | grep -aE 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
    echo "$ts pid=$pid rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$OUT"
    if [ "$rssmb" -gt 4096 ] && [ "$SAMPLED" = 0 ]; then SAMPLED=1; sample "$pid" 5 -file "$OUT.sample-${ts//:/}.txt" >/dev/null 2>&1 & fi
  else
    echo "$ts no slice worker" >> "$OUT"
  fi
  sleep 5
done
echo "service log at the end: $(newest)" >> "$OUT"; echo "closure entries after: $(entries)" >> "$OUT"; rm -f "$OUT.stop"

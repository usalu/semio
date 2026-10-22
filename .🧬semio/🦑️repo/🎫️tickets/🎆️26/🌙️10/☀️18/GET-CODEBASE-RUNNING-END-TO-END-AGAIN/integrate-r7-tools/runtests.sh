#!/bin/sh
# usage: runtests.sh <list-file> <outdir>   one `bun test ./<file>` at a time, nice 10, waits while 1-min load >= 45
cd /Users/ueli/Documents/semio || exit 1
list="$1"; out="$2"; mkdir -p "$out"
n=0
while IFS= read -r f; do
  n=$((n+1))
  while :; do l=$(uptime | sed 's/.*load averages*: *//' | awk '{gsub(",","",$1); print int($1)}'); [ "$l" -lt 45 ] && break; sleep 10; done
  log="$out/$(printf %03d $n).log"
  start=$(date +%s)
  nice -n 10 bun test "./$f" > "$log" 2>&1
  rc=$?
  end=$(date +%s)
  pass=$(grep -E '^ *[0-9]+ pass' "$log" | tail -1 | awk '{print $1}')
  fail=$(grep -E '^ *[0-9]+ fail' "$log" | tail -1 | awk '{print $1}')
  echo "$n rc=$rc pass=${pass:-?} fail=${fail:-?} $((end-start))s $f" | tee -a "$out/summary.txt"
done < "$list"

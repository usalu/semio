#!/bin/zsh
OUT=$1; N=${2:-45}; : > $OUT
for i in $(seq 1 $N); do
 for m in g1-m01 g1-m02 g1-m03 g1-m06 g1-m07 g1-m08 g1-m10; do
  f=/Users/Shared/semio-store/v1/origins/$m
  line=$(grep -h "taken [0-9]* of [0-9]*" $f/ready-log-workspace.txt 2>/dev/null | tail -1 | sed -E 's/.*\(([0-9]+) s\): taken ([0-9]+) of ([0-9]+).*/\1 \2 \3/')
  st=$(python3 -c "
import json
d=json.load(open('$f/ready.json'));print(d.get('state'),(d.get('reason') or {}).get('code'),(d.get('refusal') or {}).get('code'))" 2>/dev/null)
  echo "$(date -u +%H:%M:%S) $m $line | $st" >> $OUT
 done
 sleep 20
done

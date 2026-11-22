#!/bin/sh
cd /Users/Shared/semio-store/v1/origins || exit 1
date +%T
for pat in "Original committed Nx native shard required" "Unfunded Nx native shard retained" "Exact funded Nx native shard required" "Ambiguous funded Nx native shard" "Funded Nx graph cache capacity exhausted"; do
  n=$(cat */ready-log-*.txt */service-failure*.txt 2>/dev/null | grep -c "$pat")
  echo "$n x $pat"
done
for o in g1-*; do [ -f $o/ready.json ] && python3 -c "
import json,sys
d=json.load(open('$o/ready.json'));print('$o',d['stage'],d['state'],d.get('attempts'),(d.get('tail') or [''])[-1][:70])"; done 2>/dev/null | head -14

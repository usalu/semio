#!/usr/bin/env bash
D="$(dirname "$0")"
for i in $(seq 1 8); do
  out="$(bash "$D/g6_run.sh")"
  echo "$out" | tail -n 8
  echo "$out" | grep -q "^OK" && exit 0
  sleep 420
done
exit 1

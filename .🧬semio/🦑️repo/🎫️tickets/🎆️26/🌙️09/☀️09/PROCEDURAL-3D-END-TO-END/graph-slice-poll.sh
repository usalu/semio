#!/bin/sh
# Samples the client progress and the inventory source of the slice report of a running read every 30 s.
T="$(dirname "$0")/🗑️generated"; R="$(git rev-parse --show-toplevel)"
while kill -0 "$1" 2>/dev/null; do
  { date +%T; tail -n 1 "$T/$2" | cut -c1-260; python3 -c "
import json;d=json.load(open('$R/.cache/f/slice-memory.json'));i=d.get('sources',{}).get('inventory',{});print(d.get('reason'),d.get('uptimeSeconds'),'heap',d.get('memory',{}).get('heapUsed'),'active',i.get('active'),'owner',i.get('activeOwner'))" 2>&1; } >> "$T/react-v15-samples.txt"
  sleep 30
done

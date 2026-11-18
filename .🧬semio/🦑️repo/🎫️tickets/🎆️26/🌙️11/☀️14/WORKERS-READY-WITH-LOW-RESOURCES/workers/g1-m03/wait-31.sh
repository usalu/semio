#!/bin/bash
# ⏳ Round 31: waits 9.5 minutes (or until ready.json says ready), writes the next observation with observe-31.sh and prints the round line and next.md. Usage: wait-31.sh
here=$(cd "$(dirname "$0")" && pwd); n=0
while [ $n -lt 57 ]; do sleep 10; n=$((n+1)); grep -q '"ready":true' /Volumes/semio-store/v1/origins/g1-m03/ready.json 2>/dev/null && break; done
bash "$here/observe-31.sh"; head -1 "$here/../../ROUND.md"; cat "$here/next.md" 2>/dev/null | head -5
tail -2 /Volumes/semio-store/v1/origins/g1-m03/ready-log-"$(python3 -I -c 'import json;print(json.load(open("/Volumes/semio-store/v1/origins/g1-m03/ready.json")).get("stage"))')".txt | cut -c1-250

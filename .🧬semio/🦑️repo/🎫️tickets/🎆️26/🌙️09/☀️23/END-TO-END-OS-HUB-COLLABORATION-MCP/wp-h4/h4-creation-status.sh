#!/bin/zsh
# 🌱️ H4: polls one artifact-creation status until it leaves "accepted"/"preparing". Usage: h4-creation-status.sh <port> <requestId> <seconds>
port=$1; request=$2; budget=$3; space=01a0ca9d-eb18-7dfe-bd15-7b8fd3dcdae9
token=$(curl -s -X POST http://127.0.0.1:$port/auth/sessions -H 'content-type: application/json' -d "{\"schema\":\"semio.hub.auth.credential-sign-in/v1\",\"email\":\"user1@semio.dev\",\"password\":\"gm1-local-dev-pass-1\",\"deviceInstanceId\":\"h4status$RANDOM$RANDOM\",\"clientClass\":\"browser\"}" | python3 -c 'import sys,json;print(json.load(sys.stdin).get("token",""))')
start=$(date +%s)
while true; do
  body=$(curl -s http://127.0.0.1:$port/spaces/$space/artifact-creations/$request -H "authorization: Bearer $token")
  phase=$(print -r -- $body | python3 -c 'import sys,json;print(json.load(sys.stdin).get("phase"))')
  if [[ $phase != accepted && $phase != preparing ]] || (( $(date +%s) - start > budget )); then print -r -- "phase=$phase after $(( $(date +%s) - start ))s ${body[1,500]}"; break; fi
  sleep 10
done

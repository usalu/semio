#!/bin/bash
# usage: spin-watchdog.sh — kills cargo test binaries (…/deps/ or …/out/<name>-<hex>) that run >480 s at >=90% CPU; logs each kill to the claims log
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while sleep 60; do
 ps -axo pid=,etime=,pcpu=,args= | awk '$4 ~ /\/(deps|out)\/[^ ]*-[0-9a-f]+$/ && $3>=90 {n=split($2,t,/[-:]/); s=(n==2)?t[1]*60+t[2]:(n==3)?t[1]*3600+t[2]*60+t[3]:86400*t[1]+t[2]*3600+t[3]*60+t[4]; if(s>480) print $1, $4}' | while read -r pid bin; do
  kill -TERM "$pid" 2>/dev/null; sleep 5; kill -KILL "$pid" 2>/dev/null
  echo "$(date +%H:%M) (11/09) WATCHDOG killed spinning test binary pid $pid ${bin##*/} (>8 min at >=90% CPU): owning lane, add a fail-fast stall assert" >> "$C/1105b-claims.md"
 done
done

#!/bin/zsh
B="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/native/build-domain/debug/build"
for p in "$B"/*(N/); do
  hs=(${(f)"$(ls -t "$p")"})
  for h in ${hs[2,-1]}; do rm -rf "${p:?}/${h:?}"; done
done
df -h /System/Volumes/Data | tail -1

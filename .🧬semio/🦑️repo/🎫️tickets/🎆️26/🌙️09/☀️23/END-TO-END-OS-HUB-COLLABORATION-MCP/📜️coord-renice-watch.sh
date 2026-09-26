#!/bin/zsh
# 🐢 Coordinator renice watch: every 60 s, lower non-W2 cargo/rustc below W2's niced catalog lanes (preamble rule 18); stops when `renice-stop` exists.
stop="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-coord-logs/renice-stop"
log="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-coord-logs/renice.txt"
while [ ! -e "$stop" ]; do
  snap=$(ps -axo pid=,ppid=,ni=,comm=,args=)
  exempt=" $(print -r -- "$snap" | awk '{pid=$1; pp[pid]=$2; line=$0} /w2-|wasm w2 |hub w2 /{root[pid]=1} END{for(p in pp){q=p; n=0; while(q>1 && n<64){if(q in root){print p; break} q=pp[q]; n++}}}' | tr '\n' ' ') "
  n=0
  for p in $(print -r -- "$snap" | awk '$4 ~ /(rustc|cargo)$/ && $3 < 12 {print $1}'); do
    case "$exempt" in *" $p "*) continue;; esac
    renice -n 12 -p $p >/dev/null 2>&1 && n=$((n+1))
  done
  [ $n -gt 0 ] && echo "$(date '+%T') reniced $n" >> "$log"
  sleep 60
done

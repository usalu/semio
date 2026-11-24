#!/bin/sh
out=$1
while :; do
  pid=$(ps -axo pid=,command= | grep -a "crate-name semio_hub_flow_component" | grep -v "grep\|capture-final" | awk 'NR==1{print $1}')
  if [ -n "$pid" ]; then
    ps -ww -p "$pid" -o command= > "$out/final-cmd.txt"
    ps -ww eww -p "$pid" | tr ' ' '\n' | grep -a "^SEMIO_\|^CARGO_\|^RUST\|^MAKEFLAGS\|^PATH=" > "$out/final-env.txt"
    pwd > "$out/final-cwd.txt"; lsof -p "$pid" 2>/dev/null | awk '$4=="cwd"{print $9}' > "$out/final-cwd.txt"
    exit 0
  fi
  sleep 1
done

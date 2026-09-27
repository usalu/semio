#!/bin/zsh
# 🛰️ Coordinator watch: memory, swap, disk and idle-cargo convoys; one line per state change.
last=""
while true; do
  free=$(memory_pressure 2>/dev/null | awk -F': ' '/free percentage/{gsub("%","",$2);print $2}')
  swap=$(sysctl -n vm.swapusage | awk '{gsub("M","",$6);print int($6/1024)}')
  disk=$(df -g /Users/ueli/Documents/semio | awk 'NR==2{print $4}')
  cargos=$(pgrep -x cargo | wc -l | tr -d ' ')
  rustcs=$(pgrep -x rustc | wc -l | tr -d ' ')
  state=""
  [ "${free:-100}" -lt 35 ] && state="$state mem-free=${free}%"
  [ "${swap:-0}" -ge 14 ] && state="$state swap=${swap}G"
  [ "${disk:-999}" -lt 90 ] && state="$state disk=${disk}G"
  [ "$cargos" -ge 3 ] && [ "$rustcs" -eq 0 ] && state="$state convoy?cargo=${cargos},rustc=0"
  if [ "$state" != "$last" ]; then echo "$(date +%H:%M) ${state:- ok (free ${free}%, swap ${swap}G, disk ${disk}G, cargo ${cargos}/rustc ${rustcs})}"; last="$state"; fi
  sleep 60
done

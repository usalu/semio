#!/bin/zsh
# usage: lock.sh take|release area...   (areas must be given alphabetically)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
mode=$1; shift
for a in "$@"; do
  d="$T/landing.lock.$a"
  if [[ $mode == release ]]; then [[ "$(head -c2 $d/owner 2>/dev/null)" == E2 && "$(grep -c E25 $d/owner)" == 1 ]] && rm -rf "$d"; continue; fi
  if mkdir "$d" 2>/dev/null; then :; else
    hm=$(grep -oE '[0-9]{2}:[0-9]{2}' "$d/owner" 2>/dev/null | head -1); [[ -z $hm ]] && hm=00:00; age=$(( $(date +%s) - $(date -j -f "%H:%M" "$hm" +%s) ))
    if (( age > 2400 )); then rm -rf "$d"; mkdir "$d" || { echo "BUSY $a"; exit 1; }; else echo "HELD $a age ${age}s: $(cat $d/owner)"; exit 1; fi
  fi
  echo "E25 $(date +%H:%M) tests-build types" > "$d/owner"; touch "$d/owner"
  echo "took $a"
done

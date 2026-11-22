#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
T=$(echo .*semio/*repo/*tickets/*26/*11/*13/FLEET-ONE*/)
op=$1; shift
for a in $(printf '%s\n' "$@" | sort); do
  d="${T}landing.lock.$a"
  if [ "$op" = take ]; then
    until mkdir "$d" 2>/dev/null; do
      age=$(( $(date +%s) - $(stat -f %m "$d/owner" 2>/dev/null || stat -f %m "$d") ))
      if [ "$age" -gt 2400 ]; then rm -f "$d/owner"; rmdir "$d" 2>/dev/null; continue; fi
      echo "waiting $a: $(cat $d/owner 2>/dev/null)"; sleep 20
    done
    echo "E43 $(date +%H:%M)" > "$d/owner"
  else
    [ "$(cut -d' ' -f1 "$d/owner" 2>/dev/null)" = E43 ] && { rm -f "$d/owner"; rmdir "$d"; }
  fi
done

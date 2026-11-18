#!/bin/zsh
# R126: waits for the start of a window (minute 00-04 or 30-34 UTC), then tries the lock every 5 s until the window ends; never loops past the window. usage: go.sh
D=${0:h}
while true; do
 M=$(date -u +%M)
 if [ $((10#$M % 30)) -lt 5 ]; then
  $D/window.sh; rc=$?
  [ $rc -ne 1 ] && exit $rc
  /bin/sleep 5
 else
  /bin/sleep 5
 fi
 [ -n "$R126_STOP" ] && exit 9
done

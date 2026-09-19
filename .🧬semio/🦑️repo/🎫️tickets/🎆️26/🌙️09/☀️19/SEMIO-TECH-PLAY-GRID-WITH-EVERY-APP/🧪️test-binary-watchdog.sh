#!/bin/sh
# 🐕️ Kills native test binaries (cargo build-dir `out/` executables) running longer than $1 seconds; logs to $2.
while true; do
  ps -eo pid,etime,command | awk -v max="$1" '
    $3 ~ /cargo\/build\/debug\/build\/.*\/out\// {
      n = split($2, a, /[-:]/); s = 0
      if ($2 ~ /-/) s = a[1]*86400 + a[2]*3600 + a[3]*60 + a[4]
      else if (n == 3) s = a[1]*3600 + a[2]*60 + a[3]
      else s = a[1]*60 + a[2]
      if (s > max) print $1, $3
    }' | while read pid cmd; do
    echo "$(date +%H:%M:%S) killed $pid $cmd" >> "$2"; kill -9 "$pid" 2>/dev/null
  done
  sleep 30
done

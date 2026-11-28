#!/usr/bin/env bash
out="$1"; max=0
while true; do
  for p in /proc/[0-9]*; do
    c=$(tr '\0' ' ' < $p/cmdline 2>/dev/null) || continue
    case "$c" in *rustc*current-native-5d*) ;; *) continue;; esac
    n=$(grep -o "crate-name [a-z0-9_]*" <<<"$c" | head -1)
    h=$(awk '/VmHWM/{print $2}' $p/status 2>/dev/null) || continue
    [ -n "$h" ] && [ "$h" -gt "$max" ] && { max=$h; echo "[DEBUG] $(date +%T) peak ${h} kB $n" >> "$out"; }
  done
  sleep 5
done

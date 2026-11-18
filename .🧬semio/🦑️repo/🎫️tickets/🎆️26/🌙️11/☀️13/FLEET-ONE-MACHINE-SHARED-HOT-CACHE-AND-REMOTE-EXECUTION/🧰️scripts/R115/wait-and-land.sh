#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
while true; do
  m=$((10#$(date -u +%M))); s=$((10#$(date -u +%S)))
  if (( m == 0 && s >= 5 || m >= 1 && m <= 3 )) || (( m == 30 && s >= 5 || m >= 31 && m <= 33 )); then
    "$D/window.sh"; rc=$?
    if (( rc == 1 )); then /bin/sleep 10; continue; fi
    exit $rc
  fi
  if (( m >= 5 && m < 30 )) || (( m >= 35 )); then [ -n "$1" ] || true; fi
  /bin/sleep 5
done

#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
until [ "$((10#$(date -u +%H%M)))" -ge 200 ] && [ "$((10#$(date -u +%H%M)))" -lt 300 ] || [ "$((10#$(date -u +%H%M)))" -lt 10 ]; do sleep 0.5; done
while true; do
  out=$("$D/window.sh" 2>&1)
  case "$out" in
    held*) sleep 0.3;;
    *) echo "$out"; break;;
  esac
done
date -u

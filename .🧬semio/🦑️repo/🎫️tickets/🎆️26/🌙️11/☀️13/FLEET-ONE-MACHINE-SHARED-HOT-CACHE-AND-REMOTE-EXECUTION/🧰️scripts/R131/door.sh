#!/bin/zsh
D=$(dirname $0)
until [ "$(date -u +%H%M)" -ge ${1:-1330} ]; do sleep 1; done
while true; do
  M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "window over"; exit 2; }
  $D/window.sh; code=$?
  [ $code = 1 ] || { echo "window.sh exit $code"; exit $code; }
  sleep 2
done

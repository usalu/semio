#!/bin/zsh
S=/Volumes/semio-store/v1/origins
rd(){ timeout 60 cat $S/$1/attest.json 2>/dev/null | python3 -I -c "import json,sys;d=json.load(sys.stdin);print(d['code'],d['queue']['incoming'],d['wallMs'])" 2>/dev/null; }
for i in $(seq 1 40); do
  a=($(rd g1-m10)); b=($(rd g1-server))
  echo "$(date -u +%T) m10=${a[1]:0:12} inc=${a[2]} srv=${b[1]:0:12} inc=${b[2]}"
  [[ -n ${a[1]} && ${a[1]} == ${b[1]} && ${a[2]} == 0 ]] && { echo AGREE; exit 0; }
  sleep 60
done
echo TIMEOUT; exit 1

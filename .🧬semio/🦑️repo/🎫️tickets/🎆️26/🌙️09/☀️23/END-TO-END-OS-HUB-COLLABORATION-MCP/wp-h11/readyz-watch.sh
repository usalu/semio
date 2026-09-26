#!/bin/zsh
# 🌅️ H11: polls /readyz of <port> every <interval> s until 200 or <timeout> s; one line per poll: elapsed ms, HTTP code,
# status, artifactAuthority reason, startup progress.
# usage: readyz-watch.sh <port> <start-epoch-ms> <timeout-s> [interval]
PORT=$1; START=$2; TIMEOUT=$3; IV=${4:-5}
end=$(( $(date +%s) + TIMEOUT ))
while [ $(date +%s) -lt $end ]; do
  body=$(curl -s --max-time 4 -w '\n%{http_code}' "http://127.0.0.1:$PORT/readyz")
  code=${body##*$'\n'}; json=${body%$'\n'*}
  now=$(python3 -c 'import time;print(int(time.time()*1000))')
  echo "$((now - START)) http=$code $(printf '%s' "$json" | python3 -c 'import json,sys
try:
  d=json.load(sys.stdin); print(d.get("status"), d.get("artifactAuthority",{}).get("reason","-"), json.dumps(d.get("startup")))
except Exception: print("no-body")')"
  [ "$code" = 200 ] && exit 0
  sleep $IV
done
exit 1

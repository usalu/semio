#!/bin/sh
# usage: dog2.sh WRAPPER_PID LOG STALL_SECONDS ; kills only a DESCENDANT of WRAPPER_PID whose argv0 is a cargo test binary (…/out/semio_framework…) when LOG has not grown
W=$1; L=$2; S=${3:-240}
last=0; since=$(date +%s)
desc() { ps -ax -o pid=,ppid= | awk -v root="$1" '{p[$1]=$2} END{for(i in p){x=i;while(x>1){if(x==root){print i;break}x=p[x]}}}'; }
while kill -0 "$W" 2>/dev/null; do
  sz=$(wc -c < "$L" 2>/dev/null || echo 0); now=$(date +%s)
  if [ "$sz" != "$last" ]; then last=$sz; since=$now; fi
  if [ $((now-since)) -gt "$S" ]; then
    for p in $(desc $W); do
      c=$(ps -o command= -p $p 2>/dev/null | awk '{print $1}')
      case "$c" in */out/semio_framework*) echo "[dog2] stall $((now-since))s kill test binary $p $c" >> "$L"; kill $p;; esac
    done
    since=$now
  fi
  sleep 10
done

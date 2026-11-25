#!/bin/zsh
# W15: samples every slice-pool worker of this machine (pid, rss MB, elapsed) and the coordinator's slice report heap every 5 s into $1 until $2 exists.
while [[ ! -e $2 ]]; do
  ps -A -o pid=,rss=,etime=,args= | LC_ALL=C command grep -a "slice-pool/" | LC_ALL=C command grep -a "worker.ts" | LC_ALL=C command grep -av grep | awk -v t=$(date +%s) '{printf "{\"t\":%s,\"pid\":%s,\"rssMB\":%d,\"etime\":\"%s\"}\n",t,$1,$2/1024,$3}' >> $1
  sleep 5
done

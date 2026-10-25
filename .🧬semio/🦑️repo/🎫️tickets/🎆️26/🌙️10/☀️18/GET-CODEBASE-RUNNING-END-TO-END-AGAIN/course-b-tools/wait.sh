#!/bin/sh
# usage: wait.sh <pid> <iterations of 10s>
pid=$1; n=${2:-55}
i=0; while [ $i -lt $n ]; do kill -0 $pid 2>/dev/null || { echo "pid $pid exited"; break; }; sleep 10; i=$((i+1)); done

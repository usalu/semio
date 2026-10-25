#!/bin/sh
# usage: after.sh <pid> <lstart-substring> cmd...  waits until pid (checked by lstart) is gone, then runs cmd
pid=$1; ls=$2; shift 2
while ps -o lstart= -p "$pid" 2>/dev/null | /usr/bin/grep -q "$ls"; do sleep 20; done
exec "$@"

#!/bin/sh
# usage: after-pid.sh <pid> <command...>  waits until pid exits, then runs the command
pid=$1; shift
while kill -0 "$pid" 2>/dev/null; do sleep 15; done
exec "$@"

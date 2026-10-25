#!/bin/sh
# usage: then.sh <pid> <command...>  waits for pid then execs command
while kill -0 "$1" 2>/dev/null; do sleep 15; done
shift
exec "$@"

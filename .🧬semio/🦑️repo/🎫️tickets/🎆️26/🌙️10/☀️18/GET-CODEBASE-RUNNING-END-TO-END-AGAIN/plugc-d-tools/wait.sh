#!/bin/sh
# usage: wait.sh <pid> [loops]  bounded wait (10s per loop)
for i in $(seq 1 ${2:-50}); do kill -0 "$1" 2>/dev/null || { echo done; exit 0; }; sleep 10; done; echo running

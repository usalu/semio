#!/bin/sh
# usage: probe.sh <tag> <label>   warm run on scratch <tag>, prints computed
T=$(cd "$(dirname "$0")" && pwd)
out=$(sh $T/inf.sh ${N:-12} $1 KEEP=1 PASS=warm 2>&1)
echo "$2: $(echo "$out" | grep -o '"computed":[0-9]*,"computedOwners":[0-9]*' | tr '\n' ' ') $(echo "$out" | grep -o '"wallS":[0-9.]*')"
echo "$out" | grep -i "error\|refused" | head -3

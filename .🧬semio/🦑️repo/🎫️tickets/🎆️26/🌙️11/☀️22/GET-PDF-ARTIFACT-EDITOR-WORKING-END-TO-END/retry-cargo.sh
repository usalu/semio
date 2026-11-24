#!/bin/sh
# usage: retry-cargo.sh LOG pattern cargo-args...   (retries on compile errors only)
cd "$(dirname "$0")"
log=$1; pat=$2; shift 2
i=0
while [ $i -lt 8 ]; do
  sh ./run-cargo.sh "$log" "$@"
  strings "$log" | grep -qE "$pat" && exit 0
  i=$((i+1)); sleep 200
done

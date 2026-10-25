#!/bin/bash
# Ticket input script: one vitest filter of the course client through repo, retried over peer churn.
#   unit.sh <name> <filter>
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
gen="$here/../$(printf '\xf0\x9f\x97\x91\xef\xb8\x8fgenerated')/creload-d"
name="$1"; filter="$2"
cd /Users/ueli/Documents/semio || exit 1
for attempt in 1 2 3 4 5; do
  nice -n 10 bun repo --tool=bazel run @semio-tech/course-react:test -- quick "$filter" > "$gen/$name-$attempt.log" 2>&1
  code=$?
  echo "$code" > "$gen/$name.rc"
  [ "$code" = 0 ] && exit 0
  if grep -q -e "Cannot find module" -e "build:graph-plan failed" -e "Authorizer source" -e "is not a function" -e "ENOENT" -e "Stale loaded Source revision" -e "released-read" -e "is stale" "$gen/$name-$attempt.log"; then sleep 60; continue; fi
  exit "$code"
done
exit 75

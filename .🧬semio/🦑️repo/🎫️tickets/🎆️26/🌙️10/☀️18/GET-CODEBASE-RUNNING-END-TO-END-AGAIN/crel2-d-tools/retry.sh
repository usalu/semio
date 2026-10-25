#!/bin/sh
# Ticket input script: run.sh again while the run died on a peer's live edit (a module moved under it, a graph plan that cannot read a half-written file).
#   retry.sh <name> <bun repo args...>
here=$(cd "$(dirname "$0")" && pwd)
ticket=$(cd "$here/.." && pwd)
generated="$ticket/$(printf '\360\237\227\221\357\270\217generated')/crel2-d"
name=$1
attempt=0
while [ "$attempt" -lt 5 ]; do
  attempt=$((attempt + 1))
  sh "$here/run.sh" "$@"
  code=$(cat "$generated/$name.rc" 2>/dev/null)
  [ "$code" = 0 ] && exit 0
  if grep -q -e "Cannot find module" -e "build:graph-plan failed" -e "Authorizer source" -e "is not a function" -e "ENOENT: no such file or directory, lstat" -e "Stale loaded Source revision" -e "Source revision ancestry" -e "Expected \")\" but found" "$generated/$name.log"; then
    mv "$generated/$name.log" "$generated/$name-peer-churn-$attempt.log"
    sleep 120
    continue
  fi
  exit "${code:-1}"
done
exit 75

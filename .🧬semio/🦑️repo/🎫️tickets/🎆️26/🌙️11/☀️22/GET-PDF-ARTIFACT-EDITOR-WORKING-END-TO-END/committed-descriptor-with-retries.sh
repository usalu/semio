#!/bin/sh
# usage: committed-descriptor-with-retries.sh <tag>  (up to four gated committed_descriptor runs <tag>1..4, stopping at the first that leaves no compile failure; the host warden cancels a rustc above its unit ceiling whenever other processes push the host into danger, and cargo keeps every crate that finished)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tag=${1:?tag}
for attempt in 1 2 3 4; do
  sh "$here/run-committed-descriptor.sh" "$tag$attempt"
  grep -aq "could not compile" "$here/🗑️generated/$tag$attempt.log" || { echo "built $tag$attempt"; exit 0; }
  grep -aq "signal: 15\|signal: 9" "$here/🗑️generated/$tag$attempt.log" || { echo "compile error $tag$attempt"; exit 1; }
  sleep 120
done
exit 1

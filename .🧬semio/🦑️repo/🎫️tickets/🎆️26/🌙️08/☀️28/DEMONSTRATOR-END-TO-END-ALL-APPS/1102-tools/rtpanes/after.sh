#!/bin/bash
# usage: after.sh <wait-tag> <tag> <pane...>  waits for seq <wait-tag> to finish, then runs seq <tag>
D=$(cd "$(dirname "$0")" && pwd); C=$(cd "$D/../.." && pwd)
W=$1; shift
while [ ! -f "$C/🗑️generated/1102/rtpanes-seq-$W.rc" ]; do sleep 20; done
exec "$D/seq.sh" "$@"

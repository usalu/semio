#!/bin/sh
# usage: proof.sh <name> <marker-regex> <edit-file> <steps...> -- <cmd...>
# Runs ONE real command; from the first log line that matches <marker-regex> a scripted peer (peer-edit.py) edits <edit-file> as `steps` say; ends GREEN only when the command's own rc is 0.
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/churn-d"
name=$1; marker=$2; file=$3; shift 3
steps=""
while [ "$1" != "--" ]; do steps="$steps $1"; shift; done
shift
mkdir -p "$G"
rm -f "$G/$name.run.log" "$G/$name.peer.json" "$G/$name.rc"
echo "wrapper pid $$ started $(date)" > "$G/$name.pids"
env -u SEMIO_SCRIPT_STALL_MS nice -n 10 "$@" > "$G/$name.run.log" 2>&1 &
run=$!
echo "run pid $run" >> "$G/$name.pids"
started=0
while kill -0 "$run" 2>/dev/null; do
  if [ "$started" = 0 ] && /usr/bin/grep -q -E "$marker" "$G/$name.run.log" 2>/dev/null; then
    date +%T > "$G/$name.peer.started"
    python3 "$C/churn-d-tools/peer-edit.py" "$G/$name.peer.json" "$file" "$G/$name.backup" $steps &
    peer=$!
    echo "peer pid $peer" >> "$G/$name.pids"
    started=1
  fi
  sleep 1
done
wait "$run"; rc=$?
[ -n "$peer" ] && kill -TERM "$peer" 2>/dev/null
wait
echo "rc=$rc started=$started" > "$G/$name.rc"

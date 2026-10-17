#!/bin/sh
F="$(dirname "$0")"
G="$F/../🗑️generated"
while [ ! -f "$G/pgr-queue-queue-a.txt.done" ]; do sleep 20; done
for q in queue-b queue-d queue-c; do sh "$F/run-queue.sh" "$F/$q.txt"; done

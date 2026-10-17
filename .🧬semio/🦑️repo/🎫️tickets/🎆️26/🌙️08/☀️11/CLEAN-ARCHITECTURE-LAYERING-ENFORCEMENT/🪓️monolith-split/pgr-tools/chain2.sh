#!/bin/sh
F="$(dirname "$0")"
for q in queue-a queue-b queue-d queue-c; do sh "$F/run-queue.sh" "$F/$q.txt"; done

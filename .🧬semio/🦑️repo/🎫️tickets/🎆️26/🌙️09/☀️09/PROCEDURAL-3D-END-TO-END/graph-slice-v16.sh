#!/bin/sh
# Waits for the running v15 read (pid $1), then runs one read detached into 🗑️generated/react-v16.txt and records its start and end epoch.
T="$(dirname "$0")/🗑️generated"; cd "$(git rev-parse --show-toplevel)"
while kill -0 "$1" 2>/dev/null; do sleep 10; done
date +%s > "$T/react-v16-start.txt"
repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype > "$T/react-v16.txt" 2>&1 &
RP=$!; echo $RP > "$T/react-v16-pid.txt"
wait $RP; echo "exit $?" >> "$T/react-v16-start.txt"; date +%s >> "$T/react-v16-start.txt"

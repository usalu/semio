#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T="$(dirname "$0")"; O="$T/🗑️generated/ts"; export SEMIO_TEST_ARTIFACT_DIR="$O/artifacts"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
n=0
while IFS= read -r f; do n=$((n+1)); [ -f "$f" ] || { echo "$n MISSING $f" >> "$O/summary.txt"; continue; }
repo capacity run --memory=1024 -- repo tools exec bun -- test "./$f" > "$O/$n.log" 2>&1; echo "$n exit=$? $f" >> "$O/summary.txt"; done < "$T/ts-suites.txt"
echo DONE >> "$O/summary.txt"

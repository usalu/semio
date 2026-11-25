#!/bin/sh
# 📏️ Runs one command and reports the peak of its largest process and its seconds (BSD `time -l`): peak.sh <label> <command and args…>; to be run inside a capacity grant.
# The raw `time -l` output goes to <label>.time next to the label; one line `[DEBUG] <label> maxrss <MB> MB real <s> s exit <code>` goes to stdout.
set -u
label=$1; shift
/usr/bin/time -l -o "$label.time" "$@"
code=$?
awk -v id="$label" -v code="$code" '/maximum resident set size/ { mb = $1 / 1048576 } /real/ { real = $1 } END { printf "[DEBUG] %s maxrss %.0f MB real %s s exit %s\n", id, mb, real, code }' "$label.time"
exit "$code"

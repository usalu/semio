#!/usr/bin/env bash
# ⏳️ Blocks up to N seconds (default 580) until a new topic REPORT.md appears or disk runs low, then prints a compact fleet dashboard.
G="$(cd "$(dirname "$0")" && pwd)/🗑️generated"; cd "$G" || exit 1
count() { find . -maxdepth 2 -name REPORT.md ! -path './audit-interaction/*' | wc -l | tr -d ' '; }
n0=$(count); end=$(( $(date +%s) + ${1:-580} ))
while [ "$(date +%s)" -lt "$end" ]; do
  free=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
  [ "$(count)" -gt "$n0" ] && break; [ "$free" -lt 25 ] && break; sleep 30
done
date '+%H:%M:%S'; echo "reports: $(find . -maxdepth 2 -name REPORT.md | tr '\n' ' ')"
echo "free=${free}GB $(uptime | sed 's/.*load/load/') swap=$(sysctl -n vm.swapusage | awk '{print $6}')"
echo "serve: $(tail -1 serve-6033-supervised.txt.events.txt) | visual=$(wc -l < audit-visual/results.ndjson | tr -d ' ') interaction=$(wc -l < audit-interaction/results.ndjson | tr -d ' ')"

#!/usr/bin/env bash
# ⏳️ Blocks up to N seconds (default 580) until a new tracked topic report (📓️<topic>.md), an activate request or low disk appears, then prints a compact fleet dashboard.
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; cd "$T" || exit 1
reports() { ls 📓️*.md 2>/dev/null | grep -vE 'status|app-boot-defects|audit-artifacts' | wc -l | tr -d ' '; }
requests() { ls "$G/activate.request" 2>/dev/null | wc -l | tr -d ' '; }
n0=$(reports); r0=$(requests); end=$(( $(date +%s) + ${1:-580} ))
while [ "$(date +%s)" -lt "$end" ]; do
  free=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
  [ "$(reports)" -gt "$n0" ] && break; [ "$(requests)" -gt "$r0" ] && break; [ "$free" -lt 15 ] && break; sleep 30
done
date '+%H:%M:%S'; echo "reports: $(ls 📓️*.md 2>/dev/null | grep -vE 'status|app-boot-defects|audit-artifacts' | tr '\n' ' ')"
echo "activate requests: $(ls "$G/activate.request" 2>/dev/null | tr '\n' ' ')"
echo "free=${free}GB $(uptime | sed 's/.*load/load/') swap=$(sysctl -n vm.swapusage | awk '{print $6}')"
echo "serve: $(tail -1 "$G/serve-6033-supervised.txt.events.txt" 2>/dev/null) | baseline=$(wc -l < "$G/baseline/summary.tsv" 2>/dev/null | tr -d ' ') crates | visual=$(wc -l < "$G/audit-visual/results.ndjson" 2>/dev/null | tr -d ' ') panes"
for s in "$G"/*/STATUS.md; do [ -f "$s" ] && echo "-- $(basename "$(dirname "$s")"): $(tail -1 "$s" | cut -c1-160)"; done

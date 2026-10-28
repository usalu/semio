#!/bin/sh
# usage: dash-run-smoke.sh <cycle-id> ; headless smoke of the human route `repo run @semio-tech/repo-dashboard-rs:run` (continuous TUI task) through an owned pty, EN then DE, then daemon status + stop of the isolated instance; verdict lines `smoke <lang> ...` on stdout
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/integ-g"; id="$1"
unset SEMIO_SCRIPT_STALL_MS SEMIO_LANGUAGE
bin=$(python3 -c "import json;print(json.load(open('.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/installed.json'))['path'])")
case "$bin" in /*) ;; *) bin="$PWD/$bin";; esac
rc=0
for lang in en de; do
  export SEMIO_DASHBOARD_INSTANCE=integ-g-$lang SEMIO_BUILD_MODE=prototype SEMIO_LOCALE=$lang
  case $lang in de) wanted="Neue Aufgabe";; *) wanted="New task";; esac
  s0=$(date +%s)
  DRIVE_ARGV='["repo","--tool=bazel","run","@semio-tech/repo-dashboard-rs:run"]' python3 "$C/dash-d-tools/drive-launcher.py" "$(command -v bun)" "$G/$id.run-$lang" waitfor:1500:"$wanted" snap:boot wait:3 key:02 text:q wait:3 snap:quit
  hits=$(/usr/bin/grep -c "$wanted" "$G/$id.run-$lang.boot.txt"); churn=$(/usr/bin/grep -c "edited by another process" "$G/$id.run-$lang.boot.txt")
  status=$("$bin" daemon status 2>&1 | head -1)
  "$bin" daemon stop >/dev/null 2>&1
  echo "smoke $lang wanted-hits=$hits churn-lines=$churn wall=$(( $(date +%s) - s0 ))s daemon-status='$status'"
  [ "$hits" -gt 0 ] || rc=1
done
exit $rc

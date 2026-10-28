#!/bin/sh
# usage: devcheck.sh <cycle-id> ; human dev route `repo run @teaching/architecture-course:dev` on Bazel: waits for readiness (task-ready/task-started) or an error, records the ready line + served urls, then stops its own process group
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/integ-g"; id="$1"; log="$G/$id.course-dev.log"
python3 - "$C/integ-g-tools/devrun.sh" "$log" "$G/$id.course-dev.pid" <<'PY'
import subprocess, sys
p = subprocess.Popen(["sh", sys.argv[1], sys.argv[2]], start_new_session=True)
open(sys.argv[3], "w").write(str(p.pid))
PY
pid=$(cat "$G/$id.course-dev.pid"); s0=$(date +%s); rc=1
while [ $(( $(date +%s) - s0 )) -lt 1800 ]; do
  if /usr/bin/grep -aq "task-ready\|task-started" "$log"; then rc=0; break; fi
  if /usr/bin/grep -aq "released-read\|^error:\|OwnedCommandFailure" "$log"; then break; fi
  kill -0 "$pid" 2>/dev/null || break
  sleep 5
done
sleep 10
echo "course-dev rc=$rc wall=$(( $(date +%s) - s0 ))s $(/usr/bin/grep -a 'task-ready\|task-started\|http://' "$log" | head -4 | cut -c1-200 | tr '\n' '|')"
kill -TERM -- "-$pid" 2>/dev/null; sleep 15; kill -KILL -- "-$pid" 2>/dev/null
printf '%s\tcourse-dev\trc=%s\twall=%ss\n' "$id" "$rc" "$(( $(date +%s) - s0 ))" >> "$G/cycles.tsv"
exit $rc

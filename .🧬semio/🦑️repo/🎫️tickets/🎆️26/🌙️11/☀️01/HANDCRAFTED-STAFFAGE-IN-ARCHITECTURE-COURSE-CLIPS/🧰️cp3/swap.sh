#!/usr/bin/env bash
# swap the running driver for the edited one right after it finishes a video (state.json changes), so no render is lost
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export PYTHONUTF8=1
before=$(stat -c %Y "$T/🗑️generated/cp3/state.json")
while [ "$(stat -c %Y "$T/🗑️generated/cp3/state.json")" = "$before" ]; do sleep 2; done
sleep 1
powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object { \$_.CommandLine -like '*cp3*driver.py*' } | ForEach-Object { taskkill /F /T /PID \$_.ProcessId }" > /dev/null 2>&1
sleep 3
nohup "$PY" "$T/🧰️cp3/driver.py" </dev/null > "$T/🗑️generated/cp3/driver.out" 2>&1 &
echo "swapped $(date +%T)" >> "$T/🗑️generated/cp3/progress.log"

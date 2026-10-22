#!/bin/sh
# usage: wait.sh PID LOG [loops]
PID=$1; L=$2; N=${3:-54}
i=0
while [ $i -lt $N ]; do kill -0 "$PID" 2>/dev/null || break; sleep 10; i=$((i+1)); done
if kill -0 "$PID" 2>/dev/null; then echo "RUNNING pid=$PID"; else echo "DONE pid=$PID"; fi
/usr/bin/grep -vE '^\[(script|cargo:|repository:)' "$L" | /usr/bin/grep -E '^(error|warning: unused)|could not compile|Finished|^\s+(Checking|Compiling) semio' | tail -4 | cut -c1-250
echo "errors: $(/usr/bin/grep -cE '^[^ ]+\.rs:[0-9]+:[0-9]+: error' "$L")  last-semio: $(/usr/bin/grep -E '(Checking|Compiling) semio' "$L" | tail -1)"

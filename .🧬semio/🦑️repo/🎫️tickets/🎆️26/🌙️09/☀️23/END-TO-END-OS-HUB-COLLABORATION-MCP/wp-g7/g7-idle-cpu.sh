#!/bin/bash
# g7: CPU time an idle semio-os-mcp stdio process burns in 30 s after an initialize (pool parking measure); $1 = binary, $2 = label.
DIR=$(mktemp -d)
( printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"g7-idle","version":"1"}}}'; sleep 45 ) | "$1" stdio --folder "$DIR" --no-bridge --scopes workspace.read > /dev/null 2>&1 &
sleep 10
PID=$(pgrep -f "$1 stdio --folder $DIR" | head -1)
T0=$(ps -o time= -p "$PID"); W0=$(ps -M -p "$PID" | tail -n +2 | wc -l)
sleep 30
T1=$(ps -o time= -p "$PID")
echo "$2 pid=$PID threads=$W0 cpu_at_10s=$T0 cpu_at_40s=$T1"
kill "$PID" 2>/dev/null

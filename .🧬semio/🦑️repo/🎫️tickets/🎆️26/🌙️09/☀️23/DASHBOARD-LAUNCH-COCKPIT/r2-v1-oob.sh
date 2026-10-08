#!/usr/bin/env bash
# usage: r2-v1-oob.sh <name> <cwd (bash path)> <powershell command...>  runs it outside the harness job (breakaway allowed), waits, prints the log
set -u
here="$(cd "$(dirname "$0")" && pwd)"
name="$1"; cwd="$(cygpath -w "$2")"; shift 2
mkdir -p "$here/🗑️generated"
log="$here/🗑️generated/v1-$name.log"; exit_file="$here/🗑️generated/v1-$name.exit"
pwsh -NoProfile -File "$(cygpath -w "$here/r2-v1-oob.ps1")" -Command "$*" -Log "$(cygpath -w "$log")" -Exit "$(cygpath -w "$exit_file")" -Directory "$cwd" >/dev/null
for _ in $(seq 1 ${OOB_WAIT:-100}); do [ -s "$exit_file" ] && break; sleep 2; done
[ -s "$exit_file" ] || { echo "[oob still running; log $log]"; tail -c 3000 "$log"; exit 99; }
cat "$log"
echo "[oob exit $(tr -d '\r ' < "$exit_file")]"

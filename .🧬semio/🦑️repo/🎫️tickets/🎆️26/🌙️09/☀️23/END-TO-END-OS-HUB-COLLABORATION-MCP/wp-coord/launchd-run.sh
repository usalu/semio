#!/bin/zsh
# 🚀 Coordinator: run one command as a transient launchd job (survives desktop-app restarts); launchd's keep-alive re-spawn is
# refused via a started-marker and the job removes itself when the command ends. usage: zsh launchd-run.sh <label> <marker-dir> <cmd…>
label="$1"; shift; marker="$1"; shift
if [ -e "$marker/started" ]; then launchctl remove "$label"; exit 0; fi
mkdir -p "$marker"; date '+%F %T' > "$marker/started"; echo $$ > "$marker/pid"
"$@"; rc=$?
echo "$rc $(date '+%F %T')" > "$marker/rc"
launchctl remove "$label"
exit $rc

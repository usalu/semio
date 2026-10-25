#!/bin/sh
# usage: tui-language.sh <de|en> [bin]  Drives the native dashboard TUI through an owned pty in the chosen language, starts `repo cache status` (tool:workspace/build-cache-status) from the launcher, waits for its exit and then reads the task's full log through the CLI of the same instance: the log must be in the chosen language. Isolated instance daemon `dash-f-tui-<lang>`, stopped on EXIT.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/dash-f"; mkdir -p "$G"
lang=$1
bin=${2:-$(python3 -c "import json;print(json.load(open('.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/installed.json'))['path'])")}
case "$bin" in /*) ;; *) bin="$PWD/$bin";; esac
export SEMIO_DASHBOARD_INSTANCE=dash-f-tui-$lang SEMIO_BUILD_MODE=prototype SEMIO_LOCALE=$lang
unset SEMIO_LANGUAGE
case $lang in de) wanted="Cache-Bilanz von";; *) wanted="Cache ledger of";; esac
trap '"$bin" daemon stop >/dev/null 2>&1' EXIT
python3 "$C/dash-d-tools/drive-launcher.py" "$bin" "$G/tui-$lang" wait:6 snap:boot key:02 text:n wait:2 text:build-cache-status wait:2 snap:search key:0d wait:2 snap:form key:0d wait:45 snap:task
for i in $(seq 1 60); do "$bin" tasks --json 2>/dev/null | python3 -c "import json,sys;t=[x for x in json.load(sys.stdin) if x['commandId']=='tool:workspace/build-cache-status'];sys.exit(0 if t and t[-1]['status']=='exited' else 1)" && break; sleep 5; done
id=$("$bin" tasks --json | python3 -c "import json,sys;print([x for x in json.load(sys.stdin) if x['commandId']=='tool:workspace/build-cache-status'][-1]['session'])")
"$bin" logs "$id" > "$G/tui-$lang.log.txt" 2>&1
echo "session $id lines $(wc -l < "$G/tui-$lang.log.txt") wanted-hits $(/usr/bin/grep -c "$wanted" "$G/tui-$lang.log.txt")"
/usr/bin/grep -v '^\[script\]' "$G/tui-$lang.log.txt" | head -3

#!/bin/sh
# usage: task-e2e.sh [bin]   Real task through the dashboard daemon: finite run (stream + exit code), detached long run (logs), cancellation, EN+DE.
# Isolated instance daemon `dash3-d-e2e`; always stopped on EXIT; every pid it records is in generated/dash3-d/task-e2e-pids.txt.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/dash3-d"; mkdir -p "$G"
bin=${1:-$(python3 -c "import json;print(json.load(open('.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/installed.json'))['path'])")}
case "$bin" in /*) ;; *) bin="$PWD/$bin";; esac
[ -x "$bin" ] || { echo "no executable $bin"; exit 2; }
export SEMIO_DASHBOARD_INSTANCE=dash3-d-e2e SEMIO_BUILD_MODE=prototype SEMIO_LOCALE=en
out="$G/task-e2e.out"; : > "$out"; pids="$G/task-e2e-pids.txt"; : > "$pids"
step() { echo "### $*" | tee -a "$out"; "$@" >> "$out" 2>&1; rc=$?; echo "rc=$rc" | tee -a "$out"; return $rc; }
trap '"$bin" daemon stop >> "$out" 2>&1; echo "daemon stopped" >> "$out"' EXIT
step "$bin" daemon start
step "$bin" daemon status
echo "daemon: $("$bin" daemon status 2>&1 | sed -n 's/.*daemon pid \([0-9]*\).*/\1/p')" >> "$pids"
step "$bin" run tool:workspace/build-preflight
step "$bin" run tool:workspace/build-cache-status
echo "### detached long run" | tee -a "$out"
id=$("$bin" run tool:workspace/cargo --param store=shared --param mode=prototype --detach -- check -p semio-framework-repo-dashboard-labels 2>>"$out" | head -1 | cut -f1)
echo "session: $id" | tee -a "$out"
sleep 15
step "$bin" tasks
step "$bin" tasks --json
step "$bin" logs "$id"
step "$bin" stop "$id"
sleep 5
step "$bin" tasks --json
step "$bin" tasks --language de
step "$bin" stop ghost --language de
step "$bin" daemon status --language de
echo "### leftover check (command lines naming the cancelled cargo target)" | tee -a "$out"
ps -axo pid,lstart,command | /usr/bin/grep "repo-dashboard-labels" | /usr/bin/grep -v grep | tee -a "$out"
echo "### language of the tasks (OS language is en-US: German is the proof)" | tee -a "$out"
for l in de en; do "$bin" daemon start >> "$out" 2>&1; "$bin" run tool:workspace/build-cache-status --language $l > "$G/task-e2e-cache-$l.out" 2>&1; echo "run --language $l rc=$? first line: $(/usr/bin/grep -v '^\[' "$G/task-e2e-cache-$l.out" | head -1)" | tee -a "$out"; done

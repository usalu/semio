#!/bin/sh
# Official launcher route on the Bazel tool: daemon start/status/stop through `bun repo run @semio-tech/repo-dashboard-rs:daemon`, EN then DE. Isolated instance, stopped on EXIT.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
unset SEMIO_SCRIPT_STALL_MS SEMIO_LANGUAGE
export SEMIO_DASHBOARD_INSTANCE=dash-f-launch SEMIO_BUILD_MODE=prototype SEMIO_LOCALE=en
bin=$(python3 -c "import json;print(json.load(open('.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/installed.json'))['path'])")
trap '"$bin" daemon stop; echo "stopped rc=$?"' EXIT
for step in "start" "status" "status --language de" "stop --language de"; do
  echo "### launcher daemon $step"; started=$(date +%H:%M:%S)
  nice -n 10 bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:daemon -- $step 2>&1 | /usr/bin/grep -v '^\[script\]\|^\[repository'
  echo "rc=$? from $started to $(date +%H:%M:%S)"
done

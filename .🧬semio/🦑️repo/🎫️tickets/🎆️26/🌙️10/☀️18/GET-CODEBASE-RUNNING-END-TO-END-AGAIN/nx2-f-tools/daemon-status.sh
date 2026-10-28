#!/bin/sh
# usage: daemon-status.sh   Daemon start/status EN+DE/stop on the Nx-built dashboard binary (installed.json). Isolated instance, stopped on EXIT.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/nx2-f"; mkdir -p "$G"; out="$G/daemon-status.out"; : > "$out"
unset SEMIO_LANGUAGE
bin=$(python3 -c "import json;print(json.load(open('.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/installed.json'))['path'])")
case "$bin" in /*) ;; *) bin="$PWD/$bin";; esac
export SEMIO_DASHBOARD_INSTANCE=nx2-f SEMIO_BUILD_MODE=prototype SEMIO_LOCALE=en
trap '"$bin" daemon stop >> "$out" 2>&1; echo "stopped rc=$?" >> "$out"' EXIT
echo "binary: $bin" | tee -a "$out"
shasum -a 256 "$bin" | tee -a "$out"
for step in "daemon start" "daemon status --language en" "daemon status --language de" "tasks --language de" "stop ghost --language de"; do
  echo "### $step" | tee -a "$out"; "$bin" $step >> "$out" 2>&1; echo "rc=$?" | tee -a "$out"
done

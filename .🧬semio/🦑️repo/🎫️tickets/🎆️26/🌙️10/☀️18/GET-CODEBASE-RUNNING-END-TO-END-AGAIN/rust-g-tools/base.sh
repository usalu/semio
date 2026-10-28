#!/bin/sh
# usage: base.sh <tag> [edit] : dashboard binary build through the real `bun repo cargo` ingress in its own lane: cold lane, no-op, then (edit) one-line edit of the labels crate + rebuild + revert.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/rust-g"; T="$C/rust-g-tools"; TAG="$1"; mkdir -p "$G"
unset SEMIO_SCRIPT_STALL_MS
export CLAUDE_CODE_SESSION_ID="rustg-$TAG" SEMIO_CARGO_TIMING=1
run() { sh "$T/healthy.sh"; python3 "$T/measure.py" "${LABEL:-$TAG}-$1" "$G/runs.jsonl" -- nice -n 10 bun repo cargo --store=shared --configuration=prototype -- build -p semio-framework-repo-dashboard --bin semio > "$G/${LABEL:-$TAG}-$1.log" 2>&1; }
[ -n "$SKIP_COLD" ] || run cold; run noop
if [ "$2" = "edit" ]; then
  trap 'python3 "$T/probe.py" revert labels' EXIT INT TERM
  python3 "$T/probe.py" apply labels && run edit1
  python3 "$T/probe.py" revert labels; trap - EXIT INT TERM
fi
echo done

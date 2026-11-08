#!/bin/bash
# usage: activate.sh [label]  — coordinator only. Energy-only lane activation (same target the demonstrator activate-lanes-prototype depends on, so its receipt is reused later). Log: 🗑️generated/1105b/<label>.log
T="$(cd "$(dirname "$0")/.." && pwd)"; L="${1:-canary-act}"
ROOT="$T"; while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
cd "$ROOT" || exit 2
O="$T/../🗑️generated/1105b/$L"; mkdir -p "$(dirname "$O")"
echo "start $(date '+%F %T')" > "$O.status"
SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/framework-os-dev:activate-energy-react-prototype > "$O.log" 2>&1
rc=$?; echo "$rc" > "$O.rc"; echo "exit $rc $(date '+%F %T')" >> "$O.status"; exit $rc

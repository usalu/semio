#!/bin/bash
# usage: describe.sh [label] [pluginId]  — coordinator only. Refreshes the committed descriptor pair of one component (default energy) in write mode. Run BEFORE activate.sh: a fresh build whose descriptor differs from the committed pair is refused. Arguments of the target go after `--` (workspace:components forwards all arguments).  describe.sh --help prints this and the command, runs nothing.
T="$(cd "$(dirname "$0")/.." && pwd)"
case "${1:-}" in -h|--help) sed -n 2p "$0"; echo "command: repo run workspace:components -- describe ${2:-energy} --profile prototype"; exit 0;; esac
L="${1:-canary-describe}"; P="${2:-energy}"
ROOT="$T"; while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
cd "$ROOT" || exit 2
O="$T/../🗑️generated/1105b/$L"; mkdir -p "$(dirname "$O")"
SEMIO_LANGUAGE=en repo run workspace:components -- describe "$P" --profile prototype > "$O.log" 2>&1
rc=$?; echo "$rc" > "$O.rc"; exit $rc

#!/usr/bin/env bash
set -eu
ticket=$(cd "$(dirname "$0")" && pwd)
[[ $# -ge 10 && $1 == step && $3 == "" && $4 == repo && $5 == "./📜️script.ts" && $6 == components && $7 == describe ]]
name=$2
deferred="${QUEUE_GENERATED:-$ticket/🗑️generated}/queue-deferred-steps"
if [[ -f $deferred ]]; then
  while IFS= read -r step; do
    if [[ $name == "$step" ]]; then
      printf '[queue] %s deferred by the release owner\n' "$name" >&2
      exit 78
    fi
  done < "$deferred"
fi
shift 7
ids=()
while [[ $# -gt 2 ]]; do
  ids+=("$1")
  shift
done
[[ $1 == --facet && $2 == browser && ${#ids[@]} -gt 0 ]]
exec bash "$ticket/native-lanes.sh" step "$name" "" repo "$ticket/release-pipeline.ts" release "${QUEUE_PIPELINE_OPERATION:-build}" "${ids[@]}"

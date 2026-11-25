#!/bin/zsh
# 🛤️ One queue grant, many jobs: lane.sh <lane> <MB> holds a grant of <MB> and runs every `*.job` of 🗑️generated/matrix/MX-2/jobs-<lane> through jobs.sh (ends after 150 s without a job). Put the next job file there instead of asking the queue again (queue rule: at most two requests).
self="${0:A}"; export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory="$2" -- "$self" --inside "$1"; fi
export B R S T U W M GEN LANE="$2"; source "$W/jobs.sh" "$2"

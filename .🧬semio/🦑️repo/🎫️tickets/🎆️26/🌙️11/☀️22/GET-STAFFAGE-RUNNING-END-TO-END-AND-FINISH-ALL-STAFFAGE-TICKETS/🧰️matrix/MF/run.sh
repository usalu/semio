#!/bin/zsh
# 🏃️ Runs one tool of batch MF from the multiview suite folder with the pinned bun: run.sh <script.ts> <args…>
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
s="$1"; shift; cd "$T" && timeout ${RUN_TIMEOUT:-900} "$B" "$W/$s" "$@"

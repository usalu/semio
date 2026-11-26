#!/bin/zsh
# 📮️ Drops one job for the worker of round MV-L2: `job.sh <megabytes> <name> <command line>` writes `$GEN/jobs/<megabytes, four digits>-<name>.sh`.
source "${0:A:h}/env.sh"
mb="$1"; name="$2"; shift 2; mkdir -p "$GEN/jobs"
printf 'source "%s/env.sh"\n%s\n' "$W" "$*" > "$GEN/jobs/$(printf '%04d' "$mb")-$name.sh.tmp" && mv "$GEN/jobs/$(printf '%04d' "$mb")-$name.sh.tmp" "$GEN/jobs/$(printf '%04d' "$mb")-$name.sh"

#!/bin/zsh
# 🔁️ F2 — reruns one idle census with --resume until it finishes (a hung browser exits the run with 3; hung rows are kept).
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f2
base="$1"; tag="$2"; roles="$3"
for attempt in {1..25}; do
  nice -n 10 bun f2-idle-census.mjs "$base" --tag "$tag" --roles "$roles" --resume
  code=$?
  echo "[supervise] attempt $attempt exit $code $(date '+%H:%M:%S')"
  [ $code -eq 0 ] && break
  sleep 5
done

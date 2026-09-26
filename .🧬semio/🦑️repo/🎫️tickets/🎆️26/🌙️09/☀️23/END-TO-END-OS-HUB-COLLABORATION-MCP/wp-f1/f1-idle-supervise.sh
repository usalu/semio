#!/bin/zsh
# 🔁️ F1 — reruns one idle census with --resume until it finishes (a hung browser exits the run with 3; hung rows are kept).
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f1
tag="$1"; roles="$2"
for attempt in {1..25}; do
  bun f1-idle-census.mjs http://127.0.0.1:6620/ --tag "$tag" --roles "$roles" --resume
  code=$?
  echo "[supervise] attempt $attempt exit $code $(date '+%H:%M:%S')"
  [ $code -eq 0 ] && break
  sleep 5
done

#!/bin/sh
# 🚀 Author 10: publishes the assets named, one after the other (one tool process at a time), each through rework-10-publish.sh --allow-connected; the log of each goes to 🗑️generated/rework-10/publish-<slug>.log.
cd "$(dirname "$0")" || exit 2
for a in "$@"; do
  echo "== $a $(date +%H:%M)"
  sh rework-10-publish.sh "$a" --allow-connected > "🗑️generated/rework-10/publish-$a.log" 2>&1
  tail -4 "🗑️generated/rework-10/publish-$a.log" | cut -c1-300
done
echo "ALL DONE $(date +%H:%M)"

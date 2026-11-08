#!/usr/bin/env bash
# 🔁 Restarts one closure-verify-helper shard until it reports nothing left to derive; usage: closure-helper-loop.sh <shard> <shards> <log>
cd /home/deck/semio
s="$1"; n="$2"; log="$3"; dir="$(dirname "$0")"
while :; do
  repo --service=off tools exec --light bun -- "$dir/closure-verify-helper.mjs" "$s" "$n" 1500 >> "$log" 2>&1
  tail -5 "$log" | grep -q "rss limit" || break
done
echo "[loop] shard $s done" >> "$log"

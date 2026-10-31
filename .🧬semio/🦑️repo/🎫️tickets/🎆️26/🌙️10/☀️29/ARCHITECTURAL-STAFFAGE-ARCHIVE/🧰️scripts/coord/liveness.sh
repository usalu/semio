#!/usr/bin/env bash
# Exits with a line when at least $1 of the given subagent transcripts are idle longer than 300 s.
dir=/home/deck/.claude/projects/-home-deck-semio/a9e0ee2b-ddd3-4c46-a2ad-fa9b4c96169b/subagents
need=$1; shift
while true; do
  now=$(date +%s); stale=0
  for id in "$@"; do
    m=$(stat -c %Y "$dir/agent-$id.jsonl")
    if [ $((now - m)) -gt 300 ]; then stale=$((stale + 1)); fi
  done
  if [ "$stale" -ge "$need" ]; then echo "$(date +%T) $stale of $# transcripts idle > 5 min"; exit 0; fi
  sleep 30
done

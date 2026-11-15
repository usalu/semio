#!/bin/bash
# ⏳ Waits after a consent stop: every 60 s, at most 6 hours, for a round above <reported> (exit 0), or for the graph service role to be answered by macOS, seen as a service receipt or a live service record (exit 10). Usage: wait-consent.sh <reported>
set -u
reported="$1"; M="$(cd "$(dirname "$0")" && pwd)"; W="$(cd "$M/../.." && pwd)"
for _ in $(seq 1 360); do
  f="$M/next.md"; [ -f "$f" ] || f="$W/ROUND.md"
  n="$(grep -m1 -E '^round:' "$f" 2>/dev/null | tr -dc '0-9')"
  if [ -n "$n" ] && [ "$n" -gt "$reported" ] && [ ! -f "$M/round-$n.md" ]; then echo "round $n in $(basename "$f") at $(date -u +%Y-%m-%dT%H:%M:%SZ)"; exit 0; fi
  r="$(cat "$HOME/Library/Application Support/semio/access/"*/launcher.json 2>/dev/null)"
  case "$r" in *'"state":"waiting"'*|"") ;; *) echo "launcher receipt at $(date -u +%Y-%m-%dT%H:%M:%SZ): $r"; exit 10;; esac
  sleep 60
done
echo "nothing after 6 hours"; exit 2

#!/bin/zsh
# ⏳️ Waits up to 170 s for a new result line (a finished search, a landing line) and prints the short state: wait.sh
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
state() { cat "$GEN"/land/*.out "$GEN"/auto/*.log 2>/dev/null | grep -cE '^STOP|^gate: |^matrix exit| done: '; }
start=$(state)
timeout 170 zsh -c "until [ \"\$(cat '$GEN'/land/*.out '$GEN'/auto/*.log 2>/dev/null | grep -cE '^STOP|^gate: |^matrix exit| done: ')\" != '$start' ]; do sleep 5; done"
"$W/status.sh" 2>&1 | grep -v "^timeout\|^env\|search  | land $" | sed -E 's/hard findings: /h/g; s/\[matrix\] //; s/ assets: 1 done.*//; s/ hard lines//' | cut -c1-170

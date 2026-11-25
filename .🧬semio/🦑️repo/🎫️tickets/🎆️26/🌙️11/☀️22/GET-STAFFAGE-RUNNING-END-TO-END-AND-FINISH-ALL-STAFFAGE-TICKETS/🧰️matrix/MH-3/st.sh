#!/bin/zsh
# 📟️ Waits up to the given seconds for a pattern in a chain output of the third fixer and prints the state of the running chains in a few lines: st.sh <chain file name in $GEN/t3> <pattern> [seconds]
export BATCH=MH-3
source "${0:A:h}/../MR/env.sh"
end=$(( $(date +%s) + ${3:-165} ))
until grep -q -- "$2" "$GEN/t3/$1" 2>/dev/null || [[ $(date +%s) -ge $end ]]; do sleep 5; done
grep -q -- "$2" "$GEN/t3/$1" 2>/dev/null && echo "DONE $(date +%H:%M)" || echo "waiting $(date +%H:%M)"
for f in "$GEN"/t3/final-*.out; do grep -v '^\[capacity\]' "$f" | grep -E " matrix exit | check exit |qsh exit" | tail -2 | cut -c1-200; done
for f in "$GEN"/t3/chain-*.out; do grep -q "qsh exit" "$f" || grep -v '^\[capacity\]' "$f" | grep -E "^== " | tail -1 | cut -c1-150; done

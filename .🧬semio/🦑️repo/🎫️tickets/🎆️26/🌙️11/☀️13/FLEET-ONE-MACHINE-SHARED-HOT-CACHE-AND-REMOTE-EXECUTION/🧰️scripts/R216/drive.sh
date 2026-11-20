#!/bin/zsh
# R216: waits for the 28th minute, then landing A and landing B one after the other (each waits for the window and the lock itself)
SC=${0:A:h}
until mm=$((10#$(date -u +%M))); [ $mm -ge 28 ] && [ $mm -lt 35 ]; do sleep 5; done
echo "== A $(date -u +%H:%M:%S)"; $SC/land.sh 2>&1 | cut -c1-300
echo "== B $(date -u +%H:%M:%S)"; $SC/land-b.sh 2>&1 | cut -c1-300
echo "== end $(date -u +%H:%M:%S)"

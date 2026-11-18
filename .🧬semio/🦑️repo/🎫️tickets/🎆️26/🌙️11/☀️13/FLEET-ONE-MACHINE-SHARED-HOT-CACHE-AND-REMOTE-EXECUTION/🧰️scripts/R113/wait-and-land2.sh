#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
until m=$((10#$(date -u +%M))); (( m <= 3 || (m >= 30 && m <= 33) )); do sleep 5; done
cd /Users/ueli/Documents/semio
for i in 1 2 3 4 5 6 7 8; do "$D/window.sh" && break; sleep 8; done

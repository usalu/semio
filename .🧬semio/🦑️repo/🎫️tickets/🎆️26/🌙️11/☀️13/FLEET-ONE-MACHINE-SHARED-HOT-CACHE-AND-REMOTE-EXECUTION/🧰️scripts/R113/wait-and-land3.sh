#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
until m=$((10#$(date -u +%M))); (( m <= 3 || (m >= 30 && m <= 33) )); do sleep 5; done
cd /Users/ueli/Documents/semio
while m=$((10#$(date -u +%M))); (( m <= 3 || (m >= 30 && m <= 33) )); do "$D/window.sh" >> /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R113/land4.log 2>&1; grep -q "^held" /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R113/land4.log || break; sleep 5; done

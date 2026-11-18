#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
until m=$((10#$(date -u +%M))); (( m >= 30 && m <= 34 )); do sleep 10; done
cd /Users/ueli/Documents/semio && "$D/window.sh"

#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
until [ "$(date -u +%M)" = "00" ] || [ "$(date -u +%M)" = "30" ]; do sleep 1; done
"$D/window.sh"

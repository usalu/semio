#!/bin/zsh
cd "$(dirname "$0")" && python3 modsplit.py list "$1" 2>&1 | awk -F'\t' '$5!="use" || 0' | cut -c1-${2:-130}

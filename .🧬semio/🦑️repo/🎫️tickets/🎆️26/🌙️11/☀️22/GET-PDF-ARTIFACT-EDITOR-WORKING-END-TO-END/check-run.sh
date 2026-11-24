#!/bin/sh
cd "$(dirname "$0")"
log="$1"; shift
sh native-cargo.sh check --keep-going "$@" > "🗑️generated/$log.log" 2>&1
grep -a -A12 "^error" "🗑️generated/$log.log" | grep -a -v "^warning" | head -150 | cut -c1-260
echo "exit; errors: $(grep -ac '^error' "🗑️generated/$log.log")"

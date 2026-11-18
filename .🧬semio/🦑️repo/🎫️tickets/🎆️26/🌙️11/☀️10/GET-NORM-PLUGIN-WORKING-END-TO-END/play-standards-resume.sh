#!/usr/bin/env bash
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STEP_TIMEOUT=20000 "$here/play-standards-native.sh" component >> "$here/🗑️generated/native-driver8.log" 2>&1 || { echo "component failed: $(grep 'rc=' "$here/🗑️generated/native-driver8.log" | tail -1)"; exit 1; }
STEP_TIMEOUT=20000 "$here/play-standards-native.sh" lane >> "$here/🗑️generated/native-lane.log" 2>&1 || { echo "lane failed: $(grep 'rc=' "$here/🗑️generated/native-lane.log" | tail -1)"; exit 1; }
echo "lane done"

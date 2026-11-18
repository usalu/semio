#!/usr/bin/env bash
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STEP_TIMEOUT=20000 "$here/play-standards-native.sh" component >> "$here/🗑️generated/native-driver16.log" 2>&1
STEP_TIMEOUT=20000 "$here/play-standards-native.sh" lane >> "$here/🗑️generated/native-lane8.log" 2>&1
grep -h "FAILED" "$here/🗑️generated/native-driver16.log" "$here/🗑️generated/native-lane8.log"
echo "pipeline done"

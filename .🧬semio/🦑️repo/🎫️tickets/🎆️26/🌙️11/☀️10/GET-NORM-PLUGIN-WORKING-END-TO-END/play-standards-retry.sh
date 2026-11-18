#!/usr/bin/env bash
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
while pgrep -f "play-standards-resume.sh|play-standards-native.sh describe" >/dev/null; do sleep 30; done
STEP_TIMEOUT=20000 "$here/play-standards-native.sh" component >> "$here/🗑️generated/native-driver10.log" 2>&1
STEP_TIMEOUT=20000 "$here/play-standards-native.sh" lane >> "$here/🗑️generated/native-lane2.log" 2>&1
grep -h "FAILED" "$here/🗑️generated/native-driver10.log" "$here/🗑️generated/native-lane2.log"
echo "retry done"

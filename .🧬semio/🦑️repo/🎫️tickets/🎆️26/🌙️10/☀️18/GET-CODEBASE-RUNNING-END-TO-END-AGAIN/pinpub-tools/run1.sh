#!/bin/sh
# 🧪 [pinpub] one single-file bun test run from <root>: usage run1.sh <root> <repo-relative test file> <out-log> [bun test args]; prints the summary and failing names
R="$1"; f="$2"; log="$3"; shift 3
C="$(cd "$(dirname "$0")/.." && pwd)"
: "${SEMIO_TEST_ARTIFACT_DIR:=$R/.cache/test-sandbox/pinpub}"; export SEMIO_TEST_ARTIFACT_DIR
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" "$(dirname "$log")"
cd "$R" || exit 2
nice -n 10 "${BUN:-bun}" test --timeout 170000 "./$f" "$@" > "$log" 2>&1
printf '%s: %s\n' "$f" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' "$log" | tr -s ' \n' ' ')"
/usr/bin/grep -E '^\(fail\)' "$log" | cut -c1-260
